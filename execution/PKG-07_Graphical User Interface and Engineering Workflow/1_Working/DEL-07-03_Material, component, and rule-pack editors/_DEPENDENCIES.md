# Dependencies: DEL-07-03 Material, component, and rule-pack editors

## Coordination (human-owned)

- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for PREPARATION. Dependencies are coordinated externally by humans unless a later human-approved graph mode is established.

## Upstream (I need these before I can proceed) - human-owned declarations

- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations

- Dependencies coordinated externally by humans.

## Extracted Dependency Register

- **Status:** REFRESHED
- **Dependencies.csv:** `Dependencies.csv`
- **Register schema:** v3.1
- **Total ACTIVE rows:** 14
- **ANCHOR rows:** 3
- **EXECUTION rows:** 11
- **UPSTREAM rows:** 12
- **DOWNSTREAM rows:** 2

| DependencyID | Class | Direction | Type | Target | Status | Satisfaction |
|---|---|---|---|---|---|---|
| DEP-DEL-07-03-001 | ANCHOR | UPSTREAM | OTHER | PKG-07 Graphical User Interface and Engineering Workflow | ACTIVE | NOT_APPLICABLE |
| DEP-DEL-07-03-002 | ANCHOR | UPSTREAM | OTHER | SOW-021 GUI editor scope | ACTIVE | NOT_APPLICABLE |
| DEP-DEL-07-03-003 | ANCHOR | UPSTREAM | OTHER | OBJ-006 Visible GUI workflow objective | ACTIVE | NOT_APPLICABLE |
| DEP-DEL-07-03-004 | EXECUTION | UPSTREAM | INTERFACE | DEL-00-03 Application service command-query-job model | ACTIVE | PENDING |
| DEP-DEL-07-03-005 | EXECUTION | UPSTREAM | INTERFACE | DEL-00-05 GUI state and interaction architecture | ACTIVE | PENDING |
| DEP-DEL-07-03-006 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-06 Diagnostics warning and result-envelope contract | ACTIVE | PENDING |
| DEP-DEL-07-03-007 | EXECUTION | UPSTREAM | CONSTRAINT | DEL-00-07 API boundary and adapter contract map | ACTIVE | PENDING |
| DEP-DEL-07-03-008 | EXECUTION | UPSTREAM | INTERFACE | DEL-02-01 Canonical domain model schema | ACTIVE | PENDING |
| DEP-DEL-07-03-009 | EXECUTION | UPSTREAM | INTERFACE | DEL-03-01 Material library schema with provenance | ACTIVE | PENDING |
| DEP-DEL-07-03-010 | EXECUTION | UPSTREAM | INTERFACE | DEL-03-02 Pipe section and component library schema | ACTIVE | PENDING |
| DEP-DEL-07-03-011 | EXECUTION | UPSTREAM | INTERFACE | DEL-06-01 Rule-pack schema | ACTIVE | PENDING |
| DEP-DEL-07-03-012 | EXECUTION | UPSTREAM | INTERFACE | DEL-06-03 Required-input completeness checker | ACTIVE | PENDING |
| DEP-DEL-07-03-013 | EXECUTION | DOWNSTREAM | HANDOVER | DEL-07-04 Missing-data warning and blocking UX | ACTIVE | TBD |
| DEP-DEL-07-03-014 | EXECUTION | DOWNSTREAM | HANDOVER | DEL-08-03 Warnings assumptions and provenance report section | ACTIVE | TBD |

## Run Notes

- **TaskSkill:** dependency-extract
- **MODE:** UPDATE
- **STRICTNESS:** CONSERVATIVE
- **CONSUMER_CONTEXT:** NONE
- **DECOMPOSITION_PATH:** `docs/_Decomposition/SOFTWARE_DECOMP.md`
- **SOURCE_DOCS:** AUTO
- **ANCHOR_DOC:** `Specification.md`
- **EXECUTION_DOC_ORDER:** `Procedure.md`, `Guidance.md`, `Datasheet.md`, `Specification.md`
- **Register source posture:** Evidence-first extraction from production documents, with target resolution through the accepted decomposition/register rows.
- **Parent anchor check:** PASS - one ACTIVE `IMPLEMENTS_NODE` row.
- **Warnings:** `[WARNING] ID_VALIDATOR_LEGACY_FORMAT` - `tools/validation/validate_id_format.sh` still expects three-digit project IDs (`PKG-###`, `DEL-###-##`) and rejects current `SOFTWARE_DECOMP` IDs (`PKG-07`, `DEL-07-03`). Current IDs were verified against `docs/TYPES.md` software format and the authoritative registers.
- **Notes:** Some downstream rows are marked `PROPOSAL` where the source describes information flow but the future consumer brief should confirm the handoff.

## Run History

| Timestamp | Mode | Strictness | Decomposition | Warnings | ACTIVE rows |
|---|---|---|---|---|---:|
| 2026-04-30 10:44 MDT | UPDATE | CONSERVATIVE | `docs/_Decomposition/SOFTWARE_DECOMP.md` found | ID validator legacy format warning | 14 |

## Lifecycle Summary

| Metric | Count |
|---|---:|
| ACTIVE rows | 14 |
| RETIRED rows | 0 |
| ANCHOR rows | 3 |
| EXECUTION rows | 11 |
| Satisfaction `NOT_APPLICABLE` | 3 |
| Satisfaction `PENDING` | 9 |
| Satisfaction `TBD` | 2 |

## Downstream Handoff Notes

- `DEL-07-04` should confirm whether editor-originated missing-data findings become warning UX inputs.
- `DEL-08-03` should confirm whether editor-originated provenance and assumption records become report section inputs.
- Future implementation briefs should verify upstream schema/service/rule-pack maturity before source work begins.
