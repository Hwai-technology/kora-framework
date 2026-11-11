//! Abstract Syntax Tree (AST) definitions for the Kora programming language.
//! Covers Phase 1 Foundation and Phase 5 Advanced Language constructs
//! (`module`, `import`, `pub`, `Generics <T>`, `extend`, `await`, `Channel<T>`).

use kora_span::{FileId, Span};

/// Identifier node with exact source span.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

impl Ident {
    pub fn new(name: impl Into<String>, span: Span) -> Self {
        Self {
            name: name.into(),
            span,
        }
    }
}

/// A parsed `.kora` compilation unit.
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub file_id: FileId,
    pub items: Vec<Item>,
    pub span: Span,
}

/// Top-level declaration or zero-config executable statement.
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    ModuleDecl(ModulePathDecl),
    Import(ImportDecl),
    Struct(StructDecl),
    Model(ModelDecl),
    Interface(InterfaceDecl),
    Class(ClassDecl),
    Enum(EnumDecl),
    Extend(ExtendDecl),
    Fn(FnDecl),
    Stmt(Stmt),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModulePathDecl {
    pub segments: Vec<Ident>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportDecl {
    pub symbols: Vec<Ident>,
    pub path_str: Option<String>,
    pub segments: Vec<Ident>,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Visibility {
    #[default]
    Public,
    Private,
    Protected,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub name: Ident,
    pub args: Vec<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldDecl {
    pub visibility: Visibility,
    pub is_readonly: bool,
    pub attributes: Vec<Attribute>,
    pub name: Ident,
    pub ty: TypeExpr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDecl {
    pub is_pub: bool,
    pub name: Ident,
    pub type_params: Vec<Ident>,
    pub fields: Vec<FieldDecl>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModelDecl {
    pub is_pub: bool,
    pub attributes: Vec<Attribute>,
    pub name: Ident,
    pub type_params: Vec<Ident>,
    pub fields: Vec<FieldDecl>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: Ident,
    pub ty: TypeExpr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnSig {
    pub is_pub: bool,
    pub is_async: bool,
    pub name: Ident,
    pub type_params: Vec<Ident>,
    pub params: Vec<Param>,
    pub return_ty: Option<TypeExpr>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceDecl {
    pub is_pub: bool,
    pub name: Ident,
    pub type_params: Vec<Ident>,
    pub methods: Vec<FnSig>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InitDecl {
    pub params: Vec<Param>,
    pub body: Block,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDecl {
    pub is_pub: bool,
    pub attributes: Vec<Attribute>,
    pub name: Ident,
    pub type_params: Vec<Ident>,
    pub implements: Vec<Ident>,
    pub fields: Vec<FieldDecl>,
    pub init: Option<InitDecl>,
    pub methods: Vec<FnDecl>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVariant {
    pub name: Ident,
    pub payload: Vec<TypeExpr>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDecl {
    pub is_pub: bool,
    pub name: Ident,
    pub type_params: Vec<Ident>,
    pub variants: Vec<EnumVariant>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExtendDecl {
    pub is_pub: bool,
    pub target_type: Ident,
    pub methods: Vec<FnDecl>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDecl {
    pub sig: FnSig,
    pub body: Block,
    pub span: Span,
}

/// Surface type annotation expression.
#[derive(Debug, Clone, PartialEq)]
pub enum TypeExpr {
    Named {
        name: Ident,
        args: Vec<TypeExpr>,
        span: Span,
    },
    Function {
        params: Vec<TypeExpr>,
        ret: Box<TypeExpr>,
        span: Span,
    },
}

impl TypeExpr {
    pub const fn span(&self) -> Span {
        match self {
            Self::Named { span, .. } | Self::Function { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElseBranch {
    Block(Block),
    If(Box<Stmt>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let {
        name: Ident,
        mutable: bool,
        ty: Option<TypeExpr>,
        value: Expr,
        span: Span,
    },
    Return {
        value: Option<Expr>,
        span: Span,
    },
    If {
        cond: Expr,
        then_branch: Block,
        else_branch: Option<ElseBranch>,
        span: Span,
    },
    While {
        cond: Expr,
        body: Block,
        span: Span,
    },
    For {
        var: Ident,
        iter: Expr,
        body: Block,
        span: Span,
    },
    Break {
        span: Span,
    },
    Continue {
        span: Span,
    },
    Const {
        name: Ident,
        ty: Option<TypeExpr>,
        value: Expr,
        span: Span,
    },
    CompoundAssign {
        target: Box<Expr>,
        op: CompoundAssignOp,
        value: Box<Expr>,
        span: Span,
    },
    TryCatch {
        try_block: Block,
        catch_param: Option<Ident>,
        catch_block: Block,
        finally_block: Option<Block>,
        span: Span,
    },
    Throw {
        value: Expr,
        span: Span,
    },
    Expr {
        expr: Expr,
        span: Span,
    },
}

impl Stmt {
    pub const fn span(&self) -> Span {
        match self {
            Self::Let { span, .. }
            | Self::Return { span, .. }
            | Self::If { span, .. }
            | Self::While { span, .. }
            | Self::For { span, .. }
            | Self::Break { span, .. }
            | Self::Continue { span, .. }
            | Self::Const { span, .. }
            | Self::CompoundAssign { span, .. }
            | Self::TryCatch { span, .. }
            | Self::Throw { span, .. }
            | Self::Expr { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompoundAssignOp {
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
}

impl CompoundAssignOp {
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::AddAssign => "+=",
            Self::SubAssign => "-=",
            Self::MulAssign => "*=",
            Self::DivAssign => "/=",
            Self::ModAssign => "%=",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
    Coalesce,
    Pipe,
    Range,
}

impl BinaryOp {
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Mod => "%",
            Self::Eq => "==",
            Self::Neq => "!=",
            Self::Lt => "<",
            Self::Lte => "<=",
            Self::Gt => ">",
            Self::Gte => ">=",
            Self::And => "&&",
            Self::Or => "||",
            Self::Coalesce => "??",
            Self::Pipe => "|>",
            Self::Range => "..",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldInit {
    pub name: Ident,
    pub value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClosureParam {
    pub name: Ident,
    pub ty: Option<TypeExpr>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClosureBody {
    Block(Block),
    Expr(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: ClosureBody,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Wildcard(Span),
    IntLit(i64, Span),
    StringLit(String, Span),
    BoolLit(bool, Span),
    Null(Span),
    Binding(Ident),
    Variant {
        enum_name: Option<Ident>,
        variant: Ident,
        bindings: Vec<Ident>,
        span: Span,
    },
}

impl Pattern {
    pub const fn span(&self) -> Span {
        match self {
            Self::Wildcard(s)
            | Self::IntLit(_, s)
            | Self::StringLit(_, s)
            | Self::BoolLit(_, s)
            | Self::Null(s) => *s,
            Self::Binding(id) => id.span,
            Self::Variant { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    IntLit {
        value: i64,
        span: Span,
    },
    FloatLit {
        value: f64,
        span: Span,
    },
    StringLit {
        value: String,
        span: Span,
    },
    BoolLit {
        value: bool,
        span: Span,
    },
    Null {
        span: Span,
    },
    This {
        span: Span,
    },
    Ident(Ident),
    Await {
        expr: Box<Expr>,
        span: Span,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
        span: Span,
    },
    Binary {
        op: BinaryOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        span: Span,
    },
    Assign {
        target: Box<Expr>,
        value: Box<Expr>,
        span: Span,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        span: Span,
    },
    FieldAccess {
        receiver: Box<Expr>,
        field: Ident,
        span: Span,
    },
    Index {
        receiver: Box<Expr>,
        index: Box<Expr>,
        span: Span,
    },
    StructLit {
        name: Ident,
        fields: Vec<FieldInit>,
        span: Span,
    },
    ListLit {
        elements: Vec<Expr>,
        span: Span,
    },
    Closure {
        params: Vec<ClosureParam>,
        return_ty: Option<TypeExpr>,
        body: ClosureBody,
        span: Span,
    },
    Match {
        scrutinee: Box<Expr>,
        arms: Vec<MatchArm>,
        span: Span,
    },
}

impl Expr {
    pub const fn span(&self) -> Span {
        match self {
            Self::IntLit { span, .. }
            | Self::FloatLit { span, .. }
            | Self::StringLit { span, .. }
            | Self::BoolLit { span, .. }
            | Self::Null { span }
            | Self::This { span }
            | Self::Await { span, .. }
            | Self::Unary { span, .. }
            | Self::Binary { span, .. }
            | Self::Assign { span, .. }
            | Self::Call { span, .. }
            | Self::FieldAccess { span, .. }
            | Self::Index { span, .. }
            | Self::StructLit { span, .. }
            | Self::ListLit { span, .. }
            | Self::Closure { span, .. }
            | Self::Match { span, .. } => *span,
            Self::Ident(id) => id.span,
        }
    }
}
