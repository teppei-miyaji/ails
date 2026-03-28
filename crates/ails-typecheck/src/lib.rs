use std::collections::{BTreeMap, BTreeSet};

use ails_ast::{BinaryOp, Expr, Function, MatchArm, Module, Pattern, PrimitiveType, Stmt, TypeExpr};
use thiserror::Error;

#[derive(Debug, Clone)]
struct LocalState {
    ty: TypeExpr,
    moved: bool,
    borrowed_view_count: u32,
}

#[derive(Debug, Clone)]
struct FuncSig {
    inputs: Vec<TypeExpr>,
    output: TypeExpr,
}

#[derive(Debug, Error)]
pub enum TypeCheckError {
    #[error("duplicate function name `{name}`")]
    DuplicateFunction { name: String },

    #[error("duplicate parameter name `{name}` in function `{function}`")]
    DuplicateParam { function: String, name: String },

    #[error("duplicate local name `{name}` in function `{function}`")]
    DuplicateLocal { function: String, name: String },

    #[error("unknown identifier `{name}` in function `{function}`")]
    UnknownIdentifier { function: String, name: String },

    #[error("unknown function `{name}`")]
    UnknownFunction { name: String },

    #[error("argument count mismatch calling `{callee}`: expected {expected}, found {found}")]
    ArgCountMismatch { callee: String, expected: usize, found: usize },

    #[error("type mismatch in function `{function}`: expected {expected:?}, found {found:?}")]
    TypeMismatch { function: String, expected: TypeExpr, found: TypeExpr },

    #[error("binary operator `{op:?}` requires compatible operands in function `{function}`, found lhs={lhs:?}, rhs={rhs:?}")]
    InvalidBinaryOperands { function: String, op: BinaryOp, lhs: TypeExpr, rhs: TypeExpr },

    #[error("condition in function `{function}` must be bool, found {found:?}")]
    NonBoolCondition { function: String, found: TypeExpr },

    #[error("function `{function}` is missing a return statement")]
    MissingReturn { function: String },

    #[error("`match` target in function `{function}` is not matchable: {found:?}")]
    InvalidMatchTarget { function: String, found: TypeExpr },

    #[error("non-exhaustive `match` in function `{function}`")]
    NonExhaustiveMatch { function: String },

    #[error("invalid pattern in function `{function}` for scrutinee type {scrutinee:?}")]
    InvalidPattern { function: String, scrutinee: TypeExpr },

    #[error("duplicate `match` arm in function `{function}`")]
    DuplicateMatchArm { function: String },

    #[error("move after use of `{name}` in function `{function}`")]
    MoveAfterUse { function: String, name: String },

    #[error("cannot move `{name}` while it is borrowed as `view` in function `{function}`")]
    MoveWhileBorrowed { function: String, name: String },

    #[error("cannot update `{name}` while it is borrowed as `view` in function `{function}`")]
    UpdateWhileBorrowed { function: String, name: String },

    #[error("ownership state mismatch across branches for `{name}` in function `{function}`")]
    BranchStateMismatch { function: String, name: String },

    #[error("ownership state mismatch across loop boundary for `{name}` in function `{function}`")]
    LoopStateMismatch { function: String, name: String },
}

pub fn check_module(module: &Module) -> Result<(), TypeCheckError> {
    let mut names = BTreeSet::new();
    let mut funcs = BTreeMap::<String, FuncSig>::new();

    for function in &module.functions {
        if !names.insert(function.name.clone()) {
            return Err(TypeCheckError::DuplicateFunction { name: function.name.clone() });
        }
        funcs.insert(function.name.clone(), FuncSig {
            inputs: function.inputs.iter().map(|p| p.ty.clone()).collect(),
            output: function.output.clone(),
        });
    }

    for function in &module.functions {
        check_function(function, &funcs)?;
    }

    Ok(())
}

fn check_function(function: &Function, funcs: &BTreeMap<String, FuncSig>) -> Result<(), TypeCheckError> {
    let mut locals = BTreeMap::<String, LocalState>::new();

    for param in &function.inputs {
        if locals.contains_key(&param.name) {
            return Err(TypeCheckError::DuplicateParam {
                function: function.name.clone(),
                name: param.name.clone(),
            });
        }
        locals.insert(param.name.clone(), LocalState {
            ty: param.ty.clone(),
            moved: false,
            borrowed_view_count: 0,
        });
    }

    if function.body.is_empty() && !function.output.is_unit() {
        return Err(TypeCheckError::MissingReturn { function: function.name.clone() });
    }

    let returns = check_block(&function.body, &mut locals, &function.name, &function.output, funcs)?;
    if !returns && !function.output.is_unit() {
        return Err(TypeCheckError::MissingReturn { function: function.name.clone() });
    }

    Ok(())
}

fn check_block(
    body: &[Stmt],
    locals: &mut BTreeMap<String, LocalState>,
    function_name: &str,
    ret_ty: &TypeExpr,
    funcs: &BTreeMap<String, FuncSig>,
) -> Result<bool, TypeCheckError> {
    let mut saw_return = false;
    for stmt in body {
        let stmt_returns = check_stmt(stmt, locals, function_name, ret_ty, funcs)?;
        if stmt_returns {
            saw_return = true;
        }
    }
    Ok(saw_return)
}

fn check_stmt(
    stmt: &Stmt,
    locals: &mut BTreeMap<String, LocalState>,
    function_name: &str,
    ret_ty: &TypeExpr,
    funcs: &BTreeMap<String, FuncSig>,
) -> Result<bool, TypeCheckError> {
    match stmt {
        Stmt::Let { name, ty, expr } => {
            if locals.contains_key(name) {
                return Err(TypeCheckError::DuplicateLocal {
                    function: function_name.to_string(),
                    name: name.clone(),
                });
            }
            let actual = infer_expr(expr, locals, function_name, funcs)?;
            if &actual != ty {
                return Err(TypeCheckError::TypeMismatch {
                    function: function_name.to_string(),
                    expected: ty.clone(),
                    found: actual,
                });
            }
            locals.insert(name.clone(), LocalState {
                ty: ty.clone(),
                moved: false,
                borrowed_view_count: 0,
            });
            Ok(false)
        }
        Stmt::Set { name, expr } => {
            let state = locals.get(name).cloned().ok_or_else(|| TypeCheckError::UnknownIdentifier {
                function: function_name.to_string(),
                name: name.clone(),
            })?;
            if state.borrowed_view_count > 0 {
                return Err(TypeCheckError::UpdateWhileBorrowed {
                    function: function_name.to_string(),
                    name: name.clone(),
                });
            }
            let actual = infer_expr(expr, locals, function_name, funcs)?;
            if actual != state.ty {
                return Err(TypeCheckError::TypeMismatch {
                    function: function_name.to_string(),
                    expected: state.ty,
                    found: actual,
                });
            }
            if let Some(slot) = locals.get_mut(name) {
                slot.moved = false;
            }
            Ok(false)
        }
        Stmt::Return(expr) => {
            let ty = infer_expr(expr, locals, function_name, funcs)?;
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
            let cond_ty = infer_expr(cond, locals, function_name, funcs)?;
            if !cond_ty.is_bool() {
                return Err(TypeCheckError::NonBoolCondition {
                    function: function_name.to_string(),
                    found: cond_ty,
                });
            }
            let mut then_locals = locals.clone();
            let mut else_locals = locals.clone();
            let then_returns = check_block(then_body, &mut then_locals, function_name, ret_ty, funcs)?;
            let else_returns = check_block(else_body, &mut else_locals, function_name, ret_ty, funcs)?;

            if !then_returns || !else_returns {
                merge_branch_states(locals, &then_locals, &else_locals, function_name)?;
            }

            Ok(then_returns && else_returns)
        }
        Stmt::While { cond, body } => {
            let cond_ty = infer_expr(cond, locals, function_name, funcs)?;
            if !cond_ty.is_bool() {
                return Err(TypeCheckError::NonBoolCondition {
                    function: function_name.to_string(),
                    found: cond_ty,
                });
            }
            let before_loop = locals.clone();
            let mut body_locals = locals.clone();
            let _ = check_block(body, &mut body_locals, function_name, ret_ty, funcs)?;
            merge_loop_states(locals, &before_loop, &body_locals, function_name)?;
            Ok(false)
        }
        Stmt::Match { scrutinee, arms } => check_match(scrutinee, arms, locals, function_name, ret_ty, funcs),
    }
}

fn merge_branch_states(
    base: &mut BTreeMap<String, LocalState>,
    then_locals: &BTreeMap<String, LocalState>,
    else_locals: &BTreeMap<String, LocalState>,
    function_name: &str,
) -> Result<(), TypeCheckError> {
    for (name, base_state) in base.iter_mut() {
        let then_state = then_locals.get(name).ok_or_else(|| TypeCheckError::BranchStateMismatch {
            function: function_name.to_string(),
            name: name.clone(),
        })?;
        let else_state = else_locals.get(name).ok_or_else(|| TypeCheckError::BranchStateMismatch {
            function: function_name.to_string(),
            name: name.clone(),
        })?;

        if then_state.moved != else_state.moved {
            return Err(TypeCheckError::BranchStateMismatch {
                function: function_name.to_string(),
                name: name.clone(),
            });
        }
        if then_state.borrowed_view_count != else_state.borrowed_view_count {
            return Err(TypeCheckError::BranchStateMismatch {
                function: function_name.to_string(),
                name: name.clone(),
            });
        }

        base_state.moved = then_state.moved;
        base_state.borrowed_view_count = then_state.borrowed_view_count;
    }
    Ok(())
}

fn merge_loop_states(
    base: &mut BTreeMap<String, LocalState>,
    before_loop: &BTreeMap<String, LocalState>,
    after_body: &BTreeMap<String, LocalState>,
    function_name: &str,
) -> Result<(), TypeCheckError> {
    for (name, base_state) in base.iter_mut() {
        let before = before_loop.get(name).ok_or_else(|| TypeCheckError::LoopStateMismatch {
            function: function_name.to_string(),
            name: name.clone(),
        })?;
        let after = after_body.get(name).ok_or_else(|| TypeCheckError::LoopStateMismatch {
            function: function_name.to_string(),
            name: name.clone(),
        })?;

        if before.moved != after.moved {
            return Err(TypeCheckError::LoopStateMismatch {
                function: function_name.to_string(),
                name: name.clone(),
            });
        }
        if before.borrowed_view_count != after.borrowed_view_count {
            return Err(TypeCheckError::LoopStateMismatch {
                function: function_name.to_string(),
                name: name.clone(),
            });
        }

        base_state.moved = before.moved;
        base_state.borrowed_view_count = before.borrowed_view_count;
    }
    Ok(())
}

fn check_match(
    scrutinee: &Expr,
    arms: &[MatchArm],
    locals: &mut BTreeMap<String, LocalState>,
    function_name: &str,
    ret_ty: &TypeExpr,
    funcs: &BTreeMap<String, FuncSig>,
) -> Result<bool, TypeCheckError> {
    let scrutinee_ty = infer_expr(scrutinee, locals, function_name, funcs)?;
    let mut seen = BTreeSet::new();
    let mut all_return = true;

    match &scrutinee_ty {
        TypeExpr::Option(inner) => {
            for arm in arms {
                let mut arm_locals = locals.clone();
                let key = match &arm.pattern {
                    Pattern::Some(binding) => {
                        arm_locals.insert(binding.clone(), LocalState { ty: (**inner).clone(), moved: false, borrowed_view_count: 0 });
                        "some".to_string()
                    }
                    Pattern::None => "none".to_string(),
                    _ => return Err(TypeCheckError::InvalidPattern { function: function_name.to_string(), scrutinee: scrutinee_ty.clone() }),
                };
                if !seen.insert(key) {
                    return Err(TypeCheckError::DuplicateMatchArm { function: function_name.to_string() });
                }
                let arm_returns = check_block(&arm.body, &mut arm_locals, function_name, ret_ty, funcs)?;
                all_return &= arm_returns;
            }
            if !(seen.contains("some") && seen.contains("none")) {
                return Err(TypeCheckError::NonExhaustiveMatch { function: function_name.to_string() });
            }
        }
        TypeExpr::Result(ok_ty, err_ty) => {
            for arm in arms {
                let mut arm_locals = locals.clone();
                let key = match &arm.pattern {
                    Pattern::Ok(binding) => {
                        arm_locals.insert(binding.clone(), LocalState { ty: (**ok_ty).clone(), moved: false, borrowed_view_count: 0 });
                        "ok".to_string()
                    }
                    Pattern::Err(binding) => {
                        arm_locals.insert(binding.clone(), LocalState { ty: (**err_ty).clone(), moved: false, borrowed_view_count: 0 });
                        "err".to_string()
                    }
                    _ => return Err(TypeCheckError::InvalidPattern { function: function_name.to_string(), scrutinee: scrutinee_ty.clone() }),
                };
                if !seen.insert(key) {
                    return Err(TypeCheckError::DuplicateMatchArm { function: function_name.to_string() });
                }
                let arm_returns = check_block(&arm.body, &mut arm_locals, function_name, ret_ty, funcs)?;
                all_return &= arm_returns;
            }
            if !(seen.contains("ok") && seen.contains("err")) {
                return Err(TypeCheckError::NonExhaustiveMatch { function: function_name.to_string() });
            }
        }
        _ => return Err(TypeCheckError::InvalidMatchTarget { function: function_name.to_string(), found: scrutinee_ty }),
    }

    Ok(all_return)
}

fn infer_expr(
    expr: &Expr,
    locals: &mut BTreeMap<String, LocalState>,
    function_name: &str,
    funcs: &BTreeMap<String, FuncSig>,
) -> Result<TypeExpr, TypeCheckError> {
    match expr {
        Expr::Ident(name) => use_ident(name, locals, function_name, false),
        Expr::Int(_) => Ok(TypeExpr::Primitive(PrimitiveType::I32)),
        Expr::Bool(_) => Ok(TypeExpr::Primitive(PrimitiveType::Bool)),
        Expr::Call { callee, args } => {
            let sig = funcs.get(callee).cloned().ok_or_else(|| TypeCheckError::UnknownFunction { name: callee.clone() })?;
            if sig.inputs.len() != args.len() {
                return Err(TypeCheckError::ArgCountMismatch { callee: callee.clone(), expected: sig.inputs.len(), found: args.len() });
            }

            let borrowed_names = collect_borrowed_idents(args, &sig.inputs);
            for name in &borrowed_names {
                if let Some(local) = locals.get_mut(name) {
                    local.borrowed_view_count += 1;
                }
            }

            for (arg, expected) in args.iter().zip(sig.inputs.iter()) {
                let actual = infer_call_arg(arg, expected, locals, function_name, funcs)?;
                let ok = if expected.is_view() {
                    actual.can_view_as(expected)
                } else {
                    &actual == expected
                };
                if !ok {
                    for name in &borrowed_names {
                        if let Some(local) = locals.get_mut(name) {
                            local.borrowed_view_count = local.borrowed_view_count.saturating_sub(1);
                        }
                    }
                    return Err(TypeCheckError::TypeMismatch {
                        function: function_name.to_string(),
                        expected: expected.clone(),
                        found: actual,
                    });
                }
            }

            for name in &borrowed_names {
                if let Some(local) = locals.get_mut(name) {
                    local.borrowed_view_count = local.borrowed_view_count.saturating_sub(1);
                }
            }

            Ok(sig.output)
        }
        Expr::Binary { op, lhs, rhs } => {
            let lhs_ty = infer_expr(lhs, locals, function_name, funcs)?;
            let rhs_ty = infer_expr(rhs, locals, function_name, funcs)?;
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
                    let l = lhs_ty.strip_wrappers().clone();
                    let r = rhs_ty.strip_wrappers().clone();
                    if l != r || !l.is_integer() {
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
                    let l = lhs_ty.strip_wrappers().clone();
                    let r = rhs_ty.strip_wrappers().clone();
                    if l != r {
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

fn infer_call_arg(
    expr: &Expr,
    expected: &TypeExpr,
    locals: &mut BTreeMap<String, LocalState>,
    function_name: &str,
    funcs: &BTreeMap<String, FuncSig>,
) -> Result<TypeExpr, TypeCheckError> {
    if expected.is_view() {
        match expr {
            Expr::Ident(name) => use_ident(name, locals, function_name, true),
            _ => infer_expr(expr, locals, function_name, funcs),
        }
    } else {
        infer_expr(expr, locals, function_name, funcs)
    }
}

fn collect_borrowed_idents(args: &[Expr], expected: &[TypeExpr]) -> Vec<String> {
    let mut out = Vec::new();
    for (arg, exp) in args.iter().zip(expected.iter()) {
        if exp.is_view() {
            if let Expr::Ident(name) = arg {
                out.push(name.clone());
            }
        }
    }
    out
}

fn use_ident(
    name: &str,
    locals: &mut BTreeMap<String, LocalState>,
    function_name: &str,
    as_view: bool,
) -> Result<TypeExpr, TypeCheckError> {
    let state = locals.get_mut(name).ok_or_else(|| TypeCheckError::UnknownIdentifier {
        function: function_name.to_string(),
        name: name.to_string(),
    })?;

    if state.moved {
        return Err(TypeCheckError::MoveAfterUse {
            function: function_name.to_string(),
            name: name.to_string(),
        });
    }

    if as_view {
        let base = state.ty.strip_wrappers().clone();
        return Ok(TypeExpr::View(Box::new(base)));
    }

    if state.ty.is_move_only() {
        if state.borrowed_view_count > 0 {
            return Err(TypeCheckError::MoveWhileBorrowed {
                function: function_name.to_string(),
                name: name.to_string(),
            });
        }
        state.moved = true;
    }

    Ok(state.ty.clone())
}
