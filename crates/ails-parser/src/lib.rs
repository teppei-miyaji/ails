use ails_ast::{BinaryOp, Effect, Expr, Function, MatchArm, Module, Param, Pattern, PrimitiveType, Stmt, TypeExpr};
use ails_lexer::{lex, Token, TokenKind};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error(transparent)]
    Lex(#[from] ails_lexer::LexError),
    #[error("expected {expected} at token index {index}, found {found:?}")]
    Expected { expected: &'static str, found: TokenKind, index: usize },
    #[error("invalid integer literal `{0}`")]
    InvalidInt(String),
}

pub fn parse_module(input: &str) -> Result<Module, ParseError> {
    let tokens = lex(input)?;
    let mut p = Parser { tokens, index: 0 };
    p.parse_module()
}

struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    fn parse_module(&mut self) -> Result<Module, ParseError> {
        self.skip_newlines();
        self.expect_keyword(TokenKind::Module, "`module`")?;
        let name = self.parse_qualified_name()?;
        self.expect_newline()?;
        self.skip_newlines();

        let mut functions = Vec::new();
        while !self.at_eof() {
            self.skip_newlines();
            if self.at_eof() { break; }
            functions.push(self.parse_function()?);
            self.skip_newlines();
        }

        Ok(Module { name, functions })
    }

    fn parse_function(&mut self) -> Result<Function, ParseError> {
        self.expect_keyword(TokenKind::Func, "`func`")?;
        let name = self.parse_ident()?;
        self.expect_newline()?;

        let mut inputs = Vec::new();
        let mut output = None;
        let mut effects = None;

        loop {
            self.skip_newlines();
            match self.peek() {
                TokenKind::Input => inputs.push(self.parse_input()?),
                TokenKind::Output => output = Some(self.parse_output()?),
                TokenKind::Effect => effects = Some(self.parse_effects()?),
                TokenKind::Begin => break,
                other => return Err(ParseError::Expected {
                    expected: "`input`, `output`, `effect`, or `begin`",
                    found: other.clone(),
                    index: self.index,
                }),
            }
        }

        self.expect_keyword(TokenKind::Begin, "`begin`")?;
        self.expect_newline()?;

        let body = self.parse_stmt_block(&[TokenKind::End])?;

        self.expect_keyword(TokenKind::End, "`end`")?;
        self.expect_optional_newline();

        Ok(Function {
            name,
            inputs,
            output: output.ok_or_else(|| ParseError::Expected {
                expected: "function output declaration",
                found: self.peek().clone(),
                index: self.index,
            })?,
            effects: effects.ok_or_else(|| ParseError::Expected {
                expected: "function effect declaration",
                found: self.peek().clone(),
                index: self.index,
            })?,
            body,
        })
    }

    fn parse_stmt_block(&mut self, terminators: &[TokenKind]) -> Result<Vec<Stmt>, ParseError> {
        let mut body = Vec::new();
        loop {
            self.skip_newlines();
            if self.at_any(terminators) { break; }

            let stmt = match self.peek() {
                TokenKind::Return => self.parse_return()?,
                TokenKind::If => self.parse_if()?,
                TokenKind::While => self.parse_while()?,
                TokenKind::Match => self.parse_match()?,
                other => return Err(ParseError::Expected {
                    expected: "`return`, `if`, `while`, `match`, or block terminator",
                    found: other.clone(),
                    index: self.index,
                }),
            };
            body.push(stmt);
        }
        Ok(body)
    }

    fn parse_input(&mut self) -> Result<Param, ParseError> {
        self.expect_keyword(TokenKind::Input, "`input`")?;
        let name = self.parse_ident()?;
        self.expect_keyword(TokenKind::Colon, "`:`")?;
        let ty = self.parse_type()?;
        self.expect_newline()?;
        Ok(Param { name, ty })
    }

    fn parse_output(&mut self) -> Result<TypeExpr, ParseError> {
        self.expect_keyword(TokenKind::Output, "`output`")?;
        let ty = self.parse_type()?;
        self.expect_newline()?;
        Ok(ty)
    }

    fn parse_effects(&mut self) -> Result<Vec<Effect>, ParseError> {
        self.expect_keyword(TokenKind::Effect, "`effect`")?;
        let mut effects = vec![self.parse_effect()?];
        loop {
            match self.peek() {
                TokenKind::Pure | TokenKind::Io | TokenKind::Alloc | TokenKind::Unsafe | TokenKind::Syscall => {
                    effects.push(self.parse_effect()?);
                }
                TokenKind::Newline => break,
                other => return Err(ParseError::Expected {
                    expected: "effect name or newline",
                    found: other.clone(),
                    index: self.index,
                }),
            }
        }
        self.expect_newline()?;
        Ok(effects)
    }

    fn parse_effect(&mut self) -> Result<Effect, ParseError> {
        let effect = match self.next().kind {
            TokenKind::Pure => Effect::Pure,
            TokenKind::Io => Effect::Io,
            TokenKind::Alloc => Effect::Alloc,
            TokenKind::Unsafe => Effect::Unsafe,
            TokenKind::Syscall => Effect::Syscall,
            found => return Err(ParseError::Expected {
                expected: "effect name",
                found,
                index: self.index.saturating_sub(1),
            }),
        };
        Ok(effect)
    }

    fn parse_return(&mut self) -> Result<Stmt, ParseError> {
        self.expect_keyword(TokenKind::Return, "`return`")?;
        let expr = self.parse_expr()?;
        self.expect_newline()?;
        Ok(Stmt::Return(expr))
    }

    fn parse_if(&mut self) -> Result<Stmt, ParseError> {
        self.expect_keyword(TokenKind::If, "`if`")?;
        let cond = self.parse_expr()?;
        self.expect_keyword(TokenKind::Then, "`then`")?;
        self.expect_newline()?;
        self.expect_keyword(TokenKind::Begin, "`begin`")?;
        self.expect_newline()?;
        let then_body = self.parse_stmt_block(&[TokenKind::End])?;
        self.expect_keyword(TokenKind::End, "`end`")?;
        self.expect_newline()?;
        self.expect_keyword(TokenKind::Else, "`else`")?;
        self.expect_newline()?;
        self.expect_keyword(TokenKind::Begin, "`begin`")?;
        self.expect_newline()?;
        let else_body = self.parse_stmt_block(&[TokenKind::End])?;
        self.expect_keyword(TokenKind::End, "`end`")?;
        self.expect_optional_newline();
        Ok(Stmt::If { cond, then_body, else_body })
    }

    fn parse_while(&mut self) -> Result<Stmt, ParseError> {
        self.expect_keyword(TokenKind::While, "`while`")?;
        let cond = self.parse_expr()?;
        self.expect_newline()?;
        self.expect_keyword(TokenKind::Begin, "`begin`")?;
        self.expect_newline()?;
        let body = self.parse_stmt_block(&[TokenKind::End])?;
        self.expect_keyword(TokenKind::End, "`end`")?;
        self.expect_optional_newline();
        Ok(Stmt::While { cond, body })
    }

    fn parse_match(&mut self) -> Result<Stmt, ParseError> {
        self.expect_keyword(TokenKind::Match, "`match`")?;
        let scrutinee = self.parse_expr()?;
        self.expect_newline()?;
        let mut arms = Vec::new();
        loop {
            self.skip_newlines();
            match self.peek() {
                TokenKind::Case => arms.push(self.parse_match_arm()?),
                TokenKind::End => break,
                other => return Err(ParseError::Expected {
                    expected: "`case` or `end`",
                    found: other.clone(),
                    index: self.index,
                }),
            }
        }
        self.expect_keyword(TokenKind::End, "`end`")?;
        self.expect_optional_newline();
        Ok(Stmt::Match { scrutinee, arms })
    }

    fn parse_match_arm(&mut self) -> Result<MatchArm, ParseError> {
        self.expect_keyword(TokenKind::Case, "`case`")?;
        let pattern = self.parse_pattern()?;
        self.expect_newline()?;
        self.expect_keyword(TokenKind::Begin, "`begin`")?;
        self.expect_newline()?;
        let body = self.parse_stmt_block(&[TokenKind::End])?;
        self.expect_keyword(TokenKind::End, "`end`")?;
        self.expect_optional_newline();
        Ok(MatchArm { pattern, body })
    }

    fn parse_pattern(&mut self) -> Result<Pattern, ParseError> {
        match self.next().kind {
            TokenKind::SomeKw => {
                let binding = self.parse_ident()?;
                Ok(Pattern::Some(binding))
            }
            TokenKind::NoneKw => Ok(Pattern::None),
            TokenKind::OkKw => {
                let binding = self.parse_ident()?;
                Ok(Pattern::Ok(binding))
            }
            TokenKind::ErrKw => {
                let binding = self.parse_ident()?;
                Ok(Pattern::Err(binding))
            }
            TokenKind::Ident(name) => {
                let binding = match self.peek() {
                    TokenKind::Ident(_) => Some(self.parse_ident()?),
                    _ => None,
                };
                Ok(Pattern::Case { name, binding })
            }
            found => Err(ParseError::Expected {
                expected: "pattern",
                found,
                index: self.index.saturating_sub(1),
            }),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> { self.parse_equality() }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_relational()?;
        loop {
            let op = match self.peek() {
                TokenKind::EqEq => BinaryOp::Eq,
                TokenKind::Ne => BinaryOp::Ne,
                _ => break,
            };
            self.next();
            let rhs = self.parse_relational()?;
            expr = Expr::Binary { op, lhs: Box::new(expr), rhs: Box::new(rhs) };
        }
        Ok(expr)
    }

    fn parse_relational(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_additive()?;
        loop {
            let op = match self.peek() {
                TokenKind::Lt => BinaryOp::Lt,
                TokenKind::Le => BinaryOp::Le,
                TokenKind::Gt => BinaryOp::Gt,
                TokenKind::Ge => BinaryOp::Ge,
                _ => break,
            };
            self.next();
            let rhs = self.parse_additive()?;
            expr = Expr::Binary { op, lhs: Box::new(expr), rhs: Box::new(rhs) };
        }
        Ok(expr)
    }

    fn parse_additive(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_multiplicative()?;
        loop {
            let op = match self.peek() {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Sub,
                _ => break,
            };
            self.next();
            let rhs = self.parse_multiplicative()?;
            expr = Expr::Binary { op, lhs: Box::new(expr), rhs: Box::new(rhs) };
        }
        Ok(expr)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;
        loop {
            let op = match self.peek() {
                TokenKind::Star => BinaryOp::Mul,
                TokenKind::Slash => BinaryOp::Div,
                _ => break,
            };
            self.next();
            let rhs = self.parse_primary()?;
            expr = Expr::Binary { op, lhs: Box::new(expr), rhs: Box::new(rhs) };
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match self.next().kind {
            TokenKind::Ident(name) => {
                if matches!(self.peek(), TokenKind::LParen) {
                    self.next(); // (
                    let mut args = Vec::new();
                    if !matches!(self.peek(), TokenKind::RParen) {
                        loop {
                            args.push(self.parse_expr()?);
                            if matches!(self.peek(), TokenKind::Comma) {
                                self.next();
                                continue;
                            }
                            break;
                        }
                    }
                    self.expect_keyword(TokenKind::RParen, "`)`")?;
                    Ok(Expr::Call { callee: name, args })
                } else {
                    Ok(Expr::Ident(name))
                }
            }
            TokenKind::Int(text) => text.parse::<i64>().map(Expr::Int).map_err(|_| ParseError::InvalidInt(text)),
            TokenKind::True => Ok(Expr::Bool(true)),
            TokenKind::False => Ok(Expr::Bool(false)),
            found => Err(ParseError::Expected {
                expected: "identifier, integer literal, or boolean literal",
                found,
                index: self.index.saturating_sub(1),
            }),
        }
    }

    fn parse_type(&mut self) -> Result<TypeExpr, ParseError> {
        match self.next().kind {
            TokenKind::OwnKw => {
                let inner = self.parse_type()?;
                Ok(TypeExpr::Own(Box::new(inner)))
            }
            TokenKind::ViewKw => {
                let inner = self.parse_type()?;
                Ok(TypeExpr::View(Box::new(inner)))
            }
            TokenKind::OptionKw => {
                let inner = self.parse_type()?;
                Ok(TypeExpr::Option(Box::new(inner)))
            }
            TokenKind::ResultKw => {
                let ok = self.parse_type()?;
                let err = self.parse_type()?;
                Ok(TypeExpr::Result(Box::new(ok), Box::new(err)))
            }
            TokenKind::Ident(name) => Ok(match name.as_str() {
                "bool" => TypeExpr::Primitive(PrimitiveType::Bool),
                "i32" => TypeExpr::Primitive(PrimitiveType::I32),
                "i64" => TypeExpr::Primitive(PrimitiveType::I64),
                "u32" => TypeExpr::Primitive(PrimitiveType::U32),
                "u64" => TypeExpr::Primitive(PrimitiveType::U64),
                "usize" => TypeExpr::Primitive(PrimitiveType::Usize),
                "unit" => TypeExpr::Primitive(PrimitiveType::Unit),
                _ => TypeExpr::Named(name),
            }),
            found => Err(ParseError::Expected {
                expected: "type name",
                found,
                index: self.index.saturating_sub(1),
            }),
        }
    }

    fn parse_qualified_name(&mut self) -> Result<String, ParseError> {
        let mut name = self.parse_ident()?;
        while matches!(self.peek(), TokenKind::Dot) {
            self.next();
            name.push('.');
            name.push_str(&self.parse_ident()?);
        }
        Ok(name)
    }

    fn parse_ident(&mut self) -> Result<String, ParseError> {
        match self.next().kind {
            TokenKind::Ident(name) => Ok(name),
            found => Err(ParseError::Expected {
                expected: "identifier",
                found,
                index: self.index.saturating_sub(1),
            }),
        }
    }

    fn expect_newline(&mut self) -> Result<(), ParseError> {
        match self.next().kind {
            TokenKind::Newline => Ok(()),
            found => Err(ParseError::Expected {
                expected: "newline",
                found,
                index: self.index.saturating_sub(1),
            }),
        }
    }

    fn expect_optional_newline(&mut self) {
        if matches!(self.peek(), TokenKind::Newline) { self.next(); }
    }

    fn expect_keyword(&mut self, expected_kind: TokenKind, expected: &'static str) -> Result<(), ParseError> {
        let token = self.next();
        if std::mem::discriminant(&token.kind) == std::mem::discriminant(&expected_kind) {
            Ok(())
        } else {
            Err(ParseError::Expected { expected, found: token.kind, index: self.index.saturating_sub(1) })
        }
    }

    fn skip_newlines(&mut self) {
        while matches!(self.peek(), TokenKind::Newline) { self.next(); }
    }

    fn at_eof(&self) -> bool { matches!(self.peek(), TokenKind::Eof) }

    fn at_any(&self, kinds: &[TokenKind]) -> bool {
        kinds.iter().any(|k| std::mem::discriminant(k) == std::mem::discriminant(self.peek()))
    }

    fn peek(&self) -> &TokenKind { &self.tokens[self.index].kind }

    fn next(&mut self) -> Token {
        let token = self.tokens[self.index].clone();
        self.index += 1;
        token
    }
}
