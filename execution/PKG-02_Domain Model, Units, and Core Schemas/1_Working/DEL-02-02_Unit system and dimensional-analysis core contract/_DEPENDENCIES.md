# Dependencies: DEL-02-02 Unit system and dimensional-analysis core contract

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
- **Register schema:** v3.1
- **Summary:** 11 ACTIVE rows: 3 ANCHOR rows and 8 EXECUTION rows.

| DependencyID | Class | Direction | Type / Anchor | Target | Status | Evidence |
|---|---|---|---|---|---|---|
| DEP-DEL-02-02-001 | ANCHOR | UPSTREAM | IMPLEMENTS_NODE | SOW-025 | ACTIVE | `_CONTEXT.md` Scope Coverage; `ScopeLedger.csv` row SOW-025 |
| DEP-DEL-02-02-002 | ANCHOR | UPSTREAM | TRACES_TO_REQUIREMENT | OBJ-001 | ACTIVE | `_CONTEXT.md` Objective Support; `SOFTWARE_DECOMP.md` row OBJ-001 |
| DEP-DEL-02-02-003 | ANCHOR | UPSTREAM | TRACES_TO_REQUIREMENT | OBJ-012 | ACTIVE | `_CONTEXT.md` Objective Support; `SOFTWARE_DECOMP.md` row OBJ-012 |
| DEP-DEL-02-02-004 | EXECUTION | UPSTREAM | CONSTRAINT | AB-00-01 / DEL-00-01 | ACTIVE | `_CONTEXT.md` Architecture Basis Injection; `Procedure.md` step 11 |
| DEP-DEL-02-02-005 | EXECUTION | UPSTREAM | CONSTRAINT | AB-00-02 / DEL-00-02 | ACTIVE | `_CONTEXT.md` Architecture Basis Injection; `Specification.md` U-002 |
| DEP-DEL-02-02-006 | EXECUTION | UPSTREAM | CONSTRAINT | AB-00-03 / DEL-00-03 | ACTIVE | `_CONTEXT.md` Architecture Basis Injection; `Specification.md` U-011 |
| DEP-DEL-02-02-007 | EXECUTION | UPSTREAM | CONSTRAINT | AB-00-04 / DEL-00-04 | ACTIVE | `_CONTEXT.md` Architecture Basis Injection; `Specification.md` U-008 |
| DEP-DEL-02-02-008 | EXECUTION | UPSTREAM | CONSTRAINT | AB-00-06 / DEL-00-06 | ACTIVE | `_CONTEXT.md` Architecture Basis Injection; `Specification.md` U-010 |
| DEP-DEL-02-02-009 | EXECUTION | UPSTREAM | CONSTRAINT | AB-00-07 / DEL-00-07 | ACTIVE | `_CONTEXT.md` Architecture Basis Injection; `Procedure.md` step 9 |
| DEP-DEL-02-02-010 | EXECUTION | UPSTREAM | CONSTRAINT | AB-00-08 / DEL-00-08 | ACTIVE | `_CONTEXT.md` Architecture Basis Injection; `Specification.md` U-012 |
| DEP-DEL-02-02-011 | EXECUTION | DOWNSTREAM | ENABLES | Solver and rule engine consumers (UNKNOWN target) | ACTIVE | `_CONTEXT.md` Context Envelope; `Deliverables.csv` row DEL-02-02 |

## Run Notes and History (populated by TASK+dependency-extract)
- 2026-04-30 — TASK+dependency-extract refreshed `Dependencies.csv` in `MODE=UPDATE`, `STRICTNESS=CONSERVATIVE`, `SCOPE=DEL-02-02`.
- Run root: `/Users/ryan/ai-env/projects/chirality-piping/execution`.
- Decomposition path: `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`.
- Source docs selected by AUTO: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_CONTEXT.md`, `_REFERENCES.md`, and the named decomposition/register slices.
- Anchor doc selected by AUTO: `_CONTEXT.md` plus decomposition/register rows for validation.
- Execution docs selected by AUTO: `Specification.md`, `Procedure.md`, `Guidance.md`, `Datasheet.md`.
- Defaults used: `SOURCE_DOCS=AUTO`, `DOC_ROLE_MAP=DEFAULT`, `ANCHOR_DOC=AUTO`, `EXECUTION_DOC_ORDER=AUTO`, `CONSUMER_CONTEXT=NONE`.
- `[INFO] Parent anchor found: one ACTIVE IMPLEMENTS_NODE row for SOW-025.`
- `[INFO] Downstream solver/rule-engine target kept as UNKNOWN because the sealed source identifies the consumer class but not exact deliverable IDs.`
- `[INFO] No rows were retired; no prior Dependencies.csv was present in this folder.`

## Lifecycle Summary (populated by TASK+dependency-extract)
- ACTIVE rows: 11.
- RETIRED rows: 0.
- Anchor rows: 3.
- Execution rows: 8.
- Satisfaction status: 11 PENDING.
- Closure note: all rows remain draft planning/evidence records until later implementation or human review satisfies the referenced constraints.

## Consumer Handoff Notes
- No explicit `CONSUMER_CONTEXT` was requested. Downstream aggregation/reconciliation may consume `Dependencies.csv` as a deliverable-local v3.1 register.
- The unresolved downstream consumer row (`DEP-DEL-02-02-011`) requires human/decomposition clarification before it is treated as a specific dependency edge to a named deliverable.
