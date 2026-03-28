# AILS HIR Specification

## Role of HIR

HIR is the structured, post-parse intermediate form.

HIR is responsible for:
- carrying parsed program structure
- preserving top-level declarations
- preserving statement nesting
- making later lowering easier

HIR is not responsible for:
- CFG construction
- backend scheduling
- drop insertion
- final ownership validation beyond what type checking already established
