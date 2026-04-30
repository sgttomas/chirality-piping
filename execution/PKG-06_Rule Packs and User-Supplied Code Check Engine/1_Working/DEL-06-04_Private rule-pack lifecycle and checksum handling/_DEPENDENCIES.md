# Dependencies: DEL-06-04 Private rule-pack lifecycle and checksum handling

## Coordination (human-owned)

- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for PREPARATION. Extracted edges below are setup evidence for downstream reconciliation and are not a human-approved schedule.

## Upstream (I need these before I can proceed) - human-owned declarations

- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations

- Dependencies coordinated externally by humans.

## Extracted Dependency Register

- **Status:** UPDATED
- **Dependencies.csv:** `Dependencies.csv`
- **Register schema:** v3.1
- **ACTIVE rows:** 6
- **RETIRED rows:** 0

| DependencyID | Class | Direction | TargetType | Target | SatisfactionStatus | Confidence |
|---|---|---|---|---|---|---|
| DEP-06-04-A001 | ANCHOR | UPSTREAM | WBS_NODE | SOW-042 | NOT_APPLICABLE | HIGH |
| DEP-06-04-A002 | ANCHOR | UPSTREAM | REQUIREMENT | OBJ-002 | NOT_APPLICABLE | HIGH |
| DEP-06-04-A003 | ANCHOR | UPSTREAM | REQUIREMENT | OBJ-005 | NOT_APPLICABLE | HIGH |
| DEP-06-04-E001 | EXECUTION | UPSTREAM | DELIVERABLE | DEL-06-01 Rule-pack schema | PENDING | MEDIUM |
| DEP-06-04-E002 | EXECUTION | DOWNSTREAM | DELIVERABLE | DEL-08-02 Audit manifest and model hash | PENDING | MEDIUM |
| DEP-06-04-E003 | EXECUTION | UPSTREAM | PACKAGE | PKG-12 Security Privacy and Private Data Handling | PENDING | HIGH |

## Run Notes

- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer context:** RECONCILIATION
- **Decomposition path:** `docs/_Decomposition/SOFTWARE_DECOMP.md`
- **Source documents scanned:** `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_SEMANTIC_LENSING.md`
- **Parent anchor:** present (`DEP-06-04-A001`)
- **Warnings:** none that block setup.
- **Assumptions:** `DEP-06-04-E001` and `DEP-06-04-E002` are conservative implementation-handoff assumptions made explicit in `Procedure.md`; they are not implementation work and do not authorize edits outside DEL-06-04.
- **Deferred architecture:** storage location, encryption defaults, access-control policy defaults, permission persistence, secret handling, physical project container, and non-JSON payload partition remain TBD or owned by PKG-12/PKG-02 work.
- **Protected-data boundary:** no private rule-pack content, standards-body text/tables, copied formulas, proprietary values, or certification/compliance claims were added.

## Run History

| Date | Agent/Skill | Mode | Strictness | Result |
|---|---|---|---|---|
| 2026-04-30 | TASK+dependency-extract | UPDATE | CONSERVATIVE | Wrote `Dependencies.csv` v3.1 with 6 ACTIVE rows and refreshed this index. |

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 6 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| NOT_APPLICABLE | 3 |
| PENDING | 3 |

## Downstream Handoff Notes

- Reconciliation may use the DEL-06-01 and DEL-08-02 edges as proposed information-flow checks, not as implementation approval.
- PKG-12 remains the governing package for private storage, encryption, telemetry, redaction, access-control defaults, and secret/private-library handling.
- Any future implementation brief must re-check whether the schema enum names and checksum payload partitions have been decided by the responsible schema/persistence/security deliverables.
