---
doc_id: DEV-001-BLOCKER-QUEUE
doc_kind: coordination.blocker_queue
status: computed_active_edges_only
created: 2026-04-30
updated: 2026-04-30
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
| Filesystem lifecycle `SEMANTIC_READY` | 73 |
| Advisory `UNBLOCKED` deliverables | 73 |
| Advisory `BLOCKED` deliverables | 0 |

## Package Summary

| PackageID | UNBLOCKED | BLOCKED |
|---|---:|---:|
| `PKG-00` | 8 | 0 |
| `PKG-01` | 4 | 0 |
| `PKG-02` | 5 | 0 |
| `PKG-03` | 8 | 0 |
| `PKG-04` | 6 | 0 |
| `PKG-05` | 5 | 0 |
| `PKG-06` | 5 | 0 |
| `PKG-07` | 7 | 0 |
| `PKG-08` | 5 | 0 |
| `PKG-09` | 5 | 0 |
| `PKG-10` | 5 | 0 |
| `PKG-11` | 5 | 0 |
| `PKG-12` | 5 | 0 |

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
| `DEL-01-01` | `PKG-01` | `SEMANTIC_READY` | 4 | Project governance baseline |
| `DEL-01-02` | `PKG-01` | `SEMANTIC_READY` | 5 | Copyright and protected-data boundary policy |
| `DEL-01-03` | `PKG-01` | `SEMANTIC_READY` | 6 | Contributor certification workflow |
| `DEL-01-04` | `PKG-01` | `SEMANTIC_READY` | 5 | Professional responsibility and product-claims policy |
| `DEL-02-01` | `PKG-02` | `SEMANTIC_READY` | 7 | Canonical domain model schema |
| `DEL-02-02` | `PKG-02` | `SEMANTIC_READY` | 8 | Unit system and dimensional-analysis core contract |
| `DEL-02-03` | `PKG-02` | `SEMANTIC_READY` | 8 | Code-neutral analysis boundary model |
| `DEL-02-04` | `PKG-02` | `SEMANTIC_READY` | 10 | Plugin and extension domain contracts |
| `DEL-02-05` | `PKG-02` | `SEMANTIC_READY` | 10 | Project persistence and round-trip serialization |
| `DEL-03-01` | `PKG-03` | `SEMANTIC_READY` | 10 | Material library schema with provenance |
| `DEL-03-02` | `PKG-03` | `SEMANTIC_READY` | 10 | Pipe section and component library schema |
| `DEL-03-03` | `PKG-03` | `SEMANTIC_READY` | 9 | Bend and elbow component model fields |
| `DEL-03-04` | `PKG-03` | `SEMANTIC_READY` | 9 | Branch connection component model fields |
| `DEL-03-05` | `PKG-03` | `SEMANTIC_READY` | 9 | Rigid component models for valves, flanges, reducers, and specialty items |
| `DEL-03-06` | `PKG-03` | `SEMANTIC_READY` | 9 | Expansion joint component model |
| `DEL-03-07` | `PKG-03` | `SEMANTIC_READY` | 11 | Public/private library import provenance checker |
| `DEL-03-08` | `PKG-03` | `SEMANTIC_READY` | 9 | Pipe section property and mass-property calculator |
| `DEL-04-01` | `PKG-04` | `SEMANTIC_READY` | 8 | 3D frame stiffness kernel |
| `DEL-04-02` | `PKG-04` | `SEMANTIC_READY` | 8 | Straight pipe element |
| `DEL-04-03` | `PKG-04` | `SEMANTIC_READY` | 8 | Linear support and restraint models |
| `DEL-04-04` | `PKG-04` | `SEMANTIC_READY` | 8 | Nonlinear support active-set solver |
| `DEL-04-05` | `PKG-04` | `SEMANTIC_READY` | 7 | Sparse solver performance harness |
| `DEL-04-06` | `PKG-04` | `SEMANTIC_READY` | 8 | Solver diagnostics and singularity detection |
| `DEL-05-01` | `PKG-05` | `SEMANTIC_READY` | 9 | Primitive load case engine |
| `DEL-05-02` | `PKG-05` | `SEMANTIC_READY` | 8 | Load-case algebra engine |
| `DEL-05-03` | `PKG-05` | `SEMANTIC_READY` | 10 | Fundamental stress recovery module |
| `DEL-05-04` | `PKG-05` | `SEMANTIC_READY` | 6 | Analysis status semantics |
| `DEL-05-05` | `PKG-05` | `SEMANTIC_READY` | 8 | Concentrated and distributed user load application |
| `DEL-06-01` | `PKG-06` | `SEMANTIC_READY` | 12 | Rule-pack schema |
| `DEL-06-02` | `PKG-06` | `SEMANTIC_READY` | 9 | Sandboxed unit-aware expression evaluator |
| `DEL-06-03` | `PKG-06` | `SEMANTIC_READY` | 10 | Required-input completeness checker |
| `DEL-06-04` | `PKG-06` | `SEMANTIC_READY` | 9 | Private rule-pack lifecycle and checksum handling |
| `DEL-06-05` | `PKG-06` | `SEMANTIC_READY` | 11 | Invented non-code example rule pack |
| `DEL-07-01` | `PKG-07` | `SEMANTIC_READY` | 15 | 3D viewport and centerline editor |
| `DEL-07-02` | `PKG-07` | `SEMANTIC_READY` | 10 | Model tree and property inspector |
| `DEL-07-03` | `PKG-07` | `SEMANTIC_READY` | 13 | Material, component, and rule-pack editors |
| `DEL-07-04` | `PKG-07` | `SEMANTIC_READY` | 11 | Missing-data warning and blocking UX |
| `DEL-07-05` | `PKG-07` | `SEMANTIC_READY` | 10 | Results viewer |
| `DEL-07-06` | `PKG-07` | `SEMANTIC_READY` | 13 | Accessibility and usability baseline |
| `DEL-07-07` | `PKG-07` | `SEMANTIC_READY` | 11 | Solve execution UX: progress, cancellation, and diagnostics |
| `DEL-08-01` | `PKG-08` | `SEMANTIC_READY` | 14 | Calculation report generator |
| `DEL-08-02` | `PKG-08` | `SEMANTIC_READY` | 9 | Audit manifest and model hash |
| `DEL-08-03` | `PKG-08` | `SEMANTIC_READY` | 11 | Warnings, assumptions, and provenance report section |
| `DEL-08-04` | `PKG-08` | `SEMANTIC_READY` | 11 | Result export format |
| `DEL-08-05` | `PKG-08` | `SEMANTIC_READY` | 10 | Report protected-content linter |
| `DEL-09-01` | `PKG-09` | `SEMANTIC_READY` | 9 | Mechanics benchmark suite |
| `DEL-09-02` | `PKG-09` | `SEMANTIC_READY` | 8 | Stress recovery benchmark suite |
| `DEL-09-03` | `PKG-09` | `SEMANTIC_READY` | 6 | Nonlinear support regression suite |
| `DEL-09-04` | `PKG-09` | `SEMANTIC_READY` | 8 | Validation manual skeleton |
| `DEL-09-05` | `PKG-09` | `SEMANTIC_READY` | 9 | Release quality gate checklist |
| `DEL-10-01` | `PKG-10` | `SEMANTIC_READY` | 11 | Public API and plugin boundary |
| `DEL-10-02` | `PKG-10` | `SEMANTIC_READY` | 12 | Import/export adapter framework |
| `DEL-10-03` | `PKG-10` | `SEMANTIC_READY` | 11 | Local FEA handoff data contract |
| `DEL-10-04` | `PKG-10` | `SEMANTIC_READY` | 11 | Build, packaging, and CI/CD pipeline |
| `DEL-10-05` | `PKG-10` | `SEMANTIC_READY` | 13 | Headless CLI and structured I/O analysis runner |
| `DEL-11-01` | `PKG-11` | `SEMANTIC_READY` | 10 | User guide skeleton |
| `DEL-11-02` | `PKG-11` | `SEMANTIC_READY` | 11 | Developer guide for solver and rule packs |
| `DEL-11-03` | `PKG-11` | `SEMANTIC_READY` | 9 | Theory notes: classical to modern centerline analysis |
| `DEL-11-04` | `PKG-11` | `SEMANTIC_READY` | 10 | Invented educational example models |
| `DEL-11-05` | `PKG-11` | `SEMANTIC_READY` | 8 | Contributor tutorial and onboarding |
| `DEL-12-01` | `PKG-12` | `SEMANTIC_READY` | 10 | Local-first storage and private data paths |
| `DEL-12-02` | `PKG-12` | `SEMANTIC_READY` | 13 | Private data redaction and export controls |
| `DEL-12-03` | `PKG-12` | `SEMANTIC_READY` | 9 | Telemetry off-by-default design |
| `DEL-12-04` | `PKG-12` | `SEMANTIC_READY` | 11 | Secret and private-library handling |
| `DEL-12-05` | `PKG-12` | `SEMANTIC_READY` | 10 | Security threat model |

## Selected Pilot Check

| Field | Value |
|---|---|
| Deliverable | `DEL-01-01` - Project governance baseline |
| Package | `PKG-01` |
| Lifecycle | `SEMANTIC_READY` |
| Active upstream dependencies | 4 |
| Blocking upstream dependencies | 0 |
| Advisory state | `UNBLOCKED` |

`DEL-01-01` remains completed as the DEV-001 pilot and is not a standing authorization for broad DAG execution. The next implementation step requires human review of the pilot behavior and explicit authorization of one bounded tranche.

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
