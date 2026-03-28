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

## 5. Ownership relation to MIR

Current canonical position:
- ownership legality is primarily established before MIR
- MIR validation may later check consistency of inserted drop actions and control-flow obligations
- MIR is not yet required to re-prove the whole ownership model

## 6. Planned drop insertion

Drop insertion is not part of current canonical MIR, but will be a post-lowering pass.
