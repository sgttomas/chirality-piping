# NEXT INSTANCE PROMPT - ORCHESTRATOR Control Loop

## Invariant Instructions

- Use `docs/_Decomposition/SOFTWARE_DECOMP.md` revision `0.4` as the current decomposition authority.
- Use `docs/_Registers/ScopeLedger.csv`, `docs/_Registers/Deliverables.csv`, and `docs/_Registers/ContextBudgetQA.csv` as the machine-readable registers.
- Treat `SCA-001` as executed: `PKG-00 - Software Architecture Runway` at `SEMANTIC_READY` supplies architecture-basis constraints for sealed brief injection, without marking `PKG-00` as `ISSUED`.
- Use `plans/SCA-001_DOWNSTREAM_FOUR_DOC_REFRESH_PLAN.md` as the active follow-on workflow plan.
- Coordination mode is Full DAG, but DAG authoring is deferred. Do not compute blocked/unblocked states until a human-approved acyclic DAG exists.
- Report lifecycle state from filesystem truth only.
- Do not create protected standards/code data, proprietary tables, copied formulas, or code-compliance claims.

## Standard Control Loop

1. ORCHESTRATOR scan: package/deliverable counts, lifecycle states, SCA-001 context propagation coverage, four-document kit presence, and tool-root presence.
2. Implement only the current authorized tranche from the active plan.
3. Dispatch one bounded TASK per deliverable only after the human approves the tranche/batch.
4. Rerun deliverable-local extraction/enrichment only for touched deliverables when needed.
5. Run REVIEW after each deliverable or approved batch.
6. Run RECONCILIATION on touched interfaces before broader cross-package propagation continues.
7. Run AUDIT_* checks after meaningful batches and final closeout, bounded to the active write surface.
8. Run full dependency closure only after a human-approved DAG exists.
9. Hand off coherent state to CHANGE or WORKING_ITEMS as appropriate.

## Current Tranche Rule

- Current plan: `plans/SCA-001_DOWNSTREAM_FOUR_DOC_REFRESH_PLAN.md`.
- Current objective: implement the post-SCA-001 downstream four-document refresh workflow.
- Current authorized start: ORCHESTRATOR Phase 1 scan and tranche setup.
- The first production-doc refresh batch must be explicitly approved by the human before any downstream `Datasheet.md`, `Specification.md`, `Guidance.md`, or `Procedure.md` files are edited.
- Recommended initial batching is the plan's conservative order, starting with `PKG-02` unless the human chooses another batch.
- `PKG-00` remains `SEMANTIC_READY`, not `ISSUED`.

## TASK Dispatch Model

- Dispatch one bounded TASK per deliverable.
- Every TASK brief must include `DeliverableID`, `PackageID`, `ScopePath`, applicable SOW/OBJ rows from `_Registers/Deliverables.csv`, applicable invariants from `docs/CONTRACT.md`, the deliverable `_CONTEXT.md`, explicit write scope, applicable `AB-00-*` architecture basis IDs, resolved SCA-001 baseline choices, and remaining implementation-level TBDs.
- Write scope is deliverable-local unless an agent instruction explicitly grants project-level control-plane writes.
- For downstream four-document refresh, permitted deliverable-local targets are only the authorized deliverable's `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, and run/evidence records requested by the sealed brief.
- TASK outputs must include durable run records in the deliverable folder.

## Architecture Basis

Future downstream TASK briefs must carry the applicable SCA-001 architecture basis from `SOFTWARE_DECOMP.md` revision `0.4`:

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
| Follow-on workflow plan | `plans/SCA-001_DOWNSTREAM_FOUR_DOC_REFRESH_PLAN.md` |
| SCA-001 snapshot | `execution/_ScopeChange/SCA-001_2026-04-30_0045/` |
| Coordination policy | `execution/_Coordination/_COORDINATION.md` |
| Stable session instructions | `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` |
| Mutable handoff state | `execution/_Coordination/NEXT_INSTANCE_STATE.md` |
| Deliverable identity and scope | deliverable `_CONTEXT.md` |
| Deliverable lifecycle | deliverable `_STATUS.md` |
| Deliverable production docs | deliverable `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md` |
| Deliverable dependencies | deliverable `_DEPENDENCIES.md` and future dependency register |
| Architecture production docs | `PKG-00` deliverable folders |

## Session Startup Procedure

Read, in order:

1. `INIT.md`
2. `AGENTS.md`
3. `agents/AGENT_ORCHESTRATOR.md`
4. `agents/AGENT_TASK.md`
5. `agents/AGENT_DELIVERABLE_TASK.md`
6. `agents/AGENT_REVIEW.md`
7. `agents/AGENT_RECONCILIATION.md`
8. `agents/AGENT_AUDIT_DECOMP.md`
9. `docs/README.md`
10. `docs/CONTRACT.md`
11. `docs/IP_AND_DATA_BOUNDARY.md`
12. `docs/VALIDATION_STRATEGY.md`
13. `docs/_Decomposition/SOFTWARE_DECOMP.md`
14. `docs/_Registers/*.csv`
15. `execution/_Coordination/_COORDINATION.md`
16. `execution/_Coordination/NEXT_INSTANCE_STATE.md`
17. `plans/SCA-001_DOWNSTREAM_FOUR_DOC_REFRESH_PLAN.md`
18. `execution/_ScopeChange/SCA-001_2026-04-30_0045/RUN_SUMMARY.md`

Then scan filesystem state and implement the current plan's Phase 1.

## Starter Prompt

Read `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`, `execution/_Coordination/NEXT_INSTANCE_STATE.md`, and `plans/SCA-001_DOWNSTREAM_FOUR_DOC_REFRESH_PLAN.md`; adopt ORCHESTRATOR; scan the workspace; then implement Phase 1 of the downstream four-document refresh plan by producing a concrete tranche proposal and asking for the required human approval before editing downstream production documents.
