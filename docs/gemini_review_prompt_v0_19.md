# Gemini Review Prompt for AILS v0.19

Please review this repository as a **specification review first**, not as an implementation review.

Repository:
https://github.com/teppei-miyaji/ails

Most important review targets:
- `spec/ownership.md`
- `spec/pattern-matching.md`
- `spec/control-flow-join.md`
- `spec/type-system.md`
- `spec/mir.md`
- `spec/implementation-mapping.md`
- `spec/diagnostics.md`

Premises:
- `spec/` is the canonical source of truth
- Rust implementation is still a draft and may lag the spec
- Do not infer missing semantics from implementation details
- Do not assume build failures unless logs are provided
- Do not use other repositories as evidence

New v0.19 points to review:
1. by-value `match` consumes the scrutinee even for payload-less cases
2. function call arguments are evaluated left to right for borrow legality
3. MIR drop insertion strategy is defined at the placement-class boundary, not as a full algorithm

Please evaluate:

1. Whether scrutinee-consumption rules are now fully consistent
2. Whether payload-less case semantics are unambiguous enough for implementation
3. Whether left-to-right call evaluation sufficiently closes borrow-chain ambiguity
4. Whether expression-scoped borrow plus left-to-right evaluation is coherent
5. Whether MIR drop placement-class boundaries are clear enough for the next implementation stage
6. Whether the current diagnostics guidance is sufficient for forbidden partial-move cases
7. Whether the spec set is now stable enough to proceed to implementation-alignment and MIR validation work

Output format:
- Conclusion
- What improved in v0.19
- Remaining ambiguities
- Severity-ranked issues
- Whether v0.19 is mergeable as canonical spec
- Recommended next 5 issues

Important constraints:
- Distinguish clearly between confirmed facts and design opinions
- Review the repository structure as it exists
- Do not invent file paths
- Treat unresolved future extensions as acceptable only if they are clearly marked and safely bounded
