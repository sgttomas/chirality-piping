# NEXT INSTANCE STATE

**Last Updated:** 2026-04-30
**Actor:** ORCHESTRATOR
**Current Decomposition:** docs/_Decomposition/SOFTWARE_DECOMP.md revision 0.4
**Active Plan:** plans/DAG-001_EXECUTION_DEPENDENCY_GRAPH_PLAN.md

## Current Pointers

| Artifact | Path | Status |
|---|---|---|
| Coordination policy | `execution/_Coordination/_COORDINATION.md` | current |
| Stable control-loop prompt | `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` | updated for DAG-001 |
| Init next-session prompt | `init/NEXT_SESSION_PROMPT.md` | updated for DAG-001 |
| Active plan | `plans/DAG-001_EXECUTION_DEPENDENCY_GRAPH_PLAN.md` | current active plan |
| Prior plan | `plans/SCA-001_DOWNSTREAM_FOUR_DOC_REFRESH_PLAN.md` | prior workflow context |
| Decomposition | `docs/_Decomposition/SOFTWARE_DECOMP.md` | revision 0.4 current_basis |
| Scope register | `docs/_Registers/ScopeLedger.csv` | 63 rows expected |
| Deliverables register | `docs/_Registers/Deliverables.csv` | 73 rows expected |
| Context budget register | `docs/_Registers/ContextBudgetQA.csv` | 73 rows expected |
| SCA-001 snapshot | `execution/_ScopeChange/SCA-001_2026-04-30_0045/` | completed propagation evidence |
| PKG-02 reconciliation | `execution/_Reconciliation/PKG-02_FourDocInitialization/` | completed tranche evidence |

## Current Program State

- Packages expected after v0.4 amendment: 13.
- Deliverables expected after v0.4 amendment: 73.
- Current architecture gate: `PKG-00 - Software Architecture Runway`.
- `SCA-001` accepted `PKG-00` `SEMANTIC_READY` content as architecture-basis candidate for sealed brief injection without treating `PKG-00` as `ISSUED`.
- All 65 downstream `PKG-01` through `PKG-12` `_CONTEXT.md` files have received SCA-001 architecture-basis injection.
- `PKG-02` downstream four-document initialization tranche has completed through semantic enrichment, semantic lensing, Pass 3 incorporation, REVIEW, RECONCILIATION, and AUDIT summary.
- `PKG-00` lifecycle state remains `8 / 8 SEMANTIC_READY`.
- `PKG-02` lifecycle state is `5 / 5 SEMANTIC_READY`.
- Remaining downstream deliverables are not yet initialized as production four-document kits.
- Coordination mode is Full DAG.
- DAG authoring is now authorized.
- Blocker computation remains disabled until a human-approved acyclic DAG exists.

## Active Human Rulings and Assumptions

1. Use Full DAG coordination representation.
2. Author the implementation-phase execution DAG as a separate governed artifact.
3. The DAG is deliverable-level, not package-only.
4. DAG edges mean predecessor work required before safe Type 2 product-development execution.
5. Do not compute blocked/unblocked queues until a human-approved acyclic DAG exists.
6. Use `SEMANTIC_READY` as the development-phase readiness threshold where applicable.
7. Do not mark `PKG-00` as `ISSUED`.
8. Use `SCA-001` as the amendment ID for PKG-00 architecture-basis propagation.
9. Use the following resolved architecture baseline for sealed brief injection and DAG evidence where applicable: Rust core/application services; Tauri 2 desktop shell; TypeScript/React/Vite GUI; Three.js viewport; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis for JSON payloads; Cargo/Vitest/Playwright/validation/protected-content test gates.
10. Keep exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, and physical project package/container as implementation-level TBDs unless later human approval or a sealed brief resolves them.

## Current Execution Queue

| Tier | Scope | Action |
|---|---|---|
| Active Plan | `plans/DAG-001_EXECUTION_DEPENDENCY_GRAPH_PLAN.md` | Create deliverable-level execution DAG. |
| DAG Output | `execution/_DAG/DAG-001/` | Create node register, edge register, graph JSON, cycle report, waves, and audit summary. |
| Architecture Gate | `PKG-00` | Keep as `SEMANTIC_READY`; do not mark `ISSUED`. |
| Prior Tranche Evidence | `PKG-02` | Use review/reconciliation/audit outputs as DAG evidence. |
| Deferred Scheduling | Blocker queues | Do not compute until DAG-001 is acyclic and human-approved. |

## Immediate Next Actions

1. Read the prerequisite files listed in `init/NEXT_SESSION_PROMPT.md`.
2. Confirm v0.4 register counts and filesystem coverage.
3. Create `execution/_DAG/DAG-001/`.
4. Build `DeliverableNodes.csv` from all 73 deliverables.
5. Extract and normalize evidence-backed dependency edges into `DependencyEdges.csv`.
6. Build `dag.json`.
7. Run endpoint, duplicate, self-dependency, cycle, orphan, hub, and schema checks.
8. Produce `TopologicalWaves.md` only if active edges are acyclic.
9. Produce `Cycle_Report.md` and `DAG_Audit.md`.
10. Report REVIEW/RECONCILIATION/AUDIT_DEP_CLOSURE handoffs and confirm that no blocker queue was computed.

## Update Protocol

- WORKING_ITEMS updates this file at session handoff.
- ORCHESTRATOR updates the stable prompt only when the control-loop protocol changes.
- DAG authoring outputs belong under `execution/_DAG/DAG-001/`.
- Do not write blocked/unblocked queue status until a human-approved acyclic DAG exists.
