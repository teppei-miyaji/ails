# AILS Language Specification

## Purpose

AILS is an AI-oriented low-level safe language for deterministic parsing, explicit ownership, explicit control-flow semantics, and future lowering into HIR/MIR and machine backends.

## Top-level declarations

A module may contain, in this order:
- `module`
- zero or more `import`
- zero or more `type`
- zero or more `const`
- zero or more `func`

## Statements

- `let`
- `set`
- `return`
- `if ... then ... else`
- `while`
- `match`

## Expressions

- identifiers
- integer literals
- boolean literals
- function calls
- binary operations

## Types

- primitive types
- named types
- `own T`
- `view T`
- `option T`
- `result T E`

## Safety model

The language safety model depends on:
- `ownership.md`
- `control-flow-join.md`
- `pattern-matching.md`

No implementation is allowed to silently invent additional ownership behavior not described there.
