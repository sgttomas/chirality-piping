# Dependencies: DEL-01-04 Professional responsibility and product-claims policy

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
- **ACTIVE rows:** 2
- **RETIRED rows:** 0

| DependencyID | Class | AnchorType | Direction | TargetType | TargetRefID | TargetName | Status |
|---|---|---|---|---|---|---|---|
| DEP-DEL-01-04-001 | ANCHOR | IMPLEMENTS_NODE | UPSTREAM | WBS_NODE | SOW-034 | Professional responsibility boundaries | ACTIVE |
| DEP-DEL-01-04-002 | ANCHOR | TRACES_TO_REQUIREMENT | UPSTREAM | REQUIREMENT | OBJ-011 | Preserve professional responsibility | ACTIVE |

## Run Notes
- Mode: UPDATE
- Strictness: CONSERVATIVE
- Consumer context: NONE
- Source docs: AUTO; scanned local four-document kit, _CONTEXT.md, _REFERENCES.md, and existing _DEPENDENCIES.md.
- Anchor doc: AUTO; used _CONTEXT.md and decomposition/register rows as anchor evidence.
- Execution docs: AUTO; no conservative execution edges emitted because no explicit information-flow handoff to another deliverable was stated in the local source documents.
- Decomposition path: /Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md
- Decomposition status: available; anchor labels resolved best-effort.
- Tree x DAG integrity: one ACTIVE parent anchor found; no FLOATING_NODE or AMBIGUOUS_ANCHOR warning.

## Run History
- 2026-04-30 09:39 MDT - TASK+dependency-extract MODE=UPDATE STRICTNESS=CONSERVATIVE; ACTIVE=2 RETIRED=0; warnings=none.

## Lifecycle Summary
- ACTIVE: 2
- RETIRED: 0
- SatisfactionStatus counts: TBD=2

## Downstream Handoff Notes
- Consumer context was NONE; no additional downstream handoff notes generated.
