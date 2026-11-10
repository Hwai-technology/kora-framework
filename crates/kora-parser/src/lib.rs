//! Recursive-descent and Pratt expression parser for the Kora programming language.
//! Supports Phase 1 Foundation + Phase 5 Language Features (module, import, pub, Generics, extend, await).

use kora_ast::{
    Attribute, BinaryOp, Block, ClassDecl, ClosureBody, ClosureParam, ElseBranch, EnumDecl, EnumVariant, Expr,
    ExtendDecl, FieldDecl, FieldInit, FnDecl, FnSig, Ident, ImportDecl, InitDecl, InterfaceDecl,
    Item, MatchArm, ModelDecl, Module, ModulePathDecl, Param, Pattern, Stmt, StructDecl, TypeExpr,
    UnaryOp, Visibility,
};
use kora_diagnostics::{Diagnostic, DiagnosticBag};
use kora_lexer::{Token, TokenKind, tokenize};
use kora_span::{FileId, Span};

/// Lexes and parses `src` into a [`Module`], recording any lexical or syntax diagnostics.
pub fn parse_module(file_id: FileId, src: &str, diagnostics: &mut DiagnosticBag) -> Module {
    let tokens = tokenize(file_id, src, diagnostics);
    Parser::new(file_id, tokens, diagnostics).parse_module()
}

struct Parser<'a> {
    file_id: FileId,
    tokens: Vec<Token>,
    pos: usize,
    allow_struct_lit: bool,
    diagnostics: &'a mut DiagnosticBag,
}

impl<'a> Parser<'a> {
    fn new(file_id: FileId, tokens: Vec<Token>, diagnostics: &'a mut DiagnosticBag) -> Self {
        Self {
            file_id,
            tokens,
            pos: 0,
            allow_struct_lit: true,
            diagnostics,
        }
    }

    fn parse_module(&mut self) -> Module {
        let start = self.peek().span;
        let mut items = Vec::new();

        while !self.is_at_end() {
            if self.consume(&TokenKind::Semicolon) {
                continue;
            }
            match self.parse_item() {
                Some(item) => items.push(item),
                None => self.synchronize_item(),
            }
        }

        let end = self.peek().span;
        Module {
            file_id: self.file_id,
            items,
            span: start.merge(end),
        }
    }

    fn parse_item(&mut self) -> Option<Item> {
        let mut item_attrs = self.parse_attributes_opt();
        let is_pub = self.consume(&TokenKind::Pub);
        if item_attrs.is_empty() {
            item_attrs = self.parse_attributes_opt();
        }

        match &self.peek().kind {
            TokenKind::Module => {
                let mod_decl = self.parse_module_path_decl()?;
                Some(Item::ModuleDecl(mod_decl))
            }
            TokenKind::Import => {
                let imp = self.parse_import_decl()?;
                Some(Item::Import(imp))
            }
            TokenKind::Struct => {
                let kw = self.peek().clone();
                self.diagnostics.emit(
                    Diagnostic::error(
                        "E2003",
                        "`struct` has been removed from Kora. Use `model` for data schemas/DTOs, or `class` for services/logic.",
                    )
                    .with_primary_label(kw.span, "`struct` is forbidden; use `model` or `class`"),
                );
                self.parse_struct_decl(is_pub).map(Item::Struct)
            }
            TokenKind::Model => self.parse_model_decl(is_pub, item_attrs).map(Item::Model),
            TokenKind::Interface => self.parse_interface_decl(is_pub).map(Item::Interface),
            TokenKind::Class => self.parse_class_decl(is_pub, item_attrs).map(Item::Class),
            TokenKind::Enum => self.parse_enum_decl(is_pub).map(Item::Enum),
            TokenKind::Extend => self.parse_extend_decl(is_pub).map(Item::Extend),
            TokenKind::Fn => self.parse_fn_decl(is_pub, false).map(Item::Fn),
            TokenKind::Async if self.peek_n(1).kind == TokenKind::Fn => {
                let async_tok = self.advance();
                let mut decl = self.parse_fn_decl(is_pub, true)?;
                decl.sig.span = async_tok.span.merge(decl.sig.span);
                decl.span = async_tok.span.merge(decl.span);
                Some(Item::Fn(decl))
            }
            _ => {
                if !item_attrs.is_empty() {
                    self.diagnostics.emit(
                        Diagnostic::error("E2001", "attributes are only valid on model, class, function, or field declarations")
                            .with_primary_label(item_attrs[0].span, "unexpected attribute before statement"),
                    );
                    return None;
                }
                if is_pub {
                    self.diagnostics.emit(
                        Diagnostic::error("E2001", "`pub` modifier is only valid on declarations")
                            .with_primary_label(
                                self.peek().span,
                                "unexpected `pub` before statement",
                            ),
                    );
                }
                self.parse_stmt().map(Item::Stmt)
            }
        }
    }

    fn parse_module_path_decl(&mut self) -> Option<ModulePathDecl> {
        let kw = self.expect(&TokenKind::Module, "expected `module`")?;
        let segments = self.parse_path_segments()?;
        let end_span = segments.last().map(|s| s.span).unwrap_or(kw.span);
        let _ = self.consume(&TokenKind::Semicolon);
        Some(ModulePathDecl {
            span: kw.span.merge(end_span),
            segments,
        })
    }

    fn parse_import_decl(&mut self) -> Option<ImportDecl> {
        let kw = self.expect(&TokenKind::Import, "expected `import`")?;
        let mut symbols = Vec::new();
        let mut path_str: Option<String> = None;
        let mut segments = Vec::new();
        let end_span;

        if self.consume(&TokenKind::LBrace) {
            // import { A, B, C } from ...
            while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
                symbols.push(self.expect_ident("expected imported symbol name")?);
                if !self.consume(&TokenKind::Comma) {
                    break;
                }
            }
            self.expect(&TokenKind::RBrace, "expected `}` to close import list")?;
            self.expect(&TokenKind::From, "expected `from` after import symbol list")?;

            if let TokenKind::StringLit(s) = &self.peek().kind {
                let s_clone = s.clone();
                let tok = self.advance();
                end_span = tok.span;
                path_str = Some(s_clone);
            } else {
                segments = self.parse_path_segments()?;
                end_span = segments.last().map(|s| s.span).unwrap_or(kw.span);
            }
        } else if let TokenKind::StringLit(s) = &self.peek().kind {
            // import "path/to/file"
            let s_clone = s.clone();
            let tok = self.advance();
            end_span = tok.span;
            path_str = Some(s_clone);
        } else if matches!(self.peek().kind, TokenKind::Ident(_)) {
            let first_ident = self.expect_ident("expected identifier")?;
            if self.consume(&TokenKind::From) {
                // import A from "path"
                symbols.push(first_ident);
                if let TokenKind::StringLit(s) = &self.peek().kind {
                    let s_clone = s.clone();
                    let tok = self.advance();
                    end_span = tok.span;
                    path_str = Some(s_clone);
                } else {
                    segments = self.parse_path_segments()?;
                    end_span = segments.last().map(|s| s.span).unwrap_or(kw.span);
                }
            } else {
                // import path.to.module
                segments.push(first_ident);
                while self.consume(&TokenKind::Dot) {
                    segments.push(self.expect_ident("expected path segment after `.`")?);
                }
                end_span = segments.last().map(|s| s.span).unwrap_or(kw.span);
            }
        } else {
            self.diagnostics.emit(
                Diagnostic::error("E2001", "expected module path or `{ symbols }` after `import`")
                    .with_primary_label(kw.span, "invalid import syntax"),
            );
            return None;
        }

        let _ = self.consume(&TokenKind::Semicolon);
        Some(ImportDecl {
            symbols,
            path_str,
            segments,
            span: kw.span.merge(end_span),
        })
    }

    fn parse_path_segments(&mut self) -> Option<Vec<Ident>> {
        let mut segments = Vec::new();
        segments.push(self.expect_ident("expected path segment")?);
        while self.consume(&TokenKind::Dot) {
            segments.push(self.expect_ident("expected path segment after `.`")?);
        }
        Some(segments)
    }

    fn parse_type_params_opt(&mut self) -> Option<Vec<Ident>> {
        if self.consume(&TokenKind::Lt) {
            let mut params = Vec::new();
            while !self.check(&TokenKind::Gt) && !self.is_at_end() {
                params.push(self.expect_ident("expected generic type parameter name")?);
                if !self.consume(&TokenKind::Comma) {
                    break;
                }
            }
            self.expect(
                &TokenKind::Gt,
                "expected `>` to close generic type parameters",
            )?;
            Some(params)
        } else {
            Some(Vec::new())
        }
    }

    fn parse_struct_decl(&mut self, is_pub: bool) -> Option<StructDecl> {
        let kw = self.expect(&TokenKind::Struct, "expected `struct`")?;
        let name = self.expect_ident("expected struct name after `struct`")?;
        let type_params = self.parse_type_params_opt()?;
        let (_, fields, close_span) = self.parse_field_block()?;
        Some(StructDecl {
            is_pub,
            name,
            type_params,
            fields,
            span: kw.span.merge(close_span),
        })
    }

    fn parse_model_decl(&mut self, is_pub: bool, attributes: Vec<Attribute>) -> Option<ModelDecl> {
        let kw = self.expect(&TokenKind::Model, "expected `model`")?;
        let name = self.expect_ident("expected model name after `model`")?;
        let type_params = self.parse_type_params_opt()?;
        let (_, fields, close_span) = self.parse_field_block()?;
        let span = if let Some(first_attr) = attributes.first() {
            first_attr.span.merge(close_span)
        } else {
            kw.span.merge(close_span)
        };
        Some(ModelDecl {
            is_pub,
            attributes,
            name,
            type_params,
            fields,
            span,
        })
    }

    fn parse_attributes_opt(&mut self) -> Vec<Attribute> {
        let mut attrs = Vec::new();
        while self.consume(&TokenKind::At) {
            if let Some(ident) = self.expect_ident("expected attribute name after `@`") {
                let mut args = Vec::new();
                let mut end_span = ident.span;
                if self.consume(&TokenKind::LParen) {
                    while !self.check(&TokenKind::RParen) && !self.is_at_end() {
                        if let Some(expr) = self.parse_expr() {
                            args.push(expr);
                        }
                        if !self.consume(&TokenKind::Comma) {
                            break;
                        }
                    }
                    if let Some(r) = self.expect(&TokenKind::RParen, "expected `)` after attribute arguments") {
                        end_span = r.span;
                    }
                }
                attrs.push(Attribute {
                    name: ident.clone(),
                    args,
                    span: ident.span.merge(end_span),
                });
            }
        }
        attrs
    }

    fn parse_field_block(&mut self) -> Option<(Span, Vec<FieldDecl>, Span)> {
        let open = self.expect(&TokenKind::LBrace, "expected `{` to open field body")?;
        let mut fields = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            let attributes = self.parse_attributes_opt();
            let mut visibility = Visibility::Public;
            if self.consume(&TokenKind::Private) {
                visibility = Visibility::Private;
            } else if self.consume(&TokenKind::Protected) {
                visibility = Visibility::Protected;
            } else {
                let _ = self.consume(&TokenKind::Public) || self.consume(&TokenKind::Pub);
            }
            let is_readonly = self.consume(&TokenKind::Readonly);

            let fname = self.expect_ident("expected field name")?;
            self.expect(&TokenKind::Colon, "expected `:` after field name")?;
            let ty = self.parse_type_expr()?;
            let fspan = fname.span.merge(ty.span());
            let _ = self.consume(&TokenKind::Comma) || self.consume(&TokenKind::Semicolon);
            fields.push(FieldDecl {
                visibility,
                is_readonly,
                attributes,
                name: fname,
                ty,
                span: fspan,
            });
        }
        let close = self.expect(&TokenKind::RBrace, "expected `}` after field declarations")?;
        Some((open.span, fields, close.span))
    }

    fn parse_interface_decl(&mut self, is_pub: bool) -> Option<InterfaceDecl> {
        let kw = self.expect(&TokenKind::Interface, "expected `interface`")?;
        let name = self.expect_ident("expected interface name")?;
        let type_params = self.parse_type_params_opt()?;
        self.expect(&TokenKind::LBrace, "expected `{` after interface name")?;
        let mut methods = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            let m_pub = self.consume(&TokenKind::Pub);
            let is_async = self.consume(&TokenKind::Async);
            let sig = self.parse_fn_sig(m_pub, is_async)?;
            let _ = self.consume(&TokenKind::Semicolon);
            methods.push(sig);
        }
        let close = self.expect(&TokenKind::RBrace, "expected `}` to close interface")?;
        Some(InterfaceDecl {
            is_pub,
            name,
            type_params,
            methods,
            span: kw.span.merge(close.span),
        })
    }

    fn parse_class_decl(&mut self, is_pub: bool, attributes: Vec<Attribute>) -> Option<ClassDecl> {
        let kw = self.expect(&TokenKind::Class, "expected `class`")?;
        let name = self.expect_ident("expected class name after `class`")?;
        let type_params = self.parse_type_params_opt()?;
        let mut implements = Vec::new();
        if self.consume(&TokenKind::Implements) {
            loop {
                implements.push(self.expect_ident("expected interface name after `implements`")?);
                if !self.consume(&TokenKind::Comma) {
                    break;
                }
            }
        }
        self.expect(&TokenKind::LBrace, "expected `{` to open class body")?;

        let mut fields = Vec::new();
        let mut init = None;
        let mut methods = Vec::new();

        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            let attributes = self.parse_attributes_opt();
            let mut visibility = Visibility::Public;
            let mut is_pub_token = false;
            if self.consume(&TokenKind::Private) {
                visibility = Visibility::Private;
            } else if self.consume(&TokenKind::Protected) {
                visibility = Visibility::Protected;
            } else if self.consume(&TokenKind::Public) || self.consume(&TokenKind::Pub) {
                visibility = Visibility::Public;
                is_pub_token = true;
            }
            let is_readonly = self.consume(&TokenKind::Readonly);

            match &self.peek().kind {
                TokenKind::Init => {
                    let init_decl = self.parse_init_decl()?;
                    if init.is_some() {
                        self.diagnostics.emit(
                            Diagnostic::error("E2004", "duplicate `init` constructor in class")
                                .with_primary_label(
                                    init_decl.span,
                                    "a class may only define a single `init` constructor",
                                ),
                        );
                    } else {
                        init = Some(init_decl);
                    }
                }
                TokenKind::Fn => {
                    methods.push(self.parse_fn_decl(is_pub_token, false)?);
                }
                TokenKind::Async if self.peek_n(1).kind == TokenKind::Fn => {
                    let async_tok = self.advance();
                    let mut m = self.parse_fn_decl(is_pub_token, true)?;
                    m.sig.span = async_tok.span.merge(m.sig.span);
                    m.span = async_tok.span.merge(m.span);
                    methods.push(m);
                }
                TokenKind::Ident(_) => {
                    let fname = self.expect_ident("expected field name")?;
                    self.expect(&TokenKind::Colon, "expected `:` after class field name")?;
                    let ty = self.parse_type_expr()?;
                    let fspan = fname.span.merge(ty.span());
                    let _ = self.consume(&TokenKind::Comma) || self.consume(&TokenKind::Semicolon);
                    fields.push(FieldDecl {
                        visibility,
                        is_readonly,
                        attributes,
                        name: fname,
                        ty,
                        span: fspan,
                    });
                }
                _ => {
                    let bad = self.advance();
                    self.diagnostics.emit(
                        Diagnostic::error(
                            "E2001",
                            format!(
                                "unexpected {} inside `class` body; expected field, `init`, or `fn`",
                                bad.kind.description()
                            ),
                        )
                        .with_primary_label(bad.span, "unexpected token in class"),
                    );
                }
            }
        }

        // Auto-constructor parameter property promotion:
        if let Some(init_decl) = &init {
            for param in &init_decl.params {
                if !fields.iter().any(|f| f.name.name == param.name.name) {
                    fields.push(FieldDecl {
                        visibility: Visibility::Public,
                        is_readonly: false,
                        attributes: Vec::new(),
                        name: param.name.clone(),
                        ty: param.ty.clone(),
                        span: param.span,
                    });
                }
            }
        }

        // Auto-constructor synthesis if fields are declared but init is omitted:
        let init = if init.is_none() && !fields.is_empty() {
            let params = fields
                .iter()
                .map(|f| Param {
                    name: f.name.clone(),
                    ty: f.ty.clone(),
                    span: f.span,
                })
                .collect();
            let mut stmts = Vec::new();
            for f in &fields {
                stmts.push(Stmt::Expr {
                    expr: Expr::Assign {
                        target: Box::new(Expr::FieldAccess {
                            receiver: Box::new(Expr::This { span: f.span }),
                            field: f.name.clone(),
                            span: f.span,
                        }),
                        value: Box::new(Expr::Ident(f.name.clone())),
                        span: f.span,
                    },
                    span: f.span,
                });
            }
            Some(InitDecl {
                params,
                body: Block {
                    stmts,
                    span: kw.span,
                },
                span: kw.span,
            })
        } else {
            init
        };

        let close = self.expect(&TokenKind::RBrace, "expected `}` to close class body")?;
        let span = if let Some(first_attr) = attributes.first() {
            first_attr.span.merge(close.span)
        } else {
            kw.span.merge(close.span)
        };
        Some(ClassDecl {
            is_pub,
            attributes,
            name,
            type_params,
            implements,
            fields,
            init,
            methods,
            span,
        })
    }

    fn parse_extend_decl(&mut self, is_pub: bool) -> Option<ExtendDecl> {
        let kw = self.expect(&TokenKind::Extend, "expected `extend`")?;
        let target_type = self.expect_ident("expected target type name to extend")?;
        self.expect(&TokenKind::LBrace, "expected `{` after extend target type")?;

        let mut methods = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            let m_pub = self.consume(&TokenKind::Pub);
            let is_async = self.consume(&TokenKind::Async);
            methods.push(self.parse_fn_decl(m_pub, is_async)?);
        }
        let close = self.expect(&TokenKind::RBrace, "expected `}` to close extend block")?;
        Some(ExtendDecl {
            is_pub,
            target_type,
            methods,
            span: kw.span.merge(close.span),
        })
    }

    fn parse_init_decl(&mut self) -> Option<InitDecl> {
        let kw = self.expect(&TokenKind::Init, "expected `init`")?;
        self.expect(&TokenKind::LParen, "expected `(` after `init`")?;
        let params = self.parse_param_list()?;
        self.expect(&TokenKind::RParen, "expected `)` after `init` parameters")?;
        let body = self.parse_block()?;
        Some(InitDecl {
            params,
            span: kw.span.merge(body.span),
            body,
        })
    }

    fn parse_enum_decl(&mut self, is_pub: bool) -> Option<EnumDecl> {
        let kw = self.expect(&TokenKind::Enum, "expected `enum`")?;
        let name = self.expect_ident("expected enum name")?;
        let type_params = self.parse_type_params_opt()?;
        self.expect(&TokenKind::LBrace, "expected `{` after enum name")?;
        let mut variants = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            let vname = self.expect_ident("expected enum variant name")?;
            let mut end_span = vname.span;
            let mut payload = Vec::new();
            if self.consume(&TokenKind::LParen) {
                while !self.check(&TokenKind::RParen) && !self.is_at_end() {
                    payload.push(self.parse_type_expr()?);
                    if !self.consume(&TokenKind::Comma) {
                        break;
                    }
                }
                let close_p =
                    self.expect(&TokenKind::RParen, "expected `)` after variant types")?;
                end_span = close_p.span;
            }
            let _ = self.consume(&TokenKind::Comma) || self.consume(&TokenKind::Semicolon);
            variants.push(EnumVariant {
                name: vname,
                payload,
                span: end_span,
            });
        }
        let close = self.expect(&TokenKind::RBrace, "expected `}` to close enum")?;
        Some(EnumDecl {
            is_pub,
            name,
            type_params,
            variants,
            span: kw.span.merge(close.span),
        })
    }

    fn parse_fn_sig(&mut self, is_pub: bool, is_async: bool) -> Option<FnSig> {
        let kw = self.expect(&TokenKind::Fn, "expected `fn`")?;
        let name = self.expect_ident("expected function name")?;
        let type_params = self.parse_type_params_opt()?;
        self.expect(&TokenKind::LParen, "expected `(` after function name")?;
        let params = self.parse_param_list()?;
        let rparen = self.expect(&TokenKind::RParen, "expected `)` after parameters")?;
        let mut end_span = rparen.span;
        let return_ty = if self.consume(&TokenKind::ThinArrow) {
            let ty = self.parse_type_expr()?;
            end_span = ty.span();
            Some(ty)
        } else {
            None
        };
        Some(FnSig {
            is_pub,
            is_async,
            name,
            type_params,
            params,
            return_ty,
            span: kw.span.merge(end_span),
        })
    }

    fn parse_fn_decl(&mut self, is_pub: bool, is_async: bool) -> Option<FnDecl> {
        let sig = self.parse_fn_sig(is_pub, is_async)?;
        let body = self.parse_block()?;
        let span = sig.span.merge(body.span);
        Some(FnDecl { sig, body, span })
    }

    fn parse_param_list(&mut self) -> Option<Vec<Param>> {
        let mut params = Vec::new();
        while !self.check(&TokenKind::RParen) && !self.is_at_end() {
            let _ = self.consume(&TokenKind::Pub)
                || self.consume(&TokenKind::Public)
                || self.consume(&TokenKind::Private)
                || self.consume(&TokenKind::Protected);
            let _ = self.consume(&TokenKind::Readonly);
            let pname = self.expect_ident("expected parameter name")?;
            self.expect(&TokenKind::Colon, "expected `:` after parameter name")?;
            let ty = self.parse_type_expr()?;
            let pspan = pname.span.merge(ty.span());
            params.push(Param {
                name: pname,
                ty,
                span: pspan,
            });
            if !self.consume(&TokenKind::Comma) {
                break;
            }
        }
        Some(params)
    }

    fn parse_type_expr(&mut self) -> Option<TypeExpr> {
        if self.check(&TokenKind::LParen) {
            let open = self.advance();
            let mut params = Vec::new();
            while !self.check(&TokenKind::RParen) && !self.is_at_end() {
                params.push(self.parse_type_expr()?);
                if !self.consume(&TokenKind::Comma) {
                    break;
                }
            }
            self.expect(&TokenKind::RParen, "expected `)` in function type")?;
            self.expect(&TokenKind::ThinArrow, "expected `->` in function type")?;
            let ret = self.parse_type_expr()?;
            let span = open.span.merge(ret.span());
            return Some(TypeExpr::Function {
                params,
                ret: Box::new(ret),
                span,
            });
        }

        if self.check(&TokenKind::Undefined) {
            let tok = self.advance();
            self.diagnostics.emit(
                Diagnostic::error(
                    "E2005",
                    "`undefined` is not permitted in Kora; use `Option<T>` or `null` exclusively.",
                )
                .with_primary_label(tok.span, "`undefined` is forbidden; use `null`"),
            );
            return None;
        }

        let name = self.expect_ident("expected type name")?;
        let mut end_span = name.span;
        let mut args = Vec::new();
        if self.consume(&TokenKind::Lt) {
            while !self.check(&TokenKind::Gt) && !self.is_at_end() {
                args.push(self.parse_type_expr()?);
                if !self.consume(&TokenKind::Comma) {
                    break;
                }
            }
            let gt = self.expect(
                &TokenKind::Gt,
                "expected `>` to close generic type arguments",
            )?;
            end_span = gt.span;
        }
        Some(TypeExpr::Named {
            name,
            args,
            span: end_span,
        })
    }

    fn parse_block(&mut self) -> Option<Block> {
        let open = self.expect(&TokenKind::LBrace, "expected `{` to start block")?;
        let mut stmts = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            if self.consume(&TokenKind::Semicolon) {
                continue;
            }
            match self.parse_stmt() {
                Some(stmt) => stmts.push(stmt),
                None => self.synchronize_stmt(),
            }
        }
        let close = self.expect(&TokenKind::RBrace, "expected `}` to close block")?;
        Some(Block {
            stmts,
            span: open.span.merge(close.span),
        })
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match &self.peek().kind {
            TokenKind::Let => self.parse_let_or_var_stmt(false),
            TokenKind::Var => self.parse_let_or_var_stmt(true),
            TokenKind::Return => self.parse_return_stmt(),
            TokenKind::If => self.parse_if_stmt(),
            TokenKind::While => self.parse_while_stmt(),
            _ => {
                let expr = self.parse_expr()?;
                let span = expr.span();
                let _ = self.consume(&TokenKind::Semicolon);
                Some(Stmt::Expr { expr, span })
            }
        }
    }

    fn parse_let_or_var_stmt(&mut self, mutable: bool) -> Option<Stmt> {
        let kw = self.advance();
        let name = self.expect_ident("expected variable name")?;
        let ty = if self.consume(&TokenKind::Colon) {
            Some(self.parse_type_expr()?)
        } else {
            None
        };
        self.expect(&TokenKind::Eq, "expected `=` in variable binding")?;
        let value = self.parse_expr()?;
        let span = kw.span.merge(value.span());
        let _ = self.consume(&TokenKind::Semicolon);
        Some(Stmt::Let {
            name,
            mutable,
            ty,
            value,
            span,
        })
    }

    fn parse_return_stmt(&mut self) -> Option<Stmt> {
        let kw = self.expect(&TokenKind::Return, "expected `return`")?;
        let has_no_value = self.peek().had_newline_before
            || self.check(&TokenKind::Semicolon)
            || self.check(&TokenKind::RBrace)
            || self.is_at_end();

        let (value, end_span) = if has_no_value {
            (None, kw.span)
        } else {
            let expr = self.parse_expr()?;
            let sp = expr.span();
            (Some(expr), sp)
        };
        let _ = self.consume(&TokenKind::Semicolon);
        Some(Stmt::Return {
            value,
            span: kw.span.merge(end_span),
        })
    }

    fn parse_if_stmt(&mut self) -> Option<Stmt> {
        let kw = self.expect(&TokenKind::If, "expected `if`")?;
        let cond = self.with_struct_lit(false, Self::parse_expr)?;
        let then_branch = self.parse_block()?;
        let mut end_span = then_branch.span;
        let else_branch = if self.consume(&TokenKind::Else) {
            if self.check(&TokenKind::If) {
                let nested_if = self.parse_if_stmt()?;
                end_span = nested_if.span();
                Some(ElseBranch::If(Box::new(nested_if)))
            } else {
                let blk = self.parse_block()?;
                end_span = blk.span;
                Some(ElseBranch::Block(blk))
            }
        } else {
            None
        };

        Some(Stmt::If {
            cond,
            then_branch,
            else_branch,
            span: kw.span.merge(end_span),
        })
    }

    fn parse_while_stmt(&mut self) -> Option<Stmt> {
        let kw = self.expect(&TokenKind::While, "expected `while`")?;
        let cond = self.with_struct_lit(false, Self::parse_expr)?;
        let body = self.parse_block()?;
        let span = kw.span.merge(body.span);
        Some(Stmt::While { cond, body, span })
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_assign_expr()
    }

    fn parse_assign_expr(&mut self) -> Option<Expr> {
        let lhs = self.parse_binary_expr(0)?;
        if self.consume(&TokenKind::Eq) {
            let rhs = self.parse_assign_expr()?;
            let span = lhs.span().merge(rhs.span());
            return Some(Expr::Assign {
                target: Box::new(lhs),
                value: Box::new(rhs),
                span,
            });
        }
        Some(lhs)
    }

    fn parse_binary_expr(&mut self, min_prec: u8) -> Option<Expr> {
        let mut lhs = self.parse_unary_expr()?;

        while let Some((op, prec)) = self.current_binary_op() {
            if prec < min_prec {
                break;
            }
            self.advance();
            let rhs = self.parse_binary_expr(prec + 1)?;
            let span = lhs.span().merge(rhs.span());
            lhs = Expr::Binary {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                span,
            };
        }

        Some(lhs)
    }

    fn current_binary_op(&self) -> Option<(BinaryOp, u8)> {
        match &self.peek().kind {
            TokenKind::OrOr => Some((BinaryOp::Or, 1)),
            TokenKind::AndAnd => Some((BinaryOp::And, 2)),
            TokenKind::EqEq => Some((BinaryOp::Eq, 3)),
            TokenKind::NotEq => Some((BinaryOp::Neq, 3)),
            TokenKind::Lt => Some((BinaryOp::Lt, 4)),
            TokenKind::LtEq => Some((BinaryOp::Lte, 4)),
            TokenKind::Gt => Some((BinaryOp::Gt, 4)),
            TokenKind::GtEq => Some((BinaryOp::Gte, 4)),
            TokenKind::Plus => Some((BinaryOp::Add, 5)),
            TokenKind::Minus => Some((BinaryOp::Sub, 5)),
            TokenKind::Star => Some((BinaryOp::Mul, 6)),
            TokenKind::Slash => Some((BinaryOp::Div, 6)),
            TokenKind::Percent => Some((BinaryOp::Mod, 6)),
            _ => None,
        }
    }

    fn parse_unary_expr(&mut self) -> Option<Expr> {
        if self.check(&TokenKind::Await) {
            let op_tok = self.advance();
            let inner = self.parse_unary_expr()?;
            let span = op_tok.span.merge(inner.span());
            return Some(Expr::Await {
                expr: Box::new(inner),
                span,
            });
        }
        if self.check(&TokenKind::Bang) {
            let op_tok = self.advance();
            let inner = self.parse_unary_expr()?;
            let span = op_tok.span.merge(inner.span());
            return Some(Expr::Unary {
                op: UnaryOp::Not,
                expr: Box::new(inner),
                span,
            });
        }
        if self.check(&TokenKind::Minus) {
            let op_tok = self.advance();
            let inner = self.parse_unary_expr()?;
            let span = op_tok.span.merge(inner.span());
            return Some(Expr::Unary {
                op: UnaryOp::Neg,
                expr: Box::new(inner),
                span,
            });
        }
        self.parse_postfix_expr()
    }

    fn parse_postfix_expr(&mut self) -> Option<Expr> {
        let mut expr = self.parse_primary_expr()?;

        loop {
            if self.check(&TokenKind::LParen) && !self.peek().had_newline_before {
                self.advance();
                let mut args = Vec::new();
                while !self.check(&TokenKind::RParen) && !self.is_at_end() {
                    let arg = self.with_struct_lit(true, Self::parse_expr)?;
                    args.push(arg);
                    if !self.consume(&TokenKind::Comma) {
                        break;
                    }
                }
                let close = self.expect(&TokenKind::RParen, "expected `)` after call arguments")?;
                let span = expr.span().merge(close.span);
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                    span,
                };
            } else if self.consume(&TokenKind::Dot) {
                let field = self.expect_ident("expected field or method name after `.`")?;
                let span = expr.span().merge(field.span);
                expr = Expr::FieldAccess {
                    receiver: Box::new(expr),
                    field,
                    span,
                };
            } else if self.check(&TokenKind::LBracket) && !self.peek().had_newline_before {
                self.advance();
                let index = self.with_struct_lit(true, Self::parse_expr)?;
                let close = self.expect(&TokenKind::RBracket, "expected `]` after index")?;
                let span = expr.span().merge(close.span);
                expr = Expr::Index {
                    receiver: Box::new(expr),
                    index: Box::new(index),
                    span,
                };
            } else {
                break;
            }
        }

        Some(expr)
    }

    fn parse_primary_expr(&mut self) -> Option<Expr> {
        let tok = self.peek().clone();
        match tok.kind {
            TokenKind::IntLit(value) => {
                self.advance();
                Some(Expr::IntLit {
                    value,
                    span: tok.span,
                })
            }
            TokenKind::FloatLit(value) => {
                self.advance();
                Some(Expr::FloatLit {
                    value,
                    span: tok.span,
                })
            }
            TokenKind::StringLit(value) => {
                self.advance();
                Some(Expr::StringLit {
                    value,
                    span: tok.span,
                })
            }
            TokenKind::True => {
                self.advance();
                Some(Expr::BoolLit {
                    value: true,
                    span: tok.span,
                })
            }
            TokenKind::False => {
                self.advance();
                Some(Expr::BoolLit {
                    value: false,
                    span: tok.span,
                })
            }
            TokenKind::Null => {
                self.advance();
                Some(Expr::Null { span: tok.span })
            }
            TokenKind::Undefined => {
                let tok = self.advance();
                self.diagnostics.emit(
                    Diagnostic::error(
                        "E2005",
                        "`undefined` is not permitted in Kora; use `null` exclusively to represent absence of value.",
                    )
                    .with_primary_label(tok.span, "`undefined` is forbidden; use `null`"),
                );
                Some(Expr::Null { span: tok.span })
            }
            TokenKind::This => {
                self.advance();
                Some(Expr::This { span: tok.span })
            }
            TokenKind::Match => self.parse_match_expr(),
            TokenKind::LBracket => self.parse_list_lit(),
            TokenKind::LParen => {
                if self.is_closure_start() {
                    self.parse_closure_expr()
                } else {
                    self.advance();
                    let inner = self.with_struct_lit(true, Self::parse_expr)?;
                    self.expect(&TokenKind::RParen, "expected `)` after expression")?;
                    Some(inner)
                }
            }
            TokenKind::Ident(name) => {
                self.advance();
                let ident = Ident::new(name, tok.span);
                if self.allow_struct_lit
                    && self.check(&TokenKind::LBrace)
                    && !self.peek().had_newline_before
                    && self.looks_like_struct_lit_body()
                {
                    self.parse_struct_lit(ident)
                } else {
                    Some(Expr::Ident(ident))
                }
            }
            _ => {
                let bad = self.advance();
                self.diagnostics.emit(
                    Diagnostic::error(
                        "E2002",
                        format!("expected expression, found {}", bad.kind.description()),
                    )
                    .with_primary_label(bad.span, "expected an expression here"),
                );
                None
            }
        }
    }

    fn parse_struct_lit(&mut self, name: Ident) -> Option<Expr> {
        let start = name.span;
        self.expect(&TokenKind::LBrace, "expected `{` in struct literal")?;
        let mut fields = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            let fname = self.expect_ident("expected field name in struct literal")?;
            self.expect(&TokenKind::Colon, "expected `:` after struct field name")?;
            let value = self.with_struct_lit(true, Self::parse_expr)?;
            let fspan = fname.span.merge(value.span());
            fields.push(FieldInit {
                name: fname,
                value,
                span: fspan,
            });
            if !self.consume(&TokenKind::Comma) {
                break;
            }
        }
        let close = self.expect(&TokenKind::RBrace, "expected `}` to close struct literal")?;
        Some(Expr::StructLit {
            name,
            fields,
            span: start.merge(close.span),
        })
    }

    fn parse_list_lit(&mut self) -> Option<Expr> {
        let open = self.expect(&TokenKind::LBracket, "expected `[`")?;
        let mut elements = Vec::new();
        while !self.check(&TokenKind::RBracket) && !self.is_at_end() {
            elements.push(self.with_struct_lit(true, Self::parse_expr)?);
            if !self.consume(&TokenKind::Comma) {
                break;
            }
        }
        let close = self.expect(&TokenKind::RBracket, "expected `]` to close list literal")?;
        Some(Expr::ListLit {
            elements,
            span: open.span.merge(close.span),
        })
    }

    fn parse_closure_expr(&mut self) -> Option<Expr> {
        let open = self.expect(&TokenKind::LParen, "expected `(` to start closure")?;
        let mut params = Vec::new();
        while !self.check(&TokenKind::RParen) && !self.is_at_end() {
            let pname = self.expect_ident("expected closure parameter name")?;
            let mut end_span = pname.span;
            let ty = if self.consume(&TokenKind::Colon) {
                let t = self.parse_type_expr()?;
                end_span = t.span();
                Some(t)
            } else {
                None
            };
            params.push(ClosureParam {
                name: pname,
                ty,
                span: end_span,
            });
            if !self.consume(&TokenKind::Comma) {
                break;
            }
        }
        self.expect(&TokenKind::RParen, "expected `)` after closure parameters")?;
        let return_ty = if self.consume(&TokenKind::ThinArrow) {
            Some(self.parse_type_expr()?)
        } else {
            None
        };
        self.expect(&TokenKind::FatArrow, "expected `=>` in closure")?;

        let (body, end_span) = if self.check(&TokenKind::LBrace) {
            let blk = self.parse_block()?;
            let sp = blk.span;
            (ClosureBody::Block(blk), sp)
        } else {
            let expr = self.parse_expr()?;
            let sp = expr.span();
            (ClosureBody::Expr(Box::new(expr)), sp)
        };

        Some(Expr::Closure {
            params,
            return_ty,
            body,
            span: open.span.merge(end_span),
        })
    }

    fn parse_match_expr(&mut self) -> Option<Expr> {
        let kw = self.expect(&TokenKind::Match, "expected `match`")?;
        let scrutinee = self.with_struct_lit(false, Self::parse_expr)?;
        self.expect(&TokenKind::LBrace, "expected `{` after `match` expression")?;
        let mut arms = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            let pattern = self.parse_pattern()?;
            self.expect(&TokenKind::FatArrow, "expected `=>` after match pattern")?;
            let (body, end_span) = if self.check(&TokenKind::LBrace) {
                let blk = self.parse_block()?;
                let sp = blk.span;
                (ClosureBody::Block(blk), sp)
            } else {
                let expr = self.with_struct_lit(true, Self::parse_expr)?;
                let sp = expr.span();
                (ClosureBody::Expr(Box::new(expr)), sp)
            };
            let _ = self.consume(&TokenKind::Comma) || self.consume(&TokenKind::Semicolon);
            arms.push(MatchArm {
                span: pattern.span().merge(end_span),
                pattern,
                body,
            });
        }
        let close = self.expect(&TokenKind::RBrace, "expected `}` to close `match`")?;
        Some(Expr::Match {
            scrutinee: Box::new(scrutinee),
            arms,
            span: kw.span.merge(close.span),
        })
    }

    fn parse_pattern(&mut self) -> Option<Pattern> {
        let tok = self.peek().clone();
        match tok.kind {
            TokenKind::Underscore => {
                self.advance();
                Some(Pattern::Wildcard(tok.span))
            }
            TokenKind::IntLit(v) => {
                self.advance();
                Some(Pattern::IntLit(v, tok.span))
            }
            TokenKind::StringLit(s) => {
                self.advance();
                Some(Pattern::StringLit(s, tok.span))
            }
            TokenKind::True => {
                self.advance();
                Some(Pattern::BoolLit(true, tok.span))
            }
            TokenKind::False => {
                self.advance();
                Some(Pattern::BoolLit(false, tok.span))
            }
            TokenKind::Null => {
                self.advance();
                Some(Pattern::Null(tok.span))
            }
            TokenKind::Undefined => {
                let tok = self.advance();
                self.diagnostics.emit(
                    Diagnostic::error(
                        "E2005",
                        "`undefined` is not permitted in Kora; use `null` exclusively to represent absence of value.",
                    )
                    .with_primary_label(tok.span, "`undefined` is forbidden; use `null`"),
                );
                Some(Pattern::Null(tok.span))
            }
            TokenKind::Ident(first_name) => {
                self.advance();
                let first = Ident::new(first_name, tok.span);
                let (enum_name, variant) = if self.consume(&TokenKind::Dot) {
                    let v = self.expect_ident("expected variant name after `.`")?;
                    (Some(first), v)
                } else {
                    (None, first)
                };

                if self.consume(&TokenKind::LParen) {
                    let mut bindings = Vec::new();
                    while !self.check(&TokenKind::RParen) && !self.is_at_end() {
                        if self.check(&TokenKind::Underscore) {
                            let u = self.advance();
                            bindings.push(Ident::new("_", u.span));
                        } else {
                            bindings.push(self.expect_ident("expected pattern binding variable")?);
                        }
                        if !self.consume(&TokenKind::Comma) {
                            break;
                        }
                    }
                    let close =
                        self.expect(&TokenKind::RParen, "expected `)` in variant pattern")?;
                    let start = enum_name.as_ref().map(|e| e.span).unwrap_or(variant.span);
                    Some(Pattern::Variant {
                        enum_name,
                        variant,
                        bindings,
                        span: start.merge(close.span),
                    })
                } else if let Some(en) = enum_name {
                    let sp = en.span.merge(variant.span);
                    Some(Pattern::Variant {
                        enum_name: Some(en),
                        variant,
                        bindings: Vec::new(),
                        span: sp,
                    })
                } else {
                    Some(Pattern::Binding(variant))
                }
            }
            _ => {
                let bad = self.advance();
                self.diagnostics.emit(
                    Diagnostic::error(
                        "E2003",
                        format!(
                            "expected pattern in `match`, found {}",
                            bad.kind.description()
                        ),
                    )
                    .with_primary_label(bad.span, "invalid match pattern"),
                );
                None
            }
        }
    }

    fn looks_like_struct_lit_body(&self) -> bool {
        match &self.peek_n(1).kind {
            TokenKind::RBrace => true,
            TokenKind::Ident(_) => self.peek_n(2).kind == TokenKind::Colon,
            _ => false,
        }
    }

    fn is_closure_start(&self) -> bool {
        if self.peek_n(1).kind == TokenKind::RParen {
            return matches!(
                self.peek_n(2).kind,
                TokenKind::FatArrow | TokenKind::ThinArrow
            );
        }
        if matches!(self.peek_n(1).kind, TokenKind::Ident(_))
            && self.peek_n(2).kind == TokenKind::Colon
        {
            return true;
        }

        let mut idx = 1;
        loop {
            match &self.peek_n(idx).kind {
                TokenKind::Ident(_) => idx += 1,
                _ => return false,
            }
            match &self.peek_n(idx).kind {
                TokenKind::Comma => {
                    idx += 1;
                    if self.peek_n(idx).kind == TokenKind::RParen {
                        idx += 1;
                        break;
                    }
                }
                TokenKind::RParen => {
                    idx += 1;
                    break;
                }
                _ => return false,
            }
        }
        matches!(
            self.peek_n(idx).kind,
            TokenKind::FatArrow | TokenKind::ThinArrow
        )
    }

    fn with_struct_lit<T>(&mut self, allow: bool, f: fn(&mut Self) -> Option<T>) -> Option<T> {
        let prev = self.allow_struct_lit;
        self.allow_struct_lit = allow;
        let out = f(self);
        self.allow_struct_lit = prev;
        out
    }

    fn synchronize_item(&mut self) {
        while !self.is_at_end() {
            match &self.peek().kind {
                TokenKind::Module
                | TokenKind::Import
                | TokenKind::Pub
                | TokenKind::Struct
                | TokenKind::Model
                | TokenKind::Interface
                | TokenKind::Class
                | TokenKind::Enum
                | TokenKind::Extend
                | TokenKind::Fn
                | TokenKind::Async => return,
                _ => {
                    if self.peek().had_newline_before {
                        return;
                    }
                    self.advance();
                }
            }
        }
    }

    fn synchronize_stmt(&mut self) {
        while !self.is_at_end() {
            if self.check(&TokenKind::RBrace) {
                return;
            }
            if self.consume(&TokenKind::Semicolon) {
                return;
            }
            if self.peek().had_newline_before {
                return;
            }
            self.advance();
        }
    }

    fn expect(&mut self, expected: &TokenKind, msg: &str) -> Option<Token> {
        if self.check(expected) {
            Some(self.advance())
        } else {
            let found = self.peek().clone();
            self.diagnostics.emit(
                Diagnostic::error(
                    "E2001",
                    format!("{msg}, found {}", found.kind.description()),
                )
                .with_primary_label(found.span, msg),
            );
            None
        }
    }

    fn expect_ident(&mut self, msg: &str) -> Option<Ident> {
        let tok = self.peek().clone();
        if let TokenKind::Ident(name) = tok.kind {
            self.advance();
            Some(Ident::new(name, tok.span))
        } else {
            self.diagnostics.emit(
                Diagnostic::error("E2001", format!("{msg}, found {}", tok.kind.description()))
                    .with_primary_label(tok.span, msg),
            );
            None
        }
    }

    fn consume(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, expected: &TokenKind) -> bool {
        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(expected)
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }

    fn peek(&self) -> &Token {
        self.peek_n(0)
    }

    fn peek_n(&self, offset: usize) -> &Token {
        self.tokens.get(self.pos + offset).unwrap_or_else(|| {
            self.tokens
                .last()
                .expect("token stream always ends with Eof")
        })
    }

    fn advance(&mut self) -> Token {
        let tok = self.peek().clone();
        if !self.is_at_end() {
            self.pos += 1;
        }
        tok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hybrid_class_model_interface_and_closures() {
        let src = r#"
model User {
    id: Int
    name: String
}

interface UserRepo {
    fn find(id: Int) -> Option<User>
}

class UserService implements UserRepo {
    db: Database

    init(db: Database) {
        this.db = db
    }

    fn find(id: Int) -> Option<User> {
        if id <= 0 {
            return None
        }
        return Some(User { id: id, name: "Ahmet" })
    }
}

get("/users/{id}", (id: Int, svc: UserService) => {
    return svc.find(id)
})
"#;
        let mut bag = DiagnosticBag::new();
        let module = parse_module(FileId(0), src, &mut bag);
        assert!(
            !bag.has_errors(),
            "unexpected parse errors: {:?}",
            bag.items()
        );
        assert_eq!(module.items.len(), 4);
    }

    #[test]
    fn parses_generics_modules_extend_and_await() {
        let src = r#"
module core.users
import core.db

pub model Page<T> {
    items: List<T>
}

pub extend Int {
    fn double() -> Int {
        return this * 2
    }
}

async fn fetch() -> Int {
    let x = await someTask()
    return x
}
"#;
        let mut bag = DiagnosticBag::new();
        let module = parse_module(FileId(0), src, &mut bag);
        assert!(
            !bag.has_errors(),
            "unexpected parse errors: {:?}",
            bag.items()
        );
        assert_eq!(module.items.len(), 5);
    }

    #[test]
    fn parses_import_from_and_destructuring() {
        let src = r#"
import { User, HttpStatus } from "models/user_model";
import OrderService from "services/order_service";
import "middleware/cors";
"#;
        let mut bag = DiagnosticBag::new();
        let module = parse_module(FileId(0), src, &mut bag);
        assert!(!bag.has_errors(), "unexpected parse errors: {:?}", bag.items());
        assert_eq!(module.items.len(), 3);
        if let Item::Import(imp) = &module.items[0] {
            assert_eq!(imp.symbols.len(), 2);
            assert_eq!(imp.symbols[0].name, "User");
            assert_eq!(imp.symbols[1].name, "HttpStatus");
            assert_eq!(imp.path_str.as_deref(), Some("models/user_model"));
        } else {
            panic!("expected Import");
        }
    }

    #[test]
    fn parses_visibility_attributes_and_auto_constructor() {
        let src = r#"
model Account {
    @unique
    @required
    id: Int

    @unique
    @email
    email: String

    @min(0)
    balance: Int
}

class AuthService {
    public repo: String
    private readonly secret: String
    protected counter: Int

    public fn verify() -> Bool {
        return true
    }
}

class AutoService {
    init(public pool: String, private token: String) {
    }
}
"#;
        let mut bag = DiagnosticBag::new();
        let module = parse_module(FileId(0), src, &mut bag);
        assert!(!bag.has_errors(), "unexpected parse errors: {:?}", bag.items());
        assert_eq!(module.items.len(), 3);

        // Check Account attributes
        if let Item::Model(m) = &module.items[0] {
            assert_eq!(m.fields[0].attributes.len(), 2);
            assert_eq!(m.fields[0].attributes[0].name.name, "unique");
            assert_eq!(m.fields[0].attributes[1].name.name, "required");
            assert_eq!(m.fields[1].attributes.len(), 2);
            assert_eq!(m.fields[1].attributes[1].name.name, "email");
            assert_eq!(m.fields[2].attributes.len(), 1);
            assert_eq!(m.fields[2].attributes[0].name.name, "min");
        } else {
            panic!("expected Model");
        }

        // Check AuthService visibility and auto-constructor synthesis
        if let Item::Class(c) = &module.items[1] {
            assert_eq!(c.fields[0].visibility, Visibility::Public);
            assert_eq!(c.fields[1].visibility, Visibility::Private);
            assert!(c.fields[1].is_readonly);
            assert_eq!(c.fields[2].visibility, Visibility::Protected);
            assert!(c.init.is_some(), "auto-constructor should be synthesized");
            assert_eq!(c.init.as_ref().unwrap().params.len(), 3);
        } else {
            panic!("expected Class");
        }

        // Check AutoService parameter property promotion
        if let Item::Class(c) = &module.items[2] {
            assert_eq!(c.fields.len(), 2, "params in init should be promoted to fields");
            assert_eq!(c.fields[0].name.name, "pool");
            assert_eq!(c.fields[1].name.name, "token");
        } else {
            panic!("expected Class");
        }
    }
}
