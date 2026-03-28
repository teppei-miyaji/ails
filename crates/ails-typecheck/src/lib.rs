use std::collections::{BTreeMap, BTreeSet};

use ails_ast::{BinaryOp, CaseDecl, Expr, FieldDecl, Function, MatchArm, Module, Pattern, PrimitiveType, Stmt, TypeDecl, TypeExpr};
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

#[derive(Debug, Clone)]
struct CaseInfo {
    fields: Vec<FieldDecl>,
}

#[derive(Debug, Clone)]
struct TypeInfo {
    cases: BTreeMap<String, CaseInfo>,
}

#[derive(Debug, Error)]
pub enum TypeCheckError {
    #[error("duplicate function name `{name}`")]
    DuplicateFunction { name: String },

    #[error("duplicate type name `{name}`")]
    DuplicateType { name: String },

    #[error("duplicate case `{case}` in type `{ty}`")]
    DuplicateCase { ty: String, case: String },

    #[error("duplicate field `{field}` in case `{case}` of type `{ty}`")]
    DuplicateField { ty: String, case: String, field: String },

    #[error("unknown type `{name}`")]
    UnknownType { name: String },

    #[error("duplicate const name `{name}`")]
    DuplicateConst { name: String },

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
    let mut type_env = BTreeMap::<String, TypeInfo>::new();
    let mut func_names = BTreeSet::new();
    let mut funcs = BTreeMap::<String, FuncSig>::new();
    let mut const_env = BTreeMap::<String, TypeExpr>::new();

    for ty in &module.types {
        register_type_decl(ty, &mut type_env)?;
    }

    for c in &module.consts {
        if const_env.contains_key(&c.name) {
            return Err(TypeCheckError::DuplicateConst { name: c.name.clone() });
        }
        ensure_type_known(&c.ty, &type_env)?;
        let mut empty_locals = BTreeMap::<String, LocalState>::new();
        let actual = infer_expr(&c.expr, &mut empty_locals, "<const>", &funcs, &const_env, &type_env)?;
        if actual != c.ty {
            return Err(TypeCheckError::TypeMismatch {
                function: "<const>".to_string(),
                expected: c.ty.clone(),
                found: actual,
            });
        }
        const_env.insert(c.name.clone(), c.ty.clone());
    }

    for function in &module.functions {
        if !func_names.insert(function.name.clone()) {
            return Err(TypeCheckError::DuplicateFunction { name: function.name.clone() });
        }
        for input in &function.inputs {
            ensure_type_known(&input.ty, &type_env)?;
        }
        ensure_type_known(&function.output, &type_env)?;
        funcs.insert(function.name.clone(), FuncSig {
            inputs: function.inputs.iter().map(|p| p.ty.clone()).collect(),
            output: function.output.clone(),
        });
    }

    for function in &module.functions {
        check_function(function, &funcs, &const_env, &type_env)?;
    }

    Ok(())
}

fn register_type_decl(ty: &TypeDecl, type_env: &mut BTreeMap<String, TypeInfo>) -> Result<(), TypeCheckError> {
    if type_env.contains_key(&ty.name) {
        return Err(TypeCheckError::DuplicateType { name: ty.name.clone() });
    }
    let mut cases = BTreeMap::<String, CaseInfo>::new();
    for case in &ty.cases {
        register_case_decl(&ty.name, case, &mut cases)?;
    }
    type_env.insert(ty.name.clone(), TypeInfo { cases });
    Ok(())
}

fn register_case_decl(
    ty_name: &str,
    case: &CaseDecl,
    cases: &mut BTreeMap<String, CaseInfo>,
) -> Result<(), TypeCheckError> {
    if cases.contains_key(&case.name) {
        return Err(TypeCheckError::DuplicateCase {
            ty: ty_name.to_string(),
            case: case.name.clone(),
        });
    }
    let mut fields = BTreeSet::new();
    for f in &case.fields {
        if !fields.insert(f.name.clone()) {
            return Err(TypeCheckError::DuplicateField {
                ty: ty_name.to_string(),
                case: case.name.clone(),
                field: f.name.clone(),
            });
        }
    }
    cases.insert(case.name.clone(), CaseInfo { fields: case.fields.clone() });
    Ok(())
}

fn ensure_type_known(ty: &TypeExpr, type_env: &BTreeMap<String, TypeInfo>) -> Result<(), TypeCheckError> {
    match ty {
        TypeExpr::Primitive(_) => Ok(()),
        TypeExpr::Named(name) => {
            if type_env.contains_key(name) {
                Ok(())
            } else {
                Err(TypeCheckError::UnknownType { name: name.clone() })
            }
        }
        TypeExpr::Own(inner) | TypeExpr::View(inner) | TypeExpr::Option(inner) => ensure_type_known(inner, type_env),
        TypeExpr::Result(ok, err) => {
            ensure_type_known(ok, type_env)?;
            ensure_type_known(err, type_env)
        }
    }
}

fn check_function(
    function: &Function,
    funcs: &BTreeMap<String, FuncSig>,
    const_env: &BTreeMap<String, TypeExpr>,
    type_env: &BTreeMap<String, TypeInfo>,
) -> Result<(), TypeCheckError> {
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

    let returns = check_block(&function.body, &mut locals, &function.name, &function.output, funcs, const_env, type_env)?;
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
    const_env: &BTreeMap<String, TypeExpr>,
    type_env: &BTreeMap<String, TypeInfo>,
) -> Result<bool, TypeCheckError> {
    let mut saw_return = false;
    for stmt in body {
        let stmt_returns = check_stmt(stmt, locals, function_name, ret_ty, funcs, const_env, type_env)?;
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
    const_env: &BTreeMap<String, TypeExpr>,
    type_env: &BTreeMap<String, TypeInfo>,
) -> Result<bool, TypeCheckError> {
    match stmt {
        Stmt::Let { name, ty, expr } => {
            if locals.contains_key(name) {
                return Err(TypeCheckError::DuplicateLocal {
                    function: function_name.to_string(),
                    name: name.clone(),
                });
            }
            ensure_type_known(ty, type_env)?;
            let actual = infer_expr(expr, locals, function_name, funcs, const_env, type_env)?;
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
            let actual = infer_expr(expr, locals, function_name, funcs, const_env, type_env)?;
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
            let ty = infer_expr(expr, locals, function_name, funcs, const_env, type_env)?;
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
            let cond_ty = infer_expr(cond, locals, function_name, funcs, const_env, type_env)?;
            if !cond_ty.is_bool() {
                return Err(TypeCheckError::NonBoolCondition {
                    function: function_name.to_string(),
                    found: cond_ty,
                });
            }
            let mut then_locals = locals.clone();
            let mut else_locals = locals.clone();
            let then_returns = check_block(then_body, &mut then_locals, function_name, ret_ty, funcs, const_env, type_env)?;
            let else_returns = check_block(else_body, &mut else_locals, function_name, ret_ty, funcs, const_env, type_env)?;

            if !then_returns || !else_returns {
                merge_branch_states(locals, &then_locals, &else_locals, function_name)?;
            }

            Ok(then_returns && else_returns)
        }
        Stmt::While { cond, body } => {
            let cond_ty = infer_expr(cond, locals, function_name, funcs, const_env, type_env)?;
            if !cond_ty.is_bool() {
                return Err(TypeCheckError::NonBoolCondition {
                    function: function_name.to_string(),
                    found: cond_ty,
                });
            }
            let before_loop = locals.clone();
            let mut body_locals = locals.clone();
            let _ = check_block(body, &mut body_locals, function_name, ret_ty, funcs, const_env, type_env)?;
            merge_loop_states(locals, &before_loop, &body_locals, function_name)?;
            Ok(false)
        }
        Stmt::Match { scrutinee, arms } => check_match(scrutinee, arms, locals, function_name, ret_ty, funcs, const_env, type_env),
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
    const_env: &BTreeMap<String, TypeExpr>,
    type_env: &BTreeMap<String, TypeInfo>,
) -> Result<bool, TypeCheckError> {
    let scrutinee_ty = infer_expr(scrutinee, locals, function_name, funcs, const_env, type_env)?;
    let mut seen = BTreeSet::new();
    let mut all_return = true;

    match scrutinee_ty.strip_wrappers() {
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
                let arm_returns = check_block(&arm.body, &mut arm_locals, function_name, ret_ty, funcs, const_env, type_env)?;
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
                let arm_returns = check_block(&arm.body, &mut arm_locals, function_name, ret_ty, funcs, const_env, type_env)?;
                all_return &= arm_returns;
            }
            if !(seen.contains("ok") && seen.contains("err")) {
                return Err(TypeCheckError::NonExhaustiveMatch { function: function_name.to_string() });
            }
        }
        TypeExpr::Named(type_name) => {
            let type_info = type_env.get(type_name).ok_or_else(|| TypeCheckError::UnknownType { name: type_name.clone() })?;
            for arm in arms {
                let mut arm_locals = locals.clone();
                let key = match &arm.pattern {
                    Pattern::Case { name, binding } => {
                        let case = type_info.cases.get(name).ok_or_else(|| TypeCheckError::InvalidPattern {
                            function: function_name.to_string(),
                            scrutinee: scrutinee_ty.clone(),
                        })?;
                        if let Some(bind) = binding {
                            if case.fields.len() == 1 {
                                arm_locals.insert(bind.clone(), LocalState {
                                    ty: case.fields[0].ty.clone(),
                                    moved: false,
                                    borrowed_view_count: 0,
                                });
                            } else {
                                return Err(TypeCheckError::InvalidPattern {
                                    function: function_name.to_string(),
                                    scrutinee: scrutinee_ty.clone(),
                                });
                            }
                        } else if !case.fields.is_empty() {
                            return Err(TypeCheckError::InvalidPattern {
                                function: function_name.to_string(),
                                scrutinee: scrutinee_ty.clone(),
                            });
                        }
                        name.clone()
                    }
                    _ => return Err(TypeCheckError::InvalidPattern {
                        function: function_name.to_string(),
                        scrutinee: scrutinee_ty.clone(),
                    }),
                };
                if !seen.insert(key) {
                    return Err(TypeCheckError::DuplicateMatchArm { function: function_name.to_string() });
                }
                let arm_returns = check_block(&arm.body, &mut arm_locals, function_name, ret_ty, funcs, const_env, type_env)?;
                all_return &= arm_returns;
            }
            let expected: BTreeSet<String> = type_info.cases.keys().cloned().collect();
            if seen != expected {
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
    const_env: &BTreeMap<String, TypeExpr>,
    type_env: &BTreeMap<String, TypeInfo>,
) -> Result<TypeExpr, TypeCheckError> {
    match expr {
        Expr::Ident(name) => {
            if let Some(ty) = const_env.get(name) {
                Ok(ty.clone())
            } else {
                use_ident(name, locals, function_name, false)
            }
        }
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
                let actual = infer_call_arg(arg, expected, locals, function_name, funcs, const_env, type_env)?;
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
            let lhs_ty = infer_expr(lhs, locals, function_name, funcs, const_env, type_env)?;
            let rhs_ty = infer_expr(rhs, locals, function_name, funcs, const_env, type_env)?;
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
    const_env: &BTreeMap<String, TypeExpr>,
    type_env: &BTreeMap<String, TypeInfo>,
) -> Result<TypeExpr, TypeCheckError> {
    if expected.is_view() {
        match expr {
            Expr::Ident(name) => {
                if let Some(ty) = const_env.get(name) {
                    Ok(TypeExpr::View(Box::new(ty.strip_wrappers().clone())))
                } else {
                    use_ident(name, locals, function_name, true)
                }
            }
            _ => infer_expr(expr, locals, function_name, funcs, const_env, type_env),
        }
    } else {
        infer_expr(expr, locals, function_name, funcs, const_env, type_env)
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
