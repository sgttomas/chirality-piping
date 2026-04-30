# NEXT INSTANCE PROMPT - ORCHESTRATOR DEV-001 Control Plane

Continue as `ORCHESTRATOR` for the OpenPipeStress SOFTWARE workflow.

The current objective is post-`DEL-01-01` pilot control-plane routing. Do not
launch a new `WORKING_ITEMS` session, dispatch `TASK`, start another deliverable,
edit deliverable-local dependency registers, or change lifecycle state unless
the human explicitly approves that next action.

## Current Ground Truth

- Active decomposition: `docs/_Decomposition/SOFTWARE_DECOMP.md` revision `0.4`.
- Accepted coordination graph: `execution/_DAG/DAG-001/`.
- Approval record: `execution/_DAG/DAG-001/APPROVAL_RECORD.md`.
- `DAG-001` active edge set is approved for development coordination.
- Blocker computation is enabled from approved `ACTIVE` `DAG-001` edges only;
  `CANDIDATE` edges remain non-gating.
- Current blocker queue: `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md`
  and `.csv`.
- `PKG-00` remains `SEMANTIC_READY`, not `ISSUED`.
- `PKG-00` is prerequisite architecture context processed through `SCA-001`;
  its applicable `AB-00-*` rows are injected into sealed briefs instead of
  being treated as implementation graph work.
- `PKG-00` does not require deliverable-local `Dependencies.csv` files.
- Non-`PKG-00` deliverable-local `Dependencies.csv` files are synchronized
  mirrors/evidence materialized from `DAG-001`, not independent sequencing
  authority.
- The `DEL-01-01 - Project governance baseline` pilot was accepted, launched,
  completed, and committed as `7650cf6 docs: tighten maintainer governance
  gates`.
- The current mutable handoff is
  `execution/_Coordination/NEXT_INSTANCE_STATE.md`.
- The next gate is pilot review and authorization of at most one next bounded
  DAG item; broad DAG execution is not authorized by default.

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
11. `docs/_Decomposition/SOFTWARE_DECOMP.md`
12. `docs/_Registers/Deliverables.csv`
13. `execution/_Coordination/_COORDINATION.md`
14. `execution/_Coordination/NEXT_INSTANCE_STATE.md`
15. `execution/_DAG/DAG-001/APPROVAL_RECORD.md`
16. `execution/_DAG/DAG-001/DAG_Audit.md`
17. `execution/_DAG/DAG-001/Cycle_Report.md`
18. `execution/_DAG/DAG-001/TopologicalWaves.md`
19. `execution/_DAG/DAG-001/DependencyEdges.csv`
20. `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md`
21. `execution/_Coordination/DEV-001_PILOT_DISPATCH_DEL-01-01.md`
22. `execution/_Reconciliation/Reconciliation_Run_Summary_2026-04-30_DEV001_CONTROL_PLANE_HARDENING.md`
23. `execution/_Reconciliation/DepClosure/CLOSURE_DEV001_POST_MATERIALIZATION_2026-04-30/RUN_SUMMARY.md`
24. `plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md`

After reading, state that reading is complete, summarize the active control
state, and ask for or consume the next human approval gate.

## Operating Rules

- Use aggregate `DAG-001` as the active graph authority.
- Use a DEV-001 implementation projection only as a derived view when useful:
  exclude `PKG-00` nodes and `ARCHITECTURE_BASIS` edges, while retaining
  `SCA-001` / `AB-00-*` brief injection evidence.
- Do not promote or use `CANDIDATE` edges for blocker queues, wave placement,
  readiness claims, scheduling, staffing, or priority.
- Route candidate-edge ambiguity and local-register conflicts to
  `RECONCILIATION`.
- Route structural graph/tool audit needs to `AUDIT_DEP_CLOSURE` or a wrapper
  that can consume aggregate `DAG-001` artifacts.
- Route file-state edits and commits through `CHANGE`.

## Next Decision Gate

The next safe step is a human choice among:

1. review the completed `DEL-01-01` pilot behavior and decide whether its
   bounded execution pattern is acceptable;
2. if accepted, authorize one next bounded DAG item, likely a Wave 2 foundation
   such as `DEL-02-01 - Canonical domain model schema`, through a sealed
   `WORKING_ITEMS` session and at most one bounded `TASK`;
3. route candidate-edge or local-register ambiguity to `RECONCILIATION`;
4. decide whether the untracked pre-DAG reconciliation artifacts should be
   tracked or ignored through `CHANGE`;
5. pause with no execution.

If the human approves the next DAG item, prepare a fresh sealed handoff brief
from `DAG-001`, `docs/_Registers/Deliverables.csv`, applicable `AB-00-*`
architecture-basis rows, and the target deliverable's local context. Do not
reuse the `DEL-01-01` pilot brief as an implementation brief for other
deliverables.

## Session Conclusion Protocol

Every session that changes program state must close the control loop before it
ends.

Required closeout actions:

1. Update `execution/_Coordination/NEXT_INSTANCE_STATE.md` with the new ground
   truth:
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
   human explicitly assigns a refresh or when a lifecycle/DAG change makes the
   existing queue stale.
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
