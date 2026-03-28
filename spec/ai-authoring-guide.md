# AILS AI Authoring Guide

## Authoring rules

- Always emit declarations in canonical order:
  `module`, `import`, `type`, `const`, `func`
- Always include `output` and `effect` in every function
- Prefer one construct per line
- Prefer explicit types on every `let`
- Avoid relying on unspecified ownership behavior
- For named sum types, prefer one-field or zero-field cases until multi-field semantics are canonicalized
- Use exhaustive `match`

## Safe code generation policy

An AI should avoid generating code that depends on:
- implicit copies of owned payloads
- merge behavior not defined in `control-flow-join.md`
- mutation through `view`
- partial move semantics not yet canonicalized
