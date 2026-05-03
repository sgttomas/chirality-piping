---
doc_id: OPS-NEXT-SESSION-PROMPT
doc_kind: init.next_session_prompt
status: evergreen_bootstrap_protocol
updated: 2026-05-03
assignment: orchestrator_bootstrap_current_control_state
approved_decomposition: execution/_Decomposition/SOFTWARE_DECOMP.md
approved_revision: "0.5"
scope: bootstrap fresh sessions into current coordination handoff and human-gated objective selection
next_workflow: ORCHESTRATOR SCA-002 downstream refresh planning; WORKING_ITEMS only after refreshed sealed one-item human authorization
exclude_scope: product implementation before refresh gates, stale DAG-001 dispatch, candidate edge promotion, lifecycle transitions without approval, unsealed implementation, Chirality app/harness integration
---

# NEXT SESSION PROMPT - DEV-001 Control-Loop Bootstrap

Bootstrap in read-only `ORCHESTRATOR` posture until the required reading is
complete. This file is only the bootstrap entrypoint. It does not name or
authorize the next deliverable, graph refresh, scaffold tranche, lifecycle
transition, or Type 2 dispatch.

A fresh session must recover the current objective from the coordination files,
git/filesystem evidence, and the latest explicit human gate before planning or
editing.

## Read First

Read these files before planning or editing:

1. `INIT.md`
2. `AGENTS.md`
3. `agents/AGENT_ORCHESTRATOR.md`
4. `agents/AGENT_RECONCILIATION.md`
5. `agents/AGENT_CHANGE.md`
6. `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`
7. `execution/_Coordination/NEXT_INSTANCE_STATE.md`
8. `execution/_Coordination/_COORDINATION.md`
9. `execution/_ScopeChange/SCA-002_2026-05-02_1854/ACCEPTANCE_RECORD.md`
10. `execution/_ScopeChange/SCA-002_2026-05-02_1854/Handoff_State.md`
11. `plans/SCA-002_DOWNSTREAM_REFRESH_PLAN.md`

Then follow the fuller required-reading list in
`execution/_Coordination/NEXT_INSTANCE_PROMPT.md`, including any dispatch brief
named by `NEXT_INSTANCE_STATE.md`.

After reading, state that reading is complete, summarize the active control
state and governing constraints, and check `git status --short`. Continue as
`ORCHESTRATOR` only after this grounding step is complete.

## State Sources

- Live protocol: `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`.
- Mutable handoff truth: `execution/_Coordination/NEXT_INSTANCE_STATE.md`.
- Durable coordination rulings: `execution/_Coordination/_COORDINATION.md`.
- SCA-002 acceptance record:
  `execution/_ScopeChange/SCA-002_2026-05-02_1854/ACCEPTANCE_RECORD.md`.
- SCA-002 downstream refresh plan:
  `plans/SCA-002_DOWNSTREAM_REFRESH_PLAN.md`.
- Active decomposition:
  `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.5`, accepted for
  downstream refresh planning.
- Companion registers: `docs/_Registers/Deliverables.csv`,
  `docs/_Registers/ScopeLedger.csv`, and
  `docs/_Registers/ContextBudgetQA.csv`.
- Historical graph evidence: `execution/_DAG/DAG-001/DependencyEdges.csv` and
  `execution/_DAG/DAG-001/APPROVAL_RECORD.md`, tied to revision `0.4`.
- Historical implementation evidence:
  `execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv`.
- Historical computed queue: `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md`
  and `.csv`, stale relative to revision `0.5`.
- Quarantined reference corpus: `docs/_ScopeChange/chirality-app-docs/`, read
  only for governance perspective and not as implementation, runtime, UI,
  dependency, or dispatch authority.

## Dependency Direction

Use this direction consistently:

- `FromDeliverableID` is the downstream consumer.
- `TargetDeliverableID` is the upstream provider.
- Therefore `FromDeliverableID` is blocked by `TargetDeliverableID`.

Use only a human-accepted revision `0.5` graph for new blocker computation or
dispatch. Until that exists, `DAG-001` is historical revision `0.4` evidence
only. `Status=CANDIDATE` edges remain non-gating until promoted through
`RECONCILIATION` and `CHANGE`.

## Readiness Rules

Keep semantic readiness and implementation readiness separate:

- Semantic readiness answers: is the deliverable context prepared for bounded
  dispatch or review?
- Implementation readiness answers: can this deliverable safely consume
  committed upstream artifacts?
- `SEMANTIC_READY` is context readiness only. It does not satisfy DEV-001
  implementation blockers.
- Non-architecture upstream providers satisfy implementation blockers only when
  they have `COMMITTED` evidence in
  `execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv`.
- `PKG-00` `ARCHITECTURE_BASIS` edges are satisfied by the accepted
  architecture baseline, not by implementation evidence.

## Authority Boundaries

- SCA-002 revision `0.5` is accepted for downstream refresh planning only.
- `DAG-001` is historical revision `0.4` coordination evidence. It is not
  revision `0.5` dispatch authority until a refreshed graph is proposed,
  checked, and human accepted.
- Non-`PKG-00` deliverable-local `Dependencies.csv` files are synchronized
  mirrors/evidence from `DAG-001`, not independent sequencing authority, and
  remain stale relative to revision `0.5` until refreshed through later gates.
- `PKG-00` remains architecture context only and does not receive local
  `Dependencies.csv` files.
- Architecture-basis rows targeting `PKG-00` remain injected context evidence.
- `docs/_ScopeChange/chirality-app-docs/` is quarantined reference material
  only. It may not generate Type 2 dispatch, DAG edges, package-local contexts,
  production artifacts, GUI/runtime work, SDK/provider integration, harness API
  work, lifecycle transitions, or dependency-register changes under SCA-002.
- Broad DAG execution and product implementation are not authorized by default.

## First Gate

Do not launch `WORKING_ITEMS`, dispatch `TASK`, edit product artifacts, change
lifecycle state, promote candidate edges, or start a new deliverable until the
human project authority approves the next bounded action.

Valid next routes are:

- run SCA-002 Phase 1 `ORCHESTRATOR` inventory;
- prepare SCA-002 Phase 2 `RECONCILIATION` request;
- propose a revision `0.5` graph refresh plan, expected as `DAG-002`, without
  mutating or approving `DAG-001`;
- route candidate-edge or dependency ambiguity to `RECONCILIATION`;
- route bounded graph/tool checks through the applicable `AUDIT_*` agent;
- route file-state handling, staging, commits, or pushes through `CHANGE`;
- pause with no execution.

If one DAG item is authorized after the refresh gates, prepare a fresh sealed
handoff brief from the accepted revision `0.5` coordination graph,
`docs/_Registers/Deliverables.csv`, applicable `AB-00-*` architecture-basis
rows, and the target deliverable's refreshed local context. Do not reuse any
pre-SCA-002 dispatch brief as an implementation brief.

## Session Conclusion Protocol

Before ending the session, close the stateful handoff loop:

1. Update `execution/_Coordination/NEXT_INSTANCE_STATE.md` so it reflects the
   new ground truth: first move the previous latest completed task into the
   compact archive table if it is not already archived, then replace the
   latest-state summary with the task just completed. Include touched
   deliverables, lifecycle changes, file changes, commit hashes, tests/audits,
   human rulings, open TBDs, blockers, and immediate next actions.
2. Update `execution/_Coordination/_COORDINATION.md` only if the session creates
   a durable human ruling or changes coordination mode, graph authority, or
   dispatch policy.
3. Update `execution/_Coordination/NEXT_INSTANCE_PROMPT.md` only if the
   control-loop protocol changes.
4. Update `init/NEXT_SESSION_PROMPT.md` only if the bootstrap protocol changes.
5. Refresh `execution/_Coordination/DEV-001_BLOCKER_QUEUE.*` only if explicitly
   assigned after a revision `0.5` graph is human accepted.
6. Route staging and commits through `CHANGE`. If approval is needed, stop with
   an explicit `APPROVE:` command list.

This is the continuous-loop handoff: `ORCHESTRATOR` controls routing,
`WORKING_ITEMS` performs deliverable-session wrap-up and updates
`NEXT_INSTANCE_STATE.md` under `agents/AGENT_WORKING_ITEMS.md` Phase 5, and
`CHANGE` performs approved file-state commits.

## Guardrails

- No protected standards text, protected code data, proprietary engineering
  values, private project data, real secrets, or private libraries.
- No certification, sealing, approval, code-compliance, or professional-reliance
  claims.
- No lifecycle transition without explicit human authorization.
- No product implementation, `PREPARATION`, DAG refresh materialization,
  dependency refresh, blocker regeneration, or Type 2 dispatch before the
  relevant SCA-002 downstream refresh gate is approved.
- No broad implementation work outside the next approved sealed deliverable
  after refresh.
- Keep candidate edges non-gating.
- Preserve `DAG-001` as historical revision `0.4` evidence and local
  `Dependencies.csv` files as mirrors/evidence until a revision `0.5` refresh
  is accepted.
- Keep `docs/_ScopeChange/chirality-app-docs/` quarantined as reference
  material unless a later explicit scope change or architecture decision
  promotes specific content.
- Route dependency ambiguity to `RECONCILIATION`.
- Route committed file-state changes through `CHANGE`.
