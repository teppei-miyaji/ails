# Diagnostics Remediation Catalog

## Purpose

Provide canonical AI-readable repair guidance for common AILS ownership failures.

## Forbidden partial move
Suggested remediations:
- consume the whole matched value once
- move the operation before by-value `match`
- introduce helper function that consumes the whole value
- redesign sum payload into smaller explicit ownership units

## Moved-after-match reuse
Suggested remediations:
- avoid reusing the original scrutinee after by-value `match`
- bind and return the consumed result instead
- switch to a non-consuming future construct only if later spec supports it

## Update while borrowed
Suggested remediations:
- perform the update before creating the `view` borrow
- split expression into steps so borrow ends before update
- avoid passing the same binding as both borrowed and consumed in one expression

## Nested-call borrow conflict
Suggested remediations:
- evaluate the consuming value before the borrowed call, when semantically valid
- introduce temporary locals to separate phases
- avoid combining `view` and `own` access to the same binding in one outer call

## Exact-equality join violation
Suggested remediations:
- make both branches consume or preserve the same ownership state
- move ownership-affecting action before the branch
- return early from both branches if merge is unnecessary
