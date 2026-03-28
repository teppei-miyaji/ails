# Changelog

## 0.1.0-draft-v0.4
- GitHub-ready skeleton retained
- parser support for if/else/while
- type checker support for boolean branch/loop conditions
- minimal HIR skeleton added

## 0.1.0-draft-v0.6
- Add TODO tracking
- Add roadmap document
- Add progress document
- Add issue draft documents for next work slices

## 0.1.0-draft-v0.7
- Add parser support for `match`
- Add parser support for `result` and `option` type syntax
- Extend AST with patterns and match arms
- Add basic type checking for `match` exhaustiveness on `result` and `option`
- Extend HIR skeleton to record match presence

## 0.1.0-draft-v0.8
- Add parser support for `own` and `view` type syntax
- Extend AST type model with `Own` and `View`
- Add first move-after-use detection in type checker
- Add first ownership-oriented diagnostics

## 0.1.0-draft-v0.9
- Add parser support for function call expressions
- Add function signature table to type checker
- Add first borrow generation for `view` call arguments
- Add move-while-borrowed rejection
