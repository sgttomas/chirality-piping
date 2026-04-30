---
doc_id: PLAN-DEV-001-PRODUCT-DEVELOPMENT-DISPATCH
doc_kind: plan.workflow
status: current_post_dag_control_plane_basis
created: 2026-04-30
updated: 2026-04-30
authority: human_requested
active_decomposition: docs/_Decomposition/SOFTWARE_DECOMP.md
active_revision: "0.4"
depends_on:
  - execution/_DAG/DAG-001/
  - execution/_DAG/DAG-001/APPROVAL_RECORD.md
---

# DEV-001 Product Development Dispatch Plan

## Purpose

This plan bridges the approved `DAG-001` coordination basis into governed
software-product development.

It answers:

> What must happen before `WORKING_ITEMS` and `TASK` begin implementing
> OpenPipeStress deliverables as product code and supporting artifacts?

It does not itself implement product code, edit deliverable production
documents, mutate deliverable-local dependency registers, or mark lifecycle
states.

## Current State

`DAG-001` is approved under:

`execution/_DAG/DAG-001/APPROVAL_RECORD.md`

Current aggregate DAG facts:

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
| `DependencyEdges.csv` schema validation | PASS |

Current lifecycle distribution from `_STATUS.md`:

| Lifecycle state | Count |
|---|---:|
| `SEMANTIC_READY` | 13 |
| `OPEN` | 60 |

Current blocker queue:

| Queue fact | Count |
|---|---:|
| Advisory `UNBLOCKED` deliverables | 17 |
| Advisory `BLOCKED` deliverables | 56 |

Important state constraints:

- `DAG-001` aggregate `DependencyEdges.csv` is the active graph authority for
  DEV-001 coordination.
- Blocker computation is enabled from approved `ACTIVE` DAG edges only.
- `CANDIDATE` edges are retained as non-gating pending later
  `RECONCILIATION`.
- `PKG-00` remains `SEMANTIC_READY`, not `ISSUED`.
- `PKG-00` architecture-basis material is injected into sealed briefs through
  `SCA-001` / applicable `AB-00-*` rows.
- `PKG-00` may be excluded from implementation graph participation and does not
  require deliverable-local `Dependencies.csv` files.
- Non-`PKG-00` local `Dependencies.csv` files exist, but are stale and
  non-authoritative for sequencing until reconciled or refreshed.

## Graph Authority

Use `execution/_DAG/DAG-001/DependencyEdges.csv` as the active coordination
graph. Use only rows with `Status=ACTIVE` for blocker computation, dependency
order, and pilot eligibility checks.

Do not use `Status=CANDIDATE` rows for blocker queues, wave placement,
readiness claims, schedule, staffing, or priority. Candidate rows remain
decision inputs for later `RECONCILIATION`.

### DEV-001 Projection

A DEV-001 implementation projection may be computed as a derived view:

- exclude `PKG-00` nodes;
- exclude `ARCHITECTURE_BASIS` edges;
- retain `SCA-001` / `AB-00-*` brief-injection requirements outside the graph.

Observed projection facts from the approved aggregate graph:

| Projection fact | State |
|---|---:|
| Nodes | 65 |
| Active edges | 227 |
| Cycle status | ACYCLIC |
| Waves | 11 |

This projection is not a replacement graph. It is only a cleaner implementation
view for downstream execution planning.

## Local Dependency Register Policy

The pre-DAG reconciliation run found that local dependency registers are not
safe as the sequencing authority:

- one 9-node local active SCC;
- three bidirectional active pairs;
- row-value/schema hygiene issues;
- two local active edges retained as `CANDIDATE` in `DAG-001`;
- 78 local active pairs absent from `DAG-001`.

Policy for DEV-001:

- local non-`PKG-00` `Dependencies.csv` files are evidence-only until refreshed
  or reconciled;
- do not silently edit local dependency registers;
- if local registers are needed later, route ambiguity to `RECONCILIATION` and
  approved file repairs to `CHANGE`;
- if aggregate DAG auditing is needed, use `AUDIT_DEP_CLOSURE` through a wrapper
  or tool path that consumes aggregate `DAG-001` artifacts.

## Role Ownership

| Role | Responsibility |
|---|---|
| `ORCHESTRATOR` | Maintains control-plane visibility, graph/projection routing, pilot gate, and next-session handoff. |
| `WORKING_ITEMS` | Runs one approved deliverable-local working session and may dispatch bounded `TASK` subagents. |
| `TASK` | Executes one sealed deliverable brief within explicit scope and write targets. |
| `REVIEW` | Checks deliverable outputs and governance evidence before advancement. |
| `RECONCILIATION` | Resolves candidate edges, local-register divergence, stale assumptions, and dependency ambiguity. |
| `AUDIT_*` | Runs bounded structural, dependency, governance, and epistemic checks. |
| `CHANGE` | Owns approved file-state edits, git state reports, staging, commits, and snapshots. |

## Post-Approval Control Sequence

Product-development implementation should not begin through broad autonomous
fan-out. The required sequence is:

1. **Keep control plane aligned**
   - Confirm `NEXT_INSTANCE_PROMPT.md`, `NEXT_INSTANCE_STATE.md`, `_COORDINATION.md`,
     this plan, and `SOFTWARE_DECOMP.md` all reflect approved `DAG-001` state.
   - Keep local registers non-authoritative unless a later human ruling changes
     the policy.

2. **Use approved queue evidence**
   - Consume `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md` for advisory
     blocked/unblocked state.
   - Do not recompute queues inside `WORKING_ITEMS` or `TASK` unless that is the
     explicit approved assignment.

3. **Human pilot launch gate**
   - The human must explicitly approve launch of the first pilot session.
   - Current pilot candidate: `DEL-01-01 - Project governance baseline`.
   - Handoff surface:
     `execution/_Coordination/DEV-001_PILOT_DISPATCH_DEL-01-01.md`.

4. **Run one pilot**
   - Start one `WORKING_ITEMS` session for `DEL-01-01`.
   - Dispatch at most one bounded `TASK` from that session.
   - Stay within the authorized write targets and guardrails recorded in the
     pilot dispatch brief.

5. **Review pilot behavior**
   - Check sealed-scope behavior, write-scope behavior, evidence/test output,
     and handoff-state update.
   - Expand to additional Wave 2 / Wave 3 deliverables only after human review.

## Dispatch Rule

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

## Initial Development Tranche

The first implementation tranche should prove the dispatch mechanics before
broad fan-out.

Recommended sequence:

1. **Wave 2 / governance and schema foundations**
   - `DEL-01-01` Project governance baseline.
   - `DEL-02-01` Canonical domain model schema.

2. **Wave 3 / data boundary, units, and status semantics foundations**
   - `DEL-01-02` Copyright and protected-data boundary policy.
   - `DEL-01-04` Professional responsibility and product-claims policy.
   - `DEL-02-02` Unit system and dimensional-analysis core contract.
   - `DEL-02-03` Code-neutral analysis boundary model.

3. **Wave 4 / persistence, extension, solver kernel, status, and rule schema**
   - Enter only after Wave 2 and Wave 3 predecessor outputs are sufficient for
     sealed briefs.
   - Prefer a small pilot subset before larger parallel implementation.

`SEMANTIC_READY` is not equivalent to implemented software. For `PKG-02`, the
next product-development work is translation of existing semantic/document
artifacts into actual schemas, contracts, modules, tests, fixtures, and evidence
where authorized by a sealed brief. It is not a rerun of four-document
initialization unless later review explicitly requests that.

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
  DAGApprovalRecord: execution/_DAG/DAG-001/APPROVAL_RECORD.md
  CoordinationMode: FULL_GRAPH
  BlockerComputation: ENABLED_ACTIVE_EDGES_ONLY
  ScopeChangeID: SCA-001

CustomInstructions:
  - Read `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, and existing production artifacts before editing.
  - Apply only applicable `AB-00-*` constraints; do not copy full `PKG-00` prose.
  - Treat protected standards/code data as out of scope.
  - Unknown engineering values remain `TBD`.
  - Do not claim certification, approval, sealing, or code compliance for reliance.
  - Do not edit files outside this deliverable folder unless explicitly authorized in the sealed brief.
  - Do not recompute or mutate blocker queues unless explicitly assigned.

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
  - No edits outside the sealed write scope.
  - No lifecycle state transition unless explicitly authorized by the human.
  - No local dependency-register edits unless explicitly assigned.
```

## Next Session Starter Prompt

```markdown
Continue as ORCHESTRATOR for the OpenPipeStress DEV-001 control plane.

Read:
1. INIT.md
2. AGENTS.md
3. agents/AGENT_ORCHESTRATOR.md
4. agents/AGENT_WORKING_ITEMS.md
5. agents/AGENT_RECONCILIATION.md
6. agents/AGENT_AUDIT_DEP_CLOSURE.md
7. agents/AGENT_CHANGE.md
8. docs/CONTRACT.md
9. docs/IP_AND_DATA_BOUNDARY.md
10. docs/_Decomposition/SOFTWARE_DECOMP.md
11. execution/_Coordination/_COORDINATION.md
12. execution/_Coordination/NEXT_INSTANCE_PROMPT.md
13. execution/_Coordination/NEXT_INSTANCE_STATE.md
14. execution/_DAG/DAG-001/APPROVAL_RECORD.md
15. execution/_DAG/DAG-001/DAG_Audit.md
16. execution/_DAG/DAG-001/Cycle_Report.md
17. execution/_DAG/DAG-001/TopologicalWaves.md
18. execution/_DAG/DAG-001/DependencyEdges.csv
19. execution/_Coordination/DEV-001_BLOCKER_QUEUE.md
20. execution/_Coordination/DEV-001_PILOT_DISPATCH_DEL-01-01.md
21. plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md

Objective:
Confirm the active control state and ask for the next human gate:
pilot launch, reconciliation, aggregate-DAG audit wrapper, or pause.

Do not launch WORKING_ITEMS, dispatch TASK, edit deliverable-local dependency
registers, change lifecycle state, or implement product code unless the human
explicitly approves that next action.
```

## Non-Goals

This plan does not:

- authorize immediate product-code implementation;
- mark `PKG-00` or any deliverable as `ISSUED`;
- promote candidate edges;
- mutate deliverable-local dependency registers;
- bulk-edit production documents;
- resolve protected engineering values or code-specific data;
- replace human review or professional judgment.
