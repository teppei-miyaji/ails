# Gemini Review Prompt for AILS v0.24 Validator Wiring Pack

Please review this repository as an **implementation wiring review**, focused on the first executable path for MIR structural validation.

Repository:
https://github.com/teppei-miyaji/ails

Primary review targets:
- `docs/implementation/validator_wiring_plan.md`
- `docs/implementation/mir_fixture_format.md`
- `docs/implementation/v0_24_first_wiring_task.md`
- `tests/mir/valid/minimal_function.ails`
- `tests/mir/valid/branch_function.ails`
- `tests/mir/invalid/README.md`

Also reference:
- `crates/ails-mir/src/validator.rs`
- `crates/ails-driver/src/main.rs`
- `docs/implementation/mir_validator_design.md`

Premises:
- this is still a draft implementation stage
- do not assume successful build without logs
- review whether the wiring plan and fixture strategy are appropriate

Please evaluate:
1. whether source-driven valid fixtures plus Rust-side invalid MIR tests is the right split
2. whether the first wiring task is small enough
3. whether the planned command surface is sufficient
4. whether any important validator-wiring risks are missing
5. whether v0.24 is mergeable as a preparation step

Output format:
- Conclusion
- Strong points
- Missing items
- Risky assumptions
- Mergeability
- Recommended next 5 issues
