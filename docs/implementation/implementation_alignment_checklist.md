# Implementation Alignment Checklist

## Purpose

Bring the Rust draft implementation into alignment with canonical spec v0.20.

## Phase A: Terminology alignment
- rename or annotate implementation concepts that disagree with spec terminology
- ensure `own`, `view`, by-value match consumption, and placement-class language match spec text
- ensure docs do not imply unsupported semantics such as partial move or `view` return

## Phase B: Typechecker alignment
- enforce ban on `view` return types
- enforce scrutinee consumption after by-value match, including payload-less cases
- enforce left-to-right call evaluation assumptions in diagnostics and checker logic
- enforce nested-call borrow legality under outer-expression evaluation context
- reject payload extraction from `view` scrutinees

## Phase C: MIR structural alignment
- ensure MIR retains enough information for post-lowering drop insertion classes
- ensure basic block and terminator structure is validation-ready
- ensure no pass silently invents ownership behavior

## Phase D: Diagnostics alignment
- add explicit messages for:
  - forbidden partial move
  - moved-after-match reuse
  - update while borrowed
  - nested-call borrow conflicts
  - exact-equality join violations

## Phase E: Golden test alignment
Prepare canonical valid/invalid samples for:
- valid minimal single-block MIR
- invalid missing-terminator MIR
- invalid dangling-block-reference MIR
- invalid unreachable-block MIR
- invalid duplicate-block-id MIR
- valid branch MIR with consistent edges

## Exit criteria
- implementation terminology is aligned
- typechecker behavior matches canonical v0.20
- MIR shape supports validator design
- diagnostics expose rule names and affected bindings
- canonical MIR samples exist for golden testing
