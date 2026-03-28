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

## 7. Planned drop insertion

Drop insertion is not part of current canonical MIR, but will be a post-lowering pass.
