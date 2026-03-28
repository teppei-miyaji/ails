# Gemini Review Prompt for AILS v0.20

Please review this repository as a **specification review first**, not as an implementation review.

Repository:
https://github.com/teppei-miyaji/ails

Most important review targets:
- `spec/ownership.md`
- `spec/pattern-matching.md`
- `spec/control-flow-join.md`
- `spec/mir.md`
- `spec/implementation-mapping.md`
- `spec/diagnostics.md`

Premises:
- `spec/` is the canonical source of truth
- Rust implementation is still a draft and may lag the spec
- Do not infer missing semantics from implementation details
- Do not assume build failures unless logs are provided
- Do not use other repositories as evidence

New v0.20 points to review:
1. nested-call borrow legality is defined under left-to-right evaluation
2. placement-class to control-flow drop responsibility mapping is defined
3. forbidden partial move diagnostics now include standard remediation patterns
4. single-drop property is introduced as an abstract validation goal
5. future `view`-scrutinee matching is explicitly isolated as a non-canonical extension area

Please evaluate:

1. Whether nested-call borrow rules are now explicit enough for implementation
2. Whether expression-scoped borrow plus nested left-to-right evaluation is coherent
3. Whether the placement-class mapping is sufficiently clear for the next MIR/drop stage
4. Whether the single-drop property is defined at an appropriate abstraction level
5. Whether diagnostics guidance is now actionable enough for AI-assisted refactoring
6. Whether the remaining unresolved future extensions are safely bounded
7. Whether v0.20 is stable enough to proceed toward MIR validation pass design and implementation alignment

Output format:
- Conclusion
- What improved in v0.20
- Remaining ambiguities
- Severity-ranked issues
- Whether v0.20 is mergeable as canonical spec
- Recommended next 5 issues

Important constraints:
- Distinguish clearly between confirmed facts and design opinions
- Review the repository structure as it exists
- Do not invent file paths
- Treat unresolved future extensions as acceptable only if they are clearly marked and safely bounded
