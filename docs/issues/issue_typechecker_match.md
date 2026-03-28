# Issue Draft: typechecker: add `match` exhaustiveness and branch typing

## Summary
Extend the type checker so `match` can be type-checked and validated for exhaustiveness.

## Scope
- validate scrutinee type
- validate case-to-type relation
- merge branch return shape conservatively
- report non-exhaustive match
- compile-fail tests

## Acceptance criteria
- missing case is rejected
- invalid case name is rejected
- valid `result` / `option` match passes

## Out of scope
- ownership-aware pattern matching


Status: partially implemented in v0.7 draft artifact.
