# Issue Draft: ast+types: add `result` / `option` and pattern forms

## Summary
Extend the AST and related type representation to cover `result`, `option`, and simple pattern bindings.

## Scope
- AST type nodes for `result T E`
- AST type nodes for `option T`
- pattern node for `case name binding`
- minimal examples

## Acceptance criteria
- parser output can represent `result` and `option`
- `match` arm payload binding is represented in AST

## Out of scope
- exhaustiveness
- borrow semantics


Status: implemented in v0.7 draft artifact.
