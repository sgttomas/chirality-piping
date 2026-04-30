# Coordination Record

**Representation:** Full graph
**Dependency tracking mode:** FULL_GRAPH
**External schedule / coordination artifact:** N/A
**Default maturity threshold:** SEMANTIC_READY
**DAG status:** DEFERRED
**Blocker computation:** DISABLED until a human-approved acyclic DAG exists

## Human Rulings

- 2026-04-30 - Use a Full DAG coordination representation for the OpenPipeStress SOFTWARE workflow.
- 2026-04-30 - Do not author or infer dependency edges yet.
- 2026-04-30 - Use `SEMANTIC_READY` as the development-phase readiness threshold before downstream workflow proceeds.
- 2026-04-30 - Treat implementation-phase dependency planning as a separate future DAG over the implementation plan, not the current decomposition scaffold.
- 2026-04-30 - Add `PKG-00 - Software Architecture Runway` as the first workflow gate before `PKG-01` through `PKG-12`.

## Operating Rules

- Report lifecycle state while the DAG is deferred.
- Do not compute blocked/unblocked states until a human-approved acyclic DAG exists.
- Do not advance `PKG-01` through `PKG-12` package-level document drafting or implementation planning until `PKG-00` reaches the selected architecture readiness threshold or the human explicitly changes the gate.
- Keep dependency proposals labeled `PROPOSAL` until accepted by the human project authority.
