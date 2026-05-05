---
doc_id: DEV-001-REV05-SEALED-BRIEF-DEL-14-05
doc_kind: coordination.sealed_type2_brief
status: sealed_brief_prepared_dispatch_not_authorized
created: 2026-05-04
prepared_by: ORCHESTRATOR
tranche: DEV-001_REV05_TRANCHE_F
deliverable_id: DEL-14-05
package_id: PKG-14
worker_launch: not_authorized
implementation_lane: contract_first_comparison_mapping_tolerance_export
source_proposal: execution/_Coordination/DEV-001_REV05_TRANCHE_F_PROPOSAL.md
---

# Sealed Brief - DEL-14-05 Comparison Mapping, Tolerance, And Export Contracts

## Dispatch Boundary

This is a sealed Type 2 implementation brief for future worker dispatch. It
does not authorize worker launch or implementation by itself. Use only after
the human separately approves implementation dispatch for Tranche F sealed
briefs.

The accepted lane is contract-first and provider-neutral. This brief does not
authorize lifecycle/evidence promotion, blocker refresh, dependency refresh,
aggregate DAG mutation, candidate promotion, commit, push, GUI/runtime work,
external prover integration, commercial-tool parser work, protected data,
private project data, result-delta engine implementation, or professional/code
compliance claims.

## Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-14-05` |
| PackageID | `PKG-14` |
| Name | Comparison mapping, tolerance, and export contracts |
| Type | `API_CONTRACT` |
| Scope item | `SOW-073` |
| Objective | `OBJ-016` |
| Context envelope | `M` |
| Lifecycle at brief prep | `SEMANTIC_READY` |
| Evidence state at brief prep | `MISSING_EVIDENCE` |
| Deliverable path | `execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-05_Comparison mapping, tolerance, and export contracts` |

## Scope And Objective

Define manual mappings, unmatched classifications, tolerance profiles, and
CSV/JSON/report-section exports.

Acceptance context from `_CONTEXT.md`: model states and/or analysis runs are
compared deterministically using stable IDs, manual mappings where required,
unit-normalized result deltas, and tolerance profiles. The package exclusion is
binding: do not ingest commercial prover outputs comprehensively or determine
external validation.

## Readiness Evidence

`DEV-001_BLOCKER_QUEUE.csv` marks `DEL-14-05` as `UNBLOCKED` with 11 active
upstreams satisfied and zero blocking upstreams. Local dependency mirror state
is `SYNCHRONIZED_FROM_APPROVED_DAG002_MIRROR_PRESENT`; the local
`Dependencies.csv` is evidence synchronized from approved `DAG-002`, not
independent sequencing authority.

| EdgeID | Upstream | Evidence |
|---|---|---|
| `DAG-002-E0695` | `DEL-00-01` Architecture decision record baseline | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0696` | `DEL-00-02` Repository and module boundary architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0697` | `DEL-00-03` Application service command-query-job model | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0698` | `DEL-00-04` Persistence and schema versioning architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0699` | `DEL-00-06` Diagnostics, warning, and result-envelope contract | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0700` | `DEL-00-07` API boundary and adapter contract map | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0701` | `DEL-00-08` Layered software test and acceptance strategy | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0788` | `DEL-14-01` Immutable model state records | `COMMITTED dcdc1ac` |
| `DAG-002-E0789` | `DEL-14-02` Analysis run records | `COMMITTED 002263b` |
| `DAG-002-E0790` | `DEL-08-04` Result export format | `COMMITTED 3e33ea4` |
| `DAG-002-E0791` | `DEL-02-02` Unit system and dimensional-analysis core contract | `COMMITTED a458cba` |

Candidate rows remain excluded from readiness and dispatch authority.

## Applicable Invariants

Apply the architecture basis IDs named in `_CONTEXT.md`: `AB-00-01`,
`AB-00-02`, `AB-00-03`, `AB-00-04`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.
Apply only the applicable constraints; do not copy full `PKG-00` prose into
deliverable artifacts.

Apply the project invariants from `docs/CONTRACT.md`, especially:

- `OPS-K-IP-1`, `OPS-K-IP-2`, `OPS-K-IP-3`
- `OPS-K-DATA-2`, `OPS-K-DATA-3`
- `OPS-K-AUTH-1`, `OPS-K-AUTH-2`
- `OPS-K-UNIT-1`
- `OPS-K-REPORT-1`, `OPS-K-REPORT-2`
- `OPS-K-PRIV-1`
- `OPS-K-AGENT-1`, `OPS-K-AGENT-2`, `OPS-K-AGENT-3`, `OPS-K-AGENT-4`

## Allowed Write Scope For Later Authorized Implementation

- `schemas/comparison_mapping.schema.json`
- `schemas/comparison_tolerance.schema.json`
- `tests/test_comparison_contracts.py`
- optional export-contract stubs only if kept under a brief-named path and
  limited to contract fixtures, not engine implementation
- deliverable-local `MEMORY.md`
- deliverable-local run notes under the `DEL-14-05` folder

Do not edit `docs/SPEC.md` or `docs/TYPES.md` from a parallel worker. Shared
documentation integration, if needed, belongs to ORCHESTRATOR after
implementation return or to a later closeout gate with explicit scope.

Do not edit local `Dependencies.csv`, `_DEPENDENCIES.md`, lifecycle
`_STATUS.md`, implementation-evidence registers, blocker queues,
dependency-status projections, aggregate DAG files, candidate rows, or
committed evidence state.

## Tasks For Future Implementation

1. Define JSON Schema 2020-12 contracts for comparison mappings and tolerance
   profiles.
2. Represent state/run comparison participants using stable references to
   immutable model-state records, analysis-run records, and result-export
   envelopes.
3. Define manual mapping records, mapping status, confidence/review metadata,
   unmatched classifications, and affected entity/result references.
4. Define unit-aware tolerance profile metadata without hard-coded default
   numerical tolerances unless separately governed.
5. Define JSON and CSV export semantics for comparison review, including
   stable IDs, mapping IDs, units, tolerance profile references, diagnostics,
   provenance, assumptions, and professional-boundary notices.
6. Reserve report-section export references without implementing final report
   rendering or comparison engines.
7. Add focused schema/contract tests and record implementation memory/run
   notes.

## Acceptance Criteria

- Schemas are strict JSON syntax and use JSON Schema 2020-12.
- Mapping records distinguish automatic matches, manual matches, unresolved
  mappings, unmatched-left, unmatched-right, ignored, and `TBD` or equivalent
  explicit unresolved states.
- Tolerance profiles carry unit/dimension metadata and do not invent project
  thresholds or code acceptance values.
- Export contracts preserve provenance, diagnostics, assumptions, hashes or
  references where applicable, and professional-boundary notices.
- The work does not implement comprehensive commercial prover ingestion,
  external validation decisions, formal acceptance statuses, or result-delta
  calculation engines.

## Required Verification For Future Implementation

- `python3 -m json.tool schemas/comparison_mapping.schema.json`
- `python3 -m json.tool schemas/comparison_tolerance.schema.json`
- `python3 tests/test_comparison_contracts.py`
- adjacent checks where referenced:
  `python3 tests/test_model_state_schema.py`,
  `python3 tests/test_analysis_run_schema.py`,
  `python3 tests/test_results_schema.py`, and
  `python3 tests/test_units_schema.py`
- `git diff --check`
- focused scans for protected standards data, private project data,
  proprietary examples, real secrets, and prohibited certification/compliance/
  sealing/professional-approval claims.

## Stop Conditions

Stop before implementation, worker launch, lifecycle/evidence update, blocker
refresh, dependency refresh, aggregate DAG mutation, candidate promotion,
commit, push, protected standards content, private project data, real secrets,
GUI runtime work, external prover integration, commercial-tool parser work,
hard-coded acceptance tolerances, result-delta engine implementation, or
professional/code compliance claims unless separately authorized.
