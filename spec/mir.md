# AILS MIR Specification

## 1. Role of MIR

MIR is the control-flow-oriented intermediate representation.

MIR is responsible for:
- basic blocks
- terminators
- explicit control-flow edges
- representation suitable for later validation and backend lowering

MIR is not yet the final machine-oriented IR.

## 2. Canonical structure

MIR contains:
- module metadata
- functions
- basic blocks
- statements
- terminators

Terminators currently include:
- `Return`
- `Goto`
- `If`
- `Match`
- `Unreachable`

## 3. MIR obligations

MIR lowering must preserve:
- control-flow meaning
- match arm dispatch shape
- statement execution order inside blocks

## 4. Required validation pass

A MIR validation pass is required after lowering.

At minimum it must verify:
- every block has exactly one terminator
- every referenced block id exists
- entry block exists per function
- no statement appears after a finalized terminator in the same block
- return expressions are type-compatible with function output
- match arm targets are valid blocks
- block graph is structurally well-formed

## 5. Drop responsibility boundary

Current canonical position:
- ownership legality is primarily established before MIR
- HIR does not contain explicit drop operations
- MIR lowering itself is not required to decide final drop insertion points
- drop insertion is a distinct post-lowering pass
- MIR validation before drop insertion checks structural correctness
- MIR validation after drop insertion must additionally check drop-placement consistency

## 6. v0.1 guarantee

v0.1 does **not** require MIR to encode a full linear type proof.
It only requires MIR to be suitable for a later drop-insertion and validation pipeline.

## 7. v0.19 drop insertion strategy boundary

For v0.19, the canonical drop insertion strategy is defined only at the level of placement classes, not a full algorithm.

Drop placement classes:
- scope-exit drops
- explicit return-path drops
- branch/loop edge drops when required to preserve ownership correctness
- critical-edge related placement after CFG shaping or edge splitting, if needed by the implementation

What is canonical:
- drop insertion happens after MIR lowering
- drop insertion may require CFG refinement such as edge splitting
- implementations must preserve semantic ownership obligations across all control-flow exits

What is not yet canonical:
- exact block-splitting algorithm
- optimal drop placement
- backend-specific destruction lowering

## 8. Placement-class to control-flow responsibility mapping

For v0.20, the mapping is canonicalized at an abstract level:

- scope-exit class  
  Responsible for values whose lifetime ends at lexical/block exit without earlier consumption

- return-path class  
  Responsible for values that must be dropped before function exit along explicit return paths

- branch-edge / loop-edge class  
  Responsible for values whose ownership obligations diverge across outgoing control-flow edges and therefore must be resolved before or along edge traversal

- critical-edge related class  
  Responsible for cases where a placement obligation cannot be attached unambiguously without CFG refinement

This mapping is normative as a classification scheme, not as a concrete algorithm.

## 9. Single-drop property

A future MIR validation pass must enforce an abstract **single-drop property**:

For every owned value introduced into MIR-level resource tracking, along any realizable path:
- it is consumed exactly once by move or drop
- it is never both moved and dropped
- it is never dropped more than once

v0.20 defines this as a validation goal, not yet as a full proof procedure.

## 10. Planned drop insertion

Drop insertion is not part of current canonical MIR, but will be a post-lowering pass.
