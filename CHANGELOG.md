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

## 0.1.0-draft-v0.10
- Add parser support for `let` and `set`
- Add AST nodes for local binding and reassignment
- Add local symbol insertion in type checker
- Add first borrow/update conflict rejection

## 0.1.0-draft-v0.11
- Add first ownership-state merge check across `if` branches
- Reject mismatched moved state across branches
- Reject mismatched borrow count across branches

## 0.1.0-draft-v0.12
- Add first ownership-state merge check across `while` loop boundaries
- Reject mismatched moved state across loop entry/body boundary
- Reject mismatched borrow count across loop entry/body boundary

## 0.1.0-draft-v0.13
- Replace placeholder HIR with real lowered HIR structures
- Lower functions, statements, expressions, and match arms into HIR
- Keep `hir` driver command aligned with real lowered output

## 0.1.0-draft-v0.14
- Add concrete MIR data structures
- Lower HIR functions into block-based MIR
- Lower `if`, `while`, and `match` into CFG-style terminators
- Add `mir` driver subcommand

## 0.1.0-draft-v0.15
- Add parser support for `import`, `type`, `case`, `field`, and `const`
- Extend AST/HIR/MIR module-level metadata for imports, types, and consts
- Add type registry and const registry to type checker
- Generalize `match` exhaustiveness checking to user-defined sum types

## 0.1.0-draft-v0.16
- Add canonical spec set under `spec/`
- Split language, grammar, type system, ownership, join rules, pattern matching, HIR, MIR, diagnostics, and AI authoring guidance
- Mark spec as source of truth over implementation when they diverge

## 0.1.0-draft-v0.17
- Canonicalize expression-scoped borrow lifetime for `view` call arguments
- Canonicalize move-binding payload semantics for by-value pattern matching
- Add abstract named sum layout document
- Add implementation mapping and MIR validation requirements

## 0.1.0-draft-v0.18
- Forbid partial move in pattern payload semantics for v0.1
- Forbid `view` return types in v0.1
- Clarify MIR drop responsibility as a post-lowering pass boundary
- Add repository-local Gemini review prompt for v0.18 spec review

## 0.1.0-draft-v0.19
- Formalize by-value scrutinee consumption for payload-less match cases
- Define left-to-right call evaluation for borrow legality
- Define MIR drop insertion strategy at placement-class granularity
- Add repository-local Gemini review prompt for v0.19 spec review

## 0.1.0-draft-v0.20
- Define nested-call borrow legality under left-to-right evaluation
- Define placement-class to control-flow drop responsibility mapping
- Add standard remediation patterns for forbidden partial-move diagnostics
- Define single-drop property as an abstract MIR validation goal
- Add repository-local Gemini review prompt for v0.20 spec review

## 0.1.0-draft-v0.21
- Add implementation transition pack for spec-to-implementation work
- Add MIR validator design draft
- Add single-drop property implementation plan
- Add diagnostics remediation catalog
- Add backend handoff draft and v0.21 implementation review prompt

## 0.1.0-draft-v0.22
- Add MIR structural validator starter pack
- Add golden test planning for valid/invalid MIR fixtures
- Add validator starter API and error category guidance
- Add repository-local Gemini review prompt for the v0.22 validator starter

## 0.1.0-draft-v0.23
- Add first draft code for MIR structural validator
- Export validator result types from `ails-mir`
- Add fixture bootstrapping notes for MIR validation
- Add repository-local Gemini review prompt for the v0.23 validator draft

## 0.1.0-draft-v0.24
- Add validator wiring plan for first executable MIR structural validation path
- Add fixture strategy separating valid source fixtures from invalid Rust-constructed MIR tests
- Add initial valid source fixture files and repository-local Gemini review prompt for v0.24 wiring
