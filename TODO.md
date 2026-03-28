# TODO

## Current phase

Public draft repository with:
- lexer
- parser
- AST
- minimal HIR skeleton
- minimal type checker
- CLI driver
- `let`
- `set`
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
- [x] Add `let` / `set`
- [x] Add first borrow/update conflict rejection
- [x] Add first branch ownership-state checking
- [x] Add first loop ownership-state checking
- [ ] Add compile-pass / compile-fail ownership examples beyond scalar cases

### P1
- [ ] Add `view` borrow creation semantics for pattern bindings
- [ ] Emit clearer diagnostics for ownership errors
- [ ] Generalize `match` to named sum types
- [ ] Refine branch merge rules beyond exact equality
- [ ] Refine loop merge rules beyond exact equality

### P2
- [ ] Lower AST into non-placeholder HIR
- [ ] Add MIR data structures
- [ ] Lower `if` / `while` / `match` into MIR CFG
- [ ] Add drop insertion pass
- [ ] Add MIR dump command to driver
