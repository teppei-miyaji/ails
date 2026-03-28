# TODO

This file tracks the practical next steps for the public AILS repository.

## Current phase

Public draft repository with:
- lexer
- parser
- AST
- minimal HIR skeleton
- minimal type checker
- CLI driver

## Immediate priorities

### P0
- [ ] Add `match` parsing support
- [ ] Add `result` / `option` syntax support in parser
- [ ] Extend AST for `match` and pattern forms
- [ ] Extend type checker with `match` exhaustiveness checks
- [ ] Add compile-pass / compile-fail tests for `match`

### P1
- [ ] Add `own` / `view` surface syntax to parser
- [ ] Add ownership state tracking in type checker
- [ ] Detect move-after-use
- [ ] Detect borrow/update conflict
- [ ] Emit clearer diagnostics for ownership errors

### P2
- [ ] Lower AST into non-placeholder HIR
- [ ] Add MIR data structures
- [ ] Lower `if` / `while` / `match` into MIR CFG
- [ ] Add drop insertion pass
- [ ] Add MIR dump command to driver

### P3
- [ ] Add first x86_64 backend path
- [ ] Decide LLVM IR text generation vs LLVM API path
- [ ] Produce minimal executable
- [ ] Run integer-only end-to-end samples

## Repository hygiene
- [ ] Add more compile-pass examples
- [ ] Add more compile-fail examples
- [ ] Add badges after CI is confirmed useful
- [ ] Fill GitHub repository description and topics
- [ ] Add release / versioning policy draft

## Stretch goals
- [ ] AArch64 backend abstraction
- [ ] FFI boundary syntax
- [ ] Stable public ABI subset
- [ ] Standard library shape draft
