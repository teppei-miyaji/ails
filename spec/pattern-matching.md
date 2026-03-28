# AILS Pattern Matching

## Matchable types

- `option T`
- `result T E`
- named sum types introduced by `type`

## Exhaustiveness

A `match` must be exhaustive.

For `option T`:
- `some`
- `none`

For `result T E`:
- `ok`
- `err`

For a named sum type:
- every declared `case` must appear exactly once

## Pattern binding rules

`option T`
- `case some x` binds `x: T`
- `case none` binds nothing

`result T E`
- `case ok x` binds `x: T`
- `case err e` binds `e: E`

Named sum type
- `case name` is valid only when that case has zero fields
- `case name x` is valid only when that case has exactly one field
- multi-field destructuring is not yet part of the canonical grammar

## Important limitation

Exact borrow-versus-move semantics for payload extraction are not yet fully canonicalized.
Implementations must use conservative behavior and document it.
