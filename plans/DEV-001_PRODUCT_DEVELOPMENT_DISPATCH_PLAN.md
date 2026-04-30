---
doc_id: PLAN-DEV-001-PRODUCT-DEVELOPMENT-DISPATCH
doc_kind: plan.workflow
status: draft_for_human_review
created: 2026-04-30
authority: human_requested
active_decomposition: docs/_Decomposition/SOFTWARE_DECOMP.md
active_revision: "0.4"
depends_on:
  - execution/_DAG/DAG-001/
---

# DEV-001 Product Development Dispatch Plan

## Purpose

This plan bridges the completed `DAG-001` coordination draft into governed software-product development.

It answers:

> What has to happen before `WORKING_ITEMS` and `TASK` begin implementing OpenPipeStress deliverables as product code and supporting artifacts?

It does not itself implement product code, edit deliverable production documents, compute blocker queues, or mark lifecycle states.

## Current State

`DAG-001` exists under:

`execution/_DAG/DAG-001/`

Current DAG facts from `execution/_DAG/DAG-001/DAG_Audit.md`:

| Fact | State |
|---|---:|
| Packages represented | 13 |
| Deliverable nodes represented | 73 / 73 |
| Active edges | 615 |
| Candidate edges | 9 |
| Active-edge cycle status | ACYCLIC |
| Candidate warning SCCs | 4 |
| Topological waves | 12 |
| Endpoint issues | 0 |
| Self-dependencies | 0 |
| Duplicate active edges | 0 |
| Orphan nodes | 0 |

Current lifecycle distribution from `_STATUS.md`:

| Lifecycle state | Count |
|---|---:|
| `SEMANTIC_READY` | 13 |
| `OPEN` | 60 |

Important state constraints:

- `PKG-00` remains `SEMANTIC_READY`, not `ISSUED`.
- `PKG-00` architecture-basis material may be injected into future sealed briefs through `SCA-001`.
- Existing deliverable-local `Dependencies.csv` files are absent (`0 / 73`).
- Existing `_DEPENDENCIES.md` files are stubs with dependency extraction `NOT_RUN_YET`.
- Blocker computation remains disabled until a human-approved acyclic DAG exists.

## Required Control Sequence

Product-development implementation should not begin through broad autonomous code-writing fan-out yet.

The required sequence is:

1. **REVIEW DAG-001**
   - Scope: `execution/_DAG/DAG-001/`
   - Focus: edge evidence quality, v3.1 schema conformance, active/candidate separation, and whether architecture-basis edges are appropriately scoped.
   - Output: review summary and findings.

2. **RECONCILE Candidate And Cross-Package Questions**
   - Scope: candidate warning SCCs and unresolved questions from `DAG_Audit.md`.
   - Required issues:
     - `DEL-04-04` / `DEL-04-06` nonlinear diagnostics ordering.
     - `DEL-10-02` / `DEL-12-01` / `DEL-12-05` adapter, storage, and threat-model ordering.
     - `DEL-08-05` / `DEL-11-04` protected-content linter and invented example fixture ordering.
     - `DEL-09-05` / `DEL-10-04` release-gate and CI implementation ordering.
     - load-case algebra versus rule expression engine reuse.
   - Output: promote, reject, or retain each candidate edge with rationale.

3. **Run Dependency Closure Audit**
   - Because `AUDIT_DEP_CLOSURE` currently expects deliverable-local `Dependencies.csv`, choose one audit route:
     - adapt or wrap the audit to consume aggregate `DependencyEdges.csv`; or
     - materialize DAG-001 edges into deliverable-local `Dependencies.csv` files through a separately approved workflow.
   - Do not silently edit deliverable dependency registers during this plan.

4. **Human DAG Approval Gate**
   - Human project authority accepts DAG-001 as the development coordination basis, or requests amendment.
   - Record the decision in `execution/_DAG/DAG-001/APPROVAL_RECORD.md` or in a `CHANGE`-managed snapshot that explicitly points to the approved DAG files and review/reconciliation evidence.
   - Only after this gate may blocker/unblocked queue computation be enabled.

5. **Update Coordination Handoff State**
   - After approval is recorded, update `execution/_Coordination/NEXT_INSTANCE_STATE.md` with:
     - the accepted DAG pointer;
     - the approval record path;
     - the selected first development tranche;
     - any remaining candidate edges or rejected edge decisions;
     - whether blocker queue computation is still disabled or explicitly enabled by the human.
   - Do this before starting deliverable-local `WORKING_ITEMS` sessions.

6. **Begin Wave-Based Product Development**
   - Use `TopologicalWaves.md` as dependency order only.
   - Do not treat waves as schedule, staffing, priority, readiness, or professional approval.

## Dispatch Model

### Role Ownership

| Role | Responsibility |
|---|---|
| `ORCHESTRATOR` | Maintains control-plane visibility, wave/tranche selection, and next-session routing. |
| `WORKING_ITEMS` | Runs one deliverable-local working session and may dispatch bounded `TASK` subagents. |
| `TASK` | Executes one sealed deliverable brief within explicit scope and write targets. |
| `REVIEW` | Checks deliverable outputs and DAG/governance evidence before advancement. |
| `RECONCILIATION` | Resolves cross-package conflicts, candidate cycles, stale assumptions, and dependency ambiguity. |
| `AUDIT_*` | Runs bounded structural, dependency, governance, and epistemic checks. |
| `CHANGE` | Owns file-state reports, staging, commits, and snapshots when requested. |

### Dispatch Rule

Each product-development `TASK` must be sealed to exactly one deliverable.

Every implementation brief must include:

- `DeliverableID`
- `PackageID`
- `ScopePath`
- scope items and objectives from `docs/_Registers/Deliverables.csv`
- applicable invariants from `docs/CONTRACT.md`
- acceptance criteria from the deliverable `_CONTEXT.md` or sealed brief
- explicit write scope
- applicable active upstream dependencies from `execution/_DAG/DAG-001/DependencyEdges.csv`
- applicable `SCA-001` architecture basis IDs (`AB-00-*`)
- resolved architecture baseline and remaining implementation-level TBDs
- protected-data and professional-boundary guardrails

### Concurrency Model

After DAG approval, development may fan out only within a human-selected tranche.

Default dispatch policy:

- one `WORKING_ITEMS` session per active deliverable;
- one or more bounded `TASK` subagents only inside that deliverable's sealed write scope;
- no cross-deliverable writes by a `TASK`;
- no package-wide bulk implementation unless each deliverable has a separate sealed brief;
- rerun dependency extraction or DAG update only for touched deliverables, unless the human authorizes a full refresh.
- no automatic broad fan-out immediately after DAG approval; prove the dispatch loop with a small pilot first.

## Initial Development Tranche

The first implementation tranche should prove the dispatch mechanics before broad fan-out.

Recommended sequence after DAG approval:

1. **Wave 2 / governance and schema foundations**
   - `DEL-01-01` Project governance baseline.
   - `DEL-02-01` Canonical domain model schema.

2. **Wave 3 / data boundary, units, and status semantics foundations**
   - `DEL-01-02` Copyright and protected-data boundary policy.
   - `DEL-01-04` Professional responsibility and product-claims policy.
   - `DEL-02-02` Unit system and dimensional-analysis core contract.
   - `DEL-02-03` Code-neutral analysis boundary model.

3. **Wave 4 / persistence, extension, solver kernel, status, and rule schema**
   - Enter only after Wave 2 and Wave 3 predecessor outputs are sufficient for sealed briefs.
   - Prefer a small pilot subset before larger parallel implementation.

Do not skip foundation waves because later GUI, solver, or packaging work is more product-visible.

`SEMANTIC_READY` is not equivalent to implemented software. For `PKG-02`, the next product-development work is translation of existing semantic/document artifacts into actual schemas, contracts, modules, tests, fixtures, and evidence where authorized by a sealed brief. It is not a rerun of the four-document initialization tranche unless a later review explicitly requests that.

Initial pilot recommendation:

1. Run one `WORKING_ITEMS` session for `DEL-01-01` or `DEL-02-01`.
2. Dispatch at most one bounded `TASK` subagent from that session.
3. Review the resulting sealed brief quality, write-scope behavior, test/evidence output, and handoff update before opening additional parallel sessions.
4. Expand to the rest of Wave 2/3 only after the pilot proves the control loop.

## Sealed TASK Brief Template

Use this shape for product-development dispatches:

```markdown
PURPOSE: Implement the sealed OpenPipeStress deliverable within explicit scope.
RequestedBy: WORKING_ITEMS

ScopePath: <absolute path to deliverable folder>
DeliverablePath: <same absolute path>
TaskProfile: DELIVERABLE_TASK

DeliverableID: <DEL-XX-YY>
PackageID: <PKG-XX>

Tasks:
  - Implement only the artifacts authorized for this deliverable.
  - Preserve all applicable contract invariants and architecture-basis constraints.
  - Produce or update tests/evidence appropriate to the deliverable type.
  - Record a run summary and open issues.

RuntimeOverrides:
  DecompositionPath: docs/_Decomposition/SOFTWARE_DECOMP.md
  DecompositionRevision: "0.4"
  DAGPath: execution/_DAG/DAG-001/DependencyEdges.csv
  CoordinationMode: FULL_GRAPH
  BlockerComputation: DISABLED_UNTIL_HUMAN_APPROVED_DAG
  ScopeChangeID: SCA-001

CustomInstructions:
  - Read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, and existing production artifacts before editing.
  - Apply only applicable `AB-00-*` constraints; do not copy full `PKG-00` prose.
  - Treat protected standards/code data as out of scope.
  - Unknown engineering values remain `TBD`.
  - Do not claim certification, approval, sealing, or code compliance for reliance.
  - Do not edit files outside this deliverable folder.

AllowedWriteTargets:
  - <deliverable-local implementation files>
  - <deliverable-local tests/evidence>
  - <deliverable-local MEMORY.md or run records when applicable>

ExpectedOutputs:
  - Implemented deliverable artifacts listed in `_CONTEXT.md` or this brief.
  - Tests, fixtures, or validation evidence appropriate to the deliverable.
  - TASK run record under `_run_records/`.
  - Open issue list for unresolved `TBD`, assumptions, or cross-deliverable dependencies.

EXCLUSIONS:
  - No protected standards text, tables, examples, or proprietary engineering values.
  - No edits outside the sealed deliverable path.
  - No lifecycle state transition unless explicitly authorized by the human.
  - No blocked/unblocked queue computation.
```

## Acceptance Checks For DEV-001 Handoff

This plan is ready to hand to a next session agent when:

- DAG-001 output paths are explicitly listed.
- REVIEW, RECONCILIATION, AUDIT, human approval, and implementation dispatch are separated.
- The required DAG approval record location is explicit.
- The coordination handoff update after DAG approval is explicit.
- The role boundary among `ORCHESTRATOR`, `WORKING_ITEMS`, and `TASK` is explicit.
- The first tranche is identified without treating it as a schedule or readiness queue.
- The first tranche starts as a pilot, not broad automatic fan-out.
- The sealed `TASK` brief template is available for future dispatches.
- The plan confirms no product-code implementation occurs before DAG approval.

## Next Session Starter Prompt

```markdown
Continue as ORCHESTRATOR for the OpenPipeStress SOFTWARE workflow.

Read:
1. INIT.md
2. AGENTS.md
3. agents/AGENT_ORCHESTRATOR.md
4. agents/AGENT_WORKING_ITEMS.md
5. agents/AGENT_TASK.md
6. agents/AGENT_REVIEW.md
7. agents/AGENT_RECONCILIATION.md
8. agents/AGENT_AUDIT_DEP_CLOSURE.md
9. docs/CONTRACT.md
10. docs/_Decomposition/SOFTWARE_DECOMP.md
11. execution/_Coordination/NEXT_INSTANCE_PROMPT.md
12. execution/_Coordination/NEXT_INSTANCE_STATE.md
13. execution/_DAG/DAG-001/DAG_Audit.md
14. execution/_DAG/DAG-001/Cycle_Report.md
15. execution/_DAG/DAG-001/TopologicalWaves.md
16. plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md

Objective:
Prepare DAG-001 for human approval and product-development dispatch.

Do not implement product code yet.
Run the control sequence from DEV-001:
REVIEW DAG-001, route candidate loops to RECONCILIATION, determine the dependency-closure audit route, record human DAG approval if granted, update NEXT_INSTANCE_STATE.md with the accepted DAG and selected pilot tranche, and prepare the first sealed WORKING_ITEMS/TASK dispatch only after approval is recorded.

Confirm explicitly that no blocker queue is computed until the human approves the acyclic DAG.
```

## Non-Goals

This plan does not:

- authorize immediate product-code implementation;
- mark `PKG-00` or any deliverable as `ISSUED`;
- compute blocked/unblocked queues;
- mutate deliverable-local dependency registers;
- bulk-edit production documents;
- resolve protected engineering values or code-specific data;
- replace human review or professional judgment.
