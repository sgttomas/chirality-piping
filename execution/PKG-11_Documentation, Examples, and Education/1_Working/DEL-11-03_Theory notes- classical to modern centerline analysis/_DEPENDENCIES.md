# Dependencies: DEL-11-03 Theory notes: classical to modern centerline analysis

## Coordination (human-owned)

- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for PREPARATION. Dependencies below are extracted information-flow and governance anchors only.

## Upstream (I need these before I can proceed) - human-owned declarations

- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations

- Dependencies coordinated externally by humans.

## Extracted Dependency Register

- **Status:** REFRESHED
- **Dependencies.csv:** `Dependencies.csv`
- **Register Schema Version:** v3.1
- **ACTIVE rows:** 6
- **RETIRED rows:** 0
- **Parent anchors:** 1
- **Trace anchors:** 2
- **Execution constraints:** 3

| DependencyID | Class | Direction | Type | Target | Satisfaction | Evidence |
|---|---|---|---|---|---|---|
| DEP-11-03-001 | ANCHOR | UPSTREAM | OTHER | `SOW-033` Documentation and invented examples | SATISFIED | `docs/_Registers/ScopeLedger.csv` row `SOW-033` |
| DEP-11-03-002 | ANCHOR | UPSTREAM | OTHER | `OBJ-001` Open auditable platform objective | SATISFIED | `docs/_Decomposition/SOFTWARE_DECOMP.md` row `OBJ-001` |
| DEP-11-03-003 | ANCHOR | UPSTREAM | OTHER | `OBJ-003` Centerline frame solver objective | SATISFIED | `docs/_Decomposition/SOFTWARE_DECOMP.md` row `OBJ-003` |
| DEP-11-03-004 | EXECUTION | UPSTREAM | CONSTRAINT | `docs/CONTRACT.md` invariant catalog | SATISFIED | `docs/CONTRACT.md` invariant rows |
| DEP-11-03-005 | EXECUTION | UPSTREAM | CONSTRAINT | `INIT.md` root bootstrap boundaries | SATISFIED | `INIT.md` four-boundary section |
| DEP-11-03-006 | EXECUTION | UPSTREAM | CONSTRAINT | Sealed deliverable context | SATISFIED | `_CONTEXT.md` Context Budget QA |

## Run Notes

- `MODE=UPDATE`
- `STRICTNESS=CONSERVATIVE`
- `CONSUMER_CONTEXT=NONE`
- `DECOMPOSITION_PATH=/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`
- Source documents scanned: `_CONTEXT.md`, `_REFERENCES.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `docs/CONTRACT.md`, `INIT.md`, `docs/_Decomposition/SOFTWARE_DECOMP.md`, `docs/_Registers/Deliverables.csv`, `docs/_Registers/ScopeLedger.csv`, `docs/_Registers/ContextBudgetQA.csv`.
- No declared dependency rows were present to preserve beyond the human-owned `NOT_TRACKED` coordination note.
- Exactly one ACTIVE `IMPLEMENTS_NODE` parent anchor was extracted.
- ID-format validator compatibility note: local `tools/validation/validate_id_format.sh` expects legacy three-digit IDs (`DEL-NNN-NN`, `PKG-NNN`), while `docs/TYPES.md` and this SOFTWARE_DECOMP use `DEL-XX-YY` and `PKG-XX`. This is a tool compatibility warning, not a register defect.
- Future public/permissive mechanics source selection remains a documentation production `TBD`, not an ACTIVE execution dependency for this setup run.

## Run History

- 2026-04-30T11:55:00-0600 - TASK+dependency-extract refreshed `Dependencies.csv` v3.1 with 6 ACTIVE rows; schema and enum validation passed; no protected data imported.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 6 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 6 |
| TBD | 0 |
| PENDING | 0 |
| IN_PROGRESS | 0 |
| WAIVED | 0 |
| NOT_APPLICABLE | 0 |

## Downstream Handoff Notes

- Reconciliation can consume `Dependencies.csv` as an extracted setup register.
- Future production drafting should select public/permissive theory sources and update final documentation outside this setup run under a separate authorized write scope.
- This dependency register does not move the deliverable to `ISSUED` and does not assert engineering compliance.
