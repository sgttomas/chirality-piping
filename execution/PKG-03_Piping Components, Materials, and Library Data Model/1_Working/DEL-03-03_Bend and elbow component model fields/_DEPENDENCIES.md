# Dependencies: DEL-03-03 Bend and elbow component model fields

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
- **Summary:** 5 ACTIVE rows: 2 ANCHOR, 3 EXECUTION, 0 RETIRED.

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---|---|---|---|
| DEP-03-03-001 | ANCHOR / IMPLEMENTS_NODE | UPSTREAM | OTHER | `SOW-007` Bends and elbows with user-entered SIFs and flexibility factors | ACTIVE |
| DEP-03-03-002 | ANCHOR / TRACES_TO_REQUIREMENT | UPSTREAM | OTHER | `OBJ-004` Piping-specific components and private libraries | ACTIVE |
| DEP-03-03-003 | EXECUTION | UPSTREAM | CONSTRAINT | `docs/CONTRACT.md` invariant catalog | ACTIVE |
| DEP-03-03-004 | EXECUTION | UPSTREAM | CONSTRAINT | `docs/_Decomposition/SOFTWARE_DECOMP.md` architecture basis constraints | ACTIVE |
| DEP-03-03-005 | EXECUTION | UPSTREAM | CONSTRAINT | `docs/_Registers/ScopeLedger.csv` protected-content note | ACTIVE |

## Run Notes and History (populated by TASK+dependency-extract)
- 2026-04-30 09:51 - `TASK+dependency-extract`, `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `SCOPE=DEL-03-03`.
- Decomposition path used: `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4.
- Source docs scanned: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_CONTEXT.md`, `_REFERENCES.md`, existing `_DEPENDENCIES.md`, and decomposition/register source slices named in the sealed brief.
- Anchor doc selection: `Datasheet.md` plus `_CONTEXT.md` and register/decomposition rows for explicit identifiers.
- Execution doc order: `Specification.md`, `Procedure.md`, `Guidance.md`, `Datasheet.md`.
- Defaults used: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`, `CONSUMER_CONTEXT=NONE`.
- Warnings: none for parent anchor integrity. Exactly one ACTIVE `IMPLEMENTS_NODE` anchor was found.
- Tool note: `tools/validation/validate_id_format.sh` was not used because the available helper uses legacy three-digit ID patterns incompatible with current `DEL-03-03` and `PKG-03` identifiers.

## Lifecycle Summary (populated by TASK+dependency-extract)
- ACTIVE: 5
- RETIRED: 0
- RequiredMaturity:
  - `SEMANTIC_READY`: 5
- ProposedMaturity:
  - `SEMANTIC_READY`: 5
- SatisfactionStatus:
  - `SATISFIED`: 5

## Consumer Handoff Notes
- `CONSUMER_CONTEXT=NONE`; no estimating, aggregation, or reconciliation-specific extension columns were added.
- Downstream consumers should treat these rows as deliverable-local extraction evidence, not a project-level dependency graph.
