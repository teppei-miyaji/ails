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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Ident(String),
    Int(i64),
    Bool(bool),
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
}
