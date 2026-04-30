# Dependencies: DEL-04-01 3D frame stiffness kernel

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
- **Summary:** 7 ACTIVE rows: 3 ANCHOR rows and 4 EXECUTION rows.

| DependencyID | Class | Direction | Type | Target | Status |
|---|---|---|---|---|---|
| DEP-04-01-001 | ANCHOR | UPSTREAM | OTHER | SOW-005 | ACTIVE |
| DEP-04-01-002 | ANCHOR | UPSTREAM | OTHER | SOW-035 | ACTIVE |
| DEP-04-01-003 | ANCHOR | UPSTREAM | OTHER | OBJ-003 | ACTIVE |
| DEP-04-01-004 | EXECUTION | UPSTREAM | CONSTRAINT | Invariant catalog | ACTIVE |
| DEP-04-01-005 | EXECUTION | UPSTREAM | CONSTRAINT | Architecture basis constraints | ACTIVE |
| DEP-04-01-006 | EXECUTION | DOWNSTREAM | ENABLES | DEL-04-02 | ACTIVE |
| DEP-04-01-007 | EXECUTION | DOWNSTREAM | ENABLES | DEL-04-05 | ACTIVE |

## Run Notes and History (populated by TASK+dependency-extract)
- 2026-04-30 - TASK+dependency-extract MODE=UPDATE STRICTNESS=CONSERVATIVE.
- Decomposition path: `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`.
- Source docs scanned: local four-doc kit, `_CONTEXT.md`, `_REFERENCES.md`, `docs/_Registers/ScopeLedger.csv`, `docs/_Registers/Deliverables.csv`, `docs/_Registers/ContextBudgetQA.csv`, `docs/CONTRACT.md`, and decomposition slices for DEL-04-01, PKG-04, OBJ-003, SOW-005, SOW-035, and applicable architecture basis IDs.
- `[WARNING] SHARED_SCOPE_ITEM`: SOW-035 is shared with DEL-04-05. This register records the explicit shared scope and a downstream handoff surface, not a schedule dependency.
- `[WARNING] TBD_TARGETS`: Solver library, sparse storage, tolerances, coordinate convention, and performance targets remain TBD.

## Lifecycle Summary (populated by TASK+dependency-extract)
- ACTIVE: 7
- RETIRED: 0
- SatisfactionStatus counts:
  - SATISFIED: 5
  - TBD: 2

## Consumer Handoff Notes
- Downstream consumers should treat this register as a setup-level dependency map only.
- DEP-04-01-006 and DEP-04-01-007 are information/interface handoff signals, not implementation or schedule commitments.
- No protected formulas, numeric defaults, or performance targets were extracted.
