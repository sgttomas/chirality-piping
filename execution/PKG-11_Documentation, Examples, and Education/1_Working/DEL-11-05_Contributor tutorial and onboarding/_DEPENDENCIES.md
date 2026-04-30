# Dependencies: DEL-11-05 Contributor tutorial and onboarding

## Coordination (human-owned)
- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for PREPARATION.

## Upstream (I need these before I can proceed) - human-owned declarations
- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations
- Dependencies coordinated externally by humans.

## Extracted Dependency Register (populated by TASK+dependency-extract)
- **Status:** COMPLETE
- **Dependencies.csv:** `Dependencies.csv`
- **RegisterSchemaVersion:** v3.1
- **ACTIVE rows:** 8
- **RETIRED rows:** 0
- **ANCHOR rows:** 3
- **EXECUTION rows:** 5

| DependencyID | Class | Direction | TargetType | Target | Status | Notes |
|---|---|---|---|---|---|---|
| DEP-11-05-001 | ANCHOR | UPSTREAM | WBS_NODE | SOW-033 | ACTIVE | Implements documentation and invented-example education scope. |
| DEP-11-05-002 | ANCHOR | UPSTREAM | REQUIREMENT | OBJ-001 | ACTIVE | Supports open, auditable platform objective. |
| DEP-11-05-003 | ANCHOR | UPSTREAM | REQUIREMENT | OBJ-002 | ACTIVE | Supports protected IP separation objective. |
| DEP-11-05-004 | EXECUTION | UPSTREAM | DOCUMENT | INIT.md; AGENTS.md | ACTIVE | Bootstrap and dispatch constraints. |
| DEP-11-05-005 | EXECUTION | UPSTREAM | DOCUMENT | docs/CONTRACT.md | ACTIVE | Invariant constraints. |
| DEP-11-05-006 | EXECUTION | UPSTREAM | DOCUMENT | docs/AGENTIC_DEVELOPMENT_WORKFLOW.md | ACTIVE | Type 1/Type 2 workflow constraints. |
| DEP-11-05-007 | EXECUTION | UPSTREAM | DOCUMENT | docs/IP_AND_DATA_BOUNDARY.md | ACTIVE | Public/private data and quarantine policy. |
| DEP-11-05-008 | EXECUTION | UPSTREAM | DOCUMENT | _CONTEXT.md; SCA-001 Handoff_State.md | ACTIVE | Architecture-basis use without ISSUED inference. |

## Run Notes and History (populated by TASK+dependency-extract)
- 2026-04-30 - TASK+dependency-extract ran in conservative mode for DEL-11-05.
- Source docs scanned: `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_CONTEXT.md`, `_REFERENCES.md`, `docs/_Registers/Deliverables.csv`, `docs/_Registers/ScopeLedger.csv`, `docs/_Decomposition/SOFTWARE_DECOMP.md`, and governing references cited in the four-document kit.
- No deliverable-to-deliverable execution edges were emitted because the source documents state constraints and future publication targets, but do not state an executable handoff requiring another deliverable.
- No protected standards examples, vendor data, private rule packs, owner standards, commercial software examples, or certification/compliance claims were used as dependency evidence.
- `Dependencies.csv` was validated with `tools/validation/validate_dependencies_schema.py`.

## Lifecycle Summary (populated by TASK+dependency-extract)
- ACTIVE: 8
- RETIRED: 0
- Parent anchor: present (`DEP-11-05-001`)
- SatisfactionStatus: all ACTIVE rows `SATISFIED` for setup context
- RequiredMaturity: `SEMANTIC_READY`
- ProposedMaturity: `SEMANTIC_READY`

## Consumer Handoff Notes
- Downstream consumers should treat these rows as setup/documentation constraints, not schedule logic.
- The register is information-flow evidence only and does not compute blocked/unblocked status because the human-owned coordination mode remains `NOT_TRACKED`.
- Future repo-level onboarding publication requires a separate approved task or human action.
