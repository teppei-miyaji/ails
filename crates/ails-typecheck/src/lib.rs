use std::collections::{BTreeMap, BTreeSet};

use ails_ast::{BinaryOp, Expr, Function, Module, PrimitiveType, Stmt, TypeExpr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TypeCheckError {
    #[error("duplicate function name `{name}`")]
    DuplicateFunction { name: String },

    #[error("duplicate parameter name `{name}` in function `{function}`")]
    DuplicateParam { function: String, name: String },

    #[error("unknown identifier `{name}` in function `{function}`")]
    UnknownIdentifier { function: String, name: String },

    #[error("type mismatch in function `{function}`: expected {expected:?}, found {found:?}")]
    TypeMismatch {
        function: String,
        expected: TypeExpr,
        found: TypeExpr,
    },

    #[error("binary operator `{op:?}` requires compatible operands in function `{function}`, found lhs={lhs:?}, rhs={rhs:?}")]
    InvalidBinaryOperands {
        function: String,
        op: BinaryOp,
        lhs: TypeExpr,
        rhs: TypeExpr,
    },

    #[error("condition in function `{function}` must be bool, found {found:?}")]
    NonBoolCondition {
        function: String,
        found: TypeExpr,
    },

    #[error("function `{function}` is missing a return statement")]
    MissingReturn { function: String },
}

pub fn check_module(module: &Module) -> Result<(), TypeCheckError> {
    let mut names = BTreeSet::new();
    for function in &module.functions {
        if !names.insert(function.name.clone()) {
            return Err(TypeCheckError::DuplicateFunction { name: function.name.clone() });
        }
    }

    for function in &module.functions {
        check_function(function)?;
    }

    Ok(())
}

fn check_function(function: &Function) -> Result<(), TypeCheckError> {
    let mut locals = BTreeMap::<String, TypeExpr>::new();

    for param in &function.inputs {
        if locals.insert(param.name.clone(), param.ty.clone()).is_some() {
            return Err(TypeCheckError::DuplicateParam {
                function: function.name.clone(),
                name: param.name.clone(),
            });
        }
    }

    if function.body.is_empty() && !function.output.is_unit() {
        return Err(TypeCheckError::MissingReturn { function: function.name.clone() });
    }

    let returns = check_block(&function.body, &locals, &function.name, &function.output)?;
    if !returns && !function.output.is_unit() {
        return Err(TypeCheckError::MissingReturn { function: function.name.clone() });
    }

    Ok(())
}

fn check_block(
    body: &[Stmt],
    locals: &BTreeMap<String, TypeExpr>,
    function_name: &str,
    ret_ty: &TypeExpr,
) -> Result<bool, TypeCheckError> {
    let mut saw_return = false;
    for stmt in body {
        let stmt_returns = check_stmt(stmt, locals, function_name, ret_ty)?;
        if stmt_returns {
            saw_return = true;
        }
    }
    Ok(saw_return)
}

fn check_stmt(
    stmt: &Stmt,
    locals: &BTreeMap<String, TypeExpr>,
    function_name: &str,
    ret_ty: &TypeExpr,
) -> Result<bool, TypeCheckError> {
    match stmt {
        Stmt::Return(expr) => {
            let ty = infer_expr(expr, locals, function_name)?;
            if &ty != ret_ty {
                return Err(TypeCheckError::TypeMismatch {
                    function: function_name.to_string(),
                    expected: ret_ty.clone(),
                    found: ty,
                });
            }
            Ok(true)
        }
        Stmt::If { cond, then_body, else_body } => {
            let cond_ty = infer_expr(cond, locals, function_name)?;
            if !cond_ty.is_bool() {
                return Err(TypeCheckError::NonBoolCondition {
                    function: function_name.to_string(),
                    found: cond_ty,
                });
            }
            let then_returns = check_block(then_body, locals, function_name, ret_ty)?;
            let else_returns = check_block(else_body, locals, function_name, ret_ty)?;
            Ok(then_returns && else_returns)
        }
        Stmt::While { cond, body } => {
            let cond_ty = infer_expr(cond, locals, function_name)?;
            if !cond_ty.is_bool() {
                return Err(TypeCheckError::NonBoolCondition {
                    function: function_name.to_string(),
                    found: cond_ty,
                });
            }
            let _ = check_block(body, locals, function_name, ret_ty)?;
            Ok(false)
        }
    }
}

fn infer_expr(
    expr: &Expr,
    locals: &BTreeMap<String, TypeExpr>,
    function_name: &str,
) -> Result<TypeExpr, TypeCheckError> {
    match expr {
        Expr::Ident(name) => locals.get(name).cloned().ok_or_else(|| TypeCheckError::UnknownIdentifier {
            function: function_name.to_string(),
            name: name.clone(),
        }),
        Expr::Int(_) => Ok(TypeExpr::Primitive(PrimitiveType::I32)),
        Expr::Bool(_) => Ok(TypeExpr::Primitive(PrimitiveType::Bool)),
        Expr::Binary { op, lhs, rhs } => {
            let lhs_ty = infer_expr(lhs, locals, function_name)?;
            let rhs_ty = infer_expr(rhs, locals, function_name)?;
            match op {
                BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => {
                    if lhs_ty != rhs_ty || !lhs_ty.is_integer() {
                        return Err(TypeCheckError::InvalidBinaryOperands {
                            function: function_name.to_string(),
                            op: *op,
                            lhs: lhs_ty,
                            rhs: rhs_ty,
                        });
                    }
                    Ok(lhs_ty)
                }
                BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
                    if lhs_ty != rhs_ty || !lhs_ty.is_integer() {
                        return Err(TypeCheckError::InvalidBinaryOperands {
                            function: function_name.to_string(),
                            op: *op,
                            lhs: lhs_ty,
                            rhs: rhs_ty,
                        });
                    }
                    Ok(TypeExpr::Primitive(PrimitiveType::Bool))
                }
                BinaryOp::Eq | BinaryOp::Ne => {
                    if lhs_ty != rhs_ty {
                        return Err(TypeCheckError::InvalidBinaryOperands {
                            function: function_name.to_string(),
                            op: *op,
                            lhs: lhs_ty,
                            rhs: rhs_ty,
                        });
                    }
                    Ok(TypeExpr::Primitive(PrimitiveType::Bool))
                }
            }
        }
    }
}
