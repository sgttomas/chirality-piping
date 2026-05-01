# NEXT INSTANCE STATE

**Last Updated:** 2026-05-01
**Actor:** ORCHESTRATOR / WORKING_ITEMS DEL-03-06 bounded item closeout
**Current Decomposition:** `docs/_Decomposition/SOFTWARE_DECOMP.md` revision `0.4`
**Current Mode:** DEV-001 `DEL-03-06` bounded item completed and awaiting CHANGE file-state approval; no broad fan-out

## Active Control State

| Surface | Current state |
|---|---|
| Coordination mode | `FULL_GRAPH` |
| Accepted graph | `execution/_DAG/DAG-001/` |
| Graph approval | `execution/_DAG/DAG-001/APPROVAL_RECORD.md` |
| Active graph authority | Aggregate `DAG-001` `DependencyEdges.csv` |
| Blocker computation | Enabled from approved `ACTIVE` DAG edges only |
| Candidate edges | Retained as non-gating pending `RECONCILIATION` |
| Maturity threshold | `SEMANTIC_READY` |
| Blocker queue | `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md` / `.csv` |
| Selected pilot | `DEL-01-01 - Project governance baseline` |
| DEV-001 hardening acceptance | Granted in-session by human project authority on 2026-04-30 |
| Pilot status | Launched and completed as a bounded governance-file patch |
| Pilot commit | `7650cf6 docs: tighten maintainer governance gates` |
| Pilot pattern | Accepted and used for `DEL-02-01`; future items still require explicit one-item gates |
| Earlier bounded item | `DEL-01-02 - Copyright and protected-data boundary policy` |
| Earlier bounded item commit | `0d729cf docs: tighten protected data boundary` |
| DEL-02-01 handoff correction commit | `8f57f85 docs: record del-02-01 commit handoff` |
| DEL-02-02 handoff correction commit | `ce94de3 docs: record del-02-02 commit handoff` |
| DEL-02-03 handoff correction commit | `f19cf2a docs: record del-02-03 commit handoff` |
| DEL-02-05 handoff correction commit | `4e18a0f docs: record del-02-05 commit handoff` |
| DEL-02-04 handoff correction commit | `a37a0a1 docs: record del-02-04 commit handoff` |
| Previous governance item | `DEL-01-04 - Professional responsibility and product-claims policy` |
| Previous governance item commit | `65f3119 docs: add professional boundary policy` |
| DEL-01-04 handoff correction commit | `1a996ac docs: record del-01-04 commit handoff` |
| DEL-01-04 clean handoff state commit | `474b56d docs: record del-01-04 clean handoff state` |
| Prior bounded item | `DEL-01-03 - Contributor certification workflow` |
| Prior bounded item commit | `df461f8 docs: add contributor certification workflow` |
| Previous bounded item | `DEL-03-01 - Material library schema with provenance` |
| Previous bounded item commit | `3793e87 schema: add material library provenance contract` |
| DEL-03-01 handoff correction commit | `f749a1c docs: record del-03-01 commit handoff` |
| Previous bounded item | `DEL-03-02 - Pipe section and component library schema` |
| Previous bounded item commit | `f0fdeac schema: add section and component library contracts` |
| Previous bounded item | `DEL-03-03 - Bend and elbow component model fields` |
| Previous bounded item commit | `7a84472 schema: add bend elbow component contract` |
| DEL-03-03 handoff correction commit | `753b096 docs: record del-03-03 commit handoff` |
| DEL-03-03 pushed state | `main` pushed to `origin/main` through CHANGE approval on 2026-05-01 |
| Previous bounded item | `DEL-03-04 - Branch connection component model fields` |
| Previous bounded item commit | `ae693b6 schema: add branch component contract` |
| Previous bounded item | `DEL-03-05 - Rigid component models for valves, flanges, reducers, and specialty items` |
| Previous bounded item commit | `d8ee0db schema: add rigid component contract` |
| DEL-03-05 handoff correction commit | `fd695c0 docs: record del-03-05 handoff state` |
| Last bounded item | `DEL-03-06 - Expansion joint component model` |
| Last bounded item commit | `Pending CHANGE approval` |
| Current authorized item | `None beyond DEL-03-06 closeout / CHANGE handling` |
| Current dispatch brief | `execution/_Coordination/DEV-001_DISPATCH_DEL-03-06.md` |
| Root next-session prompt posture | Stable bootstrap; delegate current objective discovery to coordination state and latest human gate |
| Next-instance prompt posture | Stable protocol; derive current objective from this file, `_COORDINATION.md`, `DAG-001`, current blocker evidence, and the latest human gate |

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

`execution/_Coordination/DEV-001_BLOCKER_QUEUE.md` was refreshed from current
filesystem `_STATUS.md` files and approved active `DAG-001` edges after the
`DEL-01-01` pilot closeout. It records:

| Queue fact | Count |
|---|---:|
| Filesystem lifecycle `SEMANTIC_READY` | 73 |
| Advisory `UNBLOCKED` deliverables | 73 |
| Advisory `BLOCKED` deliverables | 0 |
| Candidate edges used | 0 |

All deliverables currently meet the `SEMANTIC_READY` maturity threshold used by
the advisory blocker queue. This means there are no active-DAG maturity
blockers under the current threshold. It is not a lifecycle approval, schedule,
priority, staffing decision, implementation completeness claim, or professional
approval.

## Completed Bounded Item History (Compacted)

This section compactly records completed DEV-001 bounded items before the most
recent active closeout. Historical details remain recoverable from the named
dispatch briefs, deliverable `MEMORY.md` files, commits, and git history.

Universal historical guardrails preserved across the completed bounded items:

- No broad fan-out was started.
- No lifecycle state transition was made.
- No blocker queue refresh was run unless explicitly authorized.
- No `DAG-001`, candidate-edge, `Dependencies.csv`, or `_DEPENDENCIES.md`
  mutation occurred during product-deliverable execution.
- No protected standards text, protected tables, proprietary engineering
  values, private data, or automatic code-compliance/certification/sealing
  claims were introduced.
- `CANDIDATE` edges remained non-gating.

| Deliverable | Result / commit | Primary changed surfaces | Verification summary | Remaining notable TBDs |
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

## Bootstrap and Next-Instance Prompt Posture

Human project authority accepted objective-neutral bootstrap/control-loop posture:

- `NEXT_INSTANCE_PROMPT.md` is stable control-loop protocol. Agents derive the
  current objective from `NEXT_INSTANCE_STATE.md`, `_COORDINATION.md`, accepted
  `DAG-001` artifacts, current blocker evidence when explicitly current, and
  the latest human approval gate.
- `init/NEXT_SESSION_PROMPT.md` is a stable bootstrap entrypoint. It routes
  fresh sessions into the coordination protocol and mutable handoff state, not
  a hard-coded next deliverable objective.

## DEL-03-06 Bounded Item Closeout

Human project authority authorized exactly one bounded DAG item:
`DEL-03-06 - Expansion joint component model`. ORCHESTRATOR / WORKING_ITEMS
completed the bounded item inside the explicit write scope. Broad fan-out,
lifecycle transition, candidate-edge promotion, blocker-queue refresh, and
dependency-register edits were not performed.

Dispatch evidence:

- Fresh sealed dispatch brief:
  `execution/_Coordination/DEV-001_DISPATCH_DEL-03-06.md`.
- Active upstream dependencies were consumed from approved `DAG-001` active
  rows: `DEL-00-01`, `DEL-00-02`, `DEL-00-04`, `DEL-00-06`, `DEL-00-07`,
  `DEL-00-08`, `DEL-03-02`, `DEL-02-02`, and `DEL-01-02`.
- `CANDIDATE` rows were not promoted or used as gates.

Files changed in this bounded item:

- `schemas/component.schema.yaml`
- `fixtures/component/invented_section_component_library_valid.json`
- `tests/test_component_section_schema.py`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-06_Expansion joint component model/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-03-06.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

Verification run:

- `python3 -m json.tool schemas/component.schema.yaml` passed.
- `python3 -m json.tool fixtures/component/invented_section_component_library_valid.json` passed.
- `python3 tests/test_component_section_schema.py` passed.
- Existing schema tests passed: `test_material_schema.py`,
  `test_model_schema.py`, `test_units_schema.py`, `test_persistence_schema.py`,
  `test_plugin_manifest_schema.py`, `test_analysis_status_schema.py`, and
  `test_analysis_boundary_schema.py`.
- `git diff --check` passed.
- Focused forbidden-claim/protected-content scan over affected DEL-03-06
  product surfaces found only existing negative boundary statements and
  test-denylist literals, not product certification/compliance claims or
  protected/manufacturer expansion-joint data.

Guardrails preserved:

- No lifecycle state transition was made.
- No blocker queue refresh was run.
- No `DAG-001`, candidate-edge, `Dependencies.csv`, or `_DEPENDENCIES.md`
  mutation occurred.
- No protected standards text, protected dimensional/rating tables,
  proprietary engineering values, private data, manufacturer catalog values, or
  automatic code-compliance/certification/sealing claims were introduced.

Remaining open items:

- Accepted public expansion-joint source catalogs remain `TBD`.
- Public expansion-joint fixture value policy remains `TBD`.
- Stiffness degree-of-freedom mapping and exact solver consumption remain
  `TBD`.
- Hardware flag/enumeration taxonomy remains `TBD`.
- Concrete expansion-joint import formats remain `TBD`.
- Downstream component editor behavior remains future GUI work.
- Deliverable file-state changes are awaiting CHANGE approval.

## Immediate Next Actions

Immediate next action:

1. Route `DEL-03-06` file-state handling through `CHANGE`; do not stage or
   commit without an explicit `APPROVE:` action list.
2. Human project authority may route `RECONCILIATION`, `AUDIT_*`, pre-DAG
   artifact handling if it appears in file-state evidence, authorize exactly one
   next bounded DAG item, route `CHANGE` for push or other file-state handling,
   or pause.

Do not start broad DAG execution. No additional DAG item is currently
authorized beyond DEL-03-06 closeout / CHANGE handling.

## Guardrails

- No new `WORKING_ITEMS` launch until the human approves the next gate.
- No `TASK` dispatch until within a sealed, approved deliverable session.
- No product implementation outside an approved deliverable scope.
- No lifecycle transitions unless explicitly authorized.
- No protected standards/code data, private project data, real secrets, private
  libraries, or certification/compliance claims.
- No silent edits to deliverable-local `Dependencies.csv`.
