# Gemini Review Prompt for AILS v0.23 Structural Validator Draft

Please review this repository as an **implementation draft review**, focused on the first executable MIR structural validator slice.

Repository:
https://github.com/teppei-miyaji/ails

Primary review targets:
- `crates/ails-mir/src/validator.rs`
- `crates/ails-mir/src/lib.rs`
- `docs/implementation/mir_validator_design.md`
- `docs/implementation/v0_23_first_impl_task.md`
- `tests/mir/README.md`

Also reference:
- `spec/mir.md`
- `spec/implementation-mapping.md`
- `spec/diagnostics.md`

Premises:
- the code is still unverified and may not build
- review this as a draft implementation slice against the canonical spec
- do not assume successful compilation without logs
- do not invent missing files

Please evaluate:
1. whether the structural validator scope is correct
2. whether the chosen error categories are sufficient for the first pass
3. whether reachability belongs in the first code slice
4. whether the code starter is too ambitious or too small
5. what the first concrete follow-up implementation should be

Output format:
- Conclusion
- Good decisions
- Problems or missing checks
- Severity-ranked implementation risks
- Whether this v0.23 draft is a good first executable slice
- Recommended next 5 issues
