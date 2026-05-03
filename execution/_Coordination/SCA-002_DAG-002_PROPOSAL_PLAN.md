---
doc_id: SCA-002-DAG-002-PROPOSAL-PLAN
doc_kind: coordination.dag_proposal_plan
status: handoff_authorized_for_next_agent
created: 2026-05-03
scope_change: SCA-002
accepted_revision: "0.5"
prepared_by: ORCHESTRATOR
target_graph_if_later_authorized: execution/_DAG/DAG-002/
handoff_authorized: 2026-05-03
handoff_record: execution/_Coordination/SCA-002_DAG-002_PROPOSAL_PLAN_HANDOFF_RECORD.md
---

# SCA-002 DAG-002 Proposal Plan

## Authority And Boundary

This plan is the authorized `ORCHESTRATOR` planning output after the SCA-002
revision `0.5` compatibility reconciliation. The human project authority later
authorized this proposal-plan handoff for closeout so the next fresh agent can
implement the unapproved proposal snapshot step.

It does not create `execution/_DAG/DAG-002/`, approve a graph, mutate
`DAG-001`, regenerate blocker queues, update lifecycle states, update
implementation evidence, refresh dependency mirrors, generate Type 2 dispatch,
run `PREPARATION`, or promote any Chirality app/harness material.

`DAG-001` remains historical revision `0.4` evidence. The next `DAG-002`
proposal implementation should create a new unapproved snapshot under
`execution/_DAG/DAG-002/`, not an in-place edit of `DAG-001`.

The handoff authorization is recorded at
`execution/_Coordination/SCA-002_DAG-002_PROPOSAL_PLAN_HANDOFF_RECORD.md`.
It does not approve `DAG-002` as an active graph and does not authorize any
downstream blocker, lifecycle, implementation-evidence, dependency-mirror,
Type 2 dispatch, `PREPARATION`, or Chirality corpus action.

## Inputs

| Input | Role |
|---|---|
| `execution/_Decomposition/SOFTWARE_DECOMP.md` | Accepted revision `0.5` decomposition basis |
| `docs/_Registers/Deliverables.csv` | 92 accepted deliverable rows |
| `docs/_Registers/ScopeLedger.csv` | 76 accepted scope rows |
| `docs/_Registers/ContextBudgetQA.csv` | 92 context-budget rows |
| `execution/_Reconciliation/Reconciliation_Run_Summary_2026-05-03_SCA002_REV05_COMPATIBILITY_PLANNING.md` | Compatibility findings and handoffs |
| `execution/_Reconciliation/DepClosure/CLOSURE_SCA002_REV05_COMPATIBILITY_2026-05-03_1441/Evidence/` | Worklists and closure evidence |
| `execution/_DAG/DAG-001/` | Historical revision `0.4` graph evidence |

## Target Output For Next Proposal Snapshot

The next graph-proposal implementation should produce a complete, unapproved
revision `0.5` snapshot under `execution/_DAG/DAG-002/` with at least:

- `DeliverableNodes.csv`
- `DependencyEdges.csv`
- `dag.json`
- `DAG_Audit.md`
- `Cycle_Report.md`
- `TopologicalWaves.md`
- `PROPOSAL_RECORD.md`

`APPROVAL_RECORD.md` should not be created until the human explicitly approves
the proposed active edge set.

## Node Plan

The future `DAG-002` proposal should include all 92 accepted revision `0.5`
deliverables.

| Node class | Count | Treatment |
|---|---:|---|
| Historical `DAG-001` nodes still present in revision `0.5` | 73 | Carry forward as candidate node set after metadata refresh from revision `0.5` registers. |
| SCA-002 added nodes absent from `DAG-001` | 19 | Add to proposal node register from revision `0.5` registers only. |
| Removed historical nodes | 0 | No node retirement expected. |

The 19 added nodes are:

`DEL-07-08`, `DEL-08-06`, `DEL-13-01`, `DEL-13-02`, `DEL-13-03`,
`DEL-13-04`, `DEL-14-01`, `DEL-14-02`, `DEL-14-03`, `DEL-14-04`,
`DEL-14-05`, `DEL-15-01`, `DEL-15-02`, `DEL-15-03`, `DEL-15-04`,
`DEL-16-01`, `DEL-16-02`, `DEL-16-03`, and `DEL-16-04`.

## Edge Proposal Method

The future graph proposal should use three edge lanes.

| Lane | Source | Proposed treatment |
|---|---|---|
| Carry-forward active edges | `DAG-001` active rows whose endpoints remain in revision `0.5` | Restate as proposed active edges only after endpoint, scope, and metadata refresh. |
| Restated/retired candidate edges | `DAG-001-E0616` through `DAG-001-E0624` | Keep non-gating. Restate only if still relevant; otherwise retire in proposal notes. |
| SCA-002 new-scope candidate questions | Reconciliation candidate-question worklist | Convert questions into proposed active or candidate rows only in a later graph-authoring pass; this plan does not approve any edge. |

The future proposal should preserve the DEV-001 direction convention:
`FromDeliverableID` is the downstream consumer blocked by `TargetDeliverableID`,
the upstream provider.

## Carry-Forward Active Edge Policy

Carry-forward should be conservative:

1. Preserve all `DAG-001` active edges with revision `0.5` endpoints unless
   SCA-002 changed the relationship.
2. Refresh all node metadata from revision `0.5` registers.
3. Flag edges involving `DEL-01-04` and `DEL-02-01` for targeted review because
   those deliverables gained direct SCA-002 scope/objective mappings.
4. Keep `PKG-00` architecture-basis treatment consistent with SCA-001:
   architecture basis remains injected context, not implementation work, and
   `PKG-00` remains `SEMANTIC_READY`, not `ISSUED`.
5. Do not use implementation evidence as a reason to omit dependencies.
   Evidence belongs to blocker readiness after graph approval, not to graph
   topology itself.

## SCA-002 Edge Question Workstreams

The later graph-authoring pass should resolve these workstreams before seeking
human graph approval.

| Workstream | Required question |
|---|---|
| Physical model and canonical model | How should `DEL-02-01` physical-model source-of-truth work relate to `PKG-13` design-knowledge and transformation deliverables? |
| Constraint engine | Which unit, diagnostics, persistence, and domain-model predecessors are required before `DEL-13-02`, `DEL-13-03`, and `DEL-13-04` can be sealed? |
| Model operations | Should `PKG-16` operation schema/validation/audit deliverables precede `DEL-07-08`, or should GUI comparison work consume only an initial operation contract? |
| State/run records | Which of `DEL-08-02`, `DEL-08-04`, `DEL-05-04`, and `DEL-02-05` are prerequisites for `PKG-14` state/run/comparison contracts? |
| Handoff workflow | How do `PKG-15` handoff packages depend on `DEL-10-03`, `DEL-10-02`, `DEL-08-04`, `PKG-14`, and redaction/export controls? |
| Reports | Does `DEL-08-06` consume `PKG-14`/`PKG-15` contracts, or can it define placeholder report sections before those contracts are implemented? |
| Professional boundary | How should expanded `DEL-01-04` scope govern comparisons, handoff metadata, external references, and agent rationale without approval/compliance claims? |
| Existing candidates | Which `DAG-001` candidate SCC questions remain relevant after SCA-002, and which should be retired or restated for the new graph proposal? |
| Chirality corpus | What compatibility memo is required before any harness, runtime, SDK/provider, desktop packaging, or agent-write concept is promoted? |

All workstream outputs remain proposals until human graph approval.

## Candidate Edge Policy

- Candidate edges must not affect blocker queues, wave placement, schedule,
  staffing, priority, lifecycle state, or implementation-readiness claims.
- Candidate edges should retain explicit evidence, rationale, confidence, and
  a clear reason they were not promoted.
- Candidate-related SCCs may be reported as warnings but must not block the
  active-edge acyclicity result unless the human asks to promote one or more
  candidates.

## Validation Gate For Future DAG-002 Proposal

Before requesting approval of any `DAG-002` active edge set, the graph proposal
should pass:

1. deliverable-node coverage: 92 / 92 accepted revision `0.5` deliverables;
2. endpoint validation: 0 invalid endpoints;
3. dependency schema validation for `DependencyEdges.csv`;
4. duplicate active directed edge check: 0 duplicates;
5. self-dependency check: 0 self-dependencies;
6. active-edge SCC/cycle check: 0 active SCCs;
7. bidirectional active pair check: 0 pairs unless explicitly justified;
8. candidate-layer warning report;
9. topological-wave generation from active edges only;
10. proof that no `DAG-001` artifact was mutated.

No blocker queue may be regenerated until the human approves the refreshed
graph.

## Downstream Sequencing After Future Graph Approval

If a later human gate approves `DAG-002`, the safe sequence is:

1. record a `DAG-002` approval record for the approved active edge set only;
2. regenerate blocker queues from approved active edges only;
3. authorize a `PREPARATION` scaffold tranche for missing package/deliverable
   control surfaces;
4. refresh or explicitly defer existing stale contexts;
5. refresh local dependency mirrors only if the workflow still uses them as
   synchronized evidence;
6. prepare one fresh revision `0.5` sealed Type 2 brief after graph/context
   surfaces are accepted.

## Open Decisions

| ID | Decision | Owner |
|---|---|---|
| DAG002-OD-001 | Creation of unapproved `execution/_DAG/DAG-002/` proposal artifacts is authorized for the next fresh agent, bounded by the handoff record. | Human / ORCHESTRATOR |
| DAG002-OD-002 | Decide whether targeted review of `DEL-01-04` and `DEL-02-01` happens before or during graph proposal authoring. | Human / REVIEW / ORCHESTRATOR |
| DAG002-OD-003 | Decide whether to carry forward all eligible `DAG-001` active edges by default or require row-by-row review for changed-neighborhood packages. | Human / ORCHESTRATOR |
| DAG002-OD-004 | Decide whether the Chirality corpus needs a separate compatibility memo before any future scope-change discussion. | Human / SOFTWARE_DECOMP / SCOPE_CHANGE |

## Next Authorized Implementation Step

Create an unapproved revision `0.5` `DAG-002` proposal snapshot under
`execution/_DAG/DAG-002/` using this plan and the handoff record, preserving
`DAG-001` as historical evidence.

This remains proposal material only. No graph approval, blocker regeneration,
lifecycle change, implementation evidence update, dependency mirror refresh,
Type 2 dispatch, `PREPARATION` scaffold, or Chirality corpus promotion is
authorized by this step.
