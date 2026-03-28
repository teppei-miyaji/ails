# TODO

## Current phase

Public draft repository with:
- lexer
- parser
- AST
- initial HIR lowering
- initial MIR lowering
- minimal type checker
- CLI driver
- canonical spec draft set
- review-ready spec milestone
- implementation transition pack
- MIR structural validator starter pack
- draft MIR structural validator code
- validator wiring pack
- draft driver validation path
- `import`
- `type`
- `case`
- `field`
- `const`
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
- [x] Add draft MIR structural validator code
- [x] Add validator wiring pack
- [x] Add draft driver validation path
- [ ] Verify build integration locally
- [ ] Add Rust-side invalid MIR constructor tests
- [ ] Add canonical valid source-driven MIR validation checks

### P1
- [ ] Align Rust implementation names and structure with canonical spec terminology
- [ ] Emit clearer diagnostics for ownership errors
- [ ] Add drop insertion pass skeleton
- [ ] Add MIR validation pass implementation
- [ ] Add single-drop property checker skeleton
