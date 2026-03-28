# AILS Ownership Semantics

## Core model

Every local binding has an ownership state:
- `available`
- `moved`
- `borrowed-view(n)`

## Move-only values

Any value of type `own T` is move-only.

Consuming uses include:
- passing to a non-`view` parameter
- assigning into another `own` binding by value
- returning by value
- using as a plain identifier expression where the surrounding rule consumes ownership

After a consuming use, the local enters `moved`.

## View borrows

If a callee expects `view T`, passing a compatible identifier creates a temporary borrow.

During the borrow:
- the source local must not be moved
- the source local must not be updated

## Update rule

`set x = expr` is forbidden when `x` has active view borrows.

## Moved rule

Any use of a moved local is invalid.
