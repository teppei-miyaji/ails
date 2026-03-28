# Roadmap

## Milestone M0: Public draft repository
Status: done

Delivered:
- public repository shape
- Apache-2.0
- README / NOTICE / CI
- compiler skeleton

## Milestone M1: Front-end baseline
Status: in progress

Target:
- lexer
- parser
- AST
- names
- type checker
- `if`
- `while`
- `match`
- `result`
- `option`

Current:
- lexer
- parser subset
- AST subset
- minimal type checker
- `if`
- `while`

## Milestone M2: Ownership-aware front-end
Status: not started

Target:
- `own`
- `view`
- move-after-use check
- borrow/update conflict check
- ownership diagnostics

## Milestone M3: HIR/MIR pipeline
Status: not started

Target:
- non-placeholder HIR
- MIR
- CFG lowering
- drop insertion
- debug dumps

## Milestone M4: First executable backend
Status: not started

Target:
- x86_64 first
- minimal runtime
- integer-only executable
- end-to-end example execution

## Milestone M5: AArch64-ready architecture
Status: not started

Target:
- target abstraction
- AArch64 backend path
- ABI refinement
