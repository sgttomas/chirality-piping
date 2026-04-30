# NEXT INSTANCE STATE

**Last Updated:** 2026-04-30
**Actor:** SCOPE_CHANGE
**Current Decomposition:** docs/_Decomposition/SOFTWARE_DECOMP.md revision 0.4

## Current Pointers

| Artifact | Path | Status |
|---|---|---|
| Coordination policy | `execution/_Coordination/_COORDINATION.md` | current |
| Stable control-loop prompt | `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` | current |
| Decomposition | `docs/_Decomposition/SOFTWARE_DECOMP.md` | revision 0.4 current_basis |
| Scope register | `docs/_Registers/ScopeLedger.csv` | 63 rows expected |
| Deliverables register | `docs/_Registers/Deliverables.csv` | 73 rows expected |
| Context budget register | `docs/_Registers/ContextBudgetQA.csv` | 73 rows expected |
| Preparation summary | `execution/PREPARATION_RUN_SUMMARY.md` | original 12-package summary |
| PKG-00 preparation delta | `execution/PKG00_PREPARATION_DELTA_SUMMARY.md` | current v0.3 delta summary |

## Current Program State

- Packages expected after v0.4 amendment: 13.
- Deliverables expected after v0.4 amendment: 73.
- Current architecture gate: `PKG-00 - Software Architecture Runway`.
- `SCA-001` accepted `PKG-00` `SEMANTIC_READY` content as architecture-basis candidate for sealed brief injection without treating `PKG-00` as `ISSUED`.
- All 65 downstream `PKG-01` through `PKG-12` `_CONTEXT.md` files have received SCA-001 architecture-basis injection.
- Downstream production four-document kits remain governed by future sealed Type 2 TASK execution; SCA-001 did not edit downstream `Datasheet.md`, `Specification.md`, `Guidance.md`, or `Procedure.md`.
- Coordination mode is Full DAG, but DAG authoring is deferred; blocker computation is disabled.

## Active Human Rulings and Assumptions

1. Use Full DAG coordination representation.
2. Do not infer or author dependency edges until explicitly requested.
3. Use `SEMANTIC_READY` as the development-phase readiness threshold.
4. Treat the implementation-phase DAG as a separate future artifact.
5. Add and run `PKG-00` before continuing setup/document drafting for the remaining packages.
6. Use `SCA-001` as the amendment ID for PKG-00 architecture-basis propagation.
7. Use the following resolved architecture baseline for sealed brief injection where applicable: Rust core/application services; Tauri 2 desktop shell; TypeScript/React/Vite GUI; Three.js viewport; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis for JSON payloads; Cargo/Vitest/Playwright/validation/protected-content test gates.
8. Keep exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, and physical project package/container as implementation-level TBDs unless later human approval or a sealed brief resolves them.

## Current Execution Queue

| Tier | Scope | Action |
|---|---|---|
| Architecture Gate | `PKG-00` | Complete through `SEMANTIC_READY`; not `ISSUED`. |
| Scope Change | `SCA-001` | Gate 5 propagation executed; CHANGE handoff/audit may follow. |
| Deferred Production Docs | `PKG-01` through `PKG-12` four-document kits | Refresh only through future ORCHESTRATOR-managed sealed TASK tranches. |

## Immediate Next Actions

1. Confirm v0.4 register counts and SCA-001 context propagation coverage.
2. Hand off SCA-001 changed file set to CHANGE if a git/file-state workflow is active.
3. Run ORCHESTRATOR against `plans/SCA-001_DOWNSTREAM_FOUR_DOC_REFRESH_PLAN.md` if the human wants downstream four-document kits refreshed.
4. Run RECONCILIATION/AUDIT_DECOMP on touched interfaces before broad downstream production-doc refresh.
5. Do not compute blocked/unblocked queues until a human-approved acyclic DAG exists.

## Update Protocol

- WORKING_ITEMS updates this file at session handoff.
- ORCHESTRATOR updates the stable prompt only when the control-loop protocol changes.
- Do not write blocked/unblocked queue status until a human-approved acyclic DAG exists.
