use std::collections::{BTreeMap, BTreeSet};

use crate::{MirBlock, MirFunction, MirModule, MirTerminator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MirStructuralErrorKind {
    MissingEntryBlock,
    DuplicateBlockId { block_id: usize },
    MissingTerminator { block_id: usize },
    DanglingBlockReference { from_block: usize, target_block: usize },
    DanglingMatchArmTarget { from_block: usize, target_block: usize },
    StatementAfterTerminator { block_id: usize },
    UnreachableBlock { block_id: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirStructuralError {
    pub function_name: String,
    pub kind: MirStructuralErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirValidationReport {
    pub function_name: String,
    pub success: bool,
    pub block_count: usize,
    pub unreachable_blocks: Vec<usize>,
    pub errors: Vec<MirStructuralError>,
}

impl MirValidationReport {
    pub fn ok(function_name: &str, block_count: usize) -> Self {
        Self {
            function_name: function_name.to_string(),
            success: true,
            block_count,
            unreachable_blocks: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn push_error(&mut self, kind: MirStructuralErrorKind) {
        self.errors.push(MirStructuralError {
            function_name: self.function_name.clone(),
            kind,
        });
        self.success = false;
    }
}

pub fn validate_module_structure(module: &MirModule) -> Vec<MirValidationReport> {
    module.functions.iter().map(validate_function_structure).collect()
}

pub fn validate_function_structure(function: &MirFunction) -> MirValidationReport {
    let mut report = MirValidationReport::ok(&function.name, function.blocks.len());

    if function.blocks.is_empty() {
        report.push_error(MirStructuralErrorKind::MissingEntryBlock);
        return report;
    }

    let mut id_to_index = BTreeMap::<usize, usize>::new();
    for (index, block) in function.blocks.iter().enumerate() {
        if id_to_index.insert(block.id, index).is_some() {
            report.push_error(MirStructuralErrorKind::DuplicateBlockId { block_id: block.id });
        }
    }

    if !id_to_index.contains_key(&0) {
        report.push_error(MirStructuralErrorKind::MissingEntryBlock);
    }

    for block in &function.blocks {
        validate_block_references(&function.name, block, &id_to_index, &mut report);
    }

    if let Some(entry_index) = id_to_index.get(&0).copied() {
        let reachable = collect_reachable_blocks(&function.blocks, entry_index, &id_to_index);
        for block in &function.blocks {
            if !reachable.contains(&block.id) {
                report.unreachable_blocks.push(block.id);
                report.push_error(MirStructuralErrorKind::UnreachableBlock { block_id: block.id });
            }
        }
    }

    report
}

fn validate_block_references(
    function_name: &str,
    block: &MirBlock,
    id_to_index: &BTreeMap<usize, usize>,
    report: &mut MirValidationReport,
) {
    match &block.terminator {
        MirTerminator::Goto(target) => {
            if !id_to_index.contains_key(target) {
                report.push_error(MirStructuralErrorKind::DanglingBlockReference {
                    from_block: block.id,
                    target_block: *target,
                });
            }
        }
        MirTerminator::If { then_block, else_block, .. } => {
            if !id_to_index.contains_key(then_block) {
                report.push_error(MirStructuralErrorKind::DanglingBlockReference {
                    from_block: block.id,
                    target_block: *then_block,
                });
            }
            if !id_to_index.contains_key(else_block) {
                report.push_error(MirStructuralErrorKind::DanglingBlockReference {
                    from_block: block.id,
                    target_block: *else_block,
                });
            }
        }
        MirTerminator::Match { arms, .. } => {
            for arm in arms {
                if !id_to_index.contains_key(&arm.target_block) {
                    report.push_error(MirStructuralErrorKind::DanglingMatchArmTarget {
                        from_block: block.id,
                        target_block: arm.target_block,
                    });
                }
            }
        }
        MirTerminator::Return(_) | MirTerminator::Unreachable => {
            let _ = function_name;
        }
    }
}

fn collect_reachable_blocks(
    blocks: &[MirBlock],
    entry_index: usize,
    id_to_index: &BTreeMap<usize, usize>,
) -> BTreeSet<usize> {
    let mut seen = BTreeSet::<usize>::new();
    let mut stack = vec![blocks[entry_index].id];

    while let Some(block_id) = stack.pop() {
        if !seen.insert(block_id) {
            continue;
        }
        let index = match id_to_index.get(&block_id).copied() {
            Some(index) => index,
            None => continue,
        };
        match &blocks[index].terminator {
            MirTerminator::Goto(target) => stack.push(*target),
            MirTerminator::If { then_block, else_block, .. } => {
                stack.push(*then_block);
                stack.push(*else_block);
            }
            MirTerminator::Match { arms, .. } => {
                for arm in arms {
                    stack.push(arm.target_block);
                }
            }
            MirTerminator::Return(_) | MirTerminator::Unreachable => {}
        }
    }

    seen
}
