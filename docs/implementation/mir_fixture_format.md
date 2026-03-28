# MIR Fixture Format Strategy

## Purpose

Define how MIR fixtures should be introduced before the validator is fully wired into the normal pipeline.

## Acceptable early fixture strategies

### Strategy A: Source-driven fixtures
Keep `.ails` source files and compare validator outputs after lowering.

Pros:
- stays close to real pipeline
- exercises parse/type/HIR/MIR chain

Cons:
- validator failures may be masked by earlier phases

### Strategy B: Hand-authored MIR fixtures
Introduce a minimal textual or code-native fixture representation for MIR only.

Pros:
- isolates validator testing
- directly targets structural invalidity

Cons:
- requires a fixture representation choice

## v0.24 recommendation

Start with source-driven valid fixtures and code-native invalid MIR constructor tests.

That means:
- valid path: `.ails` inputs lowered normally
- invalid path: Rust-side constructed `MirFunction` / `MirModule` values in tests

This avoids inventing a MIR text format too early.
