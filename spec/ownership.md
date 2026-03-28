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

After a consuming use, the local enters `moved`.

## 3. View borrows

If a callee expects `view T`, passing a compatible identifier creates a temporary read-only borrow.

During the borrow:
- the source local must not be moved
- the source local must not be updated

## 4. Current canonical lifetime rule for call borrows

For the current canonical draft, a borrow created for a function call lives:
- from evaluation start of that call argument
- until the call expression finishes producing its result

This is expression-scoped, not statement-scoped.

That means:
- the borrow does not survive after the call expression completes
- nested calls may extend the effective borrow window of inner argument evaluation
- future revisions may introduce region-based lifetime modeling, but implementations must behave at least as conservatively as this rule

## 5. Update rule

`set x = expr` is forbidden when `x` has active view borrows.

## 6. Moved rule

Any use of a moved local is invalid.

## 7. Ownership categories for pattern payloads

Pattern payload extraction must not be implicit.
Current canonical rule:

- `option T`, `result T E`, and named sum payload bindings are **move-bindings** when the scrutinee is matched by value
- if the scrutinee type is wrapped in `view`, payload extraction is not currently canonicalized for direct binding
- implementations must reject ambiguous borrow-vs-move payload extraction rather than guessing

This means a pattern like:

```ails
match r
case ok value
begin
    return value
end
end
```

binds `value` by move when `r` is matched by value.

## 8. Explicitly unspecified for now

The following remain intentionally non-canonical:
- partial move from multi-field payloads
- borrow-binding of payloads from `view`-wrapped scrutinees
- mutable borrows
- non-lexical lifetime style behavior
- field-level borrow splitting

Implementations must not invent these silently.
