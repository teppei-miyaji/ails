# MIR Structural Validator Starter

## Purpose

This document defines the first implementation slice for MIR validation.

## Scope of first slice

Only structural correctness:

- entry block existence
- unique block ids
- terminator presence
- valid block references
- match arm target validity
- reachability from entry
- unreachable block reporting

## Explicit non-goals

Do not implement yet:
- single-drop proof
- full ownership validation
- backend-aware checks
- panic/unwind handling
- optimization-aware CFG rewriting

## Suggested public API shape

Examples:
- `validate_function_structure(function: &MirFunction) -> MirValidationReport`
- `validate_module_structure(module: &MirModule) -> Vec<MirValidationReport>`

## Suggested report fields

- function name
- success flag
- list of structural errors
- list of unreachable block ids
- optional block-count summary

## Suggested error categories

- `MissingEntryBlock`
- `DuplicateBlockId`
- `MissingTerminator`
- `DanglingBlockReference`
- `DanglingMatchArmTarget`
- `StatementAfterTerminator`
- `UnreachableBlock`

## First success criterion

A developer can run the validator against hand-authored MIR fixtures and deterministically distinguish valid vs invalid graph structure.
