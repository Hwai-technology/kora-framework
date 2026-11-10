//! 3-Tier Ownership, Borrow (&T / &mut T), and Async Arc<T> Escape Analyzer for Kora.
//! Validates memory safety and builds an ownership plan before lowering to Rust.

use std::collections::{HashMap, HashSet};

use kora_ast::{ClassDecl, Expr, Item, Module, Stmt};
use kora_diagnostics::{Diagnostic, DiagnosticBag};
use kora_hir::HirModule;
use kora_span::Span;
use kora_ty::{Ty, TypeCheckOutput};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassMode {
    Copy,
    SharedBorrow,
    MutableBorrow,
    Move,
    ArcPromoted,
}

#[derive(Debug, Clone)]
pub struct BindingOwnership {
    pub name: String,
    pub ty: Ty,
    pub mode: PassMode,
    pub escapes_async: bool,
    pub span: Span,
}

#[derive(Debug, Clone, Default)]
pub struct OwnershipPlan {
    pub bindings: HashMap<String, BindingOwnership>,
    pub promoted_services: HashSet<String>,
}

/// Analyzes memory escape, DI cycles, and ownership classifications.
pub fn analyze_ownership(
    hir: &HirModule,
    types: &TypeCheckOutput,
    diagnostics: &mut DiagnosticBag,
) -> OwnershipPlan {
    let mut plan = OwnershipPlan::default();

    // 1. Mark all service classes & interfaces as Arc-promoted across async boundaries
    for (cname, _) in &types.classes {
        plan.promoted_services.insert(cname.clone());
    }

    // 2. Cycle detection in DI init graph
    check_di_dependency_cycles(hir, diagnostics);

    // 3. Classify local bindings
    classify_module_bindings(&hir.ast, types, &mut plan, diagnostics);

    plan
}

fn check_di_dependency_cycles(hir: &HirModule, diagnostics: &mut DiagnosticBag) {
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    let mut spans: HashMap<String, Span> = HashMap::new();

    for item in &hir.ast.items {
        if let Item::Class(ClassDecl {
            name, init, span, ..
        }) = item
        {
            spans.insert(name.name.clone(), *span);
            let mut deps = Vec::new();
            if let Some(init_decl) = init {
                for p in &init_decl.params {
                    if let kora_ast::TypeExpr::Named { name: dt, .. } = &p.ty {
                        deps.push(dt.name.clone());
                    }
                }
            }
            adj.insert(name.name.clone(), deps);
        }
    }

    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();

    for node in adj.keys() {
        if !visited.contains(node) {
            check_cycle_dfs(
                node,
                &adj,
                &mut visited,
                &mut rec_stack,
                &spans,
                diagnostics,
            );
        }
    }
}

fn check_cycle_dfs(
    curr: &str,
    adj: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    rec_stack: &mut HashSet<String>,
    spans: &HashMap<String, Span>,
    diagnostics: &mut DiagnosticBag,
) -> bool {
    visited.insert(curr.to_string());
    rec_stack.insert(curr.to_string());

    if let Some(neighbors) = adj.get(curr) {
        for n in neighbors {
            if !visited.contains(n) {
                if check_cycle_dfs(n, adj, visited, rec_stack, spans, diagnostics) {
                    return true;
                }
            } else if rec_stack.contains(n) {
                let sp = spans.get(curr).copied().unwrap_or(Span::dummy());
                diagnostics.emit(
                    Diagnostic::error(
                        "E5003",
                        format!("cyclic dependency detected in DI constructor graph between `{curr}` and `{n}`"),
                    )
                    .with_primary_label(sp, "cyclic dependency cycle detected here")
                    .with_help("refactor shared dependencies or decouple into interfaces"),
                );
                return true;
            }
        }
    }

    rec_stack.remove(curr);
    false
}

fn classify_module_bindings(
    ast: &Module,
    _types: &TypeCheckOutput,
    plan: &mut OwnershipPlan,
    _diagnostics: &mut DiagnosticBag,
) {
    for item in &ast.items {
        if let Item::Stmt(stmt) = item {
            scan_stmt(stmt, plan);
        }
    }
}

fn scan_stmt(stmt: &Stmt, plan: &mut OwnershipPlan) {
    if let Stmt::Let {
        name,
        mutable,
        value,
        span,
        ..
    } = stmt
    {
        let escapes_async = value_escapes_async(value);
        let mode = if *mutable {
            PassMode::MutableBorrow
        } else if escapes_async {
            PassMode::ArcPromoted
        } else {
            PassMode::SharedBorrow
        };

        plan.bindings.insert(
            name.name.clone(),
            BindingOwnership {
                name: name.name.clone(),
                ty: Ty::Unknown,
                mode,
                escapes_async,
                span: *span,
            },
        );
    }
}

fn value_escapes_async(expr: &Expr) -> bool {
    matches!(expr, Expr::Closure { .. } | Expr::Await { .. })
}

#[cfg(test)]
mod tests {
    use kora_hir::lower_and_resolve;
    use kora_parser::parse_module;
    use kora_span::FileId;
    use kora_ty::check_module;

    use super::*;

    #[test]
    fn detects_di_dependency_cycle() {
        let src = r#"
class ServiceA {
    b: ServiceB
    init(b: ServiceB) { this.b = b }
}
class ServiceB {
    a: ServiceA
    init(a: ServiceA) { this.a = a }
}
"#;
        let mut bag = DiagnosticBag::new();
        let ast = parse_module(FileId(0), src, &mut bag);
        let hir = lower_and_resolve(ast, &mut bag);
        let types = check_module(&hir, &mut bag);
        let _ = analyze_ownership(&hir, &types, &mut bag);

        assert!(bag.items().iter().any(|d| d.code == "E5003"));
    }
}
