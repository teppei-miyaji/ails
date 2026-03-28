use ails_ast::{BinaryOp, TypeExpr};
use ails_hir::{HirExpr, HirFunction, HirMatchArm, HirModule, HirPattern, HirStmt};

#[derive(Debug, Clone)]
pub struct MirModule {
    pub module_name: String,
    pub functions: Vec<MirFunction>,
}

#[derive(Debug, Clone)]
pub struct MirFunction {
    pub name: String,
    pub params: Vec<MirParam>,
    pub output: TypeExpr,
    pub blocks: Vec<MirBlock>,
}

#[derive(Debug, Clone)]
pub struct MirParam {
    pub name: String,
    pub ty: TypeExpr,
}

#[derive(Debug, Clone)]
pub struct MirBlock {
    pub id: usize,
    pub statements: Vec<MirStatement>,
    pub terminator: MirTerminator,
}

#[derive(Debug, Clone)]
pub enum MirStatement {
    Let { name: String, ty: TypeExpr, expr: MirExpr },
    Set { name: String, expr: MirExpr },
    Eval(MirExpr),
}

#[derive(Debug, Clone)]
pub enum MirTerminator {
    Return(MirExpr),
    Goto(usize),
    If {
        cond: MirExpr,
        then_block: usize,
        else_block: usize,
    },
    Match {
        scrutinee: MirExpr,
        arms: Vec<MirMatchArm>,
    },
    Unreachable,
}

#[derive(Debug, Clone)]
pub struct MirMatchArm {
    pub pattern: MirPattern,
    pub target_block: usize,
}

#[derive(Debug, Clone)]
pub enum MirPattern {
    Some(String),
    None,
    Ok(String),
    Err(String),
    Case { name: String, binding: Option<String> },
}

#[derive(Debug, Clone)]
pub enum MirExpr {
    Ident(String),
    Int(i64),
    Bool(bool),
    Call {
        callee: String,
        args: Vec<MirExpr>,
    },
    Binary {
        op: BinaryOp,
        lhs: Box<MirExpr>,
        rhs: Box<MirExpr>,
    },
}

pub fn lower_module(module: &HirModule) -> MirModule {
    MirModule {
        module_name: module.module_name.clone(),
        functions: module.functions.iter().map(lower_function).collect(),
    }
}

fn lower_function(function: &HirFunction) -> MirFunction {
    let mut builder = MirBuilder::default();
    let entry = builder.new_block();
    builder.current = entry;
    lower_block(&function.body, &mut builder);

    MirFunction {
        name: function.name.clone(),
        params: function.inputs.iter().map(|p| MirParam {
            name: p.name.clone(),
            ty: p.ty.clone(),
        }).collect(),
        output: function.output.clone(),
        blocks: builder.blocks,
    }
}

#[derive(Debug, Default)]
struct MirBuilder {
    blocks: Vec<MirBlock>,
    current: usize,
}

impl MirBuilder {
    fn new_block(&mut self) -> usize {
        let id = self.blocks.len();
        self.blocks.push(MirBlock {
            id,
            statements: Vec::new(),
            terminator: MirTerminator::Unreachable,
        });
        id
    }

    fn current_block_mut(&mut self) -> &mut MirBlock {
        &mut self.blocks[self.current]
    }

    fn push_stmt(&mut self, stmt: MirStatement) {
        self.current_block_mut().statements.push(stmt);
    }

    fn set_terminator(&mut self, term: MirTerminator) {
        self.current_block_mut().terminator = term;
    }
}

fn lower_block(stmts: &[HirStmt], builder: &mut MirBuilder) {
    for stmt in stmts {
        lower_stmt(stmt, builder);
    }
}

fn lower_stmt(stmt: &HirStmt, builder: &mut MirBuilder) {
    match stmt {
        HirStmt::Let { name, ty, expr } => {
            builder.push_stmt(MirStatement::Let {
                name: name.clone(),
                ty: ty.clone(),
                expr: lower_expr(expr),
            });
        }
        HirStmt::Set { name, expr } => {
            builder.push_stmt(MirStatement::Set {
                name: name.clone(),
                expr: lower_expr(expr),
            });
        }
        HirStmt::Return(expr) => {
            builder.set_terminator(MirTerminator::Return(lower_expr(expr)));
            let next = builder.new_block();
            builder.current = next;
        }
        HirStmt::If { cond, then_body, else_body } => {
            let then_block = builder.new_block();
            let else_block = builder.new_block();
            let join_block = builder.new_block();

            builder.set_terminator(MirTerminator::If {
                cond: lower_expr(cond),
                then_block,
                else_block,
            });

            builder.current = then_block;
            lower_block(then_body, builder);
            if matches!(builder.current_block_mut().terminator, MirTerminator::Unreachable) {
                builder.set_terminator(MirTerminator::Goto(join_block));
            }

            builder.current = else_block;
            lower_block(else_body, builder);
            if matches!(builder.current_block_mut().terminator, MirTerminator::Unreachable) {
                builder.set_terminator(MirTerminator::Goto(join_block));
            }

            builder.current = join_block;
        }
        HirStmt::While { cond, body } => {
            let head_block = builder.new_block();
            let body_block = builder.new_block();
            let exit_block = builder.new_block();

            builder.set_terminator(MirTerminator::Goto(head_block));

            builder.current = head_block;
            builder.set_terminator(MirTerminator::If {
                cond: lower_expr(cond),
                then_block: body_block,
                else_block: exit_block,
            });

            builder.current = body_block;
            lower_block(body, builder);
            if matches!(builder.current_block_mut().terminator, MirTerminator::Unreachable) {
                builder.set_terminator(MirTerminator::Goto(head_block));
            }

            builder.current = exit_block;
        }
        HirStmt::Match { scrutinee, arms } => {
            let arm_blocks: Vec<usize> = arms.iter().map(|_| builder.new_block()).collect();
            let join_block = builder.new_block();

            builder.set_terminator(MirTerminator::Match {
                scrutinee: lower_expr(scrutinee),
                arms: arms.iter().zip(arm_blocks.iter()).map(|(arm, id)| MirMatchArm {
                    pattern: lower_pattern(&arm.pattern),
                    target_block: *id,
                }).collect(),
            });

            for (arm, block_id) in arms.iter().zip(arm_blocks.iter()) {
                builder.current = *block_id;
                lower_match_arm(arm, builder);
                if matches!(builder.current_block_mut().terminator, MirTerminator::Unreachable) {
                    builder.set_terminator(MirTerminator::Goto(join_block));
                }
            }

            builder.current = join_block;
        }
    }
}

fn lower_match_arm(arm: &HirMatchArm, builder: &mut MirBuilder) {
    lower_block(&arm.body, builder);
}

fn lower_pattern(pattern: &HirPattern) -> MirPattern {
    match pattern {
        HirPattern::Some(name) => MirPattern::Some(name.clone()),
        HirPattern::None => MirPattern::None,
        HirPattern::Ok(name) => MirPattern::Ok(name.clone()),
        HirPattern::Err(name) => MirPattern::Err(name.clone()),
        HirPattern::Case { name, binding } => MirPattern::Case {
            name: name.clone(),
            binding: binding.clone(),
        },
    }
}

fn lower_expr(expr: &HirExpr) -> MirExpr {
    match expr {
        HirExpr::Ident(name) => MirExpr::Ident(name.clone()),
        HirExpr::Int(v) => MirExpr::Int(*v),
        HirExpr::Bool(v) => MirExpr::Bool(*v),
        HirExpr::Call { callee, args } => MirExpr::Call {
            callee: callee.clone(),
            args: args.iter().map(lower_expr).collect(),
        },
        HirExpr::Binary { op, lhs, rhs } => MirExpr::Binary {
            op: *op,
            lhs: Box::new(lower_expr(lhs)),
            rhs: Box::new(lower_expr(rhs)),
        },
    }
}
