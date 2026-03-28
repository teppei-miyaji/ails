# AILS Diagnostics Specification

## 1. Principle

Diagnostics must prefer:
- explicit cause
- explicit location
- explicit expected vs found information

## 2. Required categories

- parse errors
- unknown identifier / function / type
- duplicate declaration
- type mismatch
- non-exhaustive match
- ownership misuse
- control-flow join state mismatch

## 3. AI-friendly diagnostics

Diagnostics should be stable and machine-readable enough that an AI can:
- identify the failing rule
- identify the source binding name
- generate a targeted fix

## 4. Additional v0.19 guidance

When partial move is forbidden, diagnostics should explicitly state:
- that partial move is not part of v0.1 canonical semantics
- whether the value was consumed by by-value `match`
- which binding became unavailable

## 5. Standard remediation patterns for forbidden partial move

Diagnostics should prefer one or more of these remediation suggestions when applicable:
- bind the whole value instead of extracting a partial sub-structure
- restructure the sum type into smaller explicitly moved units
- perform the operation before the by-value `match`
- introduce a helper function that consumes the whole matched value once
- avoid relying on unimplemented borrow-binding semantics

These are guidance patterns, not automatic rewrite guarantees.

## 6. Avoid

- vague panic-only behavior
- context-free one-word messages
- diagnostics that omit the binding or rule name
