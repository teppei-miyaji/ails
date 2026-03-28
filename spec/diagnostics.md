# AILS Diagnostics Specification

## Principle

Diagnostics must prefer:
- explicit cause
- explicit location
- explicit expected vs found information

## Required categories

- parse errors
- unknown identifier / function / type
- duplicate declaration
- type mismatch
- non-exhaustive match
- ownership misuse
- control-flow join state mismatch

## AI-friendly diagnostics

Diagnostics should be stable and machine-readable enough that an AI can identify the failing rule and generate a targeted fix.
