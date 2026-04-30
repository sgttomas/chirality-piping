---
doc_id: DEV-001-PILOT-DISPATCH-DEL-01-01
doc_kind: coordination.pilot_dispatch_brief
status: ready_for_orchestrator_handoff_to_working_items
created: 2026-04-30
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-01-01
package_id: PKG-01
blocker_computation: enabled_active_edges_only
write_scope: repo_level_targets_authorized
---

# DEV-001 Pilot Dispatch - DEL-01-01

## Dispatch Decision

Selected pilot: `DEL-01-01 - Project governance baseline`.

This is the first DEV-001 control-loop pilot after `DAG-001` approval. It is not broad fan-out and it does not implement product code by itself.

The human project authority authorized the repo-level write targets for this pilot on 2026-04-30. `ORCHESTRATOR` owns the control plane and dispatch gate; `WORKING_ITEMS` is the persona for actual deliverable work.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-01-01` |
| PackageID | `PKG-01` |
| Name | Project governance baseline |
| Type | `DOC_UPDATE` |
| Scope items | `SOW-001`, `SOW-048` |
| Objectives | `OBJ-001`, `OBJ-002` |
| Context envelope | `M` |
| Deliverable path | `execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-01_Project governance baseline` |
| Current lifecycle | `OPEN` |

## Approved DAG Preconditions

Active upstream dependencies from `DAG-001`:

| Target | Dependency type | Required maturity | Satisfaction |
|---|---|---|---|
| `DEL-00-01` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-02` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-06` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-08` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |

No `CANDIDATE` edge currently gates `DEL-01-01`.

## Applicable Architecture Basis

Use only the applicable `SCA-001` architecture-basis IDs from `_CONTEXT.md`:

- `AB-00-01` - ADR and decision-record discipline.
- `AB-00-02` - layer/module boundaries and no-bypass dependency rules.
- `AB-00-06` - diagnostics/result-envelope fields, warning classes, and no certification/compliance claims.
- `AB-00-08` - layered test and acceptance gates.

Resolved baseline to preserve where applicable: Rust core/application services; Tauri 2 desktop shell where GUI-facing; TypeScript/React/Vite GUI where GUI-facing; Three.js viewport where 3D viewport-facing; JSON Schema 2020-12 contracts; schema-first command/query/job result envelopes; canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.

Remaining implementation-level TBDs: exact dependency versions, solver numerical library, rule expression grammar/library, public API transport, import/export format list, CI provider/coverage thresholds, and physical project package/container.

## Applicable Contract Invariants

Minimum invariants to carry into the sealed brief:

- `OPS-K-HIER-1` - packages and deliverables remain stable production units.
- `OPS-K-ID-1` - stable IDs are not reused.
- `OPS-K-IP-1` through `OPS-K-IP-3` - protected standards/code/proprietary data stay out of public content.
- `OPS-K-AUTH-1` and `OPS-K-AUTH-2` - no certification, sealing, approval, or reliance claims by software/agents.
- `OPS-K-PRIV-1` and `OPS-K-PRIV-2` - private project/rule/material/component data is not transmitted or committed by default.
- `OPS-K-AGENT-1` through `OPS-K-AGENT-4` - no invented values, conflicts surfaced, Type 2 execution sealed, outputs draft until human accepted.

## Acceptance Criteria

The pilot is acceptable when:

- one `WORKING_ITEMS` session is scoped to `DEL-01-01`;
- at most one bounded `TASK` is dispatched from that session;
- the selected write scope is explicit before any edits;
- the run preserves all applicable contract invariants and `AB-00-*` constraints;
- unknown policy decisions remain `TBD` or are recorded as human rulings;
- no protected standards text, tables, proprietary values, or code-compliance claims are introduced;
- no lifecycle state transition occurs unless explicitly authorized by the human;
- no blocker queue, blocked/unblocked list, schedule, staffing plan, or readiness queue is computed.

## Write-Scope Question

Status: resolved by human ruling on 2026-04-30.

`DEL-01-01` anticipated artifacts are repo-level governance files:

- `docs/CONTRACT.md`
- `docs/DIRECTIVE.md`
- `governance/MAINTAINERS.md`

The default `DELIVERABLE_TASK` profile is deliverable-local, so repo-level edits must be carried by the `WORKING_ITEMS` session or by a separately sealed task/profile that explicitly allows the following write targets:

- `docs/CONTRACT.md`
- `docs/DIRECTIVE.md`
- `governance/MAINTAINERS.md`

Recommended pilot path:

1. Start with a `WORKING_ITEMS` session for `DEL-01-01`.
2. Use the authorized repo-level write targets above.
3. Dispatch at most one bounded `TASK` from `WORKING_ITEMS`, only if the task brief explicitly preserves the authorized write scope and guardrails.

## WORKING_ITEMS Starter

```markdown
Continue as WORKING_ITEMS for the OpenPipeStress DEV-001 pilot.

Scope deliverable:
`execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-01_Project governance baseline`

Objective:
Run the `DEL-01-01` project governance baseline pilot using the human-authorized repo-level write targets while preserving control-plane guardrails.

Use:
- `execution/_DAG/DAG-001/APPROVAL_RECORD.md`
- `execution/_DAG/DAG-001/DependencyEdges.csv`
- `execution/_Coordination/DEV-001_PILOT_DISPATCH_DEL-01-01.md`
- `docs/CONTRACT.md`
- `docs/_Registers/Deliverables.csv`
- deliverable `_CONTEXT.md`, `_STATUS.md`, `_REFERENCES.md`, `_DEPENDENCIES.md`, and `_SEMANTIC.md`

Constraints:
- at most one bounded TASK;
- no lifecycle transition;
- no protected standards/code data;
- no certification, sealing, approval, or code-compliance claims.
- blocker computation may be consumed only from the ORCHESTRATOR-generated approved-active-DAG queue; do not recompute or alter it inside WORKING_ITEMS unless explicitly assigned.
```

## Optional TASK Brief Draft

Use this only if `WORKING_ITEMS` chooses to delegate a bounded planning/checking task. Repo-level product edits should remain under the explicitly authorized `WORKING_ITEMS` write scope unless a narrower task profile is approved.

```markdown
PURPOSE: Prepare or check the DEL-01-01 governance baseline implementation patch plan inside the approved pilot scope.
RequestedBy: WORKING_ITEMS

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-01_Project governance baseline
DeliverablePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-01_Project governance baseline
TaskProfile: DELIVERABLE_TASK

DeliverableID: DEL-01-01
PackageID: PKG-01

Tasks:
  - Read the DEL-01-01 local truth set and the approved DAG approval record.
  - Inventory existing repo-level target files needed by this deliverable.
  - Produce a deliverable-local patch plan or consistency check.
  - Identify any remaining human rulings needed before or after repo-level governance file edits.

ApplyEdits: true
AdditionalTargetsToEdit:
  - DEV-001_DEL-01-01_PATCH_PLAN.md

RuntimeOverrides:
  DecompositionPath: docs/_Decomposition/SOFTWARE_DECOMP.md
  DecompositionRevision: "0.4"
  DAGPath: execution/_DAG/DAG-001/DependencyEdges.csv
  DAGApprovalRecord: execution/_DAG/DAG-001/APPROVAL_RECORD.md
  CoordinationMode: FULL_GRAPH
  BlockerComputation: ENABLED_ACTIVE_EDGES_ONLY
  ScopeChangeID: SCA-001

CustomInstructions:
  - Do not edit repo-level product artifacts from this optional TASK unless the task brief is revised to include an explicit non-deliverable-local write scope.
  - Apply only applicable AB-00-01, AB-00-02, AB-00-06, and AB-00-08 constraints.
  - Treat protected standards/code data as out of scope.
  - Unknown engineering or governance decisions remain TBD.
  - Do not claim certification, approval, sealing, or code compliance for reliance.

ExpectedOutputs:
  - DEV-001_DEL-01-01_PATCH_PLAN.md inside the deliverable folder.
  - TASK run record under _run_records/.
  - Open issue list for unresolved TBD, assumptions, or required human rulings.

EXCLUSIONS:
  - No protected standards text, tables, examples, or proprietary engineering values.
  - No edits outside the sealed deliverable path.
  - No lifecycle state transition.
  - No independent blocked/unblocked queue computation inside TASK.
```
