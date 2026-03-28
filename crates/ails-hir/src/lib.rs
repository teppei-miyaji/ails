use ails_ast::{Function, Module, Stmt};

#[derive(Debug, Clone)]
pub struct HirModule {
    pub module_name: String,
    pub functions: Vec<HirFunction>,
}

#[derive(Debug, Clone)]
pub struct HirFunction {
    pub name: String,
    pub stmt_count: usize,
    pub has_match: bool,
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
        stmt_count: function.body.len(),
        has_match: contains_match(&function.body),
    }
}

fn contains_match(stmts: &[Stmt]) -> bool {
    stmts.iter().any(|stmt| match stmt {
        Stmt::Match { .. } => true,
        Stmt::If { then_body, else_body, .. } => contains_match(then_body) || contains_match(else_body),
        Stmt::While { body, .. } => contains_match(body),
        Stmt::Return(_) => false,
    })
}
