# AILS Ownership Semantics

## 1. Core model

Every local binding has an ownership state:

- `available`
- `moved`
- `borrowed-view(n)`

`borrowed-view(n)` means there are `n` active read-only borrows.

## 2. Move-only values

Any value of type `own T` is move-only.

Consuming uses include:
- passing to a parameter that is not `view`
- assigning into another `own` binding by value
- returning by value
- binding payloads by move in pattern-matching contexts explicitly defined as move-binding
- by-value `match` over the scrutinee itself

After a consuming use, the local enters `moved`.

## 3. View borrows

If a callee expects `view T`, passing a compatible identifier creates a temporary read-only borrow.

During the borrow:
- the source local must not be moved
- the source local must not be updated

## 4. Canonical lifetime rule for call borrows

For v0.1, a borrow created for a function call lives:
- from evaluation start of that call argument
- until the call expression finishes producing its result

This is expression-scoped, not statement-scoped.

## 5. Canonical evaluation order for call arguments

Function call arguments are evaluated **left to right**.

Consequences:
- borrows created by earlier arguments remain active while later arguments are evaluated
- nested calls may create overlapping expression-scoped borrow windows
- implementations must not reorder argument evaluation in a way that changes borrow legality

## 6. Update rule

`set x = expr` is forbidden when `x` has active view borrows.

## 7. Moved rule

Any use of a moved local is invalid.

## 8. Ownership categories for pattern payloads

Pattern payload extraction must not be implicit.

Current canonical rule:
- `option T`, `result T E`, and named sum payload bindings are move-bindings when the scrutinee is matched by value
- partial move is forbidden in v0.1
- if the scrutinee type is wrapped in `view`, payload extraction with binding is not canonicalized and must be rejected
- payload-less by-value `match` still consumes the scrutinee as a whole

## 9. Explicitly unspecified for now

The following remain intentionally non-canonical:
- borrow-binding of payloads from `view`-wrapped scrutinees
- mutable borrows
- non-lexical lifetime style behavior
- field-level borrow splitting

Implementations must not invent these silently.
