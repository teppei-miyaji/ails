#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub name: String,
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub inputs: Vec<Param>,
    pub output: TypeExpr,
    pub effects: Vec<Effect>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    pub name: String,
    pub ty: TypeExpr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeExpr {
    Primitive(PrimitiveType),
    Named(String),
    Own(Box<TypeExpr>),
    View(Box<TypeExpr>),
    Option(Box<TypeExpr>),
    Result(Box<TypeExpr>, Box<TypeExpr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveType {
    Bool,
    I32,
    I64,
    U32,
    U64,
    Usize,
    Unit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    Pure,
    Io,
    Alloc,
    Unsafe,
    Syscall,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Let {
        name: String,
        ty: TypeExpr,
        expr: Expr,
    },
    Set {
        name: String,
        expr: Expr,
    },
    Return(Expr),
    If {
        cond: Expr,
        then_body: Vec<Stmt>,
        else_body: Vec<Stmt>,
    },
    While {
        cond: Expr,
        body: Vec<Stmt>,
    },
    Match {
        scrutinee: Expr,
        arms: Vec<MatchArm>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    Some(String),
    None,
    Ok(String),
    Err(String),
    Case { name: String, binding: Option<String> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Ident(String),
    Int(i64),
    Bool(bool),
    Call {
        callee: String,
        args: Vec<Expr>,
    },
    Binary {
        op: BinaryOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
}

impl TypeExpr {
    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            TypeExpr::Primitive(
                PrimitiveType::I32
                    | PrimitiveType::I64
                    | PrimitiveType::U32
                    | PrimitiveType::U64
                    | PrimitiveType::Usize
            )
        )
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, TypeExpr::Primitive(PrimitiveType::Bool))
    }

    pub fn is_unit(&self) -> bool {
        matches!(self, TypeExpr::Primitive(PrimitiveType::Unit))
    }

    pub fn strip_wrappers(&self) -> &TypeExpr {
        match self {
            TypeExpr::Own(inner) | TypeExpr::View(inner) => inner.strip_wrappers(),
            other => other,
        }
    }

    pub fn is_view(&self) -> bool {
        matches!(self, TypeExpr::View(_))
    }

    pub fn is_own(&self) -> bool {
        matches!(self, TypeExpr::Own(_))
    }

    pub fn is_move_only(&self) -> bool {
        self.is_own()
    }

    pub fn can_view_as(&self, target: &TypeExpr) -> bool {
        match target {
            TypeExpr::View(inner) => self.strip_wrappers() == inner.strip_wrappers(),
            _ => self == target,
        }
    }
}
