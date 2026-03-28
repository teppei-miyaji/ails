# v0.25 Driver Wiring

## Goal

Add a first executable draft path for MIR structural validation.

## Added command

Planned and draft-wired command:
- `validate-mir-structure <input-file>`

## Pipeline

- parse source
- typecheck
- lower to HIR
- lower to MIR
- run `ails_mir::validate_module_structure`
- print per-function reports
- return failure if any report has `success = false`

## Status

This is still an unverified draft implementation until real build/test execution confirms crate integration.
