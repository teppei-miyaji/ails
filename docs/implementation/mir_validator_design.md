# MIR Validator Design (v0.21 Draft)

## Goal

Implement the first MIR validation pass consistent with canonical spec v0.20.

## Validator layers

### Layer 1: Structural validation
Checks:
- every function has an entry block
- every block has exactly one terminator
- all referenced block ids exist
- no statements appear after terminator-finalization
- all match arm targets are valid
- graph shape is structurally valid

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

### Layer 4: Single-drop property validation (future within v0.21/v0.22)
Abstract goal:
- an owned value is consumed once by move or drop along any realizable path
- never moved and dropped both
- never dropped more than once

## Recommended implementation order

1. structural validator
2. return/type consistency validator
3. placement-class extraction helper
4. abstract single-drop checker skeleton
5. integrate with diagnostics

## Minimum artifact outputs
- validator result enum
- per-function validation report
- stable error categories
