# AILS Control-Flow Join Rules

## 1. Purpose

Ownership state after control flow must be deterministic and conservative.

## 2. If join rule (current canonical draft)

When an `if` has a reachable continuation after both branches,
the ownership state of every live local must match exactly across:
- then branch exit
- else branch exit

Exact match currently means:
- same `moved` bit
- same active `borrowed-view` count

If not equal, the program is rejected.

## 3. While loop rule (current canonical draft)

For a `while`, the loop body must preserve ownership state across one abstract iteration boundary.

The ownership state before entering the body must equal the state after the body exits.
If not equal, the program is rejected.

This is intentionally conservative.

## 4. Scope of the rule

These rules currently apply only to:
- local bindings already in scope before the branch/loop
- ownership state, not semantic value equality

They do not define:
- merge-by-dominance
- path-sensitive relaxation
- loop fixpoint inference

## 5. Canonical status

Exact-state equality is the source-of-truth rule until a later spec explicitly replaces it.
Implementations must not silently weaken it.
