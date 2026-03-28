# Golden Test Plan

## Purpose

Add stable, reviewable fixtures that compare actual HIR/MIR output and validator results against expected outcomes.

## Fixture categories

### HIR dump fixtures
- valid simple function
- valid branch function
- valid match function

### MIR dump fixtures
- valid single-block MIR
- valid branch MIR
- valid match MIR

### MIR invalid fixtures
- missing terminator
- duplicate block id
- dangling goto target
- dangling match target
- unreachable block
- statement after terminator

## Expected outputs

Each fixture should define:
- input artifact
- expected validator status
- expected error category list
- expected unreachable block list if any

## Recommendation

Keep fixtures human-readable and small.
Prefer one failure reason per invalid fixture unless testing aggregation behavior.
