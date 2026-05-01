# NEXT INSTANCE STATE

**Last Updated:** 2026-05-01
**Actor:** RECONCILIATION / AUDIT_DEP_CLOSURE bounded completed-archive audit
**Current Decomposition:** `docs/_Decomposition/SOFTWARE_DECOMP.md` revision `0.4`
**Current Mode:** Completed-archive reconciliation run written; awaiting CHANGE handling

## Active Control State

| Surface | Current state |
|---|---|
| Coordination mode | `FULL_GRAPH` |
| Accepted graph | `execution/_DAG/DAG-001/` |
| Graph approval | `execution/_DAG/DAG-001/APPROVAL_RECORD.md` |
| Active graph authority | Aggregate `DAG-001` `DependencyEdges.csv` |
| Blocker computation | Enabled from approved `ACTIVE` DAG edges only; implementation-readiness semantics |
| Candidate edges | Retained as non-gating pending `RECONCILIATION` |
| Semantic context threshold | `SEMANTIC_READY` |
| Implementation blocker threshold | `COMMITTED` evidence in `execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv` |
| Blocker queue | `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md` / `.csv` |
| Selected pilot | `DEL-01-01 - Project governance baseline` |
| DEV-001 hardening acceptance | Granted in-session by human project authority on 2026-04-30 |
| Pilot status | Launched and completed as a bounded governance-file patch |
| Pilot commit | `7650cf6 docs: tighten maintainer governance gates` |
| Pilot pattern | Accepted and used for `DEL-02-01`; future items still require explicit one-item gates |
| Latest state task | `DEV001 completed archive reconciliation` |
| Latest state commit | Uncommitted; route through `CHANGE` if accepted |
| Previous completed task archive status | `DEL-03-08` moved into the compact task archive table |
| Current authorized item | `RECONCILIATION` with `AUDIT_DEP_CLOSURE` over completed archive product deliverables was authorized and completed |
| Current dispatch brief | `execution/_Reconciliation/DepClosure/CLOSURE_DEV001_COMPLETED_ARCHIVE_RECONCILIATION_2026-04-30_2220/Brief.md` |
| Root next-session prompt posture | Stable bootstrap; delegate current objective discovery to coordination state and latest human gate |
| Next-instance prompt posture | Stable protocol; derive current objective from this file, `_COORDINATION.md`, `DAG-001`, current implementation-readiness queue/evidence, and the latest human gate |

## DAG Evidence

| Fact | State |
|---|---:|
| Deliverable nodes in `DAG-001` | 73 |
| Active edges | 615 |
| Candidate edges | 9 |
| Active-edge cycle status | ACYCLIC |
| Topological waves | 12 |
| Schema validation | `tools/validation/validate_dependencies_schema.py` passes on `DependencyEdges.csv` |

Derived DEV-001 implementation projection, when needed:

| Projection rule | Result |
|---|---:|
| Exclude `PKG-00` nodes and `ARCHITECTURE_BASIS` edges | 65 nodes / 227 active edges |
| Projection cycle status | ACYCLIC |
| Projection waves | 11 |

This projection is a coordination view only. It does not replace `DAG-001` and
does not remove `SCA-001` / `AB-00-*` architecture-basis injection from sealed
briefs.

## PKG-00 Ruling

`PKG-00` was processed through `SCOPE_CHANGE` as prerequisite architecture
context. It remains `SEMANTIC_READY`, not `ISSUED`. Its architecture-basis
content is injected into downstream sealed briefs through applicable `AB-00-*`
rows and the resolved architecture baseline.

`PKG-00` may be excluded from implementation graph participation and does not
require deliverable-local `Dependencies.csv` files.

## Local Dependency Register Status

Non-`PKG-00` deliverable-local `Dependencies.csv` files are synchronized
mirrors/evidence materialized from `DAG-001`, not independent sequencing
authority. `PKG-00` remains architecture context only and does not receive local
dependency registers.

Post-materialization evidence:

- `execution/_DAG/DAG-001/evidence/dev001_local_materialization_summary.json`
  records 65 non-`PKG-00` local registers written from `DAG-001`.
- `execution/_Reconciliation/DepClosure/CLOSURE_DEV001_POST_MATERIALIZATION_2026-04-30/RUN_SUMMARY.md`
  records 65 schema-valid local registers, 624 local rows, 0 active SCCs, and 0
  bidirectional active pairs.
- `execution/_Reconciliation/Reconciliation_Run_Summary_2026-04-30_DEV001_CONTROL_PLANE_HARDENING.md`
  records the authority boundary: aggregate `DAG-001` remains sequencing
  authority, local registers remain synchronized mirrors, and `CANDIDATE` edges
  remain non-gating.

The older pre-DAG reconciliation artifacts are still present as untracked local
evidence and should not be treated as current sequencing authority.

## Current Blocker Queue

`execution/_Coordination/DEV-001_BLOCKER_QUEUE.md` was refreshed on
2026-05-01 after `DEL-03-08` implementation evidence was added. It reads approved active
`DAG-001` edges and `execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv`.
`FromDeliverableID` is treated as the downstream consumer blocked by
`TargetDeliverableID`, the upstream provider.

Semantic readiness answers whether task context is prepared. Implementation
readiness answers whether a consumer can safely rely on committed upstream
artifacts. `SEMANTIC_READY` no longer satisfies DEV-001 implementation
blockers by itself.

| Queue fact | Count |
|---|---:|
| Filesystem lifecycle `SEMANTIC_READY` (display only) | 73 |
| Implementation evidence records | 17 |
| Committed implementation evidence | 17 |
| PKG-00 architecture-basis edges satisfied by baseline | 388 |
| Implementation `UNBLOCKED` deliverables | 33 |
| Implementation `BLOCKED` deliverables | 40 |
| Candidate edges used | 0 |

The queue now contains blockers for consumers whose upstream providers do not
yet have `COMMITTED` implementation evidence. It is not a lifecycle approval,
schedule, priority, staffing decision, implementation completeness claim, or
professional approval.

## Completed Task History (Compacted)

This archive compactly records completed tasks before the latest state summary.
After each task completes, move the previous latest task here before replacing
the latest-state summary. Historical details remain recoverable from named
dispatch briefs, deliverable `MEMORY.md` files, commits, and git history.

Universal historical guardrails preserved across the completed bounded items:

- No broad fan-out was started.
- No lifecycle state transition was made.
- No blocker queue refresh was run unless explicitly authorized or a committed
  implementation-evidence change made the queue stale.
- No `DAG-001`, candidate-edge, `Dependencies.csv`, or `_DEPENDENCIES.md`
  mutation occurred during product-deliverable execution.
- No protected standards text, protected tables, proprietary engineering
  values, private data, or automatic code-compliance/certification/sealing
  claims were introduced.
- `CANDIDATE` edges remained non-gating.

| Task | Result / commit | Primary changed surfaces | Verification summary | Remaining notable TBDs |
|---|---|---|---|---|
| `DEL-01-01` Project governance baseline | Completed pilot; `7650cf6 docs: tighten maintainer governance gates` | `governance/MAINTAINERS.md` plus control-plane handoff | `git diff --check`; focused prohibited-claim scan; evidence-slot scan | Maintainer/legal/release governance details remain governed by later human rulings. |
| `DEL-02-01` Canonical domain model schema | Completed; `7b256f3 schema: tighten canonical domain model contract`; handoff `8f57f85` | `schemas/model.schema.yaml`, `docs/TYPES.md`, model schema tests, dispatch/state/memory | Model, units, analysis status, persistence, analysis boundary, plugin manifest tests; `git diff --check`; forbidden-schema-text scan | `C-02-01-001`, `C-02-01-002`, physical package/container and migration framework. |
| `DEL-02-02` Unit system and dimensional-analysis core contract | Completed; `a458cba schema: tighten unit system contract`; handoff `ce94de3` | `schemas/units.schema.yaml`, `core/units/README.md`, `docs/SPEC.md`, unit tests, dispatch/state/memory | Unit and adjacent schema tests; `git diff --check`; focused forbidden-claim/protected-code-name scan | Unit catalog, conversion source set, dimensions, tolerance, offset/gauge semantics, angle/rotation treatment remain `TBD` or decision-gated. |
| `DEL-02-03` Code-neutral analysis boundary model | Completed; handoff `f19cf2a` | `schemas/analysis_boundary.schema.yaml`, `docs/architecture/code_neutral_analysis_boundary.md`, `docs/SPEC.md`, `docs/TYPES.md`, tests, dispatch/state/memory | Analysis boundary/status and adjacent schema tests; `git diff --check`; focused forbidden-claim scan | Downstream semantics remain bounded by future consumers and human rulings. |
| `DEL-02-05` Project persistence and round-trip serialization | Completed; handoff `4e18a0f` | Persistence schema/tests and related docs/control-plane evidence | Persistence and adjacent schema tests; `git diff --check`; protected/claim scans | Physical project package/container and migration framework remain `TBD`. |
| `DEL-02-04` Plugin and extension domain contracts | Completed; handoff `a37a0a1` | Plugin manifest schema/tests and related docs/control-plane evidence | Plugin manifest and adjacent schema tests; `git diff --check`; protected/claim scans | Public transport, concrete extension API, and permissions details remain `TBD`. |
| `DEL-01-02` Copyright and protected-data boundary policy | Completed; `0d729cf docs: tighten protected data boundary` | IP/data boundary policy and related governance surfaces | Diff/prohibited-claim checks and policy evidence review | Legal review authority and final contribution-review workflow details remain governed separately. |
| `DEL-01-04` Professional responsibility and product-claims policy | Completed; `65f3119 docs: add professional boundary policy`; handoffs `1a996ac`, `474b56d` | Professional-boundary docs/policy surfaces and control-plane handoff | Diff/prohibited-claim checks and boundary wording review | Final release/product-claims wording remains human-governed. |
| `DEL-01-03` Contributor certification workflow | Completed; `df461f8 docs: add contributor certification workflow` | Contributor governance/certification surfaces and handoff | Diff/prohibited-claim checks and contribution-field evidence review | Final open-source license, contributor legal mechanism, maintainer roster, quorum, legal-review authority, and release policy remain `TBD`. |
| `DEL-03-01` Material library schema with provenance | Completed; `3793e87 schema: add material library provenance contract`; handoff `f749a1c` | Material schema, fixture, tests, docs, dispatch/state/memory | Material and adjacent schema tests; `git diff --check`; protected-data/claim scans | Public material source catalogs, fixture policy, import formats, and editor behavior remain `TBD`. |
| `DEL-03-02` Pipe section and component library schema | Completed; `f0fdeac schema: add section and component library contracts` | Section/component schemas, invented fixture, tests, docs, dispatch/state/memory | Component/section and adjacent schema tests; `git diff --check`; protected-data/claim scans | Public section/component catalogs, fixture policy, section-property policy, import formats, and editor behavior remain `TBD`. |
| `DEL-03-03` Bend and elbow component model fields | Completed; `7a84472 schema: add bend elbow component contract`; handoff/push `753b096` | Component schema/fixture/tests, `docs/SPEC.md`, `docs/TYPES.md`, dispatch/state/memory | JSON checks, component/section and adjacent schema tests; `git diff --check`; focused protected-content/claim scan | Bend/elbow source catalogs, fixture policy, solver use of flexibility inputs, import formats, and editor behavior remain `TBD`. |
| `DEL-03-04` Branch connection component model fields | Completed; `ae693b6 schema: add branch component contract` | Component schema/fixture/tests, `docs/SPEC.md`, `docs/TYPES.md`, dispatch/state/memory | JSON checks, component/section and adjacent schema tests; `git diff --check`; focused protected-content/claim scan | Branch source catalogs, fixture policy, solver use of branch flexibility inputs, import formats, and editor behavior remain `TBD`. |
| `DEL-03-05` Rigid component models for valves, flanges, reducers, and specialty items | Completed; `d8ee0db schema: add rigid component contract`; handoff `fd695c0 docs: record del-03-05 handoff state` | Component schema/fixture/tests, `docs/SPEC.md`, `docs/TYPES.md`, dispatch/state/memory | JSON checks, component/section and adjacent schema tests; `git diff --check`; focused protected-content/claim scan | Rigid component source catalogs, fixture policy, semi-rigid stiffness treatment, import formats, and editor behavior remain `TBD`. |
| `DEL-03-06` Expansion joint component model | Completed; `f15cbc6 schema: add expansion joint component contract` | Component schema/fixture/tests, `docs/SPEC.md`, `docs/TYPES.md`, dispatch/state/memory | JSON checks, component/section and adjacent schema tests; `git diff --check`; focused protected-content/claim scan | Expansion-joint source catalogs, fixture policy, stiffness mapping, hardware taxonomy, import formats, and editor behavior remain `TBD`. |
| `DEL-03-07` Public/private library import provenance checker | Completed; `4d880b3 core: add library import provenance checker` | `core/library_import/`, `tests/test_library_import_provenance.py`, `docs/SPEC.md`, `docs/TYPES.md`, dispatch/state/memory | Library import provenance test; adjacent schema tests; `git diff --check`; focused forbidden-claim/protected-content scan | Concrete external import formats, legal/license interpretation, UI presentation, and downstream adapter integration remain future work. |
| `DEL-03-08` Pipe section property and mass-property calculator | Completed; `9712e98 core: add pipe section property calculator`; evidence/handoff `4a68ada coordination: record del-03-08 implementation evidence` | `core/section_properties/`, section schema/tests, `docs/SPEC.md`, `docs/TYPES.md`, dispatch/state/memory/evidence/queue | Section property, component-section, library import, and full test sweep passed; `git diff --check`; protected-content/claim scan | Unit catalog/conversion constants, public pipe section source catalogs, physical project package/container, GUI/editor presentation, and solver consumption policy remain `TBD`. |
| `DEV-001` implementation-readiness queue/bootstrap refresh | Completed; `cf1bf12 coordination: add implementation readiness queue` | `DEV-001_IMPLEMENTATION_EVIDENCE.csv`, `DEV-001_BLOCKER_QUEUE.*`, queue builder/tests, coordination prompts/state, tool registry | `pytest tools/coordination`; dependency schema validation; `audit_dag.py --strict`; `git diff --check`; queue contained 40 blocked implementation items | Future queue refreshes must be driven by DAG/evidence changes; no next product deliverable is authorized by this archive row. |
| `NEXT_INSTANCE_STATE` rotating handoff documentation | Completed; `817f677 docs: clarify next instance handoff rotation` | `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`, `execution/_Coordination/NEXT_INSTANCE_STATE.md`, `execution/_Coordination/_COORDINATION.md`, `init/NEXT_SESSION_PROMPT.md` | `git diff --check`; committed cleanly | No next product deliverable was authorized by the documentation update. |

## Bootstrap and Next-Instance Prompt Posture

Human project authority accepted objective-neutral bootstrap/control-loop posture:

- `NEXT_INSTANCE_PROMPT.md` is stable control-loop protocol. Agents derive the
  current objective from `NEXT_INSTANCE_STATE.md`, `_COORDINATION.md`, accepted
  `DAG-001` artifacts, current implementation-readiness queue/evidence, and the
  latest human approval gate.
- `init/NEXT_SESSION_PROMPT.md` is a stable bootstrap entrypoint. It routes
  fresh sessions into the coordination protocol and mutable handoff state, not
  a hard-coded next deliverable objective.

2026-05-01 clarification:

- `init/NEXT_SESSION_PROMPT.md` now states DEV-001 dependency direction and
  readiness rules directly.
- It explicitly separates semantic readiness (`SEMANTIC_READY` context
  readiness) from implementation readiness (`COMMITTED` upstream evidence).
- It removes ambiguous wording around "current blocker evidence" and states
  that queue refresh is driven by DAG/evidence changes.

## Latest State - DEV001 Completed Archive Reconciliation

Human project authority authorized one RECONCILIATION cycle using
`AUDIT_DEP_CLOSURE` over the completed product deliverables listed in the
compact archive table.

Files changed in this task:

- `execution/_Coordination/NEXT_INSTANCE_STATE.md`
- `execution/_Reconciliation/_LATEST.md`
- `execution/_Reconciliation/Reconciliation_Run_Summary_2026-05-01_DEV001_COMPLETED_ARCHIVE_RECONCILIATION.md`
- `execution/_Reconciliation/DepClosure/_LATEST.md`
- `execution/_Reconciliation/DepClosure/CLOSURE_DEV001_COMPLETED_ARCHIVE_RECONCILIATION_2026-04-30_2220/Brief.md`
- `execution/_Reconciliation/DepClosure/CLOSURE_DEV001_COMPLETED_ARCHIVE_RECONCILIATION_2026-04-30_2220/RUN_SUMMARY.md`
- `execution/_Reconciliation/DepClosure/CLOSURE_DEV001_COMPLETED_ARCHIVE_RECONCILIATION_2026-04-30_2220/QA_Report.md`
- `execution/_Reconciliation/DepClosure/CLOSURE_DEV001_COMPLETED_ARCHIVE_RECONCILIATION_2026-04-30_2220/Decision_Log.md`
- `execution/_Reconciliation/DepClosure/CLOSURE_DEV001_COMPLETED_ARCHIVE_RECONCILIATION_2026-04-30_2220/Dependency_Closure_Report.md`
- `execution/_Reconciliation/DepClosure/CLOSURE_DEV001_COMPLETED_ARCHIVE_RECONCILIATION_2026-04-30_2220/Dependency_Closure_IssueLog.csv`
- `execution/_Reconciliation/DepClosure/CLOSURE_DEV001_COMPLETED_ARCHIVE_RECONCILIATION_2026-04-30_2220/Evidence/closure_summary.json`
- `execution/_Reconciliation/DepClosure/CLOSURE_DEV001_COMPLETED_ARCHIVE_RECONCILIATION_2026-04-30_2220/Evidence/coverage.csv`
- `execution/_Reconciliation/DepClosure/CLOSURE_DEV001_COMPLETED_ARCHIVE_RECONCILIATION_2026-04-30_2220/Evidence/orphans.csv`
- `execution/_Reconciliation/DepClosure/CLOSURE_DEV001_COMPLETED_ARCHIVE_RECONCILIATION_2026-04-30_2220/Evidence/scc_summary.csv`
- `execution/_Reconciliation/DepClosure/CLOSURE_DEV001_COMPLETED_ARCHIVE_RECONCILIATION_2026-04-30_2220/Evidence/hubs.csv`
- `execution/_Reconciliation/DepClosure/CLOSURE_DEV001_COMPLETED_ARCHIVE_RECONCILIATION_2026-04-30_2220/Evidence/bidirectional_pairs.csv`
- `execution/_Reconciliation/DepClosure/CLOSURE_DEV001_COMPLETED_ARCHIVE_RECONCILIATION_2026-04-30_2220/Evidence/id_normalization.csv`

Audit summary:

- Scoped product deliverables audited: 16.
- Local `Dependencies.csv` files found: 16.
- Dependency rows loaded: 130.
- Schema-valid local registers: 16.
- Schema-invalid local registers: 0.
- Evidence populated: 130 / 130.
- Orphan deliverables: 0.
- Active SCCs larger than one node: 0.
- Bidirectional active pairs: 0.
- ID normalizations: 0.

Known informational finding:

- The analyzer reported missing `IMPLEMENTS_NODE` anchors for all 16 scoped
  local registers. This remains informational under the accepted DEV-001 policy:
  local registers are synchronized mirrors/evidence and aggregate `DAG-001`
  remains the sequencing authority.

Remaining open items:

- Reconciliation artifacts and this handoff update are uncommitted until routed
  through `CHANGE`.
- No product deliverable, lifecycle transition, candidate-edge promotion,
  dependency-register mutation, or blocker-queue refresh was authorized or
  performed by this reconciliation run.

## Immediate Next Actions

Immediate next action:

1. Route the completed-archive reconciliation artifacts and
   `NEXT_INSTANCE_STATE.md` handoff update through `CHANGE` for staging and
   commit if the human project authority accepts them.
2. Human project authority may then route another `RECONCILIATION`, `AUDIT_*`,
   artifact handling, authorize exactly one next bounded DAG item, route
   `CHANGE` for file-state handling, or pause.

Do not start broad DAG execution. No additional DAG item is currently
authorized by this reconciliation run.

## Guardrails

- No new `WORKING_ITEMS` launch until the human approves the next gate.
- No `TASK` dispatch until within a sealed, approved deliverable session.
- No product implementation outside an approved deliverable scope.
- No lifecycle transitions unless explicitly authorized.
- No protected standards/code data, private project data, real secrets, private
  libraries, or certification/compliance claims.
- No silent edits to deliverable-local `Dependencies.csv`.
