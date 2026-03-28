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
- [x] Add canonical spec draft set
- [x] Canonicalize expression-scoped call borrow lifetime
- [x] Canonicalize move-binding payload semantics for by-value match
- [x] Forbid partial move in v0.1
- [x] Forbid `view` return types in v0.1
- [x] Add abstract named sum layout document
- [x] Add HIR/MIR validation responsibility mapping
- [x] Clarify MIR drop responsibility boundary
- [x] Formalize scrutinee consumption for payload-less by-value match
- [x] Define left-to-right evaluation order for call-borrow legality
- [x] Define MIR drop insertion strategy at placement-class level
- [x] Define nested-call borrow legality
- [x] Add placement-class to control-flow drop responsibility mapping
- [x] Add standard remediation patterns for forbidden partial move diagnostics
- [x] Define single-drop property as abstract validation goal
- [ ] Obtain external review for v0.20 canonical spec

### P1
- [ ] Align Rust implementation names and structure with canonical spec terminology
- [ ] Add `view` borrow creation semantics for pattern bindings
- [ ] Emit clearer diagnostics for ownership errors
- [ ] Refine branch merge rules beyond exact equality
- [ ] Refine loop merge rules beyond exact equality
- [ ] Add drop insertion pass
- [ ] Add MIR validation pass implementation
