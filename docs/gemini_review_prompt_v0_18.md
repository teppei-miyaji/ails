# Gemini Review Prompt for AILS v0.18

Please review this repository as a **specification review first**, not as an implementation review.

Repository:
https://github.com/teppei-miyaji/ails

Most important review targets:
- `spec/ownership.md`
- `spec/pattern-matching.md`
- `spec/control-flow-join.md`
- `spec/type-system.md`
- `spec/mir.md`
- `spec/named-sum-layout.md`
- `spec/implementation-mapping.md`

Premises:
- `spec/` is the canonical source of truth
- Rust implementation is still a draft and may lag the spec
- Do not infer missing semantics from implementation details
- Do not assume build failures unless logs are provided
- Do not use other repositories as evidence

Please evaluate:

1. Whether `own` / `view` semantics are now sufficiently explicit for v0.1
2. Whether pattern payload move semantics are now canonical enough
3. Whether forbidding partial move in v0.1 is a sound temporary rule
4. Whether forbidding `view` return types in v0.1 is the right design boundary
5. Whether expression-scoped call-borrow lifetime is sufficiently precise
6. Whether exact-equality join rules for `if` / `while` are coherent and implementable
7. Whether MIR drop responsibility boundaries are now clear enough
8. Whether named sum layout is correctly split into semantic-level vs backend-level concerns
9. Whether the implementation responsibility mapping is clear and non-overlapping

Output format:
- Conclusion
- What became better in v0.18
- Remaining ambiguities
- Severity-ranked issues
- Whether this spec set is mergeable as canonical v0.18
- Recommended next 5 issues

Important constraints:
- Distinguish clearly between confirmed facts and design opinions
- Review the repository structure as it exists
- Do not invent file paths
- Treat unresolved future extensions as acceptable only if they are clearly marked and safely bounded
