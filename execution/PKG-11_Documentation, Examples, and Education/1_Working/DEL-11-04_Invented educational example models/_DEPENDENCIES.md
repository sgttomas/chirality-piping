# Dependencies: DEL-11-04 Invented educational example models

## Coordination (human-owned)
- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for PREPARATION.

## Upstream (I need these before I can proceed) - human-owned declarations
- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations
- Dependencies coordinated externally by humans.

## Extracted Dependency Register (populated by TASK+dependency-extract)
- **Status:** REFRESHED
- **Dependencies.csv:** `Dependencies.csv`
- **Schema:** v3.1
- **Summary:** 8 ACTIVE rows: 3 ANCHOR rows and 5 EXECUTION rows.

| DependencyID | Class | Direction | Type | Target | Satisfaction | Notes |
|---|---|---|---|---|---|---|
| DEP-11-04-001 | ANCHOR | UPSTREAM | OTHER | SOW-033 Documentation and invented examples | NOT_APPLICABLE | Implements-node anchor. |
| DEP-11-04-002 | ANCHOR | UPSTREAM | OTHER | OBJ-001 Open auditable platform | NOT_APPLICABLE | Objective trace. |
| DEP-11-04-003 | ANCHOR | UPSTREAM | OTHER | OBJ-008 Verification validation and regression gates | NOT_APPLICABLE | Objective trace. |
| DEP-11-04-004 | EXECUTION | UPSTREAM | INTERFACE | DEL-02-01 Canonical domain model schema | PENDING | Future external example model schema dependency. |
| DEP-11-04-005 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-05 Project persistence and round-trip serialization | PENDING | Future external example file format dependency. |
| DEP-11-04-006 | EXECUTION | UPSTREAM | INTERFACE | DEL-06-01 Rule-pack schema | PENDING | Future fake-rule-pack demo schema dependency. |
| DEP-11-04-007 | EXECUTION | UPSTREAM | INTERFACE | DEL-06-05 Invented non-code example rule pack | PENDING | Future fake-rule-pack demo boundary dependency. |
| DEP-11-04-008 | EXECUTION | DOWNSTREAM | ENABLES | DEL-09-01 Mechanics benchmark suite | PENDING | PROPOSAL: future validation use must be confirmed. |

## Run Notes and History (populated by TASK+dependency-extract)
- 2026-04-30 - TASK+dependency-extract MODE=UPDATE STRICTNESS=CONSERVATIVE CONSUMER_CONTEXT=NONE.
- Decomposition path: `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4.
- Source documents scanned: `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_REFERENCES.md`.
- Anchor doc: `_CONTEXT.md`.
- Execution docs: `Specification.md`, `Procedure.md`, `Guidance.md`, `Datasheet.md`.
- No declared dependency rows were present to preserve.
- No extracted rows were retired.
- `[WARNING] ID_FORMAT_TOOL_MISMATCH`: `tools/validation/validate_id_format.sh` expects three-digit package and deliverable IDs, while SOFTWARE_DECOMP uses short IDs such as `PKG-11` and `DEL-11-04`.

## Lifecycle Summary (populated by TASK+dependency-extract)
- **ACTIVE rows:** 8
- **RETIRED rows:** 0
- **ANCHOR rows:** 3
- **EXECUTION rows:** 5
- **SatisfactionStatus counts:** NOT_APPLICABLE=3; PENDING=5
- **Parent anchor:** PASS - one ACTIVE `IMPLEMENTS_NODE` row found.
- **Schema validation:** PASS with `tools/validation/validate_dependencies_schema.py`.

## Consumer Handoff Notes
- Future actual example model files remain out of this setup scope.
- Upstream schema and rule-pack dependencies should be resolved before external examples are materialized.
- DEP-11-04-008 is a PROPOSAL only; PKG-09 should confirm before using DEL-11-04 outputs as validation fixtures.
