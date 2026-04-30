# Dependencies: DEL-03-08 Pipe section property and mass-property calculator

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
- **Schema:** v3.1, 29 required columns, 7 data rows
- **Summary:** 2 ANCHOR rows and 5 EXECUTION rows.

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---|---|---|---|
| DEP-DEL-03-08-001 | ANCHOR | UPSTREAM | OTHER | SOW-051 | ACTIVE |
| DEP-DEL-03-08-002 | ANCHOR | UPSTREAM | OTHER | SOW-018 | ACTIVE |
| DEP-DEL-03-08-003 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-02-02 Unit system and dimensional-analysis core contract | ACTIVE |
| DEP-DEL-03-08-004 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-03-02 Pipe section and component library schema | ACTIVE |
| DEP-DEL-03-08-005 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-03-01 Material library schema with provenance | ACTIVE |
| DEP-DEL-03-08-006 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-00-06 Diagnostics, warning, and result-envelope contract | ACTIVE |
| DEP-DEL-03-08-007 | EXECUTION | UPSTREAM | CONSTRAINT | Synthetic or cleared fixture policy | ACTIVE |

## Run Notes and History (populated by TASK+dependency-extract)
### Run Notes

- SCOPE: `DEL-03-08`
- RUN_ROOT: `/Users/ryan/ai-env/projects/chirality-piping/execution`
- DECOMPOSITION_PATH: `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`
- MODE: `UPDATE`
- STRICTNESS: `CONSERVATIVE`
- SOURCE_DOCS: `AUTO`
- ANCHOR_DOC: `Datasheet.md` plus `_CONTEXT.md`/decomposition/register rows for explicit scope identifiers.
- EXECUTION_DOC_ORDER: `Procedure.md`, `Specification.md`, `Guidance.md`, `Datasheet.md`
- Decomposition status: found and used for target resolution where explicit deliverable IDs were available.
- Tree x DAG integrity: one ACTIVE `IMPLEMENTS_NODE` parent anchor was emitted for SOW-051; SOW-018 was retained as a trace anchor.
- [WARNING] LEGACY_ID_HELPER: `tools/validation/validate_id_format.sh` expects three-digit package IDs and rejects current project IDs such as `PKG-03` and `DEL-03-08`. Current project IDs were preserved as instructed.
- Unknown target retained: synthetic or cleared fixture policy has no resolved deliverable owner in the local evidence, so `TargetType=UNKNOWN`.

### Run History

- 2026-04-30 10:04 MDT - `TASK+dependency-extract`, MODE=`UPDATE`, STRICTNESS=`CONSERVATIVE`; wrote `Dependencies.csv` v3.1; ACTIVE rows: 7; warnings: `LEGACY_ID_HELPER`.

## Lifecycle Summary (populated by TASK+dependency-extract)
- ACTIVE: 7
- RETIRED: 0
- Closure status:
  - PENDING: 7
- Parent anchor status: exactly one ACTIVE `IMPLEMENTS_NODE`.

## Consumer Handoff Notes
- CONSUMER_CONTEXT: `NONE`.
- Downstream consumers should treat `DEP-DEL-03-08-007` as unresolved until a human or validation owner identifies the fixture-policy authority.
