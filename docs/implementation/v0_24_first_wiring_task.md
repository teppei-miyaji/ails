# v0.24 First Wiring Task

## Task

Create the first executable path from source input to structural MIR validation report.

## Smallest acceptable slice

- one helper function in driver-side code path
- run parse -> typecheck -> HIR -> MIR -> validate
- print debug-style report
- fail process if any function report has `success = false`

## Why this before richer tests

This proves:
- crate boundaries are adequate
- validator types are usable outside `ails-mir`
- lowering output is compatible with validator input assumptions

## Immediate follow-up

After this wiring exists:
1. add source-driven valid fixtures
2. add Rust-side invalid MIR constructor tests
3. add stable error-category assertions
