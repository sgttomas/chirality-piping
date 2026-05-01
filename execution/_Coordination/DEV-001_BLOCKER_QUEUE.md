---
doc_id: DEV-001-BLOCKER-QUEUE
doc_kind: coordination.blocker_queue
status: computed_active_edges_only
created: 2026-04-30
updated: 2026-04-30
source_graph: execution/_DAG/DAG-001/DependencyEdges.csv
implementation_evidence_source: execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv
implementation_threshold: COMMITTED
architecture_basis: satisfied_by_existing_baseline
candidate_edges: excluded
---

# DEV-001 Implementation-Readiness Blocker Queue

This blocker queue is an advisory implementation-readiness view only. It is not a schedule, staffing plan, priority list, lifecycle approval, professional approval, or readiness-for-reliance claim.

## Computation Rule

- Source graph: `execution/_DAG/DAG-001/DependencyEdges.csv`.
- Included edges: `Status=ACTIVE` only.
- Excluded edges: all `Status=CANDIDATE` rows.
- Direction convention: `FromDeliverableID` is the downstream consumer and is blocked by `TargetDeliverableID`, the upstream provider.
- Satisfaction threshold: upstream provider has `COMMITTED` evidence in `execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv`.
- `SEMANTIC_READY` remains decomposition/context readiness evidence; it does not satisfy implementation blockers by itself.
- `PKG-00` `ARCHITECTURE_BASIS` provider edges are satisfied by the accepted architecture baseline, not by implementation evidence.
- `UNBLOCKED` means all active upstream implementation dependencies satisfy the threshold or are satisfied architecture-basis edges.
- `BLOCKED` means one or more active upstream providers lack committed implementation evidence.

## Evidence Summary

| Evidence | Count |
|---|---:|
| Packages represented | 13 |
| Deliverable nodes represented | 73 |
| Active edges included | 615 |
| Candidate edges excluded | 9 |
| Implementation evidence records | 17 |
| Committed implementation evidence | 17 |
| Filesystem lifecycle `SEMANTIC_READY` (display only) | 73 |
| PKG-00 architecture-basis edges satisfied | 388 |
| Implementation `UNBLOCKED` deliverables | 33 |
| Implementation `BLOCKED` deliverables | 40 |

## Package Summary

| PackageID | UNBLOCKED | BLOCKED |
|---|---:|---:|
| `PKG-00` | 8 | 0 |
| `PKG-01` | 4 | 0 |
| `PKG-02` | 5 | 0 |
| `PKG-03` | 8 | 0 |
| `PKG-04` | 1 | 5 |
| `PKG-05` | 1 | 4 |
| `PKG-06` | 1 | 4 |
| `PKG-07` | 2 | 5 |
| `PKG-08` | 0 | 5 |
| `PKG-09` | 0 | 5 |
| `PKG-10` | 1 | 4 |
| `PKG-11` | 1 | 4 |
| `PKG-12` | 1 | 4 |

## Unblocked DAG-Ready Items

These deliverables have no active upstream implementation dependency below the `COMMITTED` threshold. Items without their own committed evidence are DAG-ready candidates, not completed work.

| DeliverableID | PackageID | Implementation evidence | Active upstream | Name |
|---|---|---|---:|---|
| `DEL-00-01` | `PKG-00` | `ARCHITECTURE_BASELINE` | 0 | Architecture decision record baseline |
| `DEL-00-02` | `PKG-00` | `ARCHITECTURE_BASELINE` | 0 | Repository and module boundary architecture |
| `DEL-00-03` | `PKG-00` | `ARCHITECTURE_BASELINE` | 0 | Application service command-query-job model |
| `DEL-00-04` | `PKG-00` | `ARCHITECTURE_BASELINE` | 0 | Persistence and schema versioning architecture |
| `DEL-00-05` | `PKG-00` | `ARCHITECTURE_BASELINE` | 0 | GUI state and interaction architecture |
| `DEL-00-06` | `PKG-00` | `ARCHITECTURE_BASELINE` | 0 | Diagnostics, warning, and result-envelope contract |
| `DEL-00-07` | `PKG-00` | `ARCHITECTURE_BASELINE` | 0 | API boundary and adapter contract map |
| `DEL-00-08` | `PKG-00` | `ARCHITECTURE_BASELINE` | 0 | Layered software test and acceptance strategy |
| `DEL-01-01` | `PKG-01` | `COMMITTED` `7650cf6` | 4 | Project governance baseline |
| `DEL-01-02` | `PKG-01` | `COMMITTED` `0d729cf` | 5 | Copyright and protected-data boundary policy |
| `DEL-01-03` | `PKG-01` | `COMMITTED` `df461f8` | 6 | Contributor certification workflow |
| `DEL-01-04` | `PKG-01` | `COMMITTED` `65f3119` | 5 | Professional responsibility and product-claims policy |
| `DEL-02-01` | `PKG-02` | `COMMITTED` `7b256f3` | 7 | Canonical domain model schema |
| `DEL-02-02` | `PKG-02` | `COMMITTED` `a458cba` | 8 | Unit system and dimensional-analysis core contract |
| `DEL-02-03` | `PKG-02` | `COMMITTED` `abc1306` | 8 | Code-neutral analysis boundary model |
| `DEL-02-04` | `PKG-02` | `COMMITTED` `ef44f4c` | 10 | Plugin and extension domain contracts |
| `DEL-02-05` | `PKG-02` | `COMMITTED` `742016e` | 10 | Project persistence and round-trip serialization |
| `DEL-03-01` | `PKG-03` | `COMMITTED` `3793e87` | 10 | Material library schema with provenance |
| `DEL-03-02` | `PKG-03` | `COMMITTED` `f0fdeac` | 10 | Pipe section and component library schema |
| `DEL-03-03` | `PKG-03` | `COMMITTED` `7a84472` | 9 | Bend and elbow component model fields |
| `DEL-03-04` | `PKG-03` | `COMMITTED` `ae693b6` | 9 | Branch connection component model fields |
| `DEL-03-05` | `PKG-03` | `COMMITTED` `d8ee0db` | 9 | Rigid component models for valves, flanges, reducers, and specialty items |
| `DEL-03-06` | `PKG-03` | `COMMITTED` `f15cbc6` | 9 | Expansion joint component model |
| `DEL-03-07` | `PKG-03` | `COMMITTED` `4d880b3` | 11 | Public/private library import provenance checker |
| `DEL-03-08` | `PKG-03` | `COMMITTED` `9712e98` | 9 | Pipe section property and mass-property calculator |
| `DEL-04-01` | `PKG-04` | `MISSING_EVIDENCE` | 8 | 3D frame stiffness kernel |
| `DEL-05-04` | `PKG-05` | `MISSING_EVIDENCE` | 6 | Analysis status semantics |
| `DEL-06-01` | `PKG-06` | `MISSING_EVIDENCE` | 12 | Rule-pack schema |
| `DEL-07-01` | `PKG-07` | `MISSING_EVIDENCE` | 15 | 3D viewport and centerline editor |
| `DEL-07-02` | `PKG-07` | `MISSING_EVIDENCE` | 10 | Model tree and property inspector |
| `DEL-10-01` | `PKG-10` | `MISSING_EVIDENCE` | 11 | Public API and plugin boundary |
| `DEL-11-05` | `PKG-11` | `MISSING_EVIDENCE` | 8 | Contributor tutorial and onboarding |
| `DEL-12-05` | `PKG-12` | `MISSING_EVIDENCE` | 10 | Security threat model |

## Blocked Items Grouped By Missing Upstream

| Missing upstream | PackageID | Evidence state | Blocked consumers | Consumer IDs | Edge IDs |
|---|---|---|---:|---|---|
| `DEL-04-01` - 3D frame stiffness kernel | `PKG-04` | `MISSING_EVIDENCE` | 13 | `DEL-04-02`; `DEL-04-03`; `DEL-04-04`; `DEL-04-05`; `DEL-04-06`; `DEL-05-01`; `DEL-05-05`; `DEL-07-07`; `DEL-09-01`; `DEL-10-03`; `DEL-10-05`; `DEL-11-02`; `DEL-11-03` | `DAG-001-E0432`; `DAG-001-E0435`; `DAG-001-E0438`; `DAG-001-E0442`; `DAG-001-E0444`; `DAG-001-E0448`; `DAG-001-E0460`; `DAG-001-E0502`; `DAG-001-E0532`; `DAG-001-E0562`; `DAG-001-E0567`; `DAG-001-E0582`; `DAG-001-E0586` |
| `DEL-04-02` - Straight pipe element | `PKG-04` | `MISSING_EVIDENCE` | 4 | `DEL-05-03`; `DEL-09-01`; `DEL-09-02`; `DEL-11-03` | `DAG-001-E0454`; `DAG-001-E0533`; `DAG-001-E0539`; `DAG-001-E0587` |
| `DEL-04-03` - Linear support and restraint models | `PKG-04` | `MISSING_EVIDENCE` | 3 | `DEL-04-04`; `DEL-05-01`; `DEL-09-01` | `DAG-001-E0441`; `DAG-001-E0449`; `DAG-001-E0534` |
| `DEL-04-04` - Nonlinear support active-set solver | `PKG-04` | `MISSING_EVIDENCE` | 1 | `DEL-09-03` | `DAG-001-E0541` |
| `DEL-04-06` - Solver diagnostics and singularity detection | `PKG-04` | `MISSING_EVIDENCE` | 7 | `DEL-04-04`; `DEL-04-05`; `DEL-07-04`; `DEL-07-05`; `DEL-07-07`; `DEL-08-03`; `DEL-09-03` | `DAG-001-E0443`; `DAG-001-E0445`; `DAG-001-E0498`; `DAG-001-E0501`; `DAG-001-E0504`; `DAG-001-E0518`; `DAG-001-E0542` |
| `DEL-05-01` - Primitive load case engine | `PKG-05` | `MISSING_EVIDENCE` | 6 | `DEL-05-02`; `DEL-05-03`; `DEL-05-05`; `DEL-07-07`; `DEL-09-01`; `DEL-10-05` | `DAG-001-E0451`; `DAG-001-E0456`; `DAG-001-E0459`; `DAG-001-E0503`; `DAG-001-E0535`; `DAG-001-E0568` |
| `DEL-05-03` - Fundamental stress recovery module | `PKG-05` | `MISSING_EVIDENCE` | 5 | `DEL-07-05`; `DEL-08-01`; `DEL-08-04`; `DEL-09-02`; `DEL-10-03` | `DAG-001-E0499`; `DAG-001-E0516`; `DAG-001-E0523`; `DAG-001-E0537`; `DAG-001-E0563` |
| `DEL-05-04` - Analysis status semantics | `PKG-05` | `MISSING_EVIDENCE` | 9 | `DEL-05-02`; `DEL-05-03`; `DEL-06-03`; `DEL-07-04`; `DEL-07-05`; `DEL-07-07`; `DEL-08-01`; `DEL-08-03`; `DEL-08-04` | `DAG-001-E0453`; `DAG-001-E0458`; `DAG-001-E0471`; `DAG-001-E0496`; `DAG-001-E0500`; `DAG-001-E0505`; `DAG-001-E0517`; `DAG-001-E0519`; `DAG-001-E0524` |
| `DEL-06-01` - Rule-pack schema | `PKG-06` | `MISSING_EVIDENCE` | 6 | `DEL-06-02`; `DEL-06-03`; `DEL-06-04`; `DEL-06-05`; `DEL-07-03`; `DEL-11-02` | `DAG-001-E0467`; `DAG-001-E0469`; `DAG-001-E0472`; `DAG-001-E0474`; `DAG-001-E0492`; `DAG-001-E0583` |
| `DEL-06-02` - Sandboxed unit-aware expression evaluator | `PKG-06` | `MISSING_EVIDENCE` | 1 | `DEL-06-05` | `DAG-001-E0475` |
| `DEL-06-03` - Required-input completeness checker | `PKG-06` | `MISSING_EVIDENCE` | 1 | `DEL-07-04` | `DAG-001-E0497` |
| `DEL-06-04` - Private rule-pack lifecycle and checksum handling | `PKG-06` | `MISSING_EVIDENCE` | 5 | `DEL-07-03`; `DEL-08-01`; `DEL-08-02`; `DEL-12-02`; `DEL-12-04` | `DAG-001-E0493`; `DAG-001-E0513`; `DAG-001-E0525`; `DAG-001-E0609`; `DAG-001-E0614` |
| `DEL-06-05` - Invented non-code example rule pack | `PKG-06` | `MISSING_EVIDENCE` | 1 | `DEL-11-04` | `DAG-001-E0590` |
| `DEL-07-01` - 3D viewport and centerline editor | `PKG-07` | `MISSING_EVIDENCE` | 2 | `DEL-07-06`; `DEL-11-01` | `DAG-001-E0506`; `DAG-001-E0575` |
| `DEL-07-02` - Model tree and property inspector | `PKG-07` | `MISSING_EVIDENCE` | 1 | `DEL-07-06` | `DAG-001-E0507` |
| `DEL-07-03` - Material, component, and rule-pack editors | `PKG-07` | `MISSING_EVIDENCE` | 2 | `DEL-07-06`; `DEL-11-01` | `DAG-001-E0508`; `DAG-001-E0576` |
| `DEL-07-04` - Missing-data warning and blocking UX | `PKG-07` | `MISSING_EVIDENCE` | 1 | `DEL-07-06` | `DAG-001-E0509` |
| `DEL-07-05` - Results viewer | `PKG-07` | `MISSING_EVIDENCE` | 2 | `DEL-07-06`; `DEL-11-01` | `DAG-001-E0510`; `DAG-001-E0577` |
| `DEL-07-07` - Solve execution UX: progress, cancellation, and diagnostics | `PKG-07` | `MISSING_EVIDENCE` | 1 | `DEL-07-06` | `DAG-001-E0511` |
| `DEL-08-01` - Calculation report generator | `PKG-08` | `MISSING_EVIDENCE` | 3 | `DEL-08-05`; `DEL-11-01`; `DEL-12-02` | `DAG-001-E0529`; `DAG-001-E0578`; `DAG-001-E0612` |
| `DEL-08-02` - Audit manifest and model hash | `PKG-08` | `MISSING_EVIDENCE` | 2 | `DEL-08-01`; `DEL-10-05` | `DAG-001-E0526`; `DAG-001-E0570` |
| `DEL-08-03` - Warnings, assumptions, and provenance report section | `PKG-08` | `MISSING_EVIDENCE` | 1 | `DEL-08-01` | `DAG-001-E0527` |
| `DEL-08-04` - Result export format | `PKG-08` | `MISSING_EVIDENCE` | 2 | `DEL-10-05`; `DEL-12-02` | `DAG-001-E0569`; `DAG-001-E0613` |
| `DEL-08-05` - Report protected-content linter | `PKG-08` | `MISSING_EVIDENCE` | 3 | `DEL-09-05`; `DEL-10-04`; `DEL-11-04` | `DAG-001-E0550`; `DAG-001-E0573`; `DAG-001-E0593` |
| `DEL-09-01` - Mechanics benchmark suite | `PKG-09` | `MISSING_EVIDENCE` | 4 | `DEL-09-04`; `DEL-09-05`; `DEL-11-03`; `DEL-11-04` | `DAG-001-E0543`; `DAG-001-E0547`; `DAG-001-E0588`; `DAG-001-E0591` |
| `DEL-09-02` - Stress recovery benchmark suite | `PKG-09` | `MISSING_EVIDENCE` | 3 | `DEL-09-04`; `DEL-09-05`; `DEL-11-04` | `DAG-001-E0544`; `DAG-001-E0548`; `DAG-001-E0592` |
| `DEL-09-03` - Nonlinear support regression suite | `PKG-09` | `MISSING_EVIDENCE` | 2 | `DEL-09-04`; `DEL-09-05` | `DAG-001-E0545`; `DAG-001-E0549` |
| `DEL-09-05` - Release quality gate checklist | `PKG-09` | `MISSING_EVIDENCE` | 1 | `DEL-10-04` | `DAG-001-E0571` |
| `DEL-10-01` - Public API and plugin boundary | `PKG-10` | `MISSING_EVIDENCE` | 4 | `DEL-10-02`; `DEL-10-03`; `DEL-10-05`; `DEL-11-02` | `DAG-001-E0556`; `DAG-001-E0561`; `DAG-001-E0565`; `DAG-001-E0584` |
| `DEL-10-05` - Headless CLI and structured I/O analysis runner | `PKG-10` | `MISSING_EVIDENCE` | 1 | `DEL-10-04` | `DAG-001-E0572` |
| `DEL-12-01` - Local-first storage and private data paths | `PKG-12` | `MISSING_EVIDENCE` | 4 | `DEL-07-03`; `DEL-10-02`; `DEL-12-02`; `DEL-12-04` | `DAG-001-E0494`; `DAG-001-E0559`; `DAG-001-E0607`; `DAG-001-E0611` |
| `DEL-12-05` - Security threat model | `PKG-12` | `MISSING_EVIDENCE` | 6 | `DEL-10-02`; `DEL-10-04`; `DEL-12-01`; `DEL-12-02`; `DEL-12-03`; `DEL-12-04` | `DAG-001-E0560`; `DAG-001-E0574`; `DAG-001-E0602`; `DAG-001-E0604`; `DAG-001-E0606`; `DAG-001-E0610` |

## Per-Deliverable Blocked Items

| DeliverableID | PackageID | Missing upstream count | Missing upstream deliverables | Name |
|---|---|---:|---|---|
| `DEL-04-02` | `PKG-04` | 1 | `DEL-04-01` | Straight pipe element |
| `DEL-04-03` | `PKG-04` | 1 | `DEL-04-01` | Linear support and restraint models |
| `DEL-04-04` | `PKG-04` | 3 | `DEL-04-03`; `DEL-04-01`; `DEL-04-06` | Nonlinear support active-set solver |
| `DEL-04-05` | `PKG-04` | 2 | `DEL-04-01`; `DEL-04-06` | Sparse solver performance harness |
| `DEL-04-06` | `PKG-04` | 1 | `DEL-04-01` | Solver diagnostics and singularity detection |
| `DEL-05-01` | `PKG-05` | 2 | `DEL-04-01`; `DEL-04-03` | Primitive load case engine |
| `DEL-05-02` | `PKG-05` | 2 | `DEL-05-01`; `DEL-05-04` | Load-case algebra engine |
| `DEL-05-03` | `PKG-05` | 3 | `DEL-04-02`; `DEL-05-01`; `DEL-05-04` | Fundamental stress recovery module |
| `DEL-05-05` | `PKG-05` | 2 | `DEL-05-01`; `DEL-04-01` | Concentrated and distributed user load application |
| `DEL-06-02` | `PKG-06` | 1 | `DEL-06-01` | Sandboxed unit-aware expression evaluator |
| `DEL-06-03` | `PKG-06` | 2 | `DEL-06-01`; `DEL-05-04` | Required-input completeness checker |
| `DEL-06-04` | `PKG-06` | 1 | `DEL-06-01` | Private rule-pack lifecycle and checksum handling |
| `DEL-06-05` | `PKG-06` | 2 | `DEL-06-01`; `DEL-06-02` | Invented non-code example rule pack |
| `DEL-07-03` | `PKG-07` | 3 | `DEL-06-01`; `DEL-06-04`; `DEL-12-01` | Material, component, and rule-pack editors |
| `DEL-07-04` | `PKG-07` | 3 | `DEL-05-04`; `DEL-06-03`; `DEL-04-06` | Missing-data warning and blocking UX |
| `DEL-07-05` | `PKG-07` | 3 | `DEL-05-03`; `DEL-05-04`; `DEL-04-06` | Results viewer |
| `DEL-07-06` | `PKG-07` | 6 | `DEL-07-01`; `DEL-07-02`; `DEL-07-03`; `DEL-07-04`; `DEL-07-05`; `DEL-07-07` | Accessibility and usability baseline |
| `DEL-07-07` | `PKG-07` | 4 | `DEL-04-01`; `DEL-05-01`; `DEL-04-06`; `DEL-05-04` | Solve execution UX: progress, cancellation, and diagnostics |
| `DEL-08-01` | `PKG-08` | 5 | `DEL-05-03`; `DEL-05-04`; `DEL-06-04`; `DEL-08-02`; `DEL-08-03` | Calculation report generator |
| `DEL-08-02` | `PKG-08` | 1 | `DEL-06-04` | Audit manifest and model hash |
| `DEL-08-03` | `PKG-08` | 2 | `DEL-04-06`; `DEL-05-04` | Warnings, assumptions, and provenance report section |
| `DEL-08-04` | `PKG-08` | 2 | `DEL-05-03`; `DEL-05-04` | Result export format |
| `DEL-08-05` | `PKG-08` | 1 | `DEL-08-01` | Report protected-content linter |
| `DEL-09-01` | `PKG-09` | 4 | `DEL-04-01`; `DEL-04-02`; `DEL-04-03`; `DEL-05-01` | Mechanics benchmark suite |
| `DEL-09-02` | `PKG-09` | 2 | `DEL-05-03`; `DEL-04-02` | Stress recovery benchmark suite |
| `DEL-09-03` | `PKG-09` | 2 | `DEL-04-04`; `DEL-04-06` | Nonlinear support regression suite |
| `DEL-09-04` | `PKG-09` | 3 | `DEL-09-01`; `DEL-09-02`; `DEL-09-03` | Validation manual skeleton |
| `DEL-09-05` | `PKG-09` | 4 | `DEL-09-01`; `DEL-09-02`; `DEL-09-03`; `DEL-08-05` | Release quality gate checklist |
| `DEL-10-02` | `PKG-10` | 3 | `DEL-10-01`; `DEL-12-01`; `DEL-12-05` | Import/export adapter framework |
| `DEL-10-03` | `PKG-10` | 3 | `DEL-10-01`; `DEL-04-01`; `DEL-05-03` | Local FEA handoff data contract |
| `DEL-10-04` | `PKG-10` | 4 | `DEL-09-05`; `DEL-10-05`; `DEL-08-05`; `DEL-12-05` | Build, packaging, and CI/CD pipeline |
| `DEL-10-05` | `PKG-10` | 5 | `DEL-10-01`; `DEL-04-01`; `DEL-05-01`; `DEL-08-04`; `DEL-08-02` | Headless CLI and structured I/O analysis runner |
| `DEL-11-01` | `PKG-11` | 4 | `DEL-07-01`; `DEL-07-03`; `DEL-07-05`; `DEL-08-01` | User guide skeleton |
| `DEL-11-02` | `PKG-11` | 3 | `DEL-04-01`; `DEL-06-01`; `DEL-10-01` | Developer guide for solver and rule packs |
| `DEL-11-03` | `PKG-11` | 3 | `DEL-04-01`; `DEL-04-02`; `DEL-09-01` | Theory notes: classical to modern centerline analysis |
| `DEL-11-04` | `PKG-11` | 4 | `DEL-06-05`; `DEL-09-01`; `DEL-09-02`; `DEL-08-05` | Invented educational example models |
| `DEL-12-01` | `PKG-12` | 1 | `DEL-12-05` | Local-first storage and private data paths |
| `DEL-12-02` | `PKG-12` | 5 | `DEL-12-05`; `DEL-12-01`; `DEL-08-01`; `DEL-08-04`; `DEL-06-04` | Private data redaction and export controls |
| `DEL-12-03` | `PKG-12` | 1 | `DEL-12-05` | Telemetry off-by-default design |
| `DEL-12-04` | `PKG-12` | 3 | `DEL-12-05`; `DEL-12-01`; `DEL-06-04` | Secret and private-library handling |

## Candidate Edges Excluded

Candidate edges remain non-gating pending later `RECONCILIATION` and `CHANGE`; they were not used in the blocker state calculation.

## Machine-Readable Queue

Full per-deliverable queue rows are recorded in `execution/_Coordination/DEV-001_BLOCKER_QUEUE.csv`.
