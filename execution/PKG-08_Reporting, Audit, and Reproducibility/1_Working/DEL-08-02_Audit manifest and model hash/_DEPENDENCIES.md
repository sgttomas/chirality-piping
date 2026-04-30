# Dependencies: DEL-08-02 Audit manifest and model hash

## Coordination (human-owned)

- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for PREPARATION. Extracted dependencies below are evidence-derived setup outputs, not a human-approved full dependency graph.

## Upstream (I need these before I can proceed) - human-owned declarations

- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations

- Dependencies coordinated externally by humans.

## Extracted Dependency Register

- **Status:** GENERATED
- **Dependencies.csv:** `Dependencies.csv`
- **Register schema:** v3.1
- **Extraction mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer context:** NONE

| Metric | Count |
|---|---:|
| ACTIVE rows | 9 |
| RETIRED rows | 0 |
| ANCHOR rows | 3 |
| EXECUTION rows | 6 |
| UPSTREAM execution rows | 4 |
| DOWNSTREAM execution rows | 2 |

### Compact Register View

| DependencyID | Class | Direction | Type | Target | SatisfactionStatus | Evidence |
|---|---|---|---|---|---|---|
| DEP-08-02-001 | ANCHOR | UPSTREAM | OTHER | SOW-039 | NOT_APPLICABLE | `_CONTEXT.md` Scope Coverage |
| DEP-08-02-002 | ANCHOR | UPSTREAM | OTHER | OBJ-007 | NOT_APPLICABLE | `_CONTEXT.md` Objective Support |
| DEP-08-02-003 | ANCHOR | UPSTREAM | OTHER | OBJ-012 | NOT_APPLICABLE | `_CONTEXT.md` Objective Support |
| DEP-08-02-004 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-05 | PENDING | `Procedure.md` Prerequisites |
| DEP-08-02-005 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-02 | PENDING | `Procedure.md` Prerequisites |
| DEP-08-02-006 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-06-04 | PENDING | `Procedure.md` Prerequisites |
| DEP-08-02-007 | EXECUTION | UPSTREAM | INTERFACE | DEL-10-05 | PENDING | `Procedure.md` Prerequisites |
| DEP-08-02-008 | EXECUTION | DOWNSTREAM | HANDOVER | DEL-08-01 | PENDING | `Procedure.md` Prerequisites |
| DEP-08-02-009 | EXECUTION | DOWNSTREAM | HANDOVER | DEL-08-04 | PENDING | `Procedure.md` Prerequisites |

## Run Notes and History

### Run Notes

- Source docs scanned: `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_REFERENCES.md`.
- Decomposition path: `docs/_Decomposition/SOFTWARE_DECOMP.md`.
- Target resolution: deliverable targets were resolved from register/decomposition IDs where explicit in the Procedure prerequisite table.
- Warning policy: no protected/private payloads were used; unresolved future implementation details remain `TBD` in production documents rather than dependency rows.
- Parent anchor check: PASS; exactly one ACTIVE `IMPLEMENTS_NODE` anchor is present.

### Run History

| Date | Actor | Mode | Strictness | Result |
|---|---|---|---|---|
| 2026-04-30 | TASK+dependency-extract | UPDATE | CONSERVATIVE | GENERATED 9 ACTIVE rows; schema validation passed. |

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 9 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| NOT_APPLICABLE | 3 |
| PENDING | 6 |

## Downstream Handoff Notes

- `DEL-08-01` should consume manifest/hash fields for report reproducibility display.
- `DEL-08-04` should carry or reference manifest/hash metadata in structured exports.
- `DEL-02-05`, `DEL-02-02`, `DEL-06-04`, and `DEL-10-05` are upstream contract surfaces; this setup artifact does not resolve their implementation details.
