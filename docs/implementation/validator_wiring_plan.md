# Validator Wiring Plan (v0.24)

## Goal

Connect the draft `MirStructuralValidator` to an executable path without yet requiring the full finalized CLI implementation.

## Intended command surface

Planned subcommand:
- `validate-mir-structure <input-file>`

## Intended pipeline

1. read source file
2. parse into AST
3. typecheck AST
4. lower AST to HIR
5. lower HIR to MIR
6. run `ails_mir::validate_module_structure`
7. emit a per-function report

## Output shape

For the first executable slice, output may remain debug-oriented.

Required minimum output:
- function name
- success/failure
- block count
- unreachable blocks
- structural error list

## Failure model

The command should:
- return non-zero if any function fails structural validation
- still print all function reports when possible
- distinguish parse/type/lowering errors from MIR structural validation errors

## Non-goals

- pretty diagnostics formatting
- snapshot stabilization
- JSON schema stability
- single-drop validation
