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

## Rule

If code and spec disagree, update code or mark implementation stale.
Do not reinterpret the spec to fit accidental implementation behavior.
