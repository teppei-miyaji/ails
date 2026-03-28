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
- [ ] Obtain targeted review for v0.23 validator draft
- [ ] Wire validator into an executable/testable path
- [ ] Add canonical valid/invalid MIR golden samples

### P1
- [ ] Align Rust implementation names and structure with canonical spec terminology
- [ ] Emit clearer diagnostics for ownership errors
- [ ] Add drop insertion pass skeleton
- [ ] Add MIR validation pass implementation
- [ ] Add single-drop property checker skeleton
