# AILS Type System

## Primitive types

- `bool`
- `i32`
- `i64`
- `u32`
- `u64`
- `usize`
- `unit`

## Nominal named types

`type` introduces a nominal sum type.

## Wrapper types

`own T`
- move-only

`view T`
- read-only borrowed access

`option T`
- `some T`
- `none`

`result T E`
- `ok T`
- `err E`

## Expression typing

- integer literals default to `i32`
- boolean literals have type `bool`
- arithmetic operators require equal integer operands
- relational operators require equal integer operands and produce `bool`
- equality operators require equal underlying types and produce `bool`

## View compatibility

When a callee expects `view T`, an argument of:
- `T`
- `own T`
- `view T`

may be borrowed as `view T` if ownership rules allow it.
