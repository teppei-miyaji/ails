# AILS Control-Flow Join Rules

## If join rule

When an `if` has a reachable continuation after both branches,
the ownership state of every live local must match exactly across:
- then branch exit
- else branch exit

Exact match currently means:
- same `moved` bit
- same active `borrowed-view` count

If not equal, the program is rejected.

## While loop rule

For a `while`, the loop body must preserve ownership state across one abstract iteration boundary.

The ownership state before entering the body must equal the state after the body exits.
If not equal, the program is rejected.

This is intentionally conservative.
