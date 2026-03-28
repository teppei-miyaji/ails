# v0.23 First Implementation Task

## Task

Implement `MirStructuralValidator` for:
- entry block presence
- unique block ids
- valid block references
- match arm target validity
- reachability from entry

## Why this first

This is the lowest-risk executable slice because:
- it depends on settled MIR structure
- it avoids unresolved ownership/drop details
- it provides immediate feedback about graph correctness
- it becomes the base for later single-drop validation

## Definition of done

- validator types exist in code
- validator entry functions exist
- structural error categories are stable
- at least one valid and one invalid sample path are documented
