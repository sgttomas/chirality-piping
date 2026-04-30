---
doc_id: OPS-NEXT-SESSION-PROMPT
doc_kind: init.next_session_prompt
status: ready_for_orchestrator_execution_dag_creation
updated: 2026-04-30
assignment: orchestrator_execution_dag_creation
approved_decomposition: docs/_Decomposition/SOFTWARE_DECOMP.md
approved_revision: "0.4"
active_plan: plans/DAG-001_EXECUTION_DEPENDENCY_GRAPH_PLAN.md
prior_plan: plans/SCA-001_DOWNSTREAM_FOUR_DOC_REFRESH_PLAN.md
scope_change_snapshot: execution/_ScopeChange/SCA-001_2026-04-30_0045/
---

# NEXT SESSION PROMPT - ORCHESTRATOR Execution DAG Creation

Continue as `ORCHESTRATOR` for the OpenPipeStress SOFTWARE workflow.

The active workflow plan is:

`plans/DAG-001_EXECUTION_DEPENDENCY_GRAPH_PLAN.md`

This is a normal ORCHESTRATOR control-loop run after `SCA-001` propagation and the first downstream `PKG-02` four-document initialization tranche. It is not SCA intake, not product-code implementation, and not blocker scheduling.

Your objective is to author the first governed deliverable-level execution DAG for product development. The DAG must represent what predecessor deliverables are required before each deliverable can be safely executed as software-product work.

Blocker computation remains disabled until a human-approved acyclic DAG exists.

---

## Prerequisite Reading

Read in this order before acting:

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
18. `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`
19. `execution/_Coordination/NEXT_INSTANCE_STATE.md`
20. `execution/_ScopeChange/SCA-001_2026-04-30_0045/RUN_SUMMARY.md`
21. `execution/_ScopeChange/SCA-001_2026-04-30_0045/Handoff_State.md`
22. `execution/_Reconciliation/PKG-02_FourDocInitialization/RECONCILIATION_SUMMARY.md`
23. `execution/_Reconciliation/PKG-02_FourDocInitialization/AUDIT_SUMMARY.md`
24. `plans/SCA-001_DOWNSTREAM_FOUR_DOC_REFRESH_PLAN.md`
25. `plans/DAG-001_EXECUTION_DEPENDENCY_GRAPH_PLAN.md`

Do not use `docs/INIT.md`; it is retired.

---

## Current Facts

- Current decomposition: `docs/_Decomposition/SOFTWARE_DECOMP.md`
- Current revision: `0.4`
- SCA amendment ID: `SCA-001`
- SCA-001 status: Gate 5 executed
- Packages expected: `13`
- Deliverables expected: `73`
- Scope items expected: `63`
- Coordination representation: Full DAG
- DAG authoring: authorized by the human project authority
- Blocker computation: disabled until a human-approved acyclic DAG exists
- `PKG-00` lifecycle state: `8 / 8 SEMANTIC_READY`
- `PKG-02` lifecycle state: `5 / 5 SEMANTIC_READY`
- Remaining downstream lifecycle state: `60 OPEN`
- Four-document kits currently exist for `PKG-00` and `PKG-02`
- Prior `PKG-02` review findings: no critical or major findings
- `PKG-00` is not `ISSUED`

---

## Authorized Work

Use `AGENT_ORCHESTRATOR.md` as the active role.

Authorized work:

1. Read and analyze source files.
2. Create the DAG output root `execution/_DAG/DAG-001/`.
3. Build a complete deliverable node register for all 73 deliverables.
4. Extract evidence-backed deliverable dependency edges.
5. Normalize edges into the v3.1 dependency-register shape where possible.
6. Create `dag.json`.
7. Run cycle, endpoint, duplicate, orphan, and schema checks.
8. Produce topological waves only if active edges are acyclic.
9. Produce `Cycle_Report.md` and `DAG_Audit.md`.
10. Route conflicts, cycles, or uncertain dependencies to `RECONCILIATION`.

Do not compute blocked/unblocked queues.

Do not implement product code.

Do not edit deliverable production documents as part of DAG authoring.

---

## Required DAG Outputs

Create these files under `execution/_DAG/DAG-001/`:

1. `DeliverableNodes.csv`
2. `DependencyEdges.csv`
3. `dag.json`
4. `TopologicalWaves.md`
5. `Cycle_Report.md`
6. `DAG_Audit.md`
7. `_LATEST.md` if a root pointer is needed

Optional scratch evidence may go under:

`execution/_DAG/DAG-001/evidence/`

---

## Required Workflow

1. Confirm filesystem and register state:
   - 13 package IDs;
   - 73 deliverable rows;
   - 63 scope-ledger rows;
   - 73 context-budget rows;
   - existing `_STATUS.md`, `_CONTEXT.md`, and `_DEPENDENCIES.md` coverage;
   - existing `PKG-00` and `PKG-02` production/evidence coverage.

2. Build `DeliverableNodes.csv`:
   - one row per deliverable from `docs/_Registers/Deliverables.csv`;
   - lifecycle from `_STATUS.md` only;
   - execution paths and evidence-file presence from filesystem truth.

3. Extract candidate edges from:
   - `_DEPENDENCIES.md`;
   - `_CONTEXT.md`;
   - deliverable descriptions and anticipated artifacts;
   - package descriptions and exclusions;
   - SCA-001 architecture basis;
   - review/reconciliation/audit evidence.

4. Normalize edges into `DependencyEdges.csv`:
   - edge direction means `FromDeliverableID` depends on `TargetDeliverableID`;
   - active edges must cite evidence;
   - uncertain edges remain `CANDIDATE`;
   - conflicts remain visible.

5. Build graph outputs:
   - `dag.json`;
   - endpoint and duplicate checks;
   - active-edge cycle check;
   - candidate-edge warning check;
   - orphan and hub scan.

6. Produce human-readable coordination outputs:
   - `TopologicalWaves.md` if acyclic;
   - cycle deferral note if not acyclic;
   - `Cycle_Report.md`;
   - `DAG_Audit.md`.

7. Run REVIEW, RECONCILIATION, and AUDIT_DEP_CLOSURE handoffs if the graph contains cycles, conflicted evidence, or unresolved cross-package decisions.

---

## Guardrails

- Do not mark `PKG-00` as `ISSUED`.
- Do not compute blockers or blocked/unblocked queues.
- Do not create protected standards/code data or proprietary engineering values.
- Do not represent generated software results as certified, approved, sealed, or code-compliant for reliance.
- Do not bypass the Type 2 execution rule from `AGENTS.md`.
- Treat `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` as evidence aids only, not independent engineering authority.
- Topological waves are dependency order only; they are not schedule, staffing, priority, or readiness claims.

---

## Completion Report

Report concisely:

- node count and package coverage;
- active edge count and candidate edge count;
- evidence-source coverage;
- cycle status;
- whether topological waves were produced;
- unresolved dependency questions;
- REVIEW/RECONCILIATION/AUDIT handoffs;
- explicit confirmation that no blocker queue was computed.
