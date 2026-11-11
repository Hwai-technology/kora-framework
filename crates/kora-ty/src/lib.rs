//! Strong static type system, bidirectional type inference, interface conformance,
//! pattern exhaustiveness, and compile-time Dependency Injection verification for Kora.

use std::collections::{HashMap, HashSet};
use std::fmt;

use kora_ast::{
    BinaryOp, Block, ClassDecl, ClosureBody, ClosureParam, ElseBranch, EnumDecl, Expr, FieldDecl,
    FnDecl, FnSig, InterfaceDecl, Item, MatchArm, ModelDecl, Pattern, Stmt, StructDecl, TypeExpr,
    UnaryOp,
};
use kora_diagnostics::{Diagnostic, DiagnosticBag};
use kora_hir::HirModule;
use kora_span::Span;

/// Canonical static type representation in Kora.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    Int,
    Float,
    Bool,
    String,
    Unit,
    Database,
    App,
    Request,
    Response,
    Headers,
    List(Box<Ty>),
    Option(Box<Ty>),
    Result(Box<Ty>, Box<Ty>),
    Struct(String),
    Model(String),
    Class(String),
    Interface(String),
    Enum(String),
    Fn {
        params: Vec<Ty>,
        ret: Box<Ty>,
        is_async: bool,
    },
    /// Type-level receiver for static ORM methods (e.g. `User.all()`, `User.find(1)`).
    MetaModel(String),
    /// Type-level receiver for enum variant access (e.g. `Status.Pending`).
    MetaEnum(String),
    /// Poison/unconstrained placeholder preventing cascading errors.
    Unknown,
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int => write!(f, "Int"),
            Self::Float => write!(f, "Float"),
            Self::Bool => write!(f, "Bool"),
            Self::String => write!(f, "String"),
            Self::Unit => write!(f, "Unit"),
            Self::Database => write!(f, "Database"),
            Self::App => write!(f, "App"),
            Self::Request => write!(f, "Request"),
            Self::Response => write!(f, "Response"),
            Self::Headers => write!(f, "Headers"),
            Self::List(inner) => write!(f, "List<{inner}>"),
            Self::Option(inner) => write!(f, "Option<{inner}>"),
            Self::Result(ok, err) => write!(f, "Result<{ok}, {err}>"),
            Self::Struct(n)
            | Self::Model(n)
            | Self::Class(n)
            | Self::Interface(n)
            | Self::Enum(n) => write!(f, "{n}"),
            Self::MetaModel(n) => write!(f, "model {n}"),
            Self::MetaEnum(n) => write!(f, "enum {n}"),
            Self::Fn {
                params,
                ret,
                is_async,
            } => {
                if *is_async {
                    write!(f, "async ")?;
                }
                write!(f, "(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{p}")?;
                }
                write!(f, ") -> {ret}")
            }
            Self::Unknown => write!(f, "_"),
        }
    }
}

impl Ty {
    pub fn contains_unknown(&self) -> bool {
        match self {
            Self::Unknown => true,
            Self::List(t) | Self::Option(t) => t.contains_unknown(),
            Self::Result(a, b) => a.contains_unknown() || b.contains_unknown(),
            Self::Fn { params, ret, .. } => {
                params.iter().any(Self::contains_unknown) || ret.contains_unknown()
            }
            _ => false,
        }
    }

    pub fn is_meta(&self) -> bool {
        matches!(self, Self::MetaModel(_) | Self::MetaEnum(_))
    }
}

#[derive(Debug, Clone)]
pub struct FunctionTypeInfo {
    pub name: String,
    pub params: Vec<(String, Ty)>,
    pub ret: Ty,
    pub is_async: bool,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ClassTypeInfo {
    pub name: String,
    pub implements: Vec<String>,
    pub fields: HashMap<String, Ty>,
    pub init_params: Vec<(String, Ty)>,
    pub methods: HashMap<String, FunctionTypeInfo>,
}

#[derive(Debug, Clone)]
pub struct InferredBinding {
    pub scope: String,
    pub name: String,
    pub mutable: bool,
    pub ty: Ty,
    pub span: Span,
}

/// Complete output of `kora-ty` static verification.
#[derive(Debug, Clone)]
pub struct TypeCheckOutput {
    pub structs: HashMap<String, HashMap<String, Ty>>,
    pub models: HashMap<String, HashMap<String, Ty>>,
    pub entity_models: HashSet<String>,
    pub model_tables: HashMap<String, String>,
    pub interfaces: HashMap<String, HashMap<String, FunctionTypeInfo>>,
    pub classes: HashMap<String, ClassTypeInfo>,
    pub enums: HashMap<String, Vec<(String, Vec<Ty>)>>,
    pub functions: HashMap<String, FunctionTypeInfo>,
    pub bindings: Vec<InferredBinding>,
}

/// Executes full static type checking, type inference, interface conformance, and DI validation.
pub fn check_module(hir: &HirModule, diagnostics: &mut DiagnosticBag) -> TypeCheckOutput {
    let mut checker = TypeChecker::new(diagnostics);
    checker.collect_declarations(hir);
    checker.verify_interfaces_and_di(hir);
    checker.check_bodies(hir);

    TypeCheckOutput {
        structs: checker.structs,
        models: checker.models,
        entity_models: checker.entity_models,
        model_tables: checker.model_tables,
        interfaces: checker.interfaces,
        classes: checker.classes,
        enums: checker.enums,
        functions: checker.functions,
        bindings: checker.recorded_bindings,
    }
}

struct TypeChecker<'a> {
    structs: HashMap<String, HashMap<String, Ty>>,
    models: HashMap<String, HashMap<String, Ty>>,
    entity_models: HashSet<String>,
    model_tables: HashMap<String, String>,
    interfaces: HashMap<String, HashMap<String, FunctionTypeInfo>>,
    classes: HashMap<String, ClassTypeInfo>,
    enums: HashMap<String, Vec<(String, Vec<Ty>)>>,
    functions: HashMap<String, FunctionTypeInfo>,
    scopes: Vec<HashMap<String, Ty>>,
    current_class: Option<String>,
    current_return_ty: Option<Ty>,
    current_scope_label: String,
    recorded_bindings: Vec<InferredBinding>,
    diagnostics: &'a mut DiagnosticBag,
}

impl<'a> TypeChecker<'a> {
    fn new(diagnostics: &'a mut DiagnosticBag) -> Self {
        Self {
            structs: HashMap::new(),
            models: HashMap::new(),
            entity_models: HashSet::new(),
            model_tables: HashMap::new(),
            interfaces: HashMap::new(),
            classes: HashMap::new(),
            enums: HashMap::new(),
            functions: HashMap::new(),
            scopes: vec![HashMap::new()],
            current_class: None,
            current_return_ty: None,
            current_scope_label: "<top-level>".to_string(),
            recorded_bindings: Vec::new(),
            diagnostics,
        }
    }

    fn collect_declarations(&mut self, hir: &HirModule) {
        // First register nominal type names so recursive/cross references resolve cleanly.
        for item in &hir.ast.items {
            match item {
                Item::Struct(StructDecl { name, .. }) => {
                    self.structs.entry(name.name.clone()).or_default();
                }
                Item::Model(ModelDecl { name, .. }) => {
                    self.models.entry(name.name.clone()).or_default();
                }
                Item::Interface(InterfaceDecl { name, .. }) => {
                    self.interfaces.entry(name.name.clone()).or_default();
                }
                Item::Class(ClassDecl { name, .. }) => {
                    self.classes.insert(
                        name.name.clone(),
                        ClassTypeInfo {
                            name: name.name.clone(),
                            implements: Vec::new(),
                            fields: HashMap::new(),
                            init_params: Vec::new(),
                            methods: HashMap::new(),
                        },
                    );
                }
                Item::Enum(EnumDecl { name, .. }) => {
                    self.enums.entry(name.name.clone()).or_default();
                }
                _ => {}
            }
        }

        // Populate fields, variants, method signatures, and top-level function signatures.
        for item in &hir.ast.items {
            match item {
                Item::Struct(s) => {
                    let map = self.lower_fields(&s.fields);
                    self.structs.insert(s.name.name.clone(), map);
                }
                Item::Model(m) => {
                    let map = self.lower_fields(&m.fields);
                    self.models.insert(m.name.name.clone(), map);

                    let has_entity_attr = m
                        .attributes
                        .iter()
                        .any(|a| a.name.name == "entity" || a.name.name == "table");
                    let has_field_primary = m.fields.iter().any(|f| {
                        f.attributes
                            .iter()
                            .any(|a| a.name.name == "primary" || a.name.name == "autoIncrement")
                    });
                    let is_entity_name = m.name.name.ends_with("Entity");

                    if has_entity_attr || has_field_primary || is_entity_name {
                        self.entity_models.insert(m.name.name.clone());
                        let mut custom_table = None;
                        for attr in &m.attributes {
                            if let ("table", Some(kora_ast::Expr::StringLit { value, .. })) =
                                (attr.name.name.as_str(), attr.args.first())
                            {
                                custom_table = Some(value.clone());
                            }
                        }
                        let tbl = custom_table.unwrap_or_else(|| {
                            let base = m.name.name.trim_end_matches("Entity");
                            format!("{}s", base.to_lowercase())
                        });
                        self.model_tables.insert(m.name.name.clone(), tbl);
                    }
                }
                Item::Interface(iface) => {
                    let mut methods = HashMap::new();
                    for sig in &iface.methods {
                        let info = self.lower_fn_sig(sig);
                        methods.insert(sig.name.name.clone(), info);
                    }
                    self.interfaces.insert(iface.name.name.clone(), methods);
                }
                Item::Class(cls) => {
                    let fields = self.lower_fields(&cls.fields);
                    let implements = cls.implements.iter().map(|i| i.name.clone()).collect();
                    let init_params = cls
                        .init
                        .as_ref()
                        .map(|init| {
                            init.params
                                .iter()
                                .map(|p| (p.name.name.clone(), self.lower_type_expr(&p.ty)))
                                .collect()
                        })
                        .unwrap_or_default();
                    let mut methods = HashMap::new();
                    for m in &cls.methods {
                        let info = self.lower_fn_sig(&m.sig);
                        methods.insert(m.sig.name.name.clone(), info);
                    }
                    self.classes.insert(
                        cls.name.name.clone(),
                        ClassTypeInfo {
                            name: cls.name.name.clone(),
                            implements,
                            fields,
                            init_params,
                            methods,
                        },
                    );
                }
                Item::Enum(en) => {
                    let variants = en
                        .variants
                        .iter()
                        .map(|v| {
                            let payload =
                                v.payload.iter().map(|t| self.lower_type_expr(t)).collect();
                            (v.name.name.clone(), payload)
                        })
                        .collect();
                    self.enums.insert(en.name.name.clone(), variants);
                }
                Item::Fn(func) => {
                    let info = self.lower_fn_sig(&func.sig);
                    self.functions.insert(func.sig.name.name.clone(), info);
                }
                Item::Stmt(_) => {}
                Item::ModuleDecl(_) | Item::Import(_) | Item::Extend(_) => {}
            }
        }
    }

    fn lower_fields(&mut self, fields: &[FieldDecl]) -> HashMap<String, Ty> {
        let mut out = HashMap::new();
        for f in fields {
            out.insert(f.name.name.clone(), self.lower_type_expr(&f.ty));
        }
        out
    }

    fn lower_fn_sig(&mut self, sig: &FnSig) -> FunctionTypeInfo {
        let params = sig
            .params
            .iter()
            .map(|p| (p.name.name.clone(), self.lower_type_expr(&p.ty)))
            .collect();
        let ret = sig
            .return_ty
            .as_ref()
            .map(|t| self.lower_type_expr(t))
            .unwrap_or(Ty::Unit);
        FunctionTypeInfo {
            name: sig.name.name.clone(),
            params,
            ret,
            is_async: sig.is_async,
            span: sig.span,
        }
    }

    fn lower_type_expr(&mut self, texpr: &TypeExpr) -> Ty {
        match texpr {
            TypeExpr::Named { name, args, span } => match name.name.as_str() {
                "Int" => {
                    self.expect_arity(&name.name, args, 0, *span);
                    Ty::Int
                }
                "Float" => {
                    self.expect_arity(&name.name, args, 0, *span);
                    Ty::Float
                }
                "Bool" => {
                    self.expect_arity(&name.name, args, 0, *span);
                    Ty::Bool
                }
                "String" => {
                    self.expect_arity(&name.name, args, 0, *span);
                    Ty::String
                }
                "Unit" => {
                    self.expect_arity(&name.name, args, 0, *span);
                    Ty::Unit
                }
                "Database" => {
                    self.expect_arity(&name.name, args, 0, *span);
                    Ty::Database
                }
                "App" => {
                    self.expect_arity(&name.name, args, 0, *span);
                    Ty::App
                }
                "Request" => {
                    self.expect_arity(&name.name, args, 0, *span);
                    Ty::Request
                }
                "Response" => {
                    self.expect_arity(&name.name, args, 0, *span);
                    Ty::Response
                }
                "Headers" => {
                    self.expect_arity(&name.name, args, 0, *span);
                    Ty::Headers
                }
                "List" => {
                    if self.expect_arity("List", args, 1, *span) {
                        Ty::List(Box::new(self.lower_type_expr(&args[0])))
                    } else {
                        Ty::List(Box::new(Ty::Unknown))
                    }
                }
                "Option" => {
                    if self.expect_arity("Option", args, 1, *span) {
                        Ty::Option(Box::new(self.lower_type_expr(&args[0])))
                    } else {
                        Ty::Option(Box::new(Ty::Unknown))
                    }
                }
                "Result" => {
                    if self.expect_arity("Result", args, 2, *span) {
                        Ty::Result(
                            Box::new(self.lower_type_expr(&args[0])),
                            Box::new(self.lower_type_expr(&args[1])),
                        )
                    } else {
                        Ty::Result(Box::new(Ty::Unknown), Box::new(Ty::Unknown))
                    }
                }
                custom => {
                    self.expect_arity(custom, args, 0, *span);
                    if self.structs.contains_key(custom) {
                        Ty::Struct(custom.to_string())
                    } else if self.models.contains_key(custom) {
                        Ty::Model(custom.to_string())
                    } else if self.classes.contains_key(custom) {
                        Ty::Class(custom.to_string())
                    } else if self.interfaces.contains_key(custom) {
                        Ty::Interface(custom.to_string())
                    } else if self.enums.contains_key(custom) {
                        Ty::Enum(custom.to_string())
                    } else {
                        Ty::Unknown
                    }
                }
            },
            TypeExpr::Function { params, ret, .. } => Ty::Fn {
                params: params.iter().map(|p| self.lower_type_expr(p)).collect(),
                ret: Box::new(self.lower_type_expr(ret)),
                is_async: false,
            },
        }
    }

    fn expect_arity(&mut self, name: &str, args: &[TypeExpr], expected: usize, span: Span) -> bool {
        if args.len() == expected {
            true
        } else {
            self.diagnostics.emit(
                Diagnostic::error(
                    "E4003",
                    format!(
                        "type `{name}` expects {expected} generic type argument(s), found {}",
                        args.len()
                    ),
                )
                .with_primary_label(span, "incorrect generic type arity"),
            );
            false
        }
    }

    fn verify_interfaces_and_di(&mut self, hir: &HirModule) {
        for item in &hir.ast.items {
            let Item::Class(cls) = item else {
                continue;
            };
            let Some(cls_info) = self.classes.get(&cls.name.name).cloned() else {
                continue;
            };

            // 1. Verify compile-time DI constructor (`init`) parameters are injectable.
            if let Some(init) = &cls.init {
                for (param_ast, (_, pty)) in init.params.iter().zip(cls_info.init_params.iter()) {
                    if !Self::is_di_injectable(pty) && *pty != Ty::Unknown {
                        self.diagnostics.emit(
                            Diagnostic::error(
                                "E4012",
                                format!(
                                    "class `{}` constructor parameter `{}` has non-injectable type `{pty}`",
                                    cls.name.name, param_ast.name.name
                                ),
                            )
                            .with_primary_label(
                                param_ast.span,
                                "DI `init` parameters must be a service `class`, `interface`, or `Database`",
                            ),
                        );
                    }
                }
            }

            // 2. Verify every declared `interface` contract is satisfied.
            for iface_ident in &cls.implements {
                let Some(iface_methods) = self.interfaces.get(&iface_ident.name).cloned() else {
                    continue;
                };
                for (mname, req_sig) in &iface_methods {
                    let Some(impl_sig) = cls_info.methods.get(mname) else {
                        self.diagnostics.emit(
                            Diagnostic::error(
                                "E4008",
                                format!(
                                    "class `{}` is missing method `{mname}` required by interface `{}`",
                                    cls.name.name, iface_ident.name
                                ),
                            )
                            .with_primary_label(
                                cls.name.span,
                                format!("missing implementation of `{mname}`"),
                            )
                            .with_secondary_label(req_sig.span, format!("`{mname}` declared here")),
                        );
                        continue;
                    };

                    let params_match = req_sig.params.len() == impl_sig.params.len()
                        && req_sig
                            .params
                            .iter()
                            .zip(impl_sig.params.iter())
                            .all(|((_, a), (_, b))| a == b);
                    let ret_matches = req_sig.ret == impl_sig.ret;
                    let async_matches = req_sig.is_async == impl_sig.is_async;

                    if !params_match || !ret_matches || !async_matches {
                        self.diagnostics.emit(
                            Diagnostic::error(
                                "E4009",
                                format!(
                                    "method `{}.{mname}` has incompatible signature for interface `{}`",
                                    cls.name.name, iface_ident.name
                                ),
                            )
                            .with_primary_label(
                                impl_sig.span,
                                "method signature does not match interface contract",
                            )
                            .with_secondary_label(req_sig.span, "expected signature declared here"),
                        );
                    }
                }
            }
        }
    }

    fn is_di_injectable(ty: &Ty) -> bool {
        matches!(ty, Ty::Class(_) | Ty::Interface(_) | Ty::Database)
    }

    fn check_bodies(&mut self, hir: &HirModule) {
        for item in &hir.ast.items {
            match item {
                Item::Class(cls) => {
                    self.current_class = Some(cls.name.name.clone());
                    if let Some(init) = &cls.init {
                        let cls_info = self.classes[&cls.name.name].clone();
                        self.push_scope();
                        self.current_scope_label = format!("{}.init", cls.name.name);
                        self.current_return_ty = Some(Ty::Unit);
                        for (pname, pty) in &cls_info.init_params {
                            self.define_local(pname.clone(), pty.clone());
                        }
                        self.check_block(&init.body);
                        self.current_return_ty = None;
                        self.pop_scope();
                    }
                    for method in &cls.methods {
                        let sig =
                            self.classes[&cls.name.name].methods[&method.sig.name.name].clone();
                        self.check_fn_decl(
                            method,
                            &sig,
                            &format!("{}.{}", cls.name.name, sig.name),
                        );
                    }
                    self.current_class = None;
                }
                Item::Fn(func) => {
                    let sig = self.functions[&func.sig.name.name].clone();
                    self.check_fn_decl(func, &sig, &sig.name.clone());
                }
                Item::Stmt(stmt) => {
                    self.current_scope_label = "<top-level>".to_string();
                    self.check_stmt(stmt);
                }
                Item::Struct(_) | Item::Model(_) | Item::Interface(_) | Item::Enum(_) => {}
                Item::ModuleDecl(_) | Item::Import(_) | Item::Extend(_) => {}
            }
        }
    }

    fn check_fn_decl(&mut self, func: &FnDecl, sig: &FunctionTypeInfo, scope_label: &str) {
        self.push_scope();
        self.current_scope_label = scope_label.to_string();
        self.current_return_ty = Some(sig.ret.clone());

        for (pname, pty) in &sig.params {
            self.define_local(pname.clone(), pty.clone());
        }

        self.check_block(&func.body);

        if sig.ret != Ty::Unit && !sig.ret.contains_unknown() && !block_always_returns(&func.body) {
            self.diagnostics.emit(
                Diagnostic::error(
                    "E4011",
                    format!(
                        "function `{}` declares return type `{}`, but not all paths return a value",
                        sig.name, sig.ret
                    ),
                )
                .with_primary_label(
                    func.sig.span,
                    format!("expected `{}` return on all paths", sig.ret),
                ),
            );
        }

        self.current_return_ty = None;
        self.pop_scope();
    }

    fn check_block(&mut self, block: &Block) {
        for stmt in &block.stmts {
            self.check_stmt(stmt);
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let {
                name,
                mutable,
                ty,
                value,
                span,
            } => {
                let declared_ty = ty.as_ref().map(|t| self.lower_type_expr(t));
                let inferred_rhs = self.check_expr(value, declared_ty.as_ref());

                if inferred_rhs.is_meta() {
                    self.diagnostics.emit(
                        Diagnostic::error(
                            "E4001",
                            format!("cannot bind type name `{inferred_rhs}` as a runtime value"),
                        )
                        .with_primary_label(value.span(), "expected a value expression"),
                    );
                }

                let final_ty = if let Some(expected) = declared_ty {
                    if !self.is_assignable(&expected, &inferred_rhs) {
                        self.emit_type_mismatch(&expected, &inferred_rhs, value.span());
                    }
                    expected
                } else {
                    if inferred_rhs.contains_unknown() && inferred_rhs != Ty::Unknown {
                        self.diagnostics.emit(
                            Diagnostic::error(
                                "E4002",
                                format!(
                                    "cannot infer complete static type `{inferred_rhs}` for `{}` without annotation",
                                    name.name
                                ),
                            )
                            .with_primary_label(*span, "add an explicit type annotation here")
                            .with_help(format!("try `let {}: <Type> = ...`", name.name)),
                        );
                    }
                    inferred_rhs
                };

                self.recorded_bindings.push(InferredBinding {
                    scope: self.current_scope_label.clone(),
                    name: name.name.clone(),
                    mutable: *mutable,
                    ty: final_ty.clone(),
                    span: name.span,
                });
                self.define_local(name.name.clone(), final_ty);
            }
            Stmt::Return { value, span } => {
                let expected = self.current_return_ty.clone().unwrap_or(Ty::Unit);
                match value {
                    Some(expr) => {
                        let actual = self.check_expr(expr, Some(&expected));
                        if self.current_return_ty.is_some()
                            && !self.is_assignable(&expected, &actual)
                        {
                            self.emit_type_mismatch(&expected, &actual, expr.span());
                        }
                    }
                    None => {
                        if self.current_return_ty.is_some() && expected != Ty::Unit {
                            self.emit_type_mismatch(&expected, &Ty::Unit, *span);
                        }
                    }
                }
            }
            Stmt::If {
                cond,
                then_branch,
                else_branch,
                ..
            } => {
                let cond_ty = self.check_expr(cond, Some(&Ty::Bool));
                if !self.is_assignable(&Ty::Bool, &cond_ty) {
                    self.emit_type_mismatch(&Ty::Bool, &cond_ty, cond.span());
                }
                self.push_scope();
                self.check_block(then_branch);
                self.pop_scope();
                if let Some(eb) = else_branch {
                    match eb {
                        ElseBranch::Block(blk) => {
                            self.push_scope();
                            self.check_block(blk);
                            self.pop_scope();
                        }
                        ElseBranch::If(nested) => self.check_stmt(nested),
                    }
                }
            }
            Stmt::While { cond, body, .. } => {
                let cond_ty = self.check_expr(cond, Some(&Ty::Bool));
                if !self.is_assignable(&Ty::Bool, &cond_ty) {
                    self.emit_type_mismatch(&Ty::Bool, &cond_ty, cond.span());
                }
                self.push_scope();
                self.check_block(body);
                self.pop_scope();
            }
            Stmt::Const {
                name,
                ty,
                value,
                span: _,
            } => {
                let declared_ty = ty.as_ref().map(|t| self.lower_type_expr(t));
                let inferred_rhs = self.check_expr(value, declared_ty.as_ref());
                let final_ty = if let Some(expected) = declared_ty {
                    if !self.is_assignable(&expected, &inferred_rhs) {
                        self.emit_type_mismatch(&expected, &inferred_rhs, value.span());
                    }
                    expected
                } else {
                    inferred_rhs
                };
                self.recorded_bindings.push(InferredBinding {
                    scope: self.current_scope_label.clone(),
                    name: name.name.clone(),
                    mutable: false,
                    ty: final_ty.clone(),
                    span: name.span,
                });
                self.define_local(name.name.clone(), final_ty);
            }
            Stmt::For {
                var, iter, body, ..
            } => {
                let iter_ty = self.check_expr(iter, None);
                let elem_ty = match iter_ty {
                    Ty::List(inner) => *inner,
                    _ => Ty::Int,
                };
                self.push_scope();
                self.define_local(var.name.clone(), elem_ty);
                self.check_block(body);
                self.pop_scope();
            }
            Stmt::Break { .. } | Stmt::Continue { .. } => {}
            Stmt::CompoundAssign { target, value, .. } => {
                let target_ty = self.check_expr(target, None);
                let _ = self.check_expr(value, Some(&target_ty));
            }
            Stmt::Throw { value, .. } => {
                let _ = self.check_expr(value, None);
            }
            Stmt::TryCatch {
                try_block,
                catch_param,
                catch_block,
                finally_block,
                ..
            } => {
                self.push_scope();
                self.check_block(try_block);
                self.pop_scope();

                self.push_scope();
                if let Some(param) = catch_param {
                    self.define_local(param.name.clone(), Ty::String);
                }
                self.check_block(catch_block);
                self.pop_scope();

                if let Some(fin) = finally_block {
                    self.push_scope();
                    self.check_block(fin);
                    self.pop_scope();
                }
            }
            Stmt::Expr { expr, .. } => {
                let _ = self.check_expr(expr, None);
            }
        }
    }

    fn check_expr(&mut self, expr: &Expr, expected: Option<&Ty>) -> Ty {
        match expr {
            Expr::IntLit { .. } => Ty::Int,
            Expr::FloatLit { .. } => Ty::Float,
            Expr::StringLit { .. } => Ty::String,
            Expr::BoolLit { .. } => Ty::Bool,
            Expr::Null { .. } => {
                if let Some(Ty::Option(inner)) = expected {
                    Ty::Option(inner.clone())
                } else {
                    Ty::Option(Box::new(Ty::Unknown))
                }
            }
            Expr::This { .. } => self
                .current_class
                .as_ref()
                .map(|c| Ty::Class(c.clone()))
                .unwrap_or(Ty::Unknown),
            Expr::Ident(id) => {
                if let Some(local_ty) = self.lookup_local(&id.name) {
                    return local_ty.clone();
                }
                if id.name == "None" {
                    if let Some(Ty::Option(inner)) = expected {
                        return Ty::Option(inner.clone());
                    }
                    return Ty::Option(Box::new(Ty::Unknown));
                }
                if let Some(fn_info) = self.functions.get(&id.name) {
                    return Ty::Fn {
                        params: fn_info.params.iter().map(|(_, t)| t.clone()).collect(),
                        ret: Box::new(fn_info.ret.clone()),
                        is_async: fn_info.is_async,
                    };
                }
                if self.models.contains_key(&id.name) {
                    return Ty::MetaModel(id.name.clone());
                }
                if self.enums.contains_key(&id.name) {
                    return Ty::MetaEnum(id.name.clone());
                }
                Ty::Unknown
            }
            Expr::Unary {
                op, expr: inner, ..
            } => {
                let inner_ty = self.check_expr(inner, None);
                match op {
                    UnaryOp::Not => {
                        if !self.is_assignable(&Ty::Bool, &inner_ty) {
                            self.emit_type_mismatch(&Ty::Bool, &inner_ty, inner.span());
                        }
                        Ty::Bool
                    }
                    UnaryOp::Neg => {
                        if inner_ty != Ty::Int && inner_ty != Ty::Float && inner_ty != Ty::Unknown {
                            self.diagnostics.emit(
                                Diagnostic::error(
                                    "E4001",
                                    format!(
                                        "unary `-` requires `Int` or `Float`, found `{inner_ty}`"
                                    ),
                                )
                                .with_primary_label(inner.span(), "expected numeric value"),
                            );
                            Ty::Unknown
                        } else {
                            inner_ty
                        }
                    }
                }
            }
            Expr::Binary { op, lhs, rhs, span } => {
                let lty = self.check_expr(lhs, None);
                let rty = self.check_expr(rhs, Some(&lty));
                self.check_binary_op(*op, &lty, &rty, *span)
            }
            Expr::Assign { target, value, .. } => {
                let target_ty = self.check_expr(target, None);
                let val_ty = self.check_expr(value, Some(&target_ty));
                if !self.is_assignable(&target_ty, &val_ty) {
                    self.emit_type_mismatch(&target_ty, &val_ty, value.span());
                }
                Ty::Unit
            }
            Expr::StructLit { name, fields, span } => {
                let (expected_fields, out_ty) = if let Some(sf) = self.structs.get(&name.name) {
                    (sf.clone(), Ty::Struct(name.name.clone()))
                } else if let Some(mf) = self.models.get(&name.name) {
                    (mf.clone(), Ty::Model(name.name.clone()))
                } else {
                    self.diagnostics.emit(
                        Diagnostic::error(
                            "E4004",
                            format!("`{}` is not a `struct` or `model`", name.name),
                        )
                        .with_primary_label(name.span, "cannot instantiate with `{ ... }` literal"),
                    );
                    return Ty::Unknown;
                };

                let mut seen = HashSet::new();
                for init in fields {
                    if !seen.insert(init.name.name.clone()) {
                        self.diagnostics.emit(
                            Diagnostic::error(
                                "E4006",
                                format!(
                                    "field `{}` initialized more than once in `{}`",
                                    init.name.name, name.name
                                ),
                            )
                            .with_primary_label(init.name.span, "duplicate field initializer"),
                        );
                        continue;
                    }
                    match expected_fields.get(&init.name.name) {
                        Some(field_ty) => {
                            let vty = self.check_expr(&init.value, Some(field_ty));
                            if !self.is_assignable(field_ty, &vty) {
                                self.emit_type_mismatch(field_ty, &vty, init.value.span());
                            }
                        }
                        None => {
                            let _ = self.check_expr(&init.value, None);
                            self.diagnostics.emit(
                                Diagnostic::error(
                                    "E4005",
                                    format!(
                                        "unknown field `{}` on `{}`",
                                        init.name.name, name.name
                                    ),
                                )
                                .with_primary_label(init.name.span, "field does not exist"),
                            );
                        }
                    }
                }

                for req_field in expected_fields.keys() {
                    if !seen.contains(req_field) {
                        self.diagnostics.emit(
                            Diagnostic::error(
                                "E4004",
                                format!(
                                    "missing required field `{req_field}` in `{}` literal",
                                    name.name
                                ),
                            )
                            .with_primary_label(*span, format!("missing `{req_field}`")),
                        );
                    }
                }

                out_ty
            }
            Expr::ListLit { elements, span } => {
                let expected_elem = match expected {
                    Some(Ty::List(inner)) => Some(inner.as_ref()),
                    _ => None,
                };
                if elements.is_empty() {
                    return Ty::List(Box::new(expected_elem.cloned().unwrap_or(Ty::Unknown)));
                }
                let first_ty = self.check_expr(&elements[0], expected_elem);
                for el in &elements[1..] {
                    let ety = self.check_expr(el, Some(&first_ty));
                    if !self.is_assignable(&first_ty, &ety) {
                        self.diagnostics.emit(
                            Diagnostic::error(
                                "E4001",
                                format!(
                                    "heterogeneous list element: expected `{first_ty}`, found `{ety}`"
                                ),
                            )
                            .with_primary_label(*span, "all list elements must share the same type"),
                        );
                    }
                }
                Ty::List(Box::new(first_ty))
            }
            Expr::FieldAccess {
                receiver, field, ..
            } => {
                let recv_ty = self.check_expr(receiver, None);
                self.check_field_access(&recv_ty, &field.name, field.span)
            }
            Expr::Index {
                receiver, index, ..
            } => {
                let recv_ty = self.check_expr(receiver, None);
                let idx_ty = self.check_expr(index, Some(&Ty::Int));
                if !self.is_assignable(&Ty::Int, &idx_ty) {
                    self.emit_type_mismatch(&Ty::Int, &idx_ty, index.span());
                }
                match recv_ty {
                    Ty::List(inner) => *inner,
                    Ty::Unknown => Ty::Unknown,
                    other => {
                        self.diagnostics.emit(
                            Diagnostic::error(
                                "E4001",
                                format!("cannot index into value of type `{other}`"),
                            )
                            .with_primary_label(receiver.span(), "expected `List<T>`"),
                        );
                        Ty::Unknown
                    }
                }
            }
            Expr::Call { callee, args, span } => self.check_call(callee, args, expected, *span),
            Expr::Closure {
                params,
                return_ty,
                body,
                span,
            } => self.check_closure(params, return_ty.as_ref(), body, expected, *span),
            Expr::Match {
                scrutinee,
                arms,
                span,
            } => self.check_match(scrutinee, arms, expected, *span),
            Expr::Await { expr: inner, .. } => self.check_expr(inner, expected),
        }
    }

    fn check_binary_op(&mut self, op: BinaryOp, lty: &Ty, rty: &Ty, span: Span) -> Ty {
        if *lty == Ty::Unknown || *rty == Ty::Unknown {
            return match op {
                BinaryOp::Eq
                | BinaryOp::Neq
                | BinaryOp::Lt
                | BinaryOp::Lte
                | BinaryOp::Gt
                | BinaryOp::Gte
                | BinaryOp::And
                | BinaryOp::Or => Ty::Bool,
                _ => Ty::Unknown,
            };
        }

        match op {
            BinaryOp::Add => {
                if *lty == Ty::Int && *rty == Ty::Int {
                    Ty::Int
                } else if *lty == Ty::Float && *rty == Ty::Float {
                    Ty::Float
                } else if *lty == Ty::String && *rty == Ty::String {
                    Ty::String
                } else {
                    self.emit_binary_error(op, lty, rty, span);
                    Ty::Unknown
                }
            }
            BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                if *lty == Ty::Int && *rty == Ty::Int {
                    Ty::Int
                } else if *lty == Ty::Float && *rty == Ty::Float && op != BinaryOp::Mod {
                    Ty::Float
                } else {
                    self.emit_binary_error(op, lty, rty, span);
                    Ty::Unknown
                }
            }
            BinaryOp::Lt | BinaryOp::Lte | BinaryOp::Gt | BinaryOp::Gte => {
                if (*lty == Ty::Int && *rty == Ty::Int) || (*lty == Ty::Float && *rty == Ty::Float)
                {
                    Ty::Bool
                } else {
                    self.emit_binary_error(op, lty, rty, span);
                    Ty::Bool
                }
            }
            BinaryOp::Eq | BinaryOp::Neq => {
                if self.is_assignable(lty, rty) || self.is_assignable(rty, lty) {
                    Ty::Bool
                } else {
                    self.emit_binary_error(op, lty, rty, span);
                    Ty::Bool
                }
            }
            BinaryOp::And | BinaryOp::Or => {
                if *lty == Ty::Bool && *rty == Ty::Bool {
                    Ty::Bool
                } else {
                    self.emit_binary_error(op, lty, rty, span);
                    Ty::Bool
                }
            }
            BinaryOp::Coalesce => match lty {
                Ty::Option(inner) => {
                    if self.is_assignable(inner, rty) {
                        (**inner).clone()
                    } else {
                        self.emit_binary_error(op, lty, rty, span);
                        (**inner).clone()
                    }
                }
                _ => lty.clone(),
            },
            BinaryOp::Pipe => match rty {
                Ty::Fn { ret, .. } => (**ret).clone(),
                _ => rty.clone(),
            },
            BinaryOp::Range => {
                if *lty == Ty::Int && *rty == Ty::Int {
                    Ty::List(Box::new(Ty::Int))
                } else {
                    self.emit_binary_error(op, lty, rty, span);
                    Ty::List(Box::new(Ty::Int))
                }
            }
        }
    }

    fn emit_binary_error(&mut self, op: BinaryOp, lty: &Ty, rty: &Ty, span: Span) {
        self.diagnostics.emit(
            Diagnostic::error(
                "E4001",
                format!(
                    "cannot apply operator `{}` to `{lty}` and `{rty}`",
                    op.symbol()
                ),
            )
            .with_primary_label(span, "incompatible operand types"),
        );
    }

    fn check_field_access(&mut self, recv_ty: &Ty, field: &str, span: Span) -> Ty {
        match recv_ty {
            Ty::Unknown => Ty::Unknown,
            Ty::Struct(s) => self.lookup_member_field(s, &self.structs.clone(), field, span),
            Ty::Model(m) => self.lookup_member_field(m, &self.models.clone(), field, span),
            Ty::Class(c) => {
                if let Some(cls) = self.classes.get(c) {
                    if let Some(fty) = cls.fields.get(field) {
                        return fty.clone();
                    }
                    if let Some(m) = cls.methods.get(field) {
                        return Ty::Fn {
                            params: m.params.iter().map(|(_, t)| t.clone()).collect(),
                            ret: Box::new(m.ret.clone()),
                            is_async: m.is_async,
                        };
                    }
                }
                self.emit_unknown_member(recv_ty, field, span)
            }
            Ty::Interface(iface) => {
                if let Some(methods) = self.interfaces.get(iface)
                    && let Some(m) = methods.get(field)
                {
                    return Ty::Fn {
                        params: m.params.iter().map(|(_, t)| t.clone()).collect(),
                        ret: Box::new(m.ret.clone()),
                        is_async: m.is_async,
                    };
                }
                self.emit_unknown_member(recv_ty, field, span)
            }
            Ty::MetaModel(m) => match field {
                "all" => Ty::Fn {
                    params: Vec::new(),
                    ret: Box::new(Ty::List(Box::new(Ty::Model(m.clone())))),
                    is_async: false,
                },
                "find" => Ty::Fn {
                    params: vec![Ty::Int],
                    ret: Box::new(Ty::Option(Box::new(Ty::Model(m.clone())))),
                    is_async: false,
                },
                _ => self.emit_unknown_member(recv_ty, field, span),
            },
            Ty::MetaEnum(en) => {
                if let Some(variants) = self.enums.get(en)
                    && let Some((_, payload)) = variants.iter().find(|(vn, _)| vn == field)
                {
                    if payload.is_empty() {
                        return Ty::Enum(en.clone());
                    }
                    return Ty::Fn {
                        params: payload.clone(),
                        ret: Box::new(Ty::Enum(en.clone())),
                        is_async: false,
                    };
                }
                self.emit_unknown_member(recv_ty, field, span)
            }
            Ty::Request => match field {
                "path" | "body" | "method" | "ip" | "url" => Ty::String,
                "headers" => Ty::Headers,
                "header" => Ty::Fn {
                    params: vec![Ty::String],
                    ret: Box::new(Ty::Option(Box::new(Ty::String))),
                    is_async: false,
                },
                "query" | "param" => Ty::Fn {
                    params: vec![Ty::String],
                    ret: Box::new(Ty::Option(Box::new(Ty::String))),
                    is_async: false,
                },
                _ => self.emit_unknown_member(recv_ty, field, span),
            },
            Ty::Headers => match field {
                "get" => Ty::Fn {
                    params: vec![Ty::String],
                    ret: Box::new(Ty::Option(Box::new(Ty::String))),
                    is_async: false,
                },
                "has" => Ty::Fn {
                    params: vec![Ty::String],
                    ret: Box::new(Ty::Bool),
                    is_async: false,
                },
                "set" => Ty::Fn {
                    params: vec![Ty::String, Ty::String],
                    ret: Box::new(Ty::Unit),
                    is_async: false,
                },
                _ => self.emit_unknown_member(recv_ty, field, span),
            },
            Ty::Response => match field {
                "statusCode" => Ty::Int,
                "body" => Ty::String,
                "status" => Ty::Fn {
                    params: vec![Ty::Int],
                    ret: Box::new(Ty::Response),
                    is_async: false,
                },
                "json" | "text" => Ty::Fn {
                    params: vec![Ty::Unknown],
                    ret: Box::new(Ty::Response),
                    is_async: false,
                },
                "header" => Ty::Fn {
                    params: vec![Ty::String, Ty::String],
                    ret: Box::new(Ty::Response),
                    is_async: false,
                },
                _ => self.emit_unknown_member(recv_ty, field, span),
            },
            Ty::Database => match field {
                "query" | "execute" => Ty::Fn {
                    params: vec![Ty::String],
                    ret: Box::new(Ty::Result(Box::new(Ty::Int), Box::new(Ty::String))),
                    is_async: false,
                },
                "findUnique" => Ty::Fn {
                    params: vec![Ty::String, Ty::String],
                    ret: Box::new(Ty::Option(Box::new(Ty::Unknown))),
                    is_async: false,
                },
                "isConnected" => Ty::Fn {
                    params: Vec::new(),
                    ret: Box::new(Ty::Bool),
                    is_async: false,
                },
                "begin" | "commit" | "rollback" | "close" => Ty::Fn {
                    params: Vec::new(),
                    ret: Box::new(Ty::Bool),
                    is_async: false,
                },
                _ => self.emit_unknown_member(recv_ty, field, span),
            },
            Ty::String => match field {
                "len" | "length" => Ty::Fn {
                    params: Vec::new(),
                    ret: Box::new(Ty::Int),
                    is_async: false,
                },
                "contains" | "startsWith" | "endsWith" => Ty::Fn {
                    params: vec![Ty::String],
                    ret: Box::new(Ty::Bool),
                    is_async: false,
                },
                "trim" | "toLower" | "toUpper" => Ty::Fn {
                    params: Vec::new(),
                    ret: Box::new(Ty::String),
                    is_async: false,
                },
                _ => self.emit_unknown_member(recv_ty, field, span),
            },
            Ty::Int => match field {
                "toString" => Ty::Fn {
                    params: Vec::new(),
                    ret: Box::new(Ty::String),
                    is_async: false,
                },
                _ => self.emit_unknown_member(recv_ty, field, span),
            },
            Ty::Float => match field {
                "toString" => Ty::Fn {
                    params: Vec::new(),
                    ret: Box::new(Ty::String),
                    is_async: false,
                },
                _ => self.emit_unknown_member(recv_ty, field, span),
            },
            Ty::List(elem) => match field {
                "len" | "length" | "count" => Ty::Fn {
                    params: Vec::new(),
                    ret: Box::new(Ty::Int),
                    is_async: false,
                },
                "isEmpty" => Ty::Fn {
                    params: Vec::new(),
                    ret: Box::new(Ty::Bool),
                    is_async: false,
                },
                "contains" => Ty::Fn {
                    params: vec![*elem.clone()],
                    ret: Box::new(Ty::Bool),
                    is_async: false,
                },
                "push" => Ty::Fn {
                    params: vec![*elem.clone()],
                    ret: Box::new(Ty::Unit),
                    is_async: false,
                },
                _ => self.emit_unknown_member(recv_ty, field, span),
            },
            _ => self.emit_unknown_member(recv_ty, field, span),
        }
    }

    fn lookup_member_field(
        &mut self,
        owner: &str,
        registry: &HashMap<String, HashMap<String, Ty>>,
        field: &str,
        span: Span,
    ) -> Ty {
        if let Some(fields) = registry.get(owner)
            && let Some(fty) = fields.get(field)
        {
            return fty.clone();
        }
        self.diagnostics.emit(
            Diagnostic::error("E4005", format!("no field `{field}` on `{owner}`"))
                .with_primary_label(span, "unknown field"),
        );
        Ty::Unknown
    }

    fn emit_unknown_member(&mut self, recv_ty: &Ty, field: &str, span: Span) -> Ty {
        self.diagnostics.emit(
            Diagnostic::error(
                "E4005",
                format!("no field or method `{field}` found on `{recv_ty}`"),
            )
            .with_primary_label(span, "unknown member"),
        );
        Ty::Unknown
    }

    fn check_call(
        &mut self,
        callee: &Expr,
        args: &[Expr],
        expected: Option<&Ty>,
        span: Span,
    ) -> Ty {
        if let Expr::Ident(id) = callee {
            match id.name.as_str() {
                "Some" => {
                    if args.len() != 1 {
                        self.emit_arg_count("Some", 1, args.len(), span);
                        return Ty::Option(Box::new(Ty::Unknown));
                    }
                    let inner_exp = match expected {
                        Some(Ty::Option(inner)) => Some(inner.as_ref()),
                        _ => None,
                    };
                    let inner = self.check_expr(&args[0], inner_exp);
                    return Ty::Option(Box::new(inner));
                }
                "Ok" => {
                    if args.len() != 1 {
                        self.emit_arg_count("Ok", 1, args.len(), span);
                        return Ty::Result(Box::new(Ty::Unknown), Box::new(Ty::Unknown));
                    }
                    let (ok_exp, err_ty) = match expected {
                        Some(Ty::Result(ok, err)) => (Some(ok.as_ref()), (**err).clone()),
                        _ => (None, Ty::Unknown),
                    };
                    let ok_ty = self.check_expr(&args[0], ok_exp);
                    return Ty::Result(Box::new(ok_ty), Box::new(err_ty));
                }
                "Err" => {
                    if args.len() != 1 {
                        self.emit_arg_count("Err", 1, args.len(), span);
                        return Ty::Result(Box::new(Ty::Unknown), Box::new(Ty::Unknown));
                    }
                    let (ok_ty, err_exp) = match expected {
                        Some(Ty::Result(ok, err)) => ((**ok).clone(), Some(err.as_ref())),
                        _ => (Ty::Unknown, None),
                    };
                    let err_ty = self.check_expr(&args[0], err_exp);
                    return Ty::Result(Box::new(ok_ty), Box::new(err_ty));
                }
                "print" => {
                    if args.len() != 1 {
                        self.emit_arg_count("print", 1, args.len(), span);
                        return Ty::Unit;
                    }
                    let aty = self.check_expr(&args[0], None);
                    if !matches!(
                        aty,
                        Ty::String | Ty::Int | Ty::Float | Ty::Bool | Ty::Unknown
                    ) {
                        self.diagnostics.emit(
                            Diagnostic::error(
                                "E4001",
                                format!(
                                    "`print` expects `String`, `Int`, `Float`, or `Bool`, found `{aty}`"
                                ),
                            )
                            .with_primary_label(args[0].span(), "not printable directly"),
                        );
                    }
                    return Ty::Unit;
                }
                "run" => {
                    if !args.is_empty() {
                        self.emit_arg_count("run", 0, args.len(), span);
                    }
                    return Ty::App;
                }
                "send" => {
                    if args.len() != 1 {
                        self.emit_arg_count("send", 1, args.len(), span);
                    } else {
                        let _ = self.check_expr(&args[0], None);
                    }
                    return Ty::Response;
                }
                "get" | "post" | "put" | "delete" => {
                    if args.len() != 2 {
                        self.emit_arg_count(&id.name, 2, args.len(), span);
                        return Ty::Unit;
                    }
                    let path_ty = self.check_expr(&args[0], Some(&Ty::String));
                    if !self.is_assignable(&Ty::String, &path_ty) {
                        self.emit_type_mismatch(&Ty::String, &path_ty, args[0].span());
                    }
                    let handler_ty = self.check_expr(&args[1], None);
                    if !matches!(handler_ty, Ty::Fn { .. } | Ty::Unknown) {
                        self.diagnostics.emit(
                            Diagnostic::error(
                                "E4001",
                                format!(
                                    "route `{}` expects a handler closure or function, found `{handler_ty}`",
                                    id.name
                                ),
                            )
                            .with_primary_label(args[1].span(), "expected handler function/closure"),
                        );
                    }
                    return Ty::Unit;
                }
                // Direct class constructor call: `UserService(db)`
                cls_name if self.classes.contains_key(cls_name) => {
                    let init_params = self.classes[cls_name].init_params.clone();
                    if args.len() != init_params.len() {
                        self.emit_arg_count(cls_name, init_params.len(), args.len(), span);
                    }
                    for (arg, (_, exp_ty)) in args.iter().zip(init_params.iter()) {
                        let actual = self.check_expr(arg, Some(exp_ty));
                        if !self.is_assignable(exp_ty, &actual) {
                            self.emit_type_mismatch(exp_ty, &actual, arg.span());
                        }
                    }
                    return Ty::Class(cls_name.to_string());
                }
                _ => {}
            }
        }

        let callee_ty = self.check_expr(callee, None);
        match callee_ty {
            Ty::Fn { params, ret, .. } => {
                if args.len() != params.len() {
                    self.emit_arg_count("function", params.len(), args.len(), span);
                }
                for (arg, exp_ty) in args.iter().zip(params.iter()) {
                    let actual = self.check_expr(arg, Some(exp_ty));
                    if !self.is_assignable(exp_ty, &actual) {
                        self.emit_type_mismatch(exp_ty, &actual, arg.span());
                    }
                }
                *ret
            }
            Ty::Unknown => {
                for a in args {
                    let _ = self.check_expr(a, None);
                }
                Ty::Unknown
            }
            other => {
                self.diagnostics.emit(
                    Diagnostic::error(
                        "E4007",
                        format!("expression of type `{other}` is not callable"),
                    )
                    .with_primary_label(callee.span(), "not a function or method"),
                );
                Ty::Unknown
            }
        }
    }

    fn check_closure(
        &mut self,
        params: &[ClosureParam],
        return_ty: Option<&TypeExpr>,
        body: &ClosureBody,
        expected: Option<&Ty>,
        span: Span,
    ) -> Ty {
        let expected_fn = match expected {
            Some(Ty::Fn { params, ret, .. }) => Some((params.clone(), (**ret).clone())),
            _ => None,
        };

        let mut param_tys = Vec::with_capacity(params.len());
        self.push_scope();
        for (idx, p) in params.iter().enumerate() {
            let pty = if let Some(ann) = &p.ty {
                self.lower_type_expr(ann)
            } else if let Some((exp_params, _)) = &expected_fn
                && let Some(et) = exp_params.get(idx)
            {
                et.clone()
            } else if p.name.name == "request" || p.name.name == "req" {
                Ty::Request
            } else {
                self.diagnostics.emit(
                    Diagnostic::error(
                        "E4002",
                        format!(
                            "cannot infer type of closure parameter `{}`; add a type annotation",
                            p.name.name
                        ),
                    )
                    .with_primary_label(p.span, "explicit parameter type required here"),
                );
                Ty::Unknown
            };
            self.define_local(p.name.name.clone(), pty.clone());
            param_tys.push(pty);
        }

        let explicit_ret = return_ty
            .map(|rt| self.lower_type_expr(rt))
            .or_else(|| expected_fn.map(|(_, r)| r));

        let prev_ret = self.current_return_ty.clone();
        let inferred_ret = match body {
            ClosureBody::Expr(expr) => {
                let r = self.check_expr(expr, explicit_ret.as_ref());
                if let Some(exp) = &explicit_ret
                    && !self.is_assignable(exp, &r)
                {
                    self.emit_type_mismatch(exp, &r, expr.span());
                }
                explicit_ret.unwrap_or(r)
            }
            ClosureBody::Block(blk) => {
                let inferred = explicit_ret
                    .clone()
                    .unwrap_or_else(|| infer_block_return_hint(blk));
                self.current_return_ty = if inferred == Ty::Unknown {
                    None
                } else {
                    Some(inferred.clone())
                };
                let inferred_from_returns = self.check_closure_block(blk, explicit_ret.as_ref());
                if let Some(exp) = &explicit_ret
                    && *exp != Ty::Unit
                    && !exp.contains_unknown()
                    && !block_always_returns(blk)
                {
                    self.diagnostics.emit(
                        Diagnostic::error(
                            "E4011",
                            format!("closure must return `{exp}` on all paths"),
                        )
                        .with_primary_label(span, "missing return value"),
                    );
                }
                explicit_ret.unwrap_or(inferred_from_returns)
            }
        };
        self.current_return_ty = prev_ret;
        self.pop_scope();

        Ty::Fn {
            params: param_tys,
            ret: Box::new(inferred_ret),
            is_async: false,
        }
    }

    fn check_closure_block(&mut self, blk: &Block, explicit_ret: Option<&Ty>) -> Ty {
        let mut found_return: Option<Ty> = None;
        for stmt in &blk.stmts {
            if let Stmt::Return { value, span } = stmt {
                let rty = match value {
                    Some(v) => self.check_expr(v, explicit_ret.or(found_return.as_ref())),
                    None => Ty::Unit,
                };
                if let Some(exp) = explicit_ret {
                    if !self.is_assignable(exp, &rty) {
                        self.emit_type_mismatch(exp, &rty, *span);
                    }
                } else if let Some(prev) = &found_return
                    && !self.is_assignable(prev, &rty)
                    && !self.is_assignable(&rty, prev)
                {
                    self.emit_type_mismatch(prev, &rty, *span);
                }
                if found_return.is_none() {
                    found_return = Some(rty);
                }
            } else {
                self.check_stmt(stmt);
            }
        }
        if let Some(rty) = found_return {
            rty
        } else if let Some(Stmt::Expr { expr, .. }) = blk.stmts.last() {
            self.check_expr(expr, explicit_ret)
        } else {
            Ty::Unit
        }
    }

    fn check_match(
        &mut self,
        scrutinee: &Expr,
        arms: &[MatchArm],
        expected: Option<&Ty>,
        span: Span,
    ) -> Ty {
        let scrut_ty = self.check_expr(scrutinee, None);
        let mut arm_ty: Option<Ty> = expected.cloned();
        let mut has_wildcard = false;
        let mut covered_variants = HashSet::new();

        for arm in arms {
            self.push_scope();
            self.check_pattern(
                &arm.pattern,
                &scrut_ty,
                &mut has_wildcard,
                &mut covered_variants,
            );
            let body_ty = match &arm.body {
                ClosureBody::Expr(e) => self.check_expr(e, arm_ty.as_ref()),
                ClosureBody::Block(blk) => self.check_closure_block(blk, arm_ty.as_ref()),
            };
            self.pop_scope();

            if let Some(first) = &arm_ty {
                if !self.is_assignable(first, &body_ty) {
                    if self.is_assignable(&body_ty, first) {
                        arm_ty = Some(body_ty);
                    } else {
                        self.emit_type_mismatch(first, &body_ty, arm.span);
                    }
                }
            } else {
                arm_ty = Some(body_ty);
            }
        }

        // Exhaustiveness verification
        if !has_wildcard {
            match &scrut_ty {
                Ty::Enum(en) => {
                    if let Some(variants) = self.enums.get(en) {
                        let missing: Vec<&str> = variants
                            .iter()
                            .map(|(vn, _)| vn.as_str())
                            .filter(|vn| !covered_variants.contains(*vn))
                            .collect();
                        if !missing.is_empty() {
                            self.diagnostics.emit(
                                Diagnostic::error(
                                    "E4010",
                                    format!(
                                        "non-exhaustive `match` on `{en}`: missing variant(s): {}",
                                        missing.join(", ")
                                    ),
                                )
                                .with_primary_label(span, "patterns do not cover all enum variants")
                                .with_help(
                                    "add arms for missing variants or a `_ => ...` wildcard arm",
                                ),
                            );
                        }
                    }
                }
                Ty::Option(_) => {
                    if !covered_variants.contains("Some") || !covered_variants.contains("None") {
                        self.diagnostics.emit(
                            Diagnostic::error(
                                "E4010",
                                "non-exhaustive `match` on `Option`: must cover both `Some(...)` and `None`",
                            )
                            .with_primary_label(span, "missing `Some` or `None` arm"),
                        );
                    }
                }
                Ty::Result(_, _) => {
                    if !covered_variants.contains("Ok") || !covered_variants.contains("Err") {
                        self.diagnostics.emit(
                            Diagnostic::error(
                                "E4010",
                                "non-exhaustive `match` on `Result`: must cover both `Ok(...)` and `Err(...)`",
                            )
                            .with_primary_label(span, "missing `Ok` or `Err` arm"),
                        );
                    }
                }
                Ty::Bool => {
                    if !covered_variants.contains("true") || !covered_variants.contains("false") {
                        self.diagnostics.emit(
                            Diagnostic::error(
                                "E4010",
                                "non-exhaustive `match` on `Bool`: must cover both `true` and `false`",
                            )
                            .with_primary_label(span, "non-exhaustive boolean match"),
                        );
                    }
                }
                Ty::Unknown => {}
                _ => {
                    self.diagnostics.emit(
                        Diagnostic::error(
                            "E4010",
                            format!(
                                "non-exhaustive `match` on `{scrut_ty}`: add a `_` catch-all arm"
                            ),
                        )
                        .with_primary_label(span, "missing `_` wildcard arm"),
                    );
                }
            }
        }

        arm_ty.unwrap_or(Ty::Unit)
    }

    fn check_pattern(
        &mut self,
        pattern: &Pattern,
        scrut_ty: &Ty,
        has_wildcard: &mut bool,
        covered: &mut HashSet<String>,
    ) {
        match pattern {
            Pattern::Wildcard(_) => {
                *has_wildcard = true;
            }
            Pattern::IntLit(_, sp) => {
                if !self.is_assignable(&Ty::Int, scrut_ty) {
                    self.emit_type_mismatch(scrut_ty, &Ty::Int, *sp);
                }
            }
            Pattern::StringLit(_, sp) => {
                if !self.is_assignable(&Ty::String, scrut_ty) {
                    self.emit_type_mismatch(scrut_ty, &Ty::String, *sp);
                }
            }
            Pattern::BoolLit(b, sp) => {
                if !self.is_assignable(&Ty::Bool, scrut_ty) {
                    self.emit_type_mismatch(scrut_ty, &Ty::Bool, *sp);
                }
                covered.insert(b.to_string());
            }
            Pattern::Null(sp) => {
                if !matches!(scrut_ty, Ty::Option(_) | Ty::Unknown) {
                    self.diagnostics.emit(
                        Diagnostic::error(
                            "E4001",
                            format!("pattern `null` cannot match `{scrut_ty}`"),
                        )
                        .with_primary_label(*sp, "expected `Option<T>` scrutinee"),
                    );
                }
                covered.insert("None".to_string());
                covered.insert("null".to_string());
            }
            Pattern::Binding(id) => {
                if id.name == "None" {
                    if !matches!(scrut_ty, Ty::Option(_) | Ty::Unknown) {
                        self.diagnostics.emit(
                            Diagnostic::error(
                                "E4001",
                                format!("pattern `None` cannot match `{scrut_ty}`"),
                            )
                            .with_primary_label(id.span, "expected `Option<T>` scrutinee"),
                        );
                    }
                    covered.insert("None".to_string());
                    return;
                }
                if let Ty::Enum(en) = scrut_ty
                    && let Some(variants) = self.enums.get(en)
                    && let Some((_, payload)) = variants.iter().find(|(vn, _)| vn == &id.name)
                {
                    if !payload.is_empty() {
                        self.diagnostics.emit(
                            Diagnostic::error(
                                "E4001",
                                format!("enum variant `{en}.{}` carries payload values", id.name),
                            )
                            .with_primary_label(id.span, "missing `(...)` payload bindings"),
                        );
                    }
                    covered.insert(id.name.clone());
                    return;
                }
                *has_wildcard = true;
                self.define_local(id.name.clone(), scrut_ty.clone());
            }
            Pattern::Variant {
                enum_name,
                variant,
                bindings,
                span,
            } => match scrut_ty {
                Ty::Option(inner) if enum_name.is_none() && variant.name == "Some" => {
                    covered.insert("Some".to_string());
                    if bindings.len() != 1 {
                        self.emit_arg_count("Some", 1, bindings.len(), *span);
                    } else if bindings[0].name != "_" {
                        self.define_local(bindings[0].name.clone(), (**inner).clone());
                    }
                }
                Ty::Result(ok_ty, err_ty)
                    if enum_name.is_none() && (variant.name == "Ok" || variant.name == "Err") =>
                {
                    covered.insert(variant.name.clone());
                    let payload_ty = if variant.name == "Ok" { ok_ty } else { err_ty };
                    if bindings.len() != 1 {
                        self.emit_arg_count(&variant.name, 1, bindings.len(), *span);
                    } else if bindings[0].name != "_" {
                        self.define_local(bindings[0].name.clone(), (**payload_ty).clone());
                    }
                }
                Ty::Enum(en) => {
                    if let Some(prefix) = enum_name
                        && prefix.name != *en
                    {
                        self.diagnostics.emit(
                            Diagnostic::error(
                                "E4001",
                                format!(
                                    "mismatched enum in pattern: expected `{en}`, found `{}`",
                                    prefix.name
                                ),
                            )
                            .with_primary_label(prefix.span, "wrong enum type"),
                        );
                        return;
                    }
                    let Some(variants) = self.enums.get(en).cloned() else {
                        return;
                    };
                    let Some((_, payload_tys)) =
                        variants.iter().find(|(vn, _)| vn == &variant.name)
                    else {
                        self.diagnostics.emit(
                            Diagnostic::error(
                                "E4005",
                                format!("unknown variant `{}` on enum `{en}`", variant.name),
                            )
                            .with_primary_label(variant.span, "variant does not exist"),
                        );
                        return;
                    };
                    covered.insert(variant.name.clone());
                    if bindings.len() != payload_tys.len() {
                        self.emit_arg_count(
                            &variant.name,
                            payload_tys.len(),
                            bindings.len(),
                            *span,
                        );
                    }
                    for (b, pty) in bindings.iter().zip(payload_tys.iter()) {
                        if b.name != "_" {
                            self.define_local(b.name.clone(), pty.clone());
                        }
                    }
                }
                Ty::Unknown => {
                    for b in bindings {
                        if b.name != "_" {
                            self.define_local(b.name.clone(), Ty::Unknown);
                        }
                    }
                }
                other => {
                    self.diagnostics.emit(
                        Diagnostic::error(
                            "E4001",
                            format!(
                                "variant pattern `{}` cannot match value of type `{other}`",
                                variant.name
                            ),
                        )
                        .with_primary_label(*span, "mismatched pattern type"),
                    );
                }
            },
        }
    }

    fn is_assignable(&self, expected: &Ty, actual: &Ty) -> bool {
        if expected == actual || *expected == Ty::Unknown || *actual == Ty::Unknown {
            return true;
        }
        match (expected, actual) {
            (Ty::List(e), Ty::List(a)) | (Ty::Option(e), Ty::Option(a)) => self.is_assignable(e, a),
            (Ty::Result(eo, ee), Ty::Result(ao, ae)) => {
                self.is_assignable(eo, ao) && self.is_assignable(ee, ae)
            }
            (Ty::Interface(iface), Ty::Class(cls)) => self
                .classes
                .get(cls)
                .is_some_and(|c| c.implements.iter().any(|imp| imp == iface)),
            (
                Ty::Fn {
                    params: ep,
                    ret: er,
                    ..
                },
                Ty::Fn {
                    params: ap,
                    ret: ar,
                    ..
                },
            ) => {
                ep.len() == ap.len()
                    && ep
                        .iter()
                        .zip(ap.iter())
                        .all(|(e, a)| self.is_assignable(e, a))
                    && self.is_assignable(er, ar)
            }
            _ => false,
        }
    }

    fn emit_type_mismatch(&mut self, expected: &Ty, actual: &Ty, span: Span) {
        self.diagnostics.emit(
            Diagnostic::error(
                "E4001",
                format!("type mismatch: expected `{expected}`, found `{actual}`"),
            )
            .with_primary_label(span, format!("expected `{expected}`, found `{actual}`")),
        );
    }

    fn emit_arg_count(&mut self, name: &str, expected: usize, actual: usize, span: Span) {
        self.diagnostics.emit(
            Diagnostic::error(
                "E4007",
                format!("`{name}` expects {expected} argument(s), but {actual} were supplied"),
            )
            .with_primary_label(span, "incorrect number of arguments"),
        );
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        let _ = self.scopes.pop();
    }

    fn define_local(&mut self, name: String, ty: Ty) {
        if let Some(top) = self.scopes.last_mut() {
            top.insert(name, ty);
        }
    }

    fn lookup_local(&self, name: &str) -> Option<&Ty> {
        for scope in self.scopes.iter().rev() {
            if let Some(t) = scope.get(name) {
                return Some(t);
            }
        }
        None
    }
}

fn infer_block_return_hint(blk: &Block) -> Ty {
    for stmt in &blk.stmts {
        if let Stmt::Return { value: None, .. } = stmt {
            return Ty::Unit;
        }
    }
    Ty::Unknown
}

fn block_always_returns(blk: &Block) -> bool {
    blk.stmts.iter().any(stmt_always_returns)
}

fn stmt_always_returns(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Return { .. } => true,
        Stmt::If {
            then_branch,
            else_branch: Some(eb),
            ..
        } => {
            let else_returns = match eb {
                ElseBranch::Block(b) => block_always_returns(b),
                ElseBranch::If(nested) => stmt_always_returns(nested),
            };
            block_always_returns(then_branch) && else_returns
        }
        Stmt::TryCatch {
            try_block,
            catch_block,
            ..
        } => block_always_returns(try_block) && block_always_returns(catch_block),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use kora_hir::lower_and_resolve;
    use kora_parser::parse_module;
    use kora_span::FileId;

    use super::*;

    fn run_check(src: &str) -> (DiagnosticBag, TypeCheckOutput) {
        let mut bag = DiagnosticBag::new();
        let ast = parse_module(FileId(0), src, &mut bag);
        let hir = lower_and_resolve(ast, &mut bag);
        let out = check_module(&hir, &mut bag);
        (bag, out)
    }

    #[test]
    fn infers_types_and_verifies_interface_conformance() {
        let src = r#"
model User {
    id: Int
    name: String
    active: Bool
}

interface UserRepo {
    fn find(id: Int) -> Option<User>
}

class SqlUserRepo implements UserRepo {
    db: Database

    init(db: Database) {
        this.db = db
    }

    fn find(id: Int) -> Option<User> {
        if id <= 0 {
            return None
        }
        return Some(User { id: id, name: "Ahmet", active: true })
    }
}

fn loadRepo(db: Database) -> UserRepo {
    let concrete = SqlUserRepo(db)
    let repo: UserRepo = concrete
    return repo
}
"#;
        let (bag, out) = run_check(src);
        assert!(
            !bag.has_errors(),
            "unexpected type errors: {:?}",
            bag.items()
        );
        assert!(out.classes.contains_key("SqlUserRepo"));
        assert!(out.functions.contains_key("loadRepo"));
    }

    #[test]
    fn catches_missing_interface_method_and_non_exhaustive_match() {
        let src = r#"
enum Role {
    Admin
    Member
    Guest
}

interface Greeter {
    fn greet(name: String) -> String
}

class BrokenGreeter implements Greeter {
}

fn roleLabel(r: Role) -> String {
    return match r {
        Role.Admin => "admin",
        Role.Member => "member",
    }
}
"#;
        let (bag, _) = run_check(src);
        assert!(bag.items().iter().any(|d| d.code == "E4008"));
        assert!(bag.items().iter().any(|d| d.code == "E4010"));
    }
}
