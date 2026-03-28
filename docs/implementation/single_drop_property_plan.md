# Single-Drop Property Plan

## Purpose

Translate the abstract single-drop property from spec v0.20 into implementation milestones.

## v0.21 target
Do not solve full linear proof.
Instead:
- define ownership-tracked entities in MIR
- define path-sensitive event kinds: `Move`, `Drop`, `ReturnEscape`
- define invalid combinations:
  - `Move` then `Drop`
  - `Drop` then `Drop`
  - inconsistent unresolved ownership at all exits

## Event model
For each tracked value:
- source binding / temp id
- introduction site
- move events
- drop events
- escape events if applicable later

## Validation approach
- begin with conservative per-block/per-edge collection
- only accept behavior proved safe
- defer optimizations and equivalence collapsing

## Non-goals for first pass
- panic/unwind
- alias-aware borrow graph
- backend register-aware destruction
