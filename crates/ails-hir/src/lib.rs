use ails_ast::{BinaryOp, Expr, Function, MatchArm, Module, Pattern, Stmt, TypeExpr};

#[derive(Debug, Clone)]
pub struct HirModule {
    pub module_name: String,
    pub functions: Vec<HirFunction>,
}

#[derive(Debug, Clone)]
pub struct HirFunction {
    pub name: String,
    pub inputs: Vec<HirParam>,
    pub output: TypeExpr,
    pub body: Vec<HirStmt>,
}

#[derive(Debug, Clone)]
pub struct HirParam {
    pub name: String,
    pub ty: TypeExpr,
}

#[derive(Debug, Clone)]
pub enum HirStmt {
    Let {
        name: String,
        ty: TypeExpr,
        expr: HirExpr,
    },
    Set {
        name: String,
        expr: HirExpr,
    },
    Return(HirExpr),
    If {
        cond: HirExpr,
        then_body: Vec<HirStmt>,
        else_body: Vec<HirStmt>,
    },
    While {
        cond: HirExpr,
        body: Vec<HirStmt>,
    },
    Match {
        scrutinee: HirExpr,
        arms: Vec<HirMatchArm>,
    },
}

#[derive(Debug, Clone)]
pub struct HirMatchArm {
    pub pattern: HirPattern,
    pub body: Vec<HirStmt>,
}

#[derive(Debug, Clone)]
pub enum HirPattern {
    Some(String),
    None,
    Ok(String),
    Err(String),
    Case {
        name: String,
        binding: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub enum HirExpr {
    Ident(String),
    Int(i64),
    Bool(bool),
    Call {
        callee: String,
        args: Vec<HirExpr>,
    },
    Binary {
        op: BinaryOp,
        lhs: Box<HirExpr>,
        rhs: Box<HirExpr>,
    },
}

pub fn lower_module(module: &Module) -> HirModule {
    HirModule {
        module_name: module.name.clone(),
        functions: module.functions.iter().map(lower_function).collect(),
    }
}

fn lower_function(function: &Function) -> HirFunction {
    HirFunction {
        name: function.name.clone(),
        inputs: function.inputs.iter().map(|p| HirParam {
            name: p.name.clone(),
            ty: p.ty.clone(),
        }).collect(),
        output: function.output.clone(),
        body: lower_block(&function.body),
    }
}

fn lower_block(stmts: &[Stmt]) -> Vec<HirStmt> {
    stmts.iter().map(lower_stmt).collect()
}

fn lower_stmt(stmt: &Stmt) -> HirStmt {
    match stmt {
        Stmt::Let { name, ty, expr } => HirStmt::Let {
            name: name.clone(),
            ty: ty.clone(),
            expr: lower_expr(expr),
        },
        Stmt::Set { name, expr } => HirStmt::Set {
            name: name.clone(),
            expr: lower_expr(expr),
        },
        Stmt::Return(expr) => HirStmt::Return(lower_expr(expr)),
        Stmt::If { cond, then_body, else_body } => HirStmt::If {
            cond: lower_expr(cond),
            then_body: lower_block(then_body),
            else_body: lower_block(else_body),
        },
        Stmt::While { cond, body } => HirStmt::While {
            cond: lower_expr(cond),
            body: lower_block(body),
        },
        Stmt::Match { scrutinee, arms } => HirStmt::Match {
            scrutinee: lower_expr(scrutinee),
            arms: arms.iter().map(lower_match_arm).collect(),
        },
    }
}

fn lower_match_arm(arm: &MatchArm) -> HirMatchArm {
    HirMatchArm {
        pattern: lower_pattern(&arm.pattern),
        body: lower_block(&arm.body),
    }
}

fn lower_pattern(pattern: &Pattern) -> HirPattern {
    match pattern {
        Pattern::Some(name) => HirPattern::Some(name.clone()),
        Pattern::None => HirPattern::None,
        Pattern::Ok(name) => HirPattern::Ok(name.clone()),
        Pattern::Err(name) => HirPattern::Err(name.clone()),
        Pattern::Case { name, binding } => HirPattern::Case {
            name: name.clone(),
            binding: binding.clone(),
        },
    }
}

fn lower_expr(expr: &Expr) -> HirExpr {
    match expr {
        Expr::Ident(name) => HirExpr::Ident(name.clone()),
        Expr::Int(v) => HirExpr::Int(*v),
        Expr::Bool(v) => HirExpr::Bool(*v),
        Expr::Call { callee, args } => HirExpr::Call {
            callee: callee.clone(),
            args: args.iter().map(lower_expr).collect(),
        },
        Expr::Binary { op, lhs, rhs } => HirExpr::Binary {
            op: *op,
            lhs: Box::new(lower_expr(lhs)),
            rhs: Box::new(lower_expr(rhs)),
        },
    }
}
