# AIIS Cross-implementation Test for AILS v0.1

This directory is the **public test-side package** for evaluating AI-to-AI semantic interoperability while reviewing the AILS repository.

It is derived from the AI-Agent-Interop (AIIS) v0.3.1 design:

- AIIS protocol: `0.3.1`
- Canonicalization: `aiis-c14n-json/0.3.1`
- Canonical fallback codec: `canonical-json/0.3.1`
- Base AIIS semantic hash: `sha256:41ec325ea6274fde44cc92dc640ea62971952edb24414f4b7831226aef65f96c`
- AILS review profile: `aiis.ails.review@0.1`

## Purpose

The test checks whether independent AI peers can inspect the same immutable AILS source snapshot and converge on the same **canonical semantics** without requiring identical natural-language wording.

The public package intentionally contains:

- operator rules,
- the AILS-specific semantic profile,
- test inputs,
- round-trip inputs,
- source fixtures.

It intentionally does **not** contain evaluator expected answers or private comparison results.

## Source snapshot rule

Every run MUST be bound to one exact Git commit.

When this test is reviewed through a pull request:

1. resolve the PR head commit SHA,
2. use that SHA as `source_commit`,
3. read all AILS sources from that same SHA,
4. include the SHA in every result,
5. do not mix files from `main` or another revision.

If two peers use different commits, their results MUST NOT be compared as a valid cross-implementation run.

## Canonical AILS authority

AILS declares `spec/` to be the canonical specification set. If implementation and spec disagree, the spec is authoritative and the implementation should be treated as stale.

Primary references for this suite:

- `spec/README.md`
- `spec/mir.md`
- `spec/ai-authoring-guide.md`
- `crates/ails-mir/src/validator.rs`
- `crates/ails-driver/src/main.rs`

## Test cases

- **X01 — Canonical spec identification**
- **X02 — MIR structural error classification**
- **X03 — Unknown / unverified validation status**
- **X04 — Proposed change without execution authority**
- **X05 — Same finding → same canonical semantics**

Each case MUST be evaluated statelessly.

## Round-trip

`roundtrip-inputs.json` contains semantic records that each peer canonicalizes independently. Cross-peer comparison is performed outside this public repository.

Ordinary result JSON object key order and indentation are not semantically significant. Canonical payload comparison is byte-identical only after the specified canonicalization process.

## Public/private boundary

Public GitHub:
- this directory,
- inputs,
- rules,
- profile,
- fixtures.

Private evaluator store:
- expected results,
- peer raw outputs,
- comparison output,
- evaluator-only notes.

This prevents an evaluated peer from reading the expected answers before producing its response.
