# Gemini Review Prompt for AILS v0.21 Implementation Transition

Please review this repository as an **implementation-transition review**, grounded in canonical spec v0.20.

Repository:
https://github.com/teppei-miyaji/ails

Primary review targets:
- `docs/implementation/implementation_alignment_checklist.md`
- `docs/implementation/mir_validator_design.md`
- `docs/implementation/single_drop_property_plan.md`
- `docs/implementation/diagnostics_remediation_catalog.md`
- `docs/implementation/backend_handoff_draft.md`
- `docs/implementation/roadmap_v0_21_to_v0_23.md`

Also reference:
- `spec/ownership.md`
- `spec/pattern-matching.md`
- `spec/mir.md`
- `spec/implementation-mapping.md`

Premises:
- canonical spec v0.20 is treated as settled baseline
- this review is about whether implementation planning is sufficient and correctly staged
- do not invent file paths or implementation details not present in the repo
- do not assume build success/failure unless logs are provided

Please evaluate:
1. whether the implementation alignment checklist is sufficient
2. whether the MIR validator design is staged correctly
3. whether the single-drop property plan is appropriately conservative
4. whether the diagnostics remediation catalog is useful for AI-assisted fixing
5. whether backend handoff is delayed until the right preconditions
6. whether roadmap ordering is realistic

Output format:
- Conclusion
- Strong points
- Missing implementation-prep items
- Severity-ranked issues
- Whether v0.21 implementation transition pack is mergeable
- Recommended next 5 issues
