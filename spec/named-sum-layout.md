# AILS Named Sum Type Layout

## 1. Status

Abstract canonical model only.
This document does not define a concrete backend ABI.

## 2. Abstract model

A named sum type value consists of:
- a discriminant identifying the active `case`
- zero or more payload fields associated with that `case`

## 3. Canonical semantic guarantees

Implementations must preserve:
- exact case identity
- declared field order within a case
- payload field type identity

## 4. v0.1 boundary

For v0.1, the following are intentionally not fixed:
- concrete byte layout
- discriminant size
- niche optimization
- field padding strategy
- backend ABI lowering

Therefore the language spec fixes semantic layout only, not physical layout.

## 5. Future backend handoff

A later backend contract may choose a concrete representation, but it must preserve the semantic guarantees above.
