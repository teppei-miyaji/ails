# AIIS Cross-implementation Test for AILS v0.1.1

This directory supersedes the experimental v0.1 package while preserving v0.1 unchanged.

AIIS baseline:
- Protocol: `0.3.1`
- Canonicalization: `aiis-c14n-json/0.3.1`
- Canonical fallback codec: `canonical-json/0.3.1`
- Base AIIS semantic hash: `sha256:41ec325ea6274fde44cc92dc640ea62971952edb24414f4b7831226aef65f96c`
- AILS review profile: `aiis.ails.review@0.1.1`

## v0.1.1 changes

1. Adds `finding_code` so a broad finding category does not erase concrete machine identifiers such as `UnreachableBlock`.
2. Adds Resource Capability declaration and the actual resource access path used for a case.
3. Adds X06 to test capability-state preservation.
4. Adds a public result schema. Expected answers remain evaluator-private.
5. Clarifies X05: `finding_category` describes the underlying finding, while semantic-equivalence is the property being tested. Therefore two unreachable-block observations normalize to the same `MIR_STRUCTURAL_ERROR` + `UnreachableBlock` semantics.

## Source snapshot

Every run MUST bind to one exact 40-hex Git commit SHA and use that same snapshot for every referenced AILS file.

AILS canonical authority remains `spec/`.

## Public/private split

Public:
- rules
- profile
- result schema
- X01-X06 inputs
- roundtrip vectors
- fixtures

Private evaluator:
- expected results
- raw peer results
- comparison output
- evaluator notes

## Google Drive transport

For peers that cannot directly read GitHub/raw URLs, the evaluator may provide an immutable partial repository snapshot through Google Drive. The peer MUST report the actual access capability and path used; it MUST NOT claim GitHub/raw access merely because the copied content originated from GitHub.
