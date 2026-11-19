//! High-Level Intermediate Representation (HIR) and multi-pass lexical symbol resolution.
//! Supports Phase 1 Foundation + Phase 5 Language Features (modules, imports, generics, extend, await).

use std::collections::{HashMap, HashSet};

use kora_ast::{
    ClassDecl, ClosureBody, ElseBranch, EnumDecl, Expr, FieldDecl, FnDecl, FnSig, Ident,
    InterfaceDecl, Item, ModelDecl, Module, Pattern, Stmt, StructDecl, TypeExpr,
};
use kora_diagnostics::{Diagnostic, DiagnosticBag};
use kora_span::Span;

/// Unique identifier for a top-level or prelude definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DefId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    BuiltinType,
    BuiltinValue,
    BuiltinTypeAndValue,
    Struct,
    Model,
    Interface,
    Class,
    Enum,
    Function,
}

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub id: DefId,
    pub name: String,
    pub kind: SymbolKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct LocalBinding {
    pub name: String,
    pub mutable: bool,
    pub span: Span,
}

/// Resolved module with verified symbol declarations, scopes, and mutability rules.
#[derive(Debug, Clone)]
pub struct HirModule {
    pub ast: Module,
    pub symbols: HashMap<String, SymbolInfo>,
}

const PRELUDE_TYPES: &[&str] = &[
    "Int",
    "Int8",
    "Int16",
    "Int32",
    "Int64",
    "UInt",
    "UInt8",
    "UInt16",
    "UInt32",
    "UInt64",
    "Float",
    "Float32",
    "Float64",
    "Decimal",
    "Bool",
    "Char",
    "String",
    "Byte",
    "Unit",
    "Never",
    "Array",
    "Vector",
    "VecDeque",
    "LinkedList",
    "Stack",
    "Queue",
    "CircularBuffer",
    "FixedArray",
    "Matrix",
    "Tensor",
    "Map",
    "HashMap",
    "BTreeMap",
    "OrderedMap",
    "MultiMap",
    "BiMap",
    "WeakMap",
    "Set",
    "HashSet",
    "BTreeSet",
    "OrderedSet",
    "BitSet",
    "MultiSet",
    "PriorityQueue",
    "BinaryHeap",
    "Trie",
    "RadixTree",
    "Graph",
    "DirectedGraph",
    "UndirectedGraph",
    "WeightedGraph",
    "DisjointSet",
    "BloomFilter",
    "LRUCache",
    "LFUCache",
    "RingBuffer",
    "List",
    "Tuple",
    "Option",
    "Result",
    "Either",
    "Range",
    "Channel",
    "Database",
    "App",
    "Request",
    "Response",
];

const PRELUDE_VALUES: &[&str] = &[
    "Some",
    "None",
    "Ok",
    "Err",
    "Left",
    "Right",
    "Array",
    "Vector",
    "VecDeque",
    "LinkedList",
    "Stack",
    "Queue",
    "CircularBuffer",
    "FixedArray",
    "Matrix",
    "Tensor",
    "Map",
    "HashMap",
    "BTreeMap",
    "OrderedMap",
    "MultiMap",
    "BiMap",
    "Set",
    "HashSet",
    "BTreeSet",
    "OrderedSet",
    "BitSet",
    "MultiSet",
    "PriorityQueue",
    "BinaryHeap",
    "Trie",
    "RadixTree",
    "Graph",
    "DirectedGraph",
    "UndirectedGraph",
    "WeightedGraph",
    "DisjointSet",
    "BloomFilter",
    "LRUCache",
    "LFUCache",
    "RingBuffer",
    "channel",
    "print",
    "println",
    "run",
    "send",
    "get",
    "post",
    "put",
    "delete",
];

/// Lowers and name-resolves a parsed [`Module`] into [`HirModule`].
pub fn lower_and_resolve(module: Module, diagnostics: &mut DiagnosticBag) -> HirModule {
    let mut resolver = Resolver::new(diagnostics);
    resolver.seed_prelude();
    resolver.collect_top_level(&module);
    resolver.resolve_items(&module);

    HirModule {
        ast: module,
        symbols: resolver.symbols,
    }
}

struct Resolver<'a> {
    symbols: HashMap<String, SymbolInfo>,
    enum_variants: HashMap<String, HashSet<String>>,
    next_def_id: u32,
    scopes: Vec<HashMap<String, LocalBinding>>,
    in_class_or_extend: bool,
    in_async: bool,
    diagnostics: &'a mut DiagnosticBag,
}

impl<'a> Resolver<'a> {
    fn new(diagnostics: &'a mut DiagnosticBag) -> Self {
        Self {
            symbols: HashMap::new(),
            enum_variants: HashMap::new(),
            next_def_id: 0,
            scopes: vec![HashMap::new()],
            in_class_or_extend: false,
            in_async: false,
            diagnostics,
        }
    }

    fn alloc_def(&mut self) -> DefId {
        let id = DefId(self.next_def_id);
        self.next_def_id += 1;
        id
    }

    fn seed_prelude(&mut self) {
        for &t in PRELUDE_TYPES {
            let id = self.alloc_def();
            self.symbols.insert(
                t.to_string(),
                SymbolInfo {
                    id,
                    name: t.to_string(),
                    kind: SymbolKind::BuiltinType,
                    span: Span::dummy(),
                },
            );
        }
        for &v in PRELUDE_VALUES {
            if let Some(existing) = self.symbols.get_mut(v) {
                if existing.kind == SymbolKind::BuiltinType {
                    existing.kind = SymbolKind::BuiltinTypeAndValue;
                    continue;
                }
            }
            let id = self.alloc_def();
            self.symbols.insert(
                v.to_string(),
                SymbolInfo {
                    id,
                    name: v.to_string(),
                    kind: SymbolKind::BuiltinValue,
                    span: Span::dummy(),
                },
            );
        }
    }

    fn register_symbol(&mut self, ident: &Ident, kind: SymbolKind) {
        if let Some(existing) = self.symbols.get(&ident.name) {
            let mut diag =
                Diagnostic::error("E3001", format!("duplicate definition of `{}`", ident.name))
                    .with_primary_label(ident.span, format!("`{}` redefined here", ident.name));
            if !existing.span.is_dummy() {
                diag = diag.with_secondary_label(existing.span, "previously defined here");
            }
            self.diagnostics.emit(diag);
            return;
        }
        let id = self.alloc_def();
        self.symbols.insert(
            ident.name.clone(),
            SymbolInfo {
                id,
                name: ident.name.clone(),
                kind,
                span: ident.span,
            },
        );
    }

    fn collect_top_level(&mut self, module: &Module) {
        for item in &module.items {
            match item {
                Item::ModuleDecl(_) | Item::Import(_) => {}
                Item::Struct(StructDecl { name, fields, .. }) => {
                    self.register_symbol(name, SymbolKind::Struct);
                    self.check_unique_fields(&name.name, fields);
                }
                Item::Model(ModelDecl { name, fields, .. }) => {
                    self.register_symbol(name, SymbolKind::Model);
                    self.check_unique_fields(&name.name, fields);
                }
                Item::Interface(InterfaceDecl { name, methods, .. }) => {
                    self.register_symbol(name, SymbolKind::Interface);
                    self.check_unique_sigs(&name.name, methods);
                }
                Item::Class(ClassDecl {
                    name,
                    fields,
                    methods,
                    ..
                }) => {
                    self.register_symbol(name, SymbolKind::Class);
                    self.check_unique_fields(&name.name, fields);
                    let sigs: Vec<FnSig> = methods.iter().map(|m| m.sig.clone()).collect();
                    self.check_unique_sigs(&name.name, &sigs);
                }
                Item::Enum(EnumDecl { name, variants, .. }) => {
                    self.register_symbol(name, SymbolKind::Enum);
                    let mut seen = HashSet::new();
                    for v in variants {
                        if !seen.insert(v.name.name.clone()) {
                            self.diagnostics.emit(
                                Diagnostic::error(
                                    "E3004",
                                    format!(
                                        "duplicate enum variant `{}.{}`",
                                        name.name, v.name.name
                                    ),
                                )
                                .with_primary_label(v.name.span, "duplicate variant"),
                            );
                        }
                    }
                    self.enum_variants.insert(name.name.clone(), seen);
                }
                Item::Extend(_) => {}
                Item::Fn(FnDecl { sig, .. }) => {
                    self.register_symbol(&sig.name, SymbolKind::Function);
                }
                Item::Stmt(_) => {}
            }
        }
    }

    fn check_unique_fields(&mut self, owner: &str, fields: &[FieldDecl]) {
        let mut seen = HashSet::new();
        for f in fields {
            if !seen.insert(f.name.name.as_str()) {
                self.diagnostics.emit(
                    Diagnostic::error(
                        "E3002",
                        format!("duplicate field `{}` in `{owner}`", f.name.name),
                    )
                    .with_primary_label(f.name.span, "duplicate field declaration"),
                );
            }
        }
    }

    fn check_unique_sigs(&mut self, owner: &str, sigs: &[FnSig]) {
        let mut seen = HashSet::new();
        for s in sigs {
            if !seen.insert(s.name.name.as_str()) {
                self.diagnostics.emit(
                    Diagnostic::error(
                        "E3003",
                        format!("duplicate method `{}` in `{owner}`", s.name.name),
                    )
                    .with_primary_label(s.name.span, "duplicate method declaration"),
                );
            }
        }
    }

    fn resolve_items(&mut self, module: &Module) {
        for item in &module.items {
            match item {
                Item::ModuleDecl(_) | Item::Import(_) => {}
                Item::Struct(s) => {
                    for f in &s.fields {
                        self.resolve_type_expr(&f.ty);
                    }
                }
                Item::Model(m) => {
                    for f in &m.fields {
                        self.resolve_type_expr(&f.ty);
                    }
                }
                Item::Interface(iface) => {
                    for sig in &iface.methods {
                        self.resolve_fn_sig(sig);
                    }
                }
                Item::Class(cls) => {
                    for iface_ident in &cls.implements {
                        match self.symbols.get(&iface_ident.name) {
                            Some(sym) if sym.kind == SymbolKind::Interface => {}
                            Some(_) => {
                                self.diagnostics.emit(
                                    Diagnostic::error(
                                        "E3007",
                                        format!("`{}` is not an `interface`", iface_ident.name),
                                    )
                                    .with_primary_label(
                                        iface_ident.span,
                                        "classes can only implement declared interfaces",
                                    ),
                                );
                            }
                            None => {
                                self.diagnostics.emit(
                                    Diagnostic::error(
                                        "E3007",
                                        format!("unknown interface `{}`", iface_ident.name),
                                    )
                                    .with_primary_label(iface_ident.span, "not found in scope"),
                                );
                            }
                        }
                    }
                    for f in &cls.fields {
                        self.resolve_type_expr(&f.ty);
                    }
                    let prev_class = self.in_class_or_extend;
                    self.in_class_or_extend = true;
                    if let Some(init) = &cls.init {
                        self.push_scope();
                        for p in &init.params {
                            self.resolve_type_expr(&p.ty);
                            self.define_local(&p.name, false);
                        }
                        for stmt in &init.body.stmts {
                            self.resolve_stmt(stmt);
                        }
                        self.pop_scope();
                    }
                    for method in &cls.methods {
                        self.resolve_fn_decl(method);
                    }
                    self.in_class_or_extend = prev_class;
                }
                Item::Enum(en) => {
                    for v in &en.variants {
                        for t in &v.payload {
                            self.resolve_type_expr(t);
                        }
                    }
                }
                Item::Extend(ext) => {
                    if !self.symbols.contains_key(&ext.target_type.name) {
                        self.diagnostics.emit(
                            Diagnostic::error(
                                "E3007",
                                format!("cannot extend unknown type `{}`", ext.target_type.name),
                            )
                            .with_primary_label(ext.target_type.span, "type not found in scope"),
                        );
                    }
                    let prev = self.in_class_or_extend;
                    self.in_class_or_extend = true;
                    for method in &ext.methods {
                        self.resolve_fn_decl(method);
                    }
                    self.in_class_or_extend = prev;
                }
                Item::Fn(func) => self.resolve_fn_decl(func),
                Item::Stmt(stmt) => self.resolve_stmt(stmt),
            }
        }
    }

    fn resolve_fn_sig(&mut self, sig: &FnSig) {
        for p in &sig.params {
            self.resolve_type_expr(&p.ty);
        }
        if let Some(ret) = &sig.return_ty {
            self.resolve_type_expr(ret);
        }
    }

    fn resolve_fn_decl(&mut self, func: &FnDecl) {
        self.resolve_fn_sig(&func.sig);
        let prev_async = self.in_async;
        self.in_async = func.sig.is_async;
        self.push_scope();
        for p in &func.sig.params {
            self.define_local(&p.name, false);
        }
        for stmt in &func.body.stmts {
            self.resolve_stmt(stmt);
        }
        self.pop_scope();
        self.in_async = prev_async;
    }

    fn resolve_type_expr(&mut self, ty: &TypeExpr) {
        match ty {
            TypeExpr::Named { name, args, .. } => {
                match self.symbols.get(&name.name) {
                    Some(sym)
                        if matches!(
                            sym.kind,
                            SymbolKind::BuiltinType
                                | SymbolKind::BuiltinTypeAndValue
                                | SymbolKind::Struct
                                | SymbolKind::Model
                                | SymbolKind::Interface
                                | SymbolKind::Class
                                | SymbolKind::Enum
                        ) || PRELUDE_TYPES.contains(&name.name.as_str()) => {}
                    _ => {
                        // Allow single-letter or generic parameter tokens (e.g. T, U, V)
                        if !name
                            .name
                            .chars()
                            .all(|c| c.is_ascii_uppercase() || c == '_')
                        {
                            self.diagnostics.emit(
                                Diagnostic::error("E3007", format!("unknown type `{}`", name.name))
                                    .with_primary_label(name.span, "unknown type name"),
                            );
                        }
                    }
                }
                for arg in args {
                    self.resolve_type_expr(arg);
                }
            }
            TypeExpr::Function { params, ret, .. } => {
                for p in params {
                    self.resolve_type_expr(p);
                }
                self.resolve_type_expr(ret);
            }
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let {
                name,
                mutable,
                ty,
                value,
                ..
            } => {
                if let Some(t) = ty {
                    self.resolve_type_expr(t);
                }
                self.resolve_expr(value);
                self.define_local(name, *mutable);
            }
            Stmt::Return { value, .. } => {
                if let Some(v) = value {
                    self.resolve_expr(v);
                }
            }
            Stmt::If {
                cond,
                then_branch,
                else_branch,
                ..
            } => {
                self.resolve_expr(cond);
                self.push_scope();
                for s in &then_branch.stmts {
                    self.resolve_stmt(s);
                }
                self.pop_scope();
                if let Some(eb) = else_branch {
                    match eb {
                        ElseBranch::Block(blk) => {
                            self.push_scope();
                            for s in &blk.stmts {
                                self.resolve_stmt(s);
                            }
                            self.pop_scope();
                        }
                        ElseBranch::If(nested) => self.resolve_stmt(nested),
                    }
                }
            }
            Stmt::While { cond, body, .. } => {
                self.resolve_expr(cond);
                self.push_scope();
                for s in &body.stmts {
                    self.resolve_stmt(s);
                }
                self.pop_scope();
            }
            Stmt::For {
                var, iter, body, ..
            } => {
                self.resolve_expr(iter);
                self.push_scope();
                self.define_local(var, false);
                for s in &body.stmts {
                    self.resolve_stmt(s);
                }
                self.pop_scope();
            }
            Stmt::Break { .. } | Stmt::Continue { .. } => {}
            Stmt::Const {
                name, ty, value, ..
            } => {
                if let Some(t) = ty {
                    self.resolve_type_expr(t);
                }
                self.resolve_expr(value);
                self.define_local(name, false);
            }
            Stmt::CompoundAssign { target, value, .. } => {
                self.resolve_expr(target);
                self.resolve_expr(value);
            }
            Stmt::Throw { value, .. } => {
                self.resolve_expr(value);
            }
            Stmt::TryCatch {
                try_block,
                catch_param,
                catch_block,
                finally_block,
                ..
            } => {
                self.push_scope();
                for s in &try_block.stmts {
                    self.resolve_stmt(s);
                }
                self.pop_scope();

                self.push_scope();
                if let Some(param) = catch_param {
                    self.define_local(param, false);
                }
                for s in &catch_block.stmts {
                    self.resolve_stmt(s);
                }
                self.pop_scope();

                if let Some(fin) = finally_block {
                    self.push_scope();
                    for s in &fin.stmts {
                        self.resolve_stmt(s);
                    }
                    self.pop_scope();
                }
            }
            Stmt::Expr { expr, .. } => self.resolve_expr(expr),
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::IntLit { .. }
            | Expr::FloatLit { .. }
            | Expr::StringLit { .. }
            | Expr::CharLit { .. }
            | Expr::BoolLit { .. }
            | Expr::Null { .. } => {}
            Expr::This { span } => {
                if !self.in_class_or_extend {
                    self.diagnostics.emit(
                        Diagnostic::error(
                            "E3005",
                            "`this` can only be used inside a `class` constructor, method, or `extend` block",
                        )
                        .with_primary_label(*span, "`this` is not valid here"),
                    );
                }
            }
            Expr::Ident(id) => {
                if self.lookup_local(&id.name).is_none() && !self.symbols.contains_key(&id.name) {
                    self.diagnostics.emit(
                        Diagnostic::error(
                            "E3006",
                            format!("cannot find value or symbol `{}` in this scope", id.name),
                        )
                        .with_primary_label(id.span, "not found in scope"),
                    );
                }
            }
            Expr::Await { expr: inner, span } => {
                if !self.in_async {
                    self.diagnostics.emit(
                        Diagnostic::error("E4013", "`await` is only valid inside an `async fn`")
                            .with_primary_label(*span, "`await` used in synchronous context"),
                    );
                }
                self.resolve_expr(inner);
            }
            Expr::Unary { expr: inner, .. } => self.resolve_expr(inner),
            Expr::Binary { lhs, rhs, .. } => {
                self.resolve_expr(lhs);
                self.resolve_expr(rhs);
            }
            Expr::Assign { target, value, .. } => {
                self.resolve_expr(target);
                self.resolve_expr(value);
                if let Expr::Ident(id) = target.as_ref()
                    && let Some(binding) = self.lookup_local(&id.name)
                    && !binding.mutable
                {
                    let decl_span = binding.span;
                    self.diagnostics.emit(
                        Diagnostic::error(
                            "E3008",
                            format!("cannot assign to immutable binding `{}`", id.name),
                        )
                        .with_primary_label(id.span, "cannot mutate immutable `let` binding")
                        .with_secondary_label(decl_span, "declared as immutable `let` here")
                        .with_help(format!(
                            "declare `{}` with `var` instead of `let` if mutation is intended",
                            id.name
                        )),
                    );
                }
            }
            Expr::Call { callee, args, .. } => {
                self.resolve_expr(callee);
                for a in args {
                    self.resolve_expr(a);
                }
            }
            Expr::FieldAccess { receiver, .. } => {
                self.resolve_expr(receiver);
            }
            Expr::Index {
                receiver, index, ..
            } => {
                self.resolve_expr(receiver);
                self.resolve_expr(index);
            }
            Expr::StructLit { name, fields, .. } => {
                if !self.symbols.contains_key(&name.name) {
                    self.diagnostics.emit(
                        Diagnostic::error(
                            "E3007",
                            format!("unknown struct or model `{}`", name.name),
                        )
                        .with_primary_label(name.span, "unknown type"),
                    );
                }
                for f in fields {
                    self.resolve_expr(&f.value);
                }
            }
            Expr::ListLit { elements, .. } => {
                for el in elements {
                    self.resolve_expr(el);
                }
            }
            Expr::Closure {
                params,
                return_ty,
                body,
                ..
            } => {
                if let Some(rt) = return_ty {
                    self.resolve_type_expr(rt);
                }
                self.push_scope();
                for p in params {
                    if let Some(pt) = &p.ty {
                        self.resolve_type_expr(pt);
                    }
                    self.define_local(&p.name, false);
                }
                self.resolve_closure_body(body);
                self.pop_scope();
            }
            Expr::Match {
                scrutinee, arms, ..
            } => {
                self.resolve_expr(scrutinee);
                for arm in arms {
                    self.push_scope();
                    self.bind_pattern(&arm.pattern);
                    self.resolve_closure_body(&arm.body);
                    self.pop_scope();
                }
            }
            Expr::TypeConstructor { type_args, .. } => {
                for t in type_args {
                    self.resolve_type_expr(t);
                }
            }
            Expr::CollectionLit {
                constructor,
                elements,
                ..
            } => {
                if let Some(c) = constructor {
                    self.resolve_expr(c);
                }
                for el in elements {
                    self.resolve_expr(el);
                }
            }
            Expr::MapLit { entries, .. } => {
                for (k, v) in entries {
                    self.resolve_expr(k);
                    self.resolve_expr(v);
                }
            }
        }
    }

    fn resolve_closure_body(&mut self, body: &ClosureBody) {
        match body {
            ClosureBody::Block(blk) => {
                for s in &blk.stmts {
                    self.resolve_stmt(s);
                }
            }
            ClosureBody::Expr(expr) => self.resolve_expr(expr),
        }
    }

    fn bind_pattern(&mut self, pattern: &Pattern) {
        match pattern {
            Pattern::Wildcard(_)
            | Pattern::IntLit(_, _)
            | Pattern::FloatLit(_, _)
            | Pattern::StringLit(_, _)
            | Pattern::CharLit(_, _)
            | Pattern::BoolLit(_, _)
            | Pattern::Null(_) => {}
            Pattern::Binding(id) => {
                if id.name != "None" && !self.is_unit_enum_variant(&id.name) {
                    self.define_local(id, false);
                }
            }
            Pattern::Variant {
                enum_name,
                bindings,
                ..
            } => {
                if let Some(en) = enum_name
                    && !self.symbols.contains_key(&en.name)
                {
                    self.diagnostics.emit(
                        Diagnostic::error("E3007", format!("unknown enum `{}`", en.name))
                            .with_primary_label(en.span, "not found in scope"),
                    );
                }
                for b in bindings {
                    if b.name != "_" {
                        self.define_local(b, false);
                    }
                }
            }
        }
    }

    fn is_unit_enum_variant(&self, name: &str) -> bool {
        self.enum_variants
            .values()
            .any(|variants| variants.contains(name))
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        let _ = self.scopes.pop();
    }

    fn define_local(&mut self, ident: &Ident, mutable: bool) {
        if let Some(current) = self.scopes.last_mut() {
            current.insert(
                ident.name.clone(),
                LocalBinding {
                    name: ident.name.clone(),
                    mutable,
                    span: ident.span,
                },
            );
        }
    }

    fn lookup_local(&self, name: &str) -> Option<&LocalBinding> {
        for scope in self.scopes.iter().rev() {
            if let Some(b) = scope.get(name) {
                return Some(b);
            }
        }
        None
    }
}

/// Enforces strict Clean Architecture rules across all Kora source files.
/// - E5001: Files inside `enums/` (or `*.enum.kora`) can only declare enums.
/// - E5002: Files inside `models/` (or `*.model.kora`) or `interfaces/` (or `*.interface.kora`)
///   cannot declare classes, statements, routes, or foreign items.
/// - E5003: Files containing a `class`, or located in `services/`, `controllers/`,
///   cannot define inline `model`, `enum`, or `interface`.
/// - E5004: Entrypoint files (`main.kora` / `main.kr`) cannot define inline `model`, `enum`, or `interface`.
pub fn verify_architectural_purity(path_str: &str, ast: &Module, bag: &mut DiagnosticBag) {
    let p = path_str.replace('\\', "/");

    let is_enum_file = p.contains("/enums/")
        || p.starts_with("enums/")
        || p.ends_with(".enum.kora")
        || p.ends_with(".enum.kr")
        || p.ends_with("_enum.kora")
        || p.ends_with("_enum.kr");

    let is_model_file = p.contains("/models/")
        || p.starts_with("models/")
        || p.contains("/entities/")
        || p.starts_with("entities/")
        || p.contains("/dto/")
        || p.starts_with("dto/")
        || p.ends_with(".model.kora")
        || p.ends_with(".model.kr")
        || p.ends_with("_model.kora")
        || p.ends_with("_model.kr")
        || p.ends_with(".entity.kora")
        || p.ends_with(".dto.kora");

    let is_interface_file = p.contains("/interfaces/")
        || p.starts_with("interfaces/")
        || p.ends_with(".interface.kora")
        || p.ends_with(".interface.kr")
        || p.ends_with("_interface.kora")
        || p.ends_with("_interface.kr");

    let is_model_or_iface_file = is_model_file || is_interface_file;

    let is_controller_or_service = !is_model_or_iface_file
        && !is_enum_file
        && (p.contains("controller")
            || p.contains("service")
            || p.contains("/controllers/")
            || p.contains("/services/")
            || p.ends_with(".controller.kora")
            || p.ends_with(".service.kora"));

    let is_main_kora =
        p.ends_with("/main.kora") || p.ends_with("/main.kr") || p == "main.kora" || p == "main.kr";

    let has_classes = ast.items.iter().any(|i| matches!(i, Item::Class(_)));

    for item in &ast.items {
        match item {
            Item::Enum(en) => {
                if has_classes {
                    bag.emit(
                        Diagnostic::error(
                            "E5003",
                            format!(
                                "architectural violation: cannot define enum `{}` and class in the same file",
                                en.name.name
                            ),
                        )
                        .with_primary_label(
                            en.span,
                            "inline enum definition forbidden alongside class; extract enums into dedicated files in `enums/`",
                        )
                        .with_help("Clean Architecture enforces strict layer segregation: enums must reside in dedicated enum files, and classes in dedicated service/controller files."),
                    );
                } else if is_controller_or_service {
                    bag.emit(
                        Diagnostic::error(
                            "E5003",
                            format!(
                                "architectural violation: controller or service cannot define inline enum `{}`",
                                en.name.name
                            ),
                        )
                        .with_primary_label(
                            en.span,
                            "inline enum definition forbidden in controllers/services; import from `enums/`",
                        )
                        .with_help("Enums must reside in dedicated files in `enums/` (e.g. `*.enum.kora`)."),
                    );
                } else if is_main_kora {
                    bag.emit(
                        Diagnostic::error(
                            "E5004",
                            format!(
                                "architectural violation: 'main.kora' cannot define inline enum `{}`",
                                en.name.name
                            ),
                        )
                        .with_primary_label(
                            en.span,
                            "inline enum definition forbidden in 'main.kora'; import from dedicated `enums/` module",
                        )
                        .with_help("Extract enums to `enums/<name>.enum.kora` and import them into `main.kora`."),
                    );
                } else if is_model_file {
                    bag.emit(
                        Diagnostic::error(
                            "E5002",
                            format!(
                                "architectural violation: file inside 'models/' cannot define enum `{}`",
                                en.name.name
                            ),
                        )
                        .with_primary_label(
                            en.span,
                            "enums forbidden inside 'models/'; extract into `enums/`",
                        ),
                    );
                }
            }
            Item::Model(m) => {
                if has_classes {
                    bag.emit(
                        Diagnostic::error(
                            "E5003",
                            format!(
                                "architectural violation: cannot define model `{}` and class in the same file",
                                m.name.name
                            ),
                        )
                        .with_primary_label(
                            m.span,
                            "inline model definition forbidden alongside class; extract models into dedicated files in `models/`",
                        )
                        .with_help("Clean Architecture enforces strict layer segregation: models (DTOs/entities) must reside in dedicated model files and classes (services/controllers) in dedicated service files."),
                    );
                } else if is_controller_or_service {
                    bag.emit(
                        Diagnostic::error(
                            "E5003",
                            format!(
                                "architectural violation: controller or service cannot define inline model `{}`",
                                m.name.name
                            ),
                        )
                        .with_primary_label(
                            m.span,
                            "inline model definition forbidden in controllers/services; import from `models/`",
                        )
                        .with_help("Models must reside in dedicated files in `models/` (e.g. `*.model.kora`)."),
                    );
                } else if is_main_kora {
                    bag.emit(
                        Diagnostic::error(
                            "E5004",
                            format!(
                                "architectural violation: 'main.kora' cannot define inline model `{}`",
                                m.name.name
                            ),
                        )
                        .with_primary_label(
                            m.span,
                            "inline model definition forbidden in 'main.kora'; import from dedicated `models/` module",
                        )
                        .with_help("Extract models to `models/<name>.model.kora` and import them into `main.kora`."),
                    );
                } else if is_enum_file {
                    bag.emit(
                        Diagnostic::error(
                            "E5001",
                            format!(
                                "architectural violation: file inside 'enums/' cannot define model `{}`",
                                m.name.name
                            ),
                        )
                        .with_primary_label(
                            m.span,
                            "models forbidden inside 'enums/'; extract into `models/`",
                        ),
                    );
                }
            }
            Item::Interface(iface) => {
                if has_classes {
                    bag.emit(
                        Diagnostic::error(
                            "E5003",
                            format!(
                                "architectural violation: cannot define interface `{}` and class in the same file",
                                iface.name.name
                            ),
                        )
                        .with_primary_label(
                            iface.span,
                            "inline interface definition forbidden alongside class; extract interfaces into dedicated files in `interfaces/`",
                        ),
                    );
                } else if is_controller_or_service {
                    bag.emit(
                        Diagnostic::error(
                            "E5003",
                            format!(
                                "architectural violation: controller or service cannot define inline interface `{}`",
                                iface.name.name
                            ),
                        )
                        .with_primary_label(
                            iface.span,
                            "inline interface definition forbidden in controllers/services; import from `interfaces/`",
                        ),
                    );
                } else if is_main_kora {
                    bag.emit(
                        Diagnostic::error(
                            "E5004",
                            format!(
                                "architectural violation: 'main.kora' cannot define inline interface `{}`",
                                iface.name.name
                            ),
                        )
                        .with_primary_label(
                            iface.span,
                            "inline interface definition forbidden in 'main.kora'; import from dedicated `interfaces/` module",
                        ),
                    );
                } else if is_enum_file {
                    bag.emit(
                        Diagnostic::error(
                            "E5001",
                            format!(
                                "architectural violation: file inside 'enums/' cannot define interface `{}`",
                                iface.name.name
                            ),
                        )
                        .with_primary_label(
                            iface.span,
                            "interfaces forbidden inside 'enums/'; extract into `interfaces/`",
                        ),
                    );
                }
            }
            Item::Fn(f) => {
                if is_enum_file {
                    bag.emit(
                        Diagnostic::error(
                            "E5001",
                            format!(
                                "architectural violation: file inside 'enums/' cannot define function `{}`",
                                f.sig.name.name
                            ),
                        )
                        .with_primary_label(f.span, "functions are forbidden inside 'enums/' directory"),
                    );
                } else if is_model_or_iface_file {
                    bag.emit(
                        Diagnostic::error(
                            "E5002",
                            format!(
                                "architectural violation: file inside 'models/' or 'interfaces/' cannot define function `{}`",
                                f.sig.name.name
                            ),
                        )
                        .with_primary_label(f.span, "functions are forbidden inside 'models/' or 'interfaces/'"),
                    );
                }
            }
            Item::Class(c) => {
                if is_enum_file {
                    bag.emit(
                        Diagnostic::error(
                            "E5001",
                            format!(
                                "architectural violation: file inside 'enums/' cannot define class `{}`",
                                c.name.name
                            ),
                        )
                        .with_primary_label(c.span, "classes are forbidden inside 'enums/' directory"),
                    );
                } else if is_model_or_iface_file {
                    bag.emit(
                        Diagnostic::error(
                            "E5002",
                            format!(
                                "architectural violation: file inside 'models/' or 'interfaces/' cannot define class `{}`",
                                c.name.name
                            ),
                        )
                        .with_primary_label(c.span, "classes are forbidden inside 'models/' or 'interfaces/'"),
                    );
                }
            }
            Item::Stmt(s) => {
                if is_enum_file {
                    bag.emit(
                        Diagnostic::error(
                            "E5001",
                            "architectural violation: file inside 'enums/' cannot define statements or HTTP routes",
                        )
                        .with_primary_label(
                            s.span(),
                            "statements/routes forbidden inside 'enums/' directory",
                        ),
                    );
                } else if is_model_or_iface_file {
                    bag.emit(
                        Diagnostic::error(
                            "E5002",
                            "architectural violation: file inside 'models/' or 'interfaces/' cannot define statements or HTTP routes",
                        )
                        .with_primary_label(
                            s.span(),
                            "statements/routes forbidden inside 'models/' or 'interfaces/'",
                        ),
                    );
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use kora_parser::parse_module;
    use kora_span::FileId;

    use super::*;

    #[test]
    fn rejects_mutation_of_let_binding_and_allows_var() {
        let src = r#"
let fixed = 10
fixed = 20
var counter = 0
counter = counter + 1
"#;
        let mut bag = DiagnosticBag::new();
        let ast = parse_module(FileId(0), src, &mut bag);
        let _ = lower_and_resolve(ast, &mut bag);
        assert_eq!(bag.error_count(), 1);
        assert_eq!(bag.items()[0].code, "E3008");
    }

    #[test]
    fn rejects_await_outside_async_fn() {
        let src = "fn bad() { let x = await 1 }\n";
        let mut bag = DiagnosticBag::new();
        let ast = parse_module(FileId(0), src, &mut bag);
        let _ = lower_and_resolve(ast, &mut bag);
        assert!(bag.items().iter().any(|d| d.code == "E4013"));
    }

    #[test]
    fn architectural_purity_enforces_rules_across_all_layers() {
        let mut bag = DiagnosticBag::new();

        // 1. Mixing model with class in the same file -> E5003
        let src_class_model = "class UserService {}\nmodel User { id: Int }\n";
        let ast = parse_module(FileId(0), src_class_model, &mut bag);
        verify_architectural_purity("services/user.service.kora", &ast, &mut bag);
        assert!(bag.items().iter().any(|d| d.code == "E5003"));

        // 2. Mixing enum with class in the same file -> E5003
        let mut bag2 = DiagnosticBag::new();
        let src_class_enum = "class AuthService {}\nenum Role { Admin, User }\n";
        let ast2 = parse_module(FileId(0), src_class_enum, &mut bag2);
        verify_architectural_purity("services/auth.service.kora", &ast2, &mut bag2);
        assert!(bag2.items().iter().any(|d| d.code == "E5003"));

        // 3. Defining inline model in main.kora -> E5004
        let mut bag3 = DiagnosticBag::new();
        let src_main_model = "model Config { host: String }\n";
        let ast3 = parse_module(FileId(0), src_main_model, &mut bag3);
        verify_architectural_purity("src/main.kora", &ast3, &mut bag3);
        assert!(bag3.items().iter().any(|d| d.code == "E5004"));

        // 4. Defining class inside enums/ -> E5001
        let mut bag4 = DiagnosticBag::new();
        let src_enum_class = "class BadInEnum {}\n";
        let ast4 = parse_module(FileId(0), src_enum_class, &mut bag4);
        verify_architectural_purity("enums/status.enum.kora", &ast4, &mut bag4);
        assert!(bag4.items().iter().any(|d| d.code == "E5001"));

        // 5. Defining statements inside models/ -> E5002
        let mut bag5 = DiagnosticBag::new();
        let src_model_stmt = "print(\"oops\")\n";
        let ast5 = parse_module(FileId(0), src_model_stmt, &mut bag5);
        verify_architectural_purity("models/user.model.kora", &ast5, &mut bag5);
        assert!(bag5.items().iter().any(|d| d.code == "E5002"));
    }

    #[test]
    fn test_vector_in_prelude() {
        let mut bag = DiagnosticBag::new();
        let src = "class Foo { init() {} fn test(v: Vector<Int>) -> Int { return 1 } }\n";
        let ast = parse_module(FileId(0), src, &mut bag);
        let _ = lower_and_resolve(ast, &mut bag);
        assert!(!bag.has_errors(), "diagnostics: {:?}", bag.items());
    }
}
