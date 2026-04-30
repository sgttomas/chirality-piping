# Dependencies: DEL-02-03 Code-neutral analysis boundary model

## Coordination (human-owned)
- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for PREPARATION.

## Upstream (I need these before I can proceed) - human-owned declarations
- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations
- Dependencies coordinated externally by humans.

## Extracted Dependency Register (populated by TASK+dependency-extract)
- **Status:** UPDATED
- **Dependencies.csv:** `Dependencies.csv`
- **RegisterSchemaVersion:** `v3.1`
- **Summary:** 6 ACTIVE rows: 3 ANCHOR, 3 EXECUTION, 0 RETIRED.

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---|---|---|---|
| DEP-02-03-001 | ANCHOR / IMPLEMENTS_NODE | UPSTREAM | OTHER | `SOW-002` Code-neutral software boundary | ACTIVE |
| DEP-02-03-002 | ANCHOR / TRACES_TO_REQUIREMENT | UPSTREAM | OTHER | `OBJ-001` Open auditable platform objective | ACTIVE |
| DEP-02-03-003 | ANCHOR / TRACES_TO_REQUIREMENT | UPSTREAM | OTHER | `OBJ-011` Professional responsibility objective | ACTIVE |
| DEP-02-03-004 | EXECUTION | UPSTREAM | CONSTRAINT | `docs/CONTRACT.md` invariant catalog | ACTIVE |
| DEP-02-03-005 | EXECUTION | UPSTREAM | CONSTRAINT | `docs/TYPES.md` analysis status vocabulary | ACTIVE |
| DEP-02-03-006 | EXECUTION | UPSTREAM | CONSTRAINT | `docs/_Decomposition/SOFTWARE_DECOMP.md` architecture basis constraints | ACTIVE |

## Run Notes and History (populated by TASK+dependency-extract)
- 2026-04-30 09:44 - `TASK+dependency-extract`, `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `SCOPE=DEL-02-03`.
- Decomposition path used: `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4.
- Source docs scanned: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_CONTEXT.md`, `_REFERENCES.md`, existing `_DEPENDENCIES.md`, and decomposition/register source slices.
- Anchor doc selection: `Datasheet.md` plus `_CONTEXT.md` and register/decomposition rows for explicit identifiers.
- Execution doc order: `Specification.md`, `Procedure.md`, `Guidance.md`, `Datasheet.md`.
- Defaults used: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`, `CONSUMER_CONTEXT=NONE`.
- Warnings: none for parent anchor integrity. Exactly one ACTIVE `IMPLEMENTS_NODE` anchor was found.
- Tool note: `tools/validation/validate_id_format.sh` uses legacy three-digit `DEL-NNN-NN` and `PKG-NNN` patterns; current OpenPipeStress IDs are `DEL-02-03` and `PKG-02`, so ID-format validation is treated as incompatible with this decomposition variant rather than a dependency-register failure.

## Lifecycle Summary (populated by TASK+dependency-extract)
- ACTIVE: 6
- RETIRED: 0
- RequiredMaturity:
  - `SEMANTIC_READY`: 6
- ProposedMaturity:
  - `SEMANTIC_READY`: 6
- SatisfactionStatus:
  - `SATISFIED`: 6

## Consumer Handoff Notes
- `CONSUMER_CONTEXT=NONE`; no estimating, aggregation, or reconciliation-specific extension columns were added.
- Downstream consumers should treat these rows as deliverable-local extraction evidence, not a project-level dependency graph.
