# Issue Draft: parser: add `match` support

## Summary
Implement `match` parsing in the public AILS compiler skeleton.

## Scope
- lexer tokens needed for `match` / `case`
- AST nodes for `match`
- parser support for `match ... case ... begin ... end`
- example file
- parser snapshot tests

## Acceptance criteria
- parser accepts a minimal `match`
- AST dump includes `match`
- one positive example and one negative example exist

## Out of scope
- type checking
- exhaustiveness
- ownership interaction


Status: implemented in v0.7 draft artifact.
