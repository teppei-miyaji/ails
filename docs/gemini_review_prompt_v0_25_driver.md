# Gemini Review Prompt for AILS v0.25 Driver Wiring Draft

Please review this repository as an **implementation draft review**, focused on the first executable path for MIR structural validation.

Repository:
https://github.com/teppei-miyaji/ails

Primary review targets:
- `crates/ails-driver/src/main.rs`
- `crates/ails-mir/src/validator.rs`
- `docs/implementation/v0_25_driver_wiring.md`
- `docs/implementation/v0_25_test_commands.md`
- `tests/mir/valid/minimal_function.ails`
- `tests/mir/valid/branch_function.ails`

Also reference:
- `docs/implementation/mir_structural_validator_starter.md`
- `docs/implementation/validator_wiring_plan.md`
- `spec/mir.md`

Premises:
- this code is still a draft and may not build
- review whether the command wiring is appropriately small and correctly staged
- do not assume successful compilation without logs
- do not invent missing files or test outcomes

Please evaluate:
1. whether `validate-mir-structure` is the right first executable entry point
2. whether the driver flow is appropriately staged
3. whether report output is sufficient for the first pass
4. whether the remaining next steps are ordered correctly
5. whether this v0.25 draft is mergeable as an implementation step

Output format:
- Conclusion
- Good decisions
- Missing or risky parts
- Severity-ranked implementation concerns
- Mergeability
- Recommended next 5 issues
