# NEXT INSTANCE PROMPT - ORCHESTRATOR DEV-001 Control Plane

Continue as `ORCHESTRATOR` for the OpenPipeStress SOFTWARE workflow.

This prompt is stable control-loop protocol. It is not the source of the next
deliverable objective. Derive the current objective from
`execution/_Coordination/NEXT_INSTANCE_STATE.md`,
`execution/_Coordination/_COORDINATION.md`, the latest DAG pointer, historical
`DAG-001` artifacts, unapproved `DAG-002` proposal artifacts when present, the
current blocker/evidence/dependency status surfaces, and the latest human
approval gate.

Do not launch a new `WORKING_ITEMS` session, dispatch `TASK`, start another
deliverable, edit deliverable-local dependency registers, or change lifecycle
state unless the human explicitly approves that next action.

## Current Ground Truth

- Mutable handoff truth:
  `execution/_Coordination/NEXT_INSTANCE_STATE.md`.
- Active decomposition: `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.5`, accepted for SCA-002 downstream refresh planning.
- SCA-002 acceptance record: `execution/_ScopeChange/SCA-002_2026-05-02_1854/ACCEPTANCE_RECORD.md`.
- Current downstream refresh plan: `plans/SCA-002_DOWNSTREAM_REFRESH_PLAN.md`.
- Quarantined reference corpus: `docs/_ScopeChange/chirality-app-docs/` may be read for governance perspective only; it is not implementation scope, runtime architecture, UI requirement, dependency authority, or dispatch authority under SCA-002.
- Historical coordination graph: `execution/_DAG/DAG-001/`.
- Historical approval record: `execution/_DAG/DAG-001/APPROVAL_RECORD.md`.
- Latest graph artifact: `execution/_DAG/DAG-002/` as an unapproved revision
  `0.5` proposal snapshot.
- No revision `0.5` graph approval record exists.
- `DAG-001` active edge set is historical revision `0.4` development
  coordination evidence only; it must not drive new revision `0.5` dispatch.
- Blocker computation is held for revision `0.5`; `CANDIDATE` edges remain
  non-gating.
- DEV-001 blocker computation uses implementation-readiness semantics:
  `FromDeliverableID` is blocked by `TargetDeliverableID`, and non-architecture
  upstreams require `COMMITTED` evidence in
  `execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv`.
- `SEMANTIC_READY` remains context readiness and does not satisfy
  implementation blockers by itself.
- Current blocker queue: `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md`
  and `.csv` as a hold-state queue, not blocked/unblocked readiness.
- Current lifecycle snapshot:
  `execution/_Coordination/REV05_LIFECYCLE_STATE_SNAPSHOT.csv`.
- Current implementation evidence projection:
  `execution/_Coordination/DEV-001_REV05_IMPLEMENTATION_EVIDENCE_STATUS.csv`.
- Current dependency-register status projection:
  `execution/_Coordination/DEV-001_REV05_DEPENDENCY_REGISTER_STATUS.csv`.
- `PKG-00` remains `SEMANTIC_READY`, not `ISSUED`.
- `PKG-00` is prerequisite architecture context processed through `SCA-001`;
  its applicable `AB-00-*` rows are injected into sealed briefs instead of
  being treated as implementation graph work.
- `PKG-00` does not require deliverable-local `Dependencies.csv` files.
- Non-`PKG-00` deliverable-local `Dependencies.csv` files are synchronized
  mirrors/evidence materialized from `DAG-001`, not independent sequencing
  authority.
- The next gate is always the latest explicit human choice: run SCA-002 refresh
  planning, authorize exactly one bounded DAG item after refresh, route
  reconciliation/change/audit, handle artifact state, or pause. Broad DAG
  execution is not authorized by default.

## First Action - Required Reading

Before planning a next action, read:

1. `INIT.md`
2. `AGENTS.md`
3. `agents/AGENT_ORCHESTRATOR.md`
4. `agents/AGENT_WORKING_ITEMS.md`
5. `agents/AGENT_RECONCILIATION.md`
6. `agents/AGENT_AUDIT_DEP_CLOSURE.md`
7. `agents/AGENT_CHANGE.md`
8. `docs/CONTRACT.md`
9. `docs/IP_AND_DATA_BOUNDARY.md`
10. `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`
11. `execution/_Decomposition/SOFTWARE_DECOMP.md`
12. `docs/_Registers/Deliverables.csv`
13. `execution/_Coordination/_COORDINATION.md`
14. `execution/_Coordination/NEXT_INSTANCE_STATE.md`
15. `execution/_ScopeChange/SCA-002_2026-05-02_1854/ACCEPTANCE_RECORD.md`
16. `execution/_ScopeChange/SCA-002_2026-05-02_1854/Handoff_State.md`
17. `plans/SCA-002_DOWNSTREAM_REFRESH_PLAN.md`
18. `execution/_DAG/_LATEST.md`
19. `execution/_DAG/DAG-002/PROPOSAL_RECORD.md`
20. `execution/_DAG/DAG-002/DAG_Audit.md`
21. `execution/_DAG/DAG-002/Cycle_Report.md`
22. `execution/_DAG/DAG-002/TopologicalWaves.md`
23. `execution/_DAG/DAG-002/DependencyEdges.csv`
24. `execution/_DAG/DAG-001/APPROVAL_RECORD.md`
25. `execution/_DAG/DAG-001/DAG_Audit.md`
26. `execution/_DAG/DAG-001/Cycle_Report.md`
27. `execution/_DAG/DAG-001/TopologicalWaves.md`
28. `execution/_DAG/DAG-001/DependencyEdges.csv`
29. `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md`
30. `execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv`
31. `execution/_Coordination/DEV-001_REV05_IMPLEMENTATION_EVIDENCE_STATUS.csv`
32. `execution/_Coordination/REV05_LIFECYCLE_STATE_SNAPSHOT.csv`
33. `execution/_Coordination/DEV-001_REV05_DEPENDENCY_REGISTER_STATUS.csv`
34. Any active, latest, or historically relevant dispatch brief named by
    `NEXT_INSTANCE_STATE.md`.
35. `execution/_Reconciliation/Reconciliation_Run_Summary_2026-04-30_DEV001_CONTROL_PLANE_HARDENING.md`
36. `execution/_Reconciliation/DepClosure/CLOSURE_DEV001_POST_MATERIALIZATION_2026-04-30/RUN_SUMMARY.md`
37. `plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md`

After reading, state that reading is complete, summarize the active control
state, and ask for or consume the next human approval gate.

## Operating Rules

- Use aggregate `DAG-001` as historical revision `0.4` coordination evidence.
  Do not use it as revision `0.5` dispatch authority until refreshed and
  accepted through the SCA-002 downstream refresh plan.
- Use aggregate `DAG-002` only as an unapproved revision `0.5` proposal until
  an explicit human graph approval record exists.
- Use a DEV-001 implementation projection only as a derived view when useful:
  exclude `PKG-00` nodes and `ARCHITECTURE_BASIS` edges, while retaining
  `SCA-001` / `AB-00-*` brief injection evidence.
- Do not promote or use `CANDIDATE` edges for blocker queues, wave placement,
  readiness claims, scheduling, staffing, or priority.
- Route candidate-edge ambiguity and local-register conflicts to
  `RECONCILIATION`.
- Route structural graph/tool audit needs to `AUDIT_DEP_CLOSURE` or a wrapper
  that can consume the relevant aggregate graph artifacts.
- Route file-state edits and commits through `CHANGE`.

## Objective Derivation and Next Gate

To determine the next safe step:

1. Read the mutable handoff and coordination record.
2. Confirm whether the prior bounded item is complete, committed, awaiting
   review, awaiting CHANGE approval, blocked, or superseded by a newer human
   instruction.
3. Consult `DAG-002`, topological waves, and the blocker queue only as
   unapproved proposal/hold-state evidence until a revision `0.5` graph is
   approved. Do not treat wave position or queue rows as schedule, staffing,
   priority, readiness, or authorization.
4. Apply the latest explicit human ruling. If no next item is authorized, ask
   for a gate among: SCA-002 refresh planning, one bounded DAG item,
   `RECONCILIATION`, `CHANGE`, `AUDIT_*`, artifact handling, or pause.
5. If exactly one bounded DAG item is authorized after refresh, prepare a fresh
   sealed handoff brief from the accepted revision `0.5` coordination graph,
   `docs/_Registers/Deliverables.csv`, applicable `AB-00-*`
   architecture-basis rows, and the target deliverable's refreshed local
   context.

Do not reuse any prior dispatch brief as an implementation brief for a later
deliverable.

## Session Conclusion Protocol

Every session that changes program state must close the control loop before it
ends.

Required closeout actions:

1. Update `execution/_Coordination/NEXT_INSTANCE_STATE.md` with the new ground
   truth:
   - first move the previous latest completed task into the compact archive
     table if it is not already archived;
   - then replace the latest-state summary with the task just completed;
   - deliverable(s) touched and current lifecycle states;
   - exact files changed and commit hashes, if committed;
   - tests, audits, scans, and their results;
   - blocker queue status, only if it was explicitly refreshed;
   - new human rulings, accepted assumptions, remaining TBDs, and blockers;
   - immediate next actions for the following session.
2. Update `execution/_Coordination/_COORDINATION.md` only when a durable human
   ruling, coordination mode, graph authority, or dispatch policy changes.
3. Update `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` only when the
   control-loop protocol itself changes.
4. Refresh `execution/_Coordination/DEV-001_BLOCKER_QUEUE.*` only when the
   human explicitly assigns a refresh or when a DAG/evidence change makes the
   existing implementation-readiness queue stale.
5. Route all committed file-state changes through `CHANGE`. If approval is
   needed, stop with an explicit `APPROVE:` command list instead of committing.

This handoff rule is backed by `agents/AGENT_WORKING_ITEMS.md` Phase 5, which
defines session handoff to `NEXT_INSTANCE_STATE.md` as part of wrap-up when the
control loop is active. ORCHESTRATOR owns routing and protocol visibility;
WORKING_ITEMS owns deliverable-session handoff updates; CHANGE owns staging and
commits after explicit approval.

## Hard Stops

- No product implementation without a sealed deliverable scope and human gate.
- No broad fan-out.
- No lifecycle transitions.
- No protected standards/code data, proprietary engineering values, private
  project data, real secrets, private libraries, or certification/compliance
  claims.
- No silent edits to deliverable-local `Dependencies.csv`.
- No Chirality app/harness implementation, UI/runtime work, SDK/provider
  integration, DAG edge, Type 2 dispatch, package-local context, lifecycle
  transition, or dependency-register change from the quarantined reference
  corpus without a later explicit scope change or architecture decision.
