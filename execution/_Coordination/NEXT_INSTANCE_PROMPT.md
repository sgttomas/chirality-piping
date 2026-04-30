# NEXT INSTANCE PROMPT - ORCHESTRATOR DEV-001 Control Plane

Continue as `ORCHESTRATOR` for the OpenPipeStress SOFTWARE workflow.

The current objective is post-DAG DEV-001 control-plane routing. Do not launch
`WORKING_ITEMS`, dispatch `TASK`, start `PKG-01` / `DEL-01-01`, edit
deliverable-local dependency registers, or change lifecycle state unless the
human explicitly approves that next action.

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
- Non-`PKG-00` deliverable-local `Dependencies.csv` files are present but are
  not the graph authority for sequencing. Treat them as stale/non-authoritative
  evidence until reconciled or refreshed.
- Selected pilot candidate: `DEL-01-01 - Project governance baseline`.
- `DEL-01-01` write-scope authorization is recorded in
  `execution/_Coordination/DEV-001_PILOT_DISPATCH_DEL-01-01.md`.

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
22. `execution/_Reconciliation/Reconciliation_Run_Summary_2026-04-30_PRE_DEPENDENCIES_DAG.md`
23. `execution/_Reconciliation/DepClosure/CLOSURE_PRE_DEPENDENCIES_DAG_2026-04-30_1508/RUN_SUMMARY.md`
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

1. approve launch of the `DEL-01-01` pilot handoff to `WORKING_ITEMS`;
2. route candidate-edge or local-register ambiguity to `RECONCILIATION`;
3. route aggregate-DAG audit wrapper work to `AUDIT_DEP_CLOSURE` tooling;
4. pause with no execution.

If the human approves the `DEL-01-01` pilot, use
`execution/_Coordination/DEV-001_PILOT_DISPATCH_DEL-01-01.md` as the handoff
surface and do not expand beyond its authorized write targets.

## Hard Stops

- No product implementation without a sealed deliverable scope and human gate.
- No broad fan-out.
- No lifecycle transitions.
- No protected standards/code data, proprietary engineering values, private
  project data, real secrets, private libraries, or certification/compliance
  claims.
- No silent edits to deliverable-local `Dependencies.csv`.
