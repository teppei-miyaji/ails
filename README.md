# AILS

**AI-Oriented Low-level Safe Language**

This repository contains a public draft compiler skeleton for AILS.

## License

This repository is licensed under **Apache-2.0**.  
See `LICENSE` and `NOTICE`.

## Status

This project is **draft / experimental**.  
It is intended to be published as a public GitHub repository and evolved incrementally.

## Implemented in this public package

- lexer
- parser
- AST
- real HIR lowering from AST
- `match` / `result` / `option` front-end slice
- minimal type checker
- ownership surface syntax for `own` / `view`
- CLI driver

Parser support currently includes:

- `module`
- `import`
- `type` / `case` / `field`
- `const`
- `func`
- `input`
- `output`
- `effect`
- `begin` / `end`
- `return`
- `let` / `set`
- `if ... then ... else`
- `while`
- `match`
- named sum type matching
- `result` types
- `option` types
- `own` types
- `view` types
- integer literals
- boolean literals
- arithmetic operators
- comparison operators
- function call expressions

Type checker support currently includes:

- duplicate type/function/const detection
- duplicate parameter detection
- unknown identifier detection
- basic integer expression typing
- return type checking
- `if` condition must be `bool`
- `while` condition must be `bool`
- first borrow generation for `view` call arguments
- move-while-borrowed rejection
- first borrow/update conflict rejection
- first branch ownership-state checking
- first loop ownership-state checking

## Not implemented yet

- ownership checking for `own` / `view`
- MIR
- code generation
- x86_64 backend
- AArch64 backend

## Build

```bash
cargo build
```

## Run

```bash
cargo run -p ails-driver -- tokens examples/add.ails
cargo run -p ails-driver -- parse examples/if_demo.ails
cargo run -p ails-driver -- check examples/if_demo.ails
cargo run -p ails-driver -- hir examples/if_demo.ails
cargo run -p ails-driver -- mir examples/hir_demo.ails
```

## Publish to GitHub from Windows Command Prompt

Create an empty public repository on GitHub first, then run:

```bat
git init
git branch -M main
git add .
git commit -m "Initial public draft of AILS compiler skeleton"

gh auth login
git remote add origin https://github.com/YOUR_ACCOUNT/ails.git
git push -u origin main
```

If you do not use `gh`, you may use HTTPS with a Personal Access Token instead of a password.

## Repository intent

This repository is structured so it can be pushed to GitHub directly and iterated in public.


## Project tracking

- `TODO.md`
- `docs/roadmap.md`
- `docs/progress.md`
- `docs/issues/`

These files are intended to make incremental public GitHub work easier to manage.


## Canonical spec set

The repository now includes a canonical spec draft under `spec/`.
This spec set should be treated as the source of truth when implementation and docs diverge.
