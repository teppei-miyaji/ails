# Gemini Review Prompt for AILS v0.22 MIR Structural Validator Starter

Please review this repository as an **implementation-starter review**, focused on MIR structural validation preparation.

Repository:
https://github.com/teppei-miyaji/ails

Primary review targets:
- `docs/implementation/mir_validator_design.md`
- `docs/implementation/mir_structural_validator_starter.md`
- `docs/implementation/golden_test_plan.md`
- `docs/implementation/implementation_alignment_checklist.md`
- `docs/implementation/roadmap_v0_21_to_v0_23.md`

Also reference:
- `spec/mir.md`
- `spec/implementation-mapping.md`
- `spec/diagnostics.md`

Premises:
- canonical spec v0.20 is the baseline
- this review is about whether the repository is ready to implement the first MIR structural validator
- do not invent implementation files or claim build results without logs

Please evaluate:
1. whether the validator starter slice is scoped correctly
2. whether the structural error categories are sufficient and well separated
3. whether the golden test plan is adequate
4. whether reachability belongs in the first validator slice
5. whether roadmap ordering is correct
6. whether anything critical is still missing before implementation starts

Output format:
- Conclusion
- Strong points
- Missing items
- Dangerous assumptions
- Mergeability of the v0.22 starter pack
- Recommended next 5 issues
- The first concrete implementation task
