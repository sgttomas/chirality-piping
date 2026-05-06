---
doc_id: DEV-001-REV05-SEALED-BRIEF-DEL-14-04
doc_kind: coordination.sealed_type2_brief
status: sealed_brief_prepared_dispatch_not_authorized
created: 2026-05-05
prepared_by: ORCHESTRATOR
tranche: DEV-001_REV05_TRANCHE_G
deliverable_id: DEL-14-04
package_id: PKG-14
worker_launch: not_authorized
implementation_lane: contract_first_analysis_run_comparison
source_proposal: execution/_Coordination/DEV-001_REV05_TRANCHE_G_PROPOSAL.md
---

# Sealed Brief - DEL-14-04 Analysis-Run Comparison Engine

## Dispatch Boundary

This is a sealed Type 2 implementation brief for future worker dispatch. It
does not authorize worker launch or implementation by itself. Use only after
the human separately approves implementation dispatch for Tranche G sealed
briefs.

The accepted lane is contract-first, provider-neutral analysis-run comparison.
This brief does not authorize lifecycle/evidence promotion, blocker refresh,
dependency refresh, aggregate DAG mutation, candidate promotion, commit, push,
GUI/runtime work, external prover integration, commercial-tool parser work,
protected data, private project data, hard-coded acceptance tolerances, or
professional/code compliance claims.

## Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-14-04` |
| PackageID | `PKG-14` |
| Name | Analysis-run comparison engine |
| Type | `BACKEND_FEATURE_SLICE` |
| Scope items | `SOW-073`, `SOW-072` |
| Objective | `OBJ-016` |
| Context envelope | `L` |
| Lifecycle at brief prep | `SEMANTIC_READY` |
| Evidence state at brief prep | `MISSING_EVIDENCE` |
| Deliverable path | `execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-04_Analysis-run comparison engine` |

## Scope And Objective

Implement unit-normalized result deltas for mapped entities, diagnostics, and
settings.

Acceptance context from `_CONTEXT.md`: comparison is diagnostic/audit
functionality only. The engine compares analysis runs, and where applicable
their bound model states, using stable IDs, manual mappings where required,
unit-normalized result deltas, diagnostics, settings, and tolerance profiles.
It does not ingest commercial prover outputs comprehensively or determine
external validation.

## Readiness Evidence

`DEV-001_BLOCKER_QUEUE.csv` marks `DEL-14-04` as `UNBLOCKED` with 11 active
upstreams satisfied and zero blocking upstreams. Local dependency mirror state
is `SYNCHRONIZED_FROM_APPROVED_DAG002_MIRROR_PRESENT`; the local
`Dependencies.csv` is evidence synchronized from approved `DAG-002`, not
independent sequencing authority.

| EdgeID | Upstream | Evidence |
|---|---|---|
| `DAG-002-E0688` | `DEL-00-01` Architecture decision record baseline | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0689` | `DEL-00-02` Repository and module boundary architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0690` | `DEL-00-03` Application service command-query-job model | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0691` | `DEL-00-04` Persistence and schema versioning architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0692` | `DEL-00-06` Diagnostics, warning, and result-envelope contract | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0693` | `DEL-00-07` API boundary and adapter contract map | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0694` | `DEL-00-08` Layered software test and acceptance strategy | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0795` | `DEL-14-02` Analysis run records | `COMMITTED 002263b` |
| `DAG-002-E0796` | `DEL-14-05` Comparison mapping, tolerance, and export contracts | `COMMITTED 05878bf` |
| `DAG-002-E0797` | `DEL-08-04` Result export format | `COMMITTED 3e33ea4` |
| `DAG-002-E0798` | `DEL-02-02` Unit system and dimensional-analysis core contract | `COMMITTED a458cba` |

Candidate rows remain excluded from readiness and dispatch authority.

## Applicable Invariants

Apply the architecture basis IDs named in `_CONTEXT.md`: `AB-00-01`,
`AB-00-02`, `AB-00-03`, `AB-00-04`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.
Apply only the applicable constraints; do not copy full `PKG-00` prose into
deliverable artifacts.

Apply the project invariants from `docs/CONTRACT.md`, especially:

- `OPS-K-IP-1`, `OPS-K-IP-2`, `OPS-K-IP-3`
- `OPS-K-DATA-1`, `OPS-K-DATA-2`, `OPS-K-DATA-3`
- `OPS-K-AUTH-1`, `OPS-K-AUTH-2`
- `OPS-K-ID-1`
- `OPS-K-UNIT-1`
- `OPS-K-REPORT-1`, `OPS-K-REPORT-2`
- `OPS-K-PRIV-1`
- `OPS-K-AGENT-1`, `OPS-K-AGENT-2`, `OPS-K-AGENT-3`, `OPS-K-AGENT-4`

## Allowed Write Scope For Later Authorized Implementation

- New analysis-run comparison module under `core/comparison/analysis_run/` or
  an equivalently narrow path named in the implementation gate
- `tests/test_analysis_run_comparison.py`
- optional comparison result contract fixtures only if scoped to analysis-run
  comparison and not shared with `DEL-14-03`
- deliverable-local `MEMORY.md`
- deliverable-local run notes under the `DEL-14-04` folder

Do not edit `docs/SPEC.md` or `docs/TYPES.md` from a parallel worker. Shared
documentation integration, if needed, belongs to ORCHESTRATOR after
implementation return or to a later closeout gate with explicit scope.

Do not edit local `Dependencies.csv`, `_DEPENDENCIES.md`, lifecycle
`_STATUS.md`, implementation-evidence registers, blocker queues,
dependency-status projections, aggregate DAG files, candidate rows, or
committed evidence state.

## Tasks For Future Implementation

1. Define a deterministic analysis-run comparison engine over analysis-run
   records or references.
2. Preserve run context: exact model-state binding, solver version, settings,
   units, load cases, diagnostics, results, rule-pack references, library
   references, and result hashes where present.
3. Use stable IDs and explicit mapping records where required for run-result
   entity matching.
4. Compute unit-normalized result deltas only where unit and dimension metadata
   permit valid comparison.
5. Apply tolerance-profile references to classification without inventing
   default project thresholds or protected code acceptance values.
6. Emit diagnostics for missing mappings, incompatible units, missing result
   data, unsupported categories, unresolved assumptions, and professional
   boundary limits.
7. Add focused result-delta tests and record implementation memory/run notes.

## Acceptance Criteria

- The same two analysis runs, mappings, tolerances, and settings produce
  deterministic comparison output.
- Run metadata and exact model-state binding are preserved or surfaced in the
  comparison context.
- Unit-normalized deltas are produced only for compatible unit/dimension
  metadata; incompatible or missing units produce diagnostics.
- Raw delta evidence is preserved separately from tolerance-profile-based
  classification.
- Diagnostics and settings deltas remain diagnostic/audit evidence, not
  external validation or engineering acceptance.
- Outputs do not claim certification, sealing, authentication, code
  compliance, professional approval, external validation, or engineering
  acceptance.

## Required Verification For Future Implementation

- Focused analysis-run comparison/result-delta tests added by this deliverable.
- Adjacent checks where referenced:
  `python3 tests/test_analysis_run_schema.py`,
  `python3 tests/test_results_schema.py`,
  `python3 tests/test_comparison_contracts.py`,
  `python3 tests/test_units_schema.py`, and
  `python3 tests/test_model_state_schema.py`.
- `git diff --check`
- focused scans for protected standards data, private project data,
  proprietary examples, real secrets, and prohibited certification/compliance/
  sealing/professional-approval claims.

## Stop Conditions

Stop before implementation, worker launch, lifecycle/evidence update, blocker
refresh, dependency refresh, aggregate DAG mutation, candidate promotion,
commit, push, protected standards content, private project data, real secrets,
GUI runtime work, external prover integration, commercial-tool parser work,
hard-coded acceptance tolerances, direct accepted-model mutation, or
professional/code compliance claims unless separately authorized.
