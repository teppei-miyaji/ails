# AILS Pattern Matching

## 1. Matchable types

Current matchable types:
- `option T`
- `result T E`
- named sum types introduced by `type`

## 2. Exhaustiveness

A `match` must be exhaustive.

For `option T`:
- `some`
- `none`

For `result T E`:
- `ok`
- `err`

For a named sum type:
- every declared `case` must appear exactly once

## 3. Pattern binding rules

### `option T`

- `case some x` binds `x: T`
- `case none` binds nothing

### `result T E`

- `case ok x` binds `x: T`
- `case err e` binds `e: E`

### Named sum type

- `case name` is valid only when that case has zero fields
- `case name x` is valid only when that case has exactly one field
- multi-field destructuring is not yet part of the canonical grammar

## 4. Payload ownership rule

Payload bindings are currently **move-bindings** when the scrutinee is matched by value.

This rule applies to:
- `some x`
- `ok x`
- `err e`
- named `case tag payload`

Therefore, after such a binding, the matched payload is considered transferred into the new local binding.

## 5. View scrutinee limitation

Pattern matching over a `view`-wrapped scrutinee is not yet canonicalized for payload extraction.

Until a later revision:
- implementations may allow case discrimination
- implementations must reject payload-binding forms that would require ambiguous borrow-vs-move extraction

## 6. Duplicate and invalid patterns

Within one `match`:
- duplicate cases are invalid
- unknown named cases are invalid
- non-exhaustive coverage is invalid

## 7. Future extension point

A future revision may introduce explicit syntax for:
- borrow-binding payloads
- multi-field destructuring
- wildcard/default arms

Until then, implementations must stay within the current strict subset.
