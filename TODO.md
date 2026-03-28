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
- [x] Add implementation transition pack
- [x] Add MIR structural validator starter pack
- [ ] Obtain external review for v0.22 MIR validator starter
- [ ] Implement MIR structural validation pass
- [ ] Add canonical valid/invalid MIR golden samples

### P1
- [ ] Align Rust implementation names and structure with canonical spec terminology
- [ ] Add `view` borrow creation semantics for pattern bindings
- [ ] Emit clearer diagnostics for ownership errors
- [ ] Add drop insertion pass skeleton
- [ ] Add MIR validation pass implementation
- [ ] Add single-drop property checker skeleton
