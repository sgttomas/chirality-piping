---
doc_id: DEV-001-BLOCKER-QUEUE
doc_kind: coordination.blocker_queue
status: computed_active_edges_only
created: 2026-04-30
source_graph: execution/_DAG/DAG-001/DependencyEdges.csv
lifecycle_source: filesystem_STATUS_files
maturity_threshold: SEMANTIC_READY
candidate_edges: excluded
---

# DEV-001 Advisory Blocker Queue

This blocker queue is an advisory coordination view only. It is not a schedule, staffing plan, priority list, lifecycle approval, professional approval, or readiness-for-reliance claim.

## Computation Rule

- Source graph: `execution/_DAG/DAG-001/DependencyEdges.csv`.
- Included edges: `Status=ACTIVE` only.
- Excluded edges: all `Status=CANDIDATE` rows.
- Direction convention: `FromDeliverableID` depends on `TargetDeliverableID`.
- Maturity threshold: `SEMANTIC_READY`.
- Lifecycle evidence: current filesystem `_STATUS.md`; `DeliverableNodes.csv` used as inventory evidence.
- `UNBLOCKED` means all active upstream dependencies meet the threshold.
- `BLOCKED` means one or more active upstream dependencies do not meet the threshold.

## Evidence Summary

| Evidence | Count |
|---|---:|
| Packages represented | 13 |
| Deliverable nodes represented | 73 |
| Active edges included | 615 |
| Candidate edges excluded | 9 |
| Filesystem lifecycle `SEMANTIC_READY` | 13 |
| Filesystem lifecycle `OPEN` | 60 |
| Advisory `UNBLOCKED` deliverables | 17 |
| Advisory `BLOCKED` deliverables | 56 |

## Package Summary

| PackageID | UNBLOCKED | BLOCKED |
|---|---:|---:|
| `PKG-00` | 8 | 0 |
| `PKG-01` | 1 | 3 |
| `PKG-02` | 5 | 0 |
| `PKG-03` | 0 | 8 |
| `PKG-04` | 1 | 5 |
| `PKG-05` | 1 | 4 |
| `PKG-06` | 0 | 5 |
| `PKG-07` | 0 | 7 |
| `PKG-08` | 0 | 5 |
| `PKG-09` | 0 | 5 |
| `PKG-10` | 1 | 4 |
| `PKG-11` | 0 | 5 |
| `PKG-12` | 0 | 5 |

## Unblocked Deliverables

These deliverables have no active upstream dependency below `SEMANTIC_READY`. This does not grant lifecycle approval or implementation priority.

| DeliverableID | PackageID | Lifecycle | Active upstream | Name |
|---|---|---|---:|---|
| `DEL-00-01` | `PKG-00` | `SEMANTIC_READY` | 0 | Architecture decision record baseline |
| `DEL-00-02` | `PKG-00` | `SEMANTIC_READY` | 0 | Repository and module boundary architecture |
| `DEL-00-03` | `PKG-00` | `SEMANTIC_READY` | 0 | Application service command-query-job model |
| `DEL-00-04` | `PKG-00` | `SEMANTIC_READY` | 0 | Persistence and schema versioning architecture |
| `DEL-00-05` | `PKG-00` | `SEMANTIC_READY` | 0 | GUI state and interaction architecture |
| `DEL-00-06` | `PKG-00` | `SEMANTIC_READY` | 0 | Diagnostics, warning, and result-envelope contract |
| `DEL-00-07` | `PKG-00` | `SEMANTIC_READY` | 0 | API boundary and adapter contract map |
| `DEL-00-08` | `PKG-00` | `SEMANTIC_READY` | 0 | Layered software test and acceptance strategy |
| `DEL-01-01` | `PKG-01` | `OPEN` | 4 | Project governance baseline |
| `DEL-02-01` | `PKG-02` | `SEMANTIC_READY` | 7 | Canonical domain model schema |
| `DEL-02-02` | `PKG-02` | `SEMANTIC_READY` | 8 | Unit system and dimensional-analysis core contract |
| `DEL-02-03` | `PKG-02` | `SEMANTIC_READY` | 8 | Code-neutral analysis boundary model |
| `DEL-02-04` | `PKG-02` | `SEMANTIC_READY` | 10 | Plugin and extension domain contracts |
| `DEL-02-05` | `PKG-02` | `SEMANTIC_READY` | 10 | Project persistence and round-trip serialization |
| `DEL-04-01` | `PKG-04` | `OPEN` | 8 | 3D frame stiffness kernel |
| `DEL-05-04` | `PKG-05` | `OPEN` | 6 | Analysis status semantics |
| `DEL-10-01` | `PKG-10` | `OPEN` | 11 | Public API and plugin boundary |

## Selected Pilot Check

| Field | Value |
|---|---|
| Deliverable | `DEL-01-01` - Project governance baseline |
| Package | `PKG-01` |
| Lifecycle | `OPEN` |
| Active upstream dependencies | 4 |
| Blocking upstream dependencies | 0 |
| Advisory state | `UNBLOCKED` |

`DEL-01-01` remains a valid pilot candidate under the approved active-edge blocker computation. Its repo-level write targets remain limited to `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, and `governance/MAINTAINERS.md` by the human ruling recorded in `execution/_DAG/DAG-001/APPROVAL_RECORD.md` and `execution/_Coordination/DEV-001_PILOT_DISPATCH_DEL-01-01.md`.

## Candidate Edges Excluded

Candidate edges remain non-gating pending later `RECONCILIATION` and were not used in the blocker state calculation.

| EdgeID | From | Target | Candidate note |
|---|---|---|---|
| `DAG-001-E0616` | `DEL-05-02` | `DEL-06-02` | load-case algebra may reuse the same sandboxed expression machinery as rule packs; current decomposition leaves expression grammar/library TBD |
| `DAG-001-E0617` | `DEL-07-05` | `DEL-08-04` | results viewer may share the structured result export schema, but GUI rendering could also consume internal result models directly |
| `DAG-001-E0618` | `DEL-10-03` | `DEL-08-04` | local FEA handoff may reuse result export envelopes, but the handoff contract may define a separate package |
| `DAG-001-E0619` | `DEL-12-05` | `DEL-10-02` | threat model may require adapter framework details; active graph currently places threat model before adapter framework to avoid bypass risk |
| `DAG-001-E0620` | `DEL-09-05` | `DEL-10-04` | CI implementation may refine release quality gate details; active graph currently treats gates as predecessor to CI/CD implementation |
| `DAG-001-E0621` | `DEL-08-05` | `DEL-11-04` | protected-content linter may need educational example fixtures; active graph currently treats examples as consumers of the linter |
| `DAG-001-E0622` | `DEL-04-06` | `DEL-04-04` | diagnostics may need nonlinear support cases to finalize all warning classes; active graph currently makes nonlinear solver consume diagnostics |
| `DAG-001-E0623` | `DEL-06-02` | `DEL-12-05` | sandboxed evaluator may require threat-model review before implementation freeze |
| `DAG-001-E0624` | `DEL-07-07` | `DEL-10-05` | solve execution UX and headless runner may share job orchestration implementation; current evidence only proves shared architecture basis |

## Machine-Readable Queue

Full per-deliverable queue rows are recorded in `execution/_Coordination/DEV-001_BLOCKER_QUEUE.csv`.

