---
doc_id: OPS-NEXT-SESSION-PROMPT
doc_kind: init.next_session_prompt
status: evergreen_bootstrap_protocol
updated: 2026-04-30
assignment: orchestrator_bootstrap_current_control_state
approved_decomposition: docs/_Decomposition/SOFTWARE_DECOMP.md
approved_revision: "0.4"
scope: bootstrap fresh sessions into current coordination handoff and human-gated objective selection
next_workflow: ORCHESTRATOR gate; WORKING_ITEMS only after explicit one-item human authorization
exclude_scope: broad fan-out, candidate edge promotion, lifecycle transitions without approval, unsealed implementation
---

# NEXT SESSION PROMPT - DEV-001 Control-Loop Bootstrap

Continue as `ORCHESTRATOR` for the OpenPipeStress SOFTWARE workflow in a fresh session.

This prompt is a stable bootstrap entrypoint. It is not the source of the next
deliverable objective. Derive the current objective from the coordination
protocol, mutable handoff state, accepted `DAG-001` artifacts, current blocker
evidence when explicitly current, git/filesystem evidence, and the latest human
approval gate.

## Current Ground Truth

Treat these as stable bootstrap facts unless contradicted by newer filesystem or
git evidence:

- `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` defines the live
  control-loop protocol.
- `execution/_Coordination/NEXT_INSTANCE_STATE.md` is the mutable handoff truth
  for the current objective, last completed bounded item, commits, tests, open
  items, and immediate next actions.
- `execution/_Coordination/_COORDINATION.md` records durable coordination
  rulings, graph authority, and dispatch policy.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision `0.4` remains the active decomposition basis.
- `DAG-001` remains the execution sequencing and blocker-computation authority.
- Non-`PKG-00` local `Dependencies.csv` files are synchronized mirrors/evidence from `DAG-001`, not independent sequencing authority.
- `PKG-00` remains architecture context only and does not receive local `Dependencies.csv` files.
- Architecture-basis rows targeting `PKG-00` remain preserved in non-`PKG-00` local mirrors as injected context evidence.
- `CANDIDATE` edges remain non-gating and require later `RECONCILIATION` plus `CHANGE` before promotion.
- Broad DAG execution is not authorized by default. A fresh session must ask for
  or consume a human gate for exactly one next bounded item or another explicit
  route.
- This file should not name the next deliverable. Use it to enter the
  coordination loop, then discover the objective from the state and approved
  control-plane documents.

## Required Reading

Before acting, read:

1. `INIT.md`
2. `AGENTS.md`
3. `agents/AGENT_ORCHESTRATOR.md`
4. `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`
5. `execution/_Coordination/NEXT_INSTANCE_STATE.md`
6. `execution/_Coordination/_COORDINATION.md`

Then follow the required-reading list and operating rules in
`execution/_Coordination/NEXT_INSTANCE_PROMPT.md`. Read any active, latest, or
historically relevant dispatch brief named by `NEXT_INSTANCE_STATE.md`.

After reading, state that reading is complete, summarize the active control
state and governing constraints, and check `git status --short`.

## First Gate

Do not launch a new `WORKING_ITEMS` session, dispatch `TASK`, edit product artifacts,
or change lifecycle state until the human project authority approves the next
bounded action.

The next safe choices must be derived from the current coordination state and
the latest human instruction. Valid routes include:

- authorize exactly one bounded DAG item through a fresh sealed dispatch brief;
- route candidate-edge or dependency ambiguity to `RECONCILIATION`;
- route file-state handling, staging, or commits through `CHANGE`;
- route bounded graph/tool checks through the applicable `AUDIT_*` agent;
- decide whether to track or ignore the untracked pre-DAG reconciliation artifacts through `CHANGE`;
- pause with no execution.

If a next DAG item is authorized, prepare a fresh sealed handoff brief from
`DAG-001`, `docs/_Registers/Deliverables.csv`, applicable `AB-00-*`
architecture-basis rows, and the target deliverable's local context.

## Bounded Dispatch Rule

Execution constraints:

- `ORCHESTRATOR` owns the dispatch gate.
- `WORKING_ITEMS` owns actual deliverable work only after the human next-item gate.
- Use one `WORKING_ITEMS` session per authorized deliverable.
- Dispatch at most one bounded `TASK` from that session unless the human explicitly broadens the pattern.
- Do not start broad fan-out.
- Do not promote candidate edges.
- Do not recompute or alter the blocker queue unless explicitly assigned or made stale by a lifecycle/DAG change.
- Do not change lifecycle state unless explicitly authorized.

## Session Conclusion Protocol

Before ending the session, close the stateful handoff loop:

1. Update `execution/_Coordination/NEXT_INSTANCE_STATE.md` so it reflects the
   new ground truth for touched deliverables, lifecycle changes, file changes,
   commit hashes, tests/audits, human rulings, open TBDs, blockers, and
   immediate next actions.
2. Update `execution/_Coordination/_COORDINATION.md` only if the session creates
   a durable human ruling or changes coordination mode, graph authority, or
   dispatch policy.
3. Update `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` only if the
   control-loop protocol changes.
4. Update `init/NEXT_SESSION_PROMPT.md` only if the bootstrap protocol changes.
5. Refresh `execution/_Coordination/DEV-001_BLOCKER_QUEUE.*` only if explicitly
   assigned or if a lifecycle/DAG change makes the current queue stale.
6. Route staging and commits through `CHANGE`. If approval is needed, stop with
   an explicit `APPROVE:` command list.

This is the continuous-loop handoff: `ORCHESTRATOR` controls routing,
`WORKING_ITEMS` performs deliverable-session wrap-up and updates
`NEXT_INSTANCE_STATE.md` under `agents/AGENT_WORKING_ITEMS.md` Phase 5, and
`CHANGE` performs approved file-state commits.

## Guardrails

- No protected standards text, protected code data, proprietary engineering values, private project data, real secrets, or private libraries.
- No certification, sealing, approval, code-compliance, or professional-reliance claims.
- No lifecycle transition without explicit human authorization.
- No broad implementation work outside the next approved sealed deliverable.
- Keep candidate edges non-gating.
- Preserve `DAG-001` as sequencing authority and local `Dependencies.csv` files as mirrors/evidence.
- Route dependency ambiguity to `RECONCILIATION`.
- Route committed file-state changes through `CHANGE`.

## Expected Closeout

Closeout must include:

- whether a next bounded DAG item or other explicit route was launched or held;
- exact files changed, if any;
- tests/audits run and their results;
- any remaining human rulings or blockers.
