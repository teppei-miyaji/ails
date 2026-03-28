# Invalid MIR Test Strategy

Invalid MIR samples are currently expected to be constructed in Rust-side tests, not encoded as `.ails` source files.

Reason:
- many invalid MIR shapes cannot be produced from valid source parsing/typechecking
- the first validator slice should test malformed MIR directly

Suggested first invalid cases:
- duplicate block id
- dangling goto target
- dangling match arm target
- unreachable block
