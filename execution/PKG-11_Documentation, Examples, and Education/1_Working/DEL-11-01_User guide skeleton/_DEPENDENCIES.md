# Dependencies: DEL-11-01 User guide skeleton

## Coordination (human-owned)
- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for PREPARATION.

## Upstream (I need these before I can proceed) - human-owned declarations
- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations
- Dependencies coordinated externally by humans.

## Extracted Dependency Register
- **Status:** UPDATED
- **Dependencies.csv:** Dependencies.csv
- **Schema Version:** v3.1
- **ACTIVE rows:** 15
- **RETIRED rows:** 0

| DependencyID | Class | AnchorType | Direction | TargetType | TargetRefID | TargetName | Status |
|---|---|---|---|---|---|---|---|
| DEP-DEL-11-01-001 | ANCHOR | IMPLEMENTS_NODE | UPSTREAM | WBS_NODE | SOW-033 | Documentation and invented-data examples for education and testing | ACTIVE |
| DEP-DEL-11-01-002 | ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | REQUIREMENT | OBJ-001 | Open auditable piping stress platform | ACTIVE |
| DEP-DEL-11-01-003 | ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | REQUIREMENT | OBJ-011 | Preserve professional responsibility | ACTIVE |
| DEP-DEL-11-01-004 | EXECUTION | NOT_APPLICABLE | UPSTREAM | DELIVERABLE | AB-00-01 | Architecture decision record baseline | ACTIVE |
| DEP-DEL-11-01-005 | EXECUTION | NOT_APPLICABLE | UPSTREAM | DELIVERABLE | AB-00-02 | Repository and module boundary architecture | ACTIVE |
| DEP-DEL-11-01-006 | EXECUTION | NOT_APPLICABLE | UPSTREAM | DELIVERABLE | AB-00-06 | Diagnostics, warning, and result-envelope contract | ACTIVE |
| DEP-DEL-11-01-007 | EXECUTION | NOT_APPLICABLE | UPSTREAM | DELIVERABLE | AB-00-07 | API boundary and adapter contract map | ACTIVE |
| DEP-DEL-11-01-008 | EXECUTION | NOT_APPLICABLE | UPSTREAM | DELIVERABLE | AB-00-08 | Layered software test and acceptance strategy | ACTIVE |
| DEP-DEL-11-01-009 | EXECUTION | NOT_APPLICABLE | UPSTREAM | DOCUMENT | OPS-INIT | OpenPipeStress Agent Bootstrap | ACTIVE |
| DEP-DEL-11-01-010 | EXECUTION | NOT_APPLICABLE | UPSTREAM | DOCUMENT | OPS-DIRECTIVE | OpenPipeStress Founding Intent and Constraints | ACTIVE |
| DEP-DEL-11-01-011 | EXECUTION | NOT_APPLICABLE | UPSTREAM | DOCUMENT | OPS-CONTRACT | OpenPipeStress Invariant Catalog | ACTIVE |
| DEP-DEL-11-01-012 | EXECUTION | NOT_APPLICABLE | UPSTREAM | DOCUMENT | OPS-TYPES | Domain vocabulary and identity | ACTIVE |
| DEP-DEL-11-01-013 | EXECUTION | NOT_APPLICABLE | UPSTREAM | DOCUMENT | OPS-SPEC | Technical and Agentic Implementation Specification | ACTIVE |
| DEP-DEL-11-01-014 | EXECUTION | NOT_APPLICABLE | UPSTREAM | DOCUMENT | OPS-IP-DATA-BOUNDARY | IP and Data Boundary Policy | ACTIVE |
| DEP-DEL-11-01-015 | EXECUTION | NOT_APPLICABLE | UPSTREAM | DOCUMENT | OPS-AGENTIC-WORKFLOW | Agentic Development Workflow | ACTIVE |

## Run Notes
- Mode: UPDATE
- Strictness: CONSERVATIVE
- Consumer context: NONE
- Source docs: AUTO; scanned the local four-document kit, `_CONTEXT.md`, `_REFERENCES.md`, and existing `_DEPENDENCIES.md`.
- Anchor doc: AUTO; used `_CONTEXT.md` and decomposition/register rows as anchor evidence.
- Execution docs: AUTO; emitted only explicit architecture-basis constraints from `_CONTEXT.md` and document constraints cited by local production documents.
- Decomposition path: `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`
- Decomposition status: available; anchor labels resolved best-effort.
- Tree x DAG integrity: one ACTIVE parent anchor found; no FLOATING_NODE or AMBIGUOUS_ANCHOR warning.
- ID validation note: `tools/validation/validate_id_format.sh` uses legacy three-digit package/deliverable patterns and rejects current SOFTWARE_DECOMP IDs such as `PKG-11` and `DEL-11-01`. Current IDs were preserved from `docs/TYPES.md`, the decomposition, and the registers.

## Run History
- 2026-04-30 11:21 MDT - TASK+dependency-extract MODE=UPDATE STRICTNESS=CONSERVATIVE; ACTIVE=15 RETIRED=0; warnings=legacy ID helper mismatch only.

## Lifecycle Summary
- ACTIVE: 15
- RETIRED: 0
- SatisfactionStatus counts: SATISFIED=12; TBD=3

## Downstream Handoff Notes
- Consumer context was NONE; no additional downstream handoff notes generated.
