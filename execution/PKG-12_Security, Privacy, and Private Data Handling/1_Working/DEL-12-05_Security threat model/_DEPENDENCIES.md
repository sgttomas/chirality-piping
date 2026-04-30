# Dependencies: DEL-12-05 Security threat model

## Coordination (human-owned)
- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for this setup run.

## Upstream (I need these before I can proceed) - human-owned declarations
- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations
- Dependencies coordinated externally by humans.

## Extracted Dependency Register
- **Status:** ACTIVE
- **Dependencies.csv:** present
- **Schema:** v3.1
- **Summary:** 8 active rows

| DependencyID | Class | Direction | TargetType | TargetName | Status |
|---|---|---|---|---|---|
| DEP-12-05-001 | ANCHOR | UPSTREAM | WBS_NODE | SOW-040 | ACTIVE |
| DEP-12-05-002 | ANCHOR | UPSTREAM | REQUIREMENT | OBJ-010 | ACTIVE |
| DEP-12-05-003 | EXECUTION | UPSTREAM | DOCUMENT | docs/CONTRACT.md | ACTIVE |
| DEP-12-05-004 | EXECUTION | UPSTREAM | DOCUMENT | docs/PRD.md | ACTIVE |
| DEP-12-05-005 | EXECUTION | UPSTREAM | DOCUMENT | docs/IP_AND_DATA_BOUNDARY.md | ACTIVE |
| DEP-12-05-006 | EXECUTION | UPSTREAM | DOCUMENT | docs/SPEC.md | ACTIVE |
| DEP-12-05-007 | EXECUTION | UPSTREAM | DOCUMENT | docs/_Decomposition/SOFTWARE_DECOMP.md | ACTIVE |
| DEP-12-05-008 | EXECUTION | UPSTREAM | DOCUMENT | docs/DIRECTIVE.md | ACTIVE |

## Run Notes
- `SOURCE_DOCS`: AUTO
- `DOC_ROLE_MAP`: DEFAULT
- `ANCHOR_DOC`: AUTO
- `EXECUTION_DOC_ORDER`: AUTO
- `MODE`: UPDATE
- `STRICTNESS`: CONSERVATIVE
- `CONSUMER_CONTEXT`: NONE
- `DECOMPOSITION_PATH`: docs/_Decomposition/SOFTWARE_DECOMP.md
- `Warnings`: none
- Parent anchor check: one active `IMPLEMENTS_NODE` anchor found.
- Evidence check: all active rows include `EvidenceFile` and `SourceRef`.
- Extraction posture: information-flow and source-constraint rows only; no scheduling dependencies inferred.

## Run History
- 2026-04-30T00:00:00-06:00 - mode=UPDATE strictness=CONSERVATIVE decomposition=docs/_Decomposition/SOFTWARE_DECOMP.md status=found warnings=none active=3
- 2026-04-30T00:00:00-06:00 - mode=UPDATE strictness=CONSERVATIVE decomposition=docs/_Decomposition/SOFTWARE_DECOMP.md status=found warnings=none active=8 schema=v3.1

## Lifecycle Summary
- ACTIVE: 8
- RETIRED: 0
- Closure State: open
- SatisfactionStatus: TBD for all rows pending human review and downstream implementation maturation.

## Downstream Handoff Notes
- No downstream consumer context was supplied for this run.
- Future aggregation may use the anchor rows for tree placement and the document rows as upstream source constraints for threat-model refresh work.

