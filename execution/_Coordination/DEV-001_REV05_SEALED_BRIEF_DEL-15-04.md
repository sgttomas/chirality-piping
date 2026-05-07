---
doc_id: DEV-001-REV05-SEALED-BRIEF-DEL-15-04
doc_kind: coordination.sealed_type2_brief
status: dispatched_working_tree_implementation_handoff
created: 2026-05-07
prepared_by: ORCHESTRATOR
tranche: DEV-001_REV05_TRANCHE_J
deliverable_id: DEL-15-04
package_id: PKG-15
worker_launch: authorized_2026-05-07
implementation_lane: external_prover_boundary_metadata
source_proposal: execution/_Coordination/DEV-001_REV05_TRANCHE_J_PROPOSAL.md
implementation_handoff: execution/_Coordination/DEV-001_REV05_TRANCHE_J_IMPLEMENTATION_HANDOFF.md
---

# Sealed Brief - DEL-15-04 External Prover Boundary Metadata

## Dispatch Boundary

This is a sealed Type 2 implementation brief for future worker dispatch. It
does not authorize worker launch or implementation by itself. Use only after
the human separately approves implementation dispatch for Tranche J sealed
briefs.

The accepted lane is flexible external-prover workflow metadata without a
formal prover-status lifecycle, automatic professional acceptance record,
hard-coded approval/certification/code-compliance statuses, target-specific
commercial parser behavior, or comprehensive commercial-tool result ingestion.
This brief does not authorize lifecycle or evidence promotion, blocker refresh,
dependency refresh, aggregate DAG mutation, candidate promotion, commit, push,
external solver/prover execution, protected data, private project data, or
professional/code-compliance claims.

## Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-15-04` |
| PackageID | `PKG-15` |
| Name | External prover boundary metadata |
| Type | `DATA_MODEL_CHANGE` |
| Scope item | `SOW-075` |
| Objectives | `OBJ-017`, `OBJ-018` |
| Context envelope | `M` |
| Lifecycle at brief prep | `SEMANTIC_READY` |
| Evidence state at brief prep | `MISSING_EVIDENCE` |
| Deliverable path | `execution/PKG-15_Handoff and External Prover Workflow/1_Working/DEL-15-04_External prover boundary metadata` |

## Scope And Objective

Implement a metadata contract for external-prover workflow references that can
record flexible names, tags, notes, external references, attachments, target
mapping links, handoff/export references, assumptions, warnings, and
unsupported-target flags as non-authoritative metadata.

Package exclusions remain binding: do not force a prover-status lifecycle,
generate professional approval records, implement commercial parser behavior,
invoke external solvers/provers, ingest commercial-tool results
comprehensively, or imply downstream validation/certification.

## Readiness Evidence

`DEV-001_BLOCKER_QUEUE.csv` marks `DEL-15-04` as `UNBLOCKED` with 12 active
upstreams satisfied and zero blocking upstreams. Local dependency mirror state
is `SYNCHRONIZED_FROM_APPROVED_DAG002_MIRROR_PRESENT`; the local
`Dependencies.csv` is evidence synchronized from approved `DAG-002`, not
independent sequencing authority.

| EdgeID | Upstream | Evidence |
|---|---|---|
| `DAG-002-E0723` | `DEL-00-01` Architecture decision record baseline | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0724` | `DEL-00-02` Repository and module boundary architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0725` | `DEL-00-03` Application service command-query-job model | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0726` | `DEL-00-04` Persistence and schema versioning architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0727` | `DEL-00-06` Diagnostics, warning, and result-envelope contract | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0728` | `DEL-00-07` API boundary and adapter contract map | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0729` | `DEL-00-08` Layered software test and acceptance strategy | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0818` | `DEL-01-04` Professional responsibility and product-claims policy | `COMMITTED 65f3119` |
| `DAG-002-E0819` | `DEL-15-01` Canonical handoff package schema and manifest | `COMMITTED 05878bf` |
| `DAG-002-E0820` | `DEL-15-02` Target mapping and unsupported-behavior contract | `COMMITTED c08b0a2` |
| `DAG-002-E0821` | `DEL-15-03` Downstream modeling export workflow | `COMMITTED 4601724` |
| `DAG-002-E0822` | `DEL-14-01` Immutable model state records | `COMMITTED dcdc1ac` |

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
- `OPS-K-UNIT-1`
- `OPS-K-REPORT-1`, `OPS-K-REPORT-2`
- `OPS-K-PRIV-1`
- `OPS-K-AGENT-1`, `OPS-K-AGENT-2`, `OPS-K-AGENT-3`, `OPS-K-AGENT-4`

## Allowed Write Scope For Later Authorized Implementation

- External-prover metadata schema and/or module under `core/handoff/external_prover/`
- Invented metadata fixtures scoped to this deliverable
- Focused tests such as `tests/test_external_prover_boundary_metadata.py`
- deliverable-local `MEMORY.md`
- deliverable-local run notes under the `DEL-15-04` folder

Do not edit `docs/SPEC.md` or `docs/TYPES.md` from a parallel worker. Shared
documentation integration, if needed, belongs to ORCHESTRATOR after
implementation return or to a later closeout gate with explicit scope.

Do not edit local `Dependencies.csv`, `_DEPENDENCIES.md`, lifecycle
`_STATUS.md`, implementation-evidence registers, blocker queues,
dependency-status projections, aggregate DAG files, candidate rows, or
committed evidence state.

## Tasks For Future Implementation

1. Define the external-prover metadata representation as flexible
   non-authoritative workflow metadata.
2. Preserve links to handoff packages, target mappings, export workflow
   records, immutable model states, assumptions, warnings, unsupported-target
   flags, and external references where available.
3. Reject or diagnose hard-coded approval, certification, code-compliance,
   sealing, external-validation, professional-acceptance, and prover-status
   lifecycle claims.
4. Keep external names, tags, notes, references, and attachments as metadata
   only; do not invoke external solvers/provers or ingest commercial results.
5. Create invented metadata fixtures with provenance notes and no commercial
   stress-tool examples or protected standards-derived values.
6. Add focused boundary validation tests and record implementation memory/run
   notes.

## Acceptance Criteria

- Metadata records are deterministic for the same input fixture and reference
  set.
- Metadata can represent external references, names, tags, notes, attachments,
  target mappings, handoff/export links, assumptions, warnings, and
  unsupported-target flags without creating professional approval states.
- Hard-coded approval, certification, code-compliance, sealing,
  external-validation, professional-acceptance, and prover-status lifecycle
  claims are rejected or surfaced as diagnostics.
- Public fixtures use invented or otherwise cleared data only.
- Outputs do not claim certification, sealing, authentication, code
  compliance, professional approval, external validation, or engineering
  acceptance.

## Required Verification For Future Implementation

- Focused external-prover boundary metadata tests.
- Adjacent checks where referenced:
  `python3 tests/test_handoff_package_schema.py`,
  `python3 tests/test_target_mapping_contract.py`,
  `python3 tests/test_handoff_export_workflow.py`,
  `python3 tests/test_model_state_schema.py`,
  `python3 tests/test_analysis_boundary_schema.py`, and
  `python3 tests/test_units_schema.py`.
- `git diff --check`
- focused scans for protected standards data, private project data,
  proprietary examples, real secrets, and prohibited certification/compliance/
  sealing/professional-approval claims.

## Stop Conditions

Stop before implementation, worker launch, lifecycle/evidence update, blocker
refresh, dependency refresh, aggregate DAG mutation, candidate promotion,
commit, push, protected standards content, private project data, real secrets,
target-specific parser behavior, external solver/prover execution,
commercial-prover validation, comprehensive result ingestion, hidden
professional acceptance state, or professional/code compliance claims unless
separately authorized.
