# Issue Draft: typechecker: add ownership checks for `own` / `view`

## Summary
Implement the first ownership-aware type checking pass.

## Scope
- surface syntax for `own` / `view`
- local ownership state
- move-after-use detection
- borrow/update conflict detection
- ownership diagnostics

## Acceptance criteria
- compile-fail move-after-use example
- compile-fail borrow-while-update example
- compile-pass read-only `view` example

## Out of scope
- non-lexical borrows
- mutable references


Status: partially implemented in v0.8 draft artifact.
