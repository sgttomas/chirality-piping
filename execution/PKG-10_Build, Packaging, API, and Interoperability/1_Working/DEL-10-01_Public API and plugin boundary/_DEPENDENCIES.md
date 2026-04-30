# Dependencies: DEL-10-01 Public API and plugin boundary

## Coordination (human-owned)

- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for PREPARATION. Extracted rows below are evidence-backed setup registers only; they are not a complete scheduling DAG.

## Upstream (I need these before I can proceed) - human-owned declarations

- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations

- Dependencies coordinated externally by humans.

## Extracted Dependency Register

- **Status:** COMPLETE
- **Dependencies.csv:** `Dependencies.csv`
- **Register schema:** v3.1
- **Extraction mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer context:** NONE

| Class | Active | Retired | Notes |
|---|---:|---:|---|
| ANCHOR | 2 | 0 | Implements SOW-030 and traces to OBJ-009. |
| EXECUTION | 5 | 0 | Captures injected architecture-basis and invariant constraints. |
| Total | 7 | 0 | All rows include evidence file and source reference. |

## Active Rows

| DependencyID | Class | Direction | Type | Target | Satisfaction | Evidence |
|---|---|---|---|---|---|---|
| DEL-10-01-DEP-001 | ANCHOR | UPSTREAM | OTHER | SOW-030 | SATISFIED | `_CONTEXT.md#Scope Coverage`; `ScopeLedger.csv` row SOW-030 |
| DEL-10-01-DEP-002 | ANCHOR | UPSTREAM | OTHER | OBJ-009 | SATISFIED | `_CONTEXT.md#Objective Support`; `SOFTWARE_DECOMP.md` objective row OBJ-009 |
| DEL-10-01-DEP-003 | EXECUTION | UPSTREAM | CONSTRAINT | AB-00-03 | PENDING | `_CONTEXT.md#Architecture Basis Injection`; architecture basis row AB-00-03 |
| DEL-10-01-DEP-004 | EXECUTION | UPSTREAM | CONSTRAINT | AB-00-06 | PENDING | `_CONTEXT.md#Architecture Basis Injection`; architecture basis row AB-00-06 |
| DEL-10-01-DEP-005 | EXECUTION | UPSTREAM | CONSTRAINT | AB-00-07 | PENDING | `_CONTEXT.md#Architecture Basis Injection`; architecture basis row AB-00-07 |
| DEL-10-01-DEP-006 | EXECUTION | UPSTREAM | CONSTRAINT | AB-00-04 | PENDING | `_CONTEXT.md#Architecture Basis Injection`; architecture basis row AB-00-04 |
| DEL-10-01-DEP-007 | EXECUTION | UPSTREAM | CONSTRAINT | CONTRACT | PENDING | `docs/CONTRACT.md#Invariant index`; `Specification.md#Standards` |

## Run Notes

- Source documents scanned: `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_REFERENCES.md`, and `docs/_Decomposition/SOFTWARE_DECOMP.md` slices cited by the deliverable context.
- Dependency extraction was conservative: architecture-basis constraints and contract invariants were extracted because they are explicitly injected into this sealed context.
- No downstream deliverable edges were declared because the local sources do not create a human-approved execution DAG. Future reconciliation may add declared downstream handoffs after human routing.
- ID-format helper `tools/validation/validate_id_format.sh` was not used as a gate because it expects three-digit software IDs such as `DEL-010-01`, while this decomposition uses current stable IDs such as `DEL-10-01`.
- No protected standards/code data, proprietary engineering values, private rule-pack content, or certification/compliance claims were introduced.

## Run History

- 2026-04-30 - TASK+dependency-extract refreshed `Dependencies.csv` and `_DEPENDENCIES.md`; schema and enum validation passed; ID-format helper noted as incompatible with current two-digit decomposition IDs.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 7 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| SATISFIED | 2 |
| PENDING | 5 |

## Consumer Handoff Notes

- Reconciliation consumers should treat these rows as deliverable-local evidence, not as a complete project schedule.
- The key active constraints for future API/plugin work are AB-00-03, AB-00-04, AB-00-06, AB-00-07, and `docs/CONTRACT.md`.
