# Backend Handoff Draft

## Purpose

Bridge canonical semantic layout and MIR placement classes toward backend work without fixing concrete ABI yet.

## Backend receives
- semantically named sum types
- placement-class annotated drop obligations (eventually)
- MIR blocks and terminators
- ownership-relevant value identities

## Backend must not assume yet
- concrete sum tag size from language spec
- niche optimization
- precise panic/unwind model
- returned `view` semantics

## First backend contract candidates
- x86_64 only
- integer primitives and unit
- no returned `view`
- no multi-field destructuring
- conservative stack-based temporaries
- explicit drop events after post-MIR insertion phase

## Recommendation
Do not start real codegen before:
1. MIR structural validator exists
2. drop placement-class extraction exists
3. implementation is aligned with v0.20 ownership rules
