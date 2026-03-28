# AILS Type System

## 1. Type categories

AILS has:
- primitive types
- nominal named types
- ownership wrappers
- sum helpers (`option`, `result`)

## 2. Primitive types

Current primitive set:
- `bool`
- `i32`
- `i64`
- `u32`
- `u64`
- `usize`
- `unit`

## 3. Nominal named types

`type` introduces a nominal sum type.
Two named types are equal only if they have the same declared name.

## 4. Wrapper types

`own T`
- means the value is move-only
- use by value consumes ownership unless a special non-consuming rule applies

`view T`
- means read-only borrowed access to a `T`
- `view T` does not own storage
- `view T` cannot be used to mutate the referent

`option T`
- sum helper with cases `some T` and `none`

`result T E`
- sum helper with cases `ok T` and `err E`

## 5. Equality rules

Type equality is structural for wrappers and nominal for named types.

## 6. Expression typing

Integer literals currently default to `i32`.

Boolean literals have type `bool`.

Binary operators:
- arithmetic operators require equal integer operands
- relational operators require equal integer operands and produce `bool`
- equality operators require equal underlying types and produce `bool`

## 7. View compatibility

When a callee expects `view T`, an argument of:
- `T`
- `own T`
- `view T`
may be borrowed as `view T` if ownership rules allow it.

This is a compatibility rule, not normal type equality.

## 8. Return-type restriction for v0.1

Functions returning `view T` are **forbidden** in v0.1.

Reason:
- borrow origin and lifetime escape semantics are not yet canonicalized
- allowing `view` returns would create specification ambiguity around returned borrow validity

A future revision may introduce returned views only after explicit lifetime/origin rules are added.
