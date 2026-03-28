# AILS Specification-to-Implementation Mapping

## Purpose

This document maps canonical spec responsibilities to implementation phases.

## Phase responsibilities

### Parser
Responsible for:
- grammar recognition
- syntactic structure
- declaration ordering acceptance/rejection
- parse errors

Not responsible for:
- ownership legality
- exhaustiveness proofs
- control-flow join validation

### Typechecker
Responsible for:
- name resolution
- type resolution
- match exhaustiveness
- ownership legality at source-level
- branch/loop join validation
- v0.1 prohibition of `view` return types
- call-argument left-to-right borrow legality

Not responsible for:
- CFG construction
- backend lowering
- final drop insertion

### HIR
Responsible for:
- structured post-parse/post-check representation
- preserving declarations and nesting
- being easy to lower into MIR

### MIR
Responsible for:
- explicit control-flow graph structure
- block/terminator validity
- representation suitable for later validation and backend stages

### Post-MIR drop insertion
Responsible for:
- inserting explicit drop actions
- refining CFG when needed for drop placement
- preserving ownership obligations on all exits

Not responsible for:
- silently inventing source-level ownership semantics absent from spec

## Rule

If code and spec disagree, update code or mark implementation stale.
Do not reinterpret the spec to fit accidental implementation behavior.
