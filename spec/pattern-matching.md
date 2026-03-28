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
- multi-field destructuring is not part of v0.1 canonical grammar

## 4. Payload ownership rule

Payload bindings are **move-bindings** when the scrutinee is matched by value.

This rule applies to:
- `some x`
- `ok x`
- `err e`
- named `case tag payload`

Therefore, after such a binding, the payload is considered transferred into the new local binding.

## 5. Partial move policy for v0.1

Partial move is **forbidden** in v0.1 canonical semantics.

That means:
- multi-field payload destructuring is not allowed
- field-by-field ownership splitting is not allowed
- pattern matching must not leave the original scrutinee partially consumed

If a future revision wants partial move, it must introduce it explicitly.

## 6. Scrutinee consumption rule for payload-less cases

In v0.19, by-value `match` consumes the scrutinee as a whole, regardless of whether the selected arm binds a payload.

This means:
- `case none`
- `case a` where `a` has zero payload fields

still consume the matched scrutinee when the scrutinee is matched by value.

As a result, after a by-value `match`, the original scrutinee must be considered unavailable for reuse in all arms and after the match.

## 7. View scrutinee limitation

Pattern matching over a `view`-wrapped scrutinee is not canonicalized for payload extraction.

For v0.1:
- implementations may allow case discrimination without payload extraction
- implementations must reject payload-binding forms that would require borrow-vs-move choice from a `view` scrutinee

## 8. Duplicate and invalid patterns

Within one `match`:
- duplicate cases are invalid
- unknown named cases are invalid
- non-exhaustive coverage is invalid

## 9. Future extension point

A future revision may introduce explicit syntax for:
- borrow-binding payloads
- multi-field destructuring
- wildcard/default arms

Until then, implementations must stay within the current strict subset.
