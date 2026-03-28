# TODO

## Current phase

Public draft repository with:
- lexer
- parser
- AST
- minimal HIR skeleton
- minimal type checker
- CLI driver
- `if`
- `while`
- `match`
- `result`
- `option`
- `own`
- `view`
- function call expressions

## Immediate priorities

### P0
- [x] Add `match` parsing support
- [x] Add `result` / `option` syntax support in parser
- [x] Extend AST for `match` and pattern forms
- [x] Extend type checker with `match` exhaustiveness checks
- [x] Add `own` / `view` surface syntax to parser
- [x] Add first move-after-use detection
- [x] Add first borrow generation for `view` call arguments
- [x] Add move-while-borrowed rejection
- [ ] Add compile-pass / compile-fail ownership examples

### P1
- [ ] Add borrow/update conflict detection for future assignment support
- [ ] Add `view` borrow creation semantics for pattern bindings
- [ ] Emit clearer diagnostics for ownership errors
- [ ] Generalize `match` to named sum types

### P2
- [ ] Lower AST into non-placeholder HIR
- [ ] Add MIR data structures
- [ ] Lower `if` / `while` / `match` into MIR CFG
- [ ] Add drop insertion pass
- [ ] Add MIR dump command to driver
