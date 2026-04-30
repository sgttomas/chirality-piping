# NEXT INSTANCE PROMPT - ORCHESTRATOR Execution DAG Creation

## Invariant Instructions

- Use `docs/_Decomposition/SOFTWARE_DECOMP.md` revision `0.4` as the current decomposition authority.
- Use `docs/_Registers/ScopeLedger.csv`, `docs/_Registers/Deliverables.csv`, and `docs/_Registers/ContextBudgetQA.csv` as the machine-readable registers.
- Treat `SCA-001` as executed: `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies architecture-basis constraints for sealed brief injection, without marking `PKG-00` as `ISSUED`.
- Use `plans/DAG-001_EXECUTION_DEPENDENCY_GRAPH_PLAN.md` as the active workflow plan.
- Treat `plans/SCA-001_DOWNSTREAM_FOUR_DOC_REFRESH_PLAN.md` as prior workflow context, not the active plan.
- Coordination mode is Full DAG. DAG authoring is authorized. Do not compute blocked/unblocked states until a human-approved acyclic DAG exists.
- Report lifecycle state from filesystem truth only.
- Do not create protected standards/code data, proprietary tables, copied formulas, or code-compliance claims.

## Standard Control Loop

1. ORCHESTRATOR scan: register counts, package/deliverable folders, lifecycle states, context/dependency coverage, and existing evidence artifacts.
2. Implement only the current authorized DAG authoring plan.
3. Build a complete deliverable node inventory before authoring dependency edges.
4. Record every active edge with evidence, explicitness, confidence, maturity expectation, and source reference.
5. Keep uncertain dependencies as `CANDIDATE`; do not silently promote low-confidence edges.
6. Run endpoint, duplicate, self-dependency, cycle, orphan, and schema checks.
7. Run REVIEW, RECONCILIATION, and AUDIT_DEP_CLOSURE handoffs for graph quality, cycles, conflicts, and dependency-closure questions.
8. Hand off coherent state to CHANGE or WORKING_ITEMS as appropriate.
9. Do not compute blocked/unblocked queues until DAG-001 is acyclic and human-approved.

## Current Tranche Rule

- Current plan: `plans/DAG-001_EXECUTION_DEPENDENCY_GRAPH_PLAN.md`.
- Current objective: author the first governed deliverable-level execution DAG for software-product development.
- Current authorized start: ORCHESTRATOR DAG-001 inventory and graph creation.
- Required output root: `execution/_DAG/DAG-001/`.
- `PKG-00` remains `SEMANTIC_READY`, not `ISSUED`.
- Topological waves may be produced only as dependency order if active edges are acyclic.
- Topological waves are not schedule, priority, staffing, readiness, or blocked/unblocked queue status.

## DAG Output Model

Required files under `execution/_DAG/DAG-001/`:

1. `DeliverableNodes.csv`
2. `DependencyEdges.csv`
3. `dag.json`
4. `TopologicalWaves.md`
5. `Cycle_Report.md`
6. `DAG_Audit.md`

`DeliverableNodes.csv` must represent all 73 deliverables from `docs/_Registers/Deliverables.csv`.

`DependencyEdges.csv` should use the existing dependency-register v3.1 schema shape where possible:

- `RegisterSchemaVersion`
- `DependencyID`
- `FromPackageID`
- `FromDeliverableID`
- `FromDeliverableName`
- `DependencyClass`
- `AnchorType`
- `Direction`
- `DependencyType`
- `TargetType`
- `TargetPackageID`
- `TargetDeliverableID`
- `TargetRefID`
- `TargetName`
- `TargetLocation`
- `Statement`
- `EvidenceFile`
- `SourceRef`
- `EvidenceQuote`
- `Explicitness`
- `RequiredMaturity`
- `ProposedMaturity`
- `SatisfactionStatus`
- `Confidence`
- `Origin`
- `FirstSeen`
- `LastSeen`
- `Status`
- `Notes`

Direction convention:

`FromDeliverableID` depends on `TargetDeliverableID`.

## Architecture Basis

Future downstream TASK briefs and DAG edges must carry applicable SCA-001 architecture basis from `SOFTWARE_DECOMP.md` revision `0.4`:

- `AB-00-01` ADR and decision-record discipline.
- `AB-00-02` layer/module boundaries and no-bypass dependency rules.
- `AB-00-03` command/query/job/result-envelope boundaries.
- `AB-00-04` deterministic versioned persistence and canonical JSON/JCS-compatible hash basis.
- `AB-00-05` GUI durable/transient state split, service-command mutation route, and scoped undo/redo.
- `AB-00-06` diagnostics/result-envelope fields, warning classes, and no certification/compliance claims.
- `AB-00-07` internal/public API, adapter, plugin, provenance, and privacy boundaries.
- `AB-00-08` layered test and acceptance gates.

Resolved baseline choices:

- Rust core/application services.
- Tauri 2 desktop shell.
- TypeScript/React/Vite GUI.
- Three.js viewport where 3D viewport-facing.
- JSON Schema 2020-12 contracts.
- Schema-first command/query/job/result envelopes.
- Canonical JSON/JCS-compatible hash basis where JSON payloads are hashed.
- Cargo, Vitest, Playwright, validation, and protected-content/provenance test gates as applicable.

Still implementation-level TBD unless later human approval or a sealed brief resolves them:

- exact dependency versions;
- solver numerical library;
- rule expression grammar/library;
- public API transport;
- import/export format list;
- CI provider and coverage/performance thresholds;
- physical project package/container and migration framework.

## Information Placement

| Information | Canonical Home |
|---|---|
| Decomposition scope and architecture basis | `docs/_Decomposition/SOFTWARE_DECOMP.md` |
| Machine-readable registers | `docs/_Registers/*.csv` |
| Active DAG plan | `plans/DAG-001_EXECUTION_DEPENDENCY_GRAPH_PLAN.md` |
| Prior downstream four-doc plan | `plans/SCA-001_DOWNSTREAM_FOUR_DOC_REFRESH_PLAN.md` |
| SCA-001 snapshot | `execution/_ScopeChange/SCA-001_2026-04-30_0045/` |
| PKG-02 reconciliation/audit evidence | `execution/_Reconciliation/PKG-02_FourDocInitialization/` |
| Coordination policy | `execution/_Coordination/_COORDINATION.md` |
| Stable session instructions | `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` |
| Mutable handoff state | `execution/_Coordination/NEXT_INSTANCE_STATE.md` |
| DAG-001 outputs | `execution/_DAG/DAG-001/` |
| Deliverable identity and scope | deliverable `_CONTEXT.md` |
| Deliverable lifecycle | deliverable `_STATUS.md` |
| Deliverable production docs | deliverable `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md` |
| Deliverable dependency notes | deliverable `_DEPENDENCIES.md` |

## Session Startup Procedure

Read, in order:

1. `INIT.md`
2. `AGENTS.md`
3. `agents/AGENT_ORCHESTRATOR.md`
4. `agents/AGENT_TASK.md`
5. `agents/AGENT_DELIVERABLE_TASK.md`
6. `agents/AGENT_REVIEW.md`
7. `agents/AGENT_RECONCILIATION.md`
8. `agents/AGENT_AUDIT_DEP_CLOSURE.md`
9. `docs/CONTRACT.md`
10. `docs/IP_AND_DATA_BOUNDARY.md`
11. `docs/VALIDATION_STRATEGY.md`
12. `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`
13. `docs/_Decomposition/SOFTWARE_DECOMP.md`
14. `docs/_Registers/ScopeLedger.csv`
15. `docs/_Registers/Deliverables.csv`
16. `docs/_Registers/ContextBudgetQA.csv`
17. `execution/_Coordination/_COORDINATION.md`
18. `execution/_Coordination/NEXT_INSTANCE_STATE.md`
19. `execution/_ScopeChange/SCA-001_2026-04-30_0045/RUN_SUMMARY.md`
20. `execution/_ScopeChange/SCA-001_2026-04-30_0045/Handoff_State.md`
21. `execution/_Reconciliation/PKG-02_FourDocInitialization/RECONCILIATION_SUMMARY.md`
22. `execution/_Reconciliation/PKG-02_FourDocInitialization/AUDIT_SUMMARY.md`
23. `plans/SCA-001_DOWNSTREAM_FOUR_DOC_REFRESH_PLAN.md`
24. `plans/DAG-001_EXECUTION_DEPENDENCY_GRAPH_PLAN.md`

Then scan filesystem state and implement DAG-001 according to the active plan.

## Starter Prompt

Read `init/NEXT_SESSION_PROMPT.md`, `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`, `execution/_Coordination/NEXT_INSTANCE_STATE.md`, and `plans/DAG-001_EXECUTION_DEPENDENCY_GRAPH_PLAN.md`; adopt ORCHESTRATOR; scan the workspace; then create DAG-001 under `execution/_DAG/DAG-001/` without computing blocked/unblocked queues.
