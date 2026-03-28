# MIR Validator Design (v0.22 Draft)

## Goal

Implement the first MIR validation pass consistent with canonical spec v0.20.

## Input assumptions

The validator receives MIR after lowering and before drop insertion.

The validator must not assume:
- backend-specific layout facts
- panic/unwind semantics
- full linear ownership proof

The validator may assume:
- one entry function object at a time
- a declared set of blocks with stable block ids
- validator input is syntactically decoded MIR, not raw source

## Required invariants for input shape

Before deeper checks, the validator treats these as mandatory invariants to verify:
- block ids are unique within a function
- there is exactly one entry block
- every block has exactly one terminator
- all block references in terminators point to declared blocks
- no statement is placed after a finalized terminator
- all match arm targets are declared blocks

## Validator layers

### Layer 1: Structural validation
Checks:
- every function has an entry block
- every block has exactly one terminator
- all referenced block ids exist
- block ids are unique
- no statements appear after terminator-finalization
- all match arm targets are valid
- graph shape is structurally valid
- all blocks are either reachable from entry or explicitly reported as unreachable

### Layer 2: Type-consistency validation
Checks:
- return expressions are compatible with function output
- terminator-carried expressions use known values
- match dispatch shape remains type-plausible

### Layer 3: Ownership/drop preparation validation
Checks:
- MIR contains sufficient ownership-relevant identity for later drop reasoning
- placement-class candidates can be derived from exits and edges
- paths that would require impossible placement are surfaced early

### Layer 4: Single-drop property validation (future within v0.22/v0.23)
Abstract goal:
- an owned value is consumed once by move or drop along any realizable path
- never moved and dropped both
- never dropped more than once

## Recommended implementation order

1. structural validator
2. reachability analysis
3. return/type consistency validator
4. placement-class extraction helper
5. abstract single-drop checker skeleton
6. integrate with diagnostics

## Minimum artifact outputs

- validator result enum
- per-function validation report
- stable error categories
- golden valid/invalid MIR samples
