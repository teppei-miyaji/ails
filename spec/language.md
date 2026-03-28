# AILS Language Specification

## 1. Purpose

AILS is an AI-oriented low-level safe language.
It is designed for:
- deterministic parsing
- explicit ownership
- explicit control-flow semantics
- implementation-friendly lowering into HIR and MIR
- future backend targeting for x86_64 and AArch64

AILS is not Rust syntax.
It only borrows some safety goals and ownership terminology.

## 2. Design principles

- Syntax must be easy for both humans and LLMs to emit correctly.
- Semantics must prefer explicitness over convenience.
- Ownership transfer must be visible in source-level reasoning.
- Control-flow joins must have explicit safety rules.
- Parser, typechecker, HIR, and MIR must each have distinct responsibilities.

## 3. Top-level declarations

A module may contain, in this order:
- `module`
- zero or more `import`
- zero or more `type`
- zero or more `const`
- zero or more `func`

## 4. Statements

Current statement set:
- `let`
- `set`
- `return`
- `if ... then ... else`
- `while`
- `match`

## 5. Expressions

Current expression set:
- identifiers
- integer literals
- boolean literals
- function calls
- binary operations

## 6. Types

Current type constructors:
- primitive types
- named types
- `own T`
- `view T`
- `option T`
- `result T E`

## 7. Canonical unresolved items

The following are acknowledged but not fully complete:
- borrow-binding payload patterns
- multi-field pattern destructuring
- named sum runtime layout details beyond abstract discriminant+payload model
- backend ABI mapping

They must remain explicit TODO items, not implicit implementation choices.

## 8. Safety model

The language safety model depends on:
- ownership rules in `ownership.md`
- join rules in `control-flow-join.md`
- pattern binding rules in `pattern-matching.md`

No implementation is allowed to silently invent additional ownership behavior not described there.
