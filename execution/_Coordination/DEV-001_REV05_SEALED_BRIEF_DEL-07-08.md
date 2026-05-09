---
doc_id: DEV-001-REV05-SEALED-BRIEF-DEL-07-08
doc_kind: coordination.sealed_type2_brief
status: sealed_brief_prepared_dispatch_not_authorized
created: 2026-05-09
prepared_by: ORCHESTRATOR
tranche: DEV-001_REV05_TRANCHE_M
deliverable_id: DEL-07-08
package_id: PKG-07
worker_launch: not_authorized
implementation_lane: gui_design_authoring_comparison_workspace
source_assessment: execution/_Coordination/DEV-001_REV05_POST_TRANCHE_L_NEXT_STEP_ASSESSMENT.md
---

# Sealed Brief - DEL-07-08 Design-Authoring State And Comparison Workspace

## Dispatch Boundary

This sealed Type 2 implementation brief is prepared for later bounded worker
dispatch only. It does not authorize worker launch, lifecycle/evidence
promotion, blocker refresh, dependency refresh, aggregate DAG mutation,
candidate promotion, commit, push, live GUI runtime work, solver/prover
execution, protected data, private project data, real secrets, hidden
accepted-model mutation, or professional/code-compliance claims.

The accepted lane is deterministic GUI-facing contract behavior for design
knowledge panels, operation/diff review, state/run browsing, comparison
tables, and graphical comparison overlay descriptors.

## Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-07-08` |
| PackageID | `PKG-07` |
| Name | Design-authoring state and comparison workspace |
| Type | `UX_UI_SLICE` |
| Scope item | `SOW-076` |
| Objectives | `OBJ-015`, `OBJ-016` |
| Context envelope | `L` / `WATCH` |
| Lifecycle at brief prep | `SEMANTIC_READY` |
| Evidence state at brief prep | `MISSING_EVIDENCE` |
| Deliverable path | `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-08_Design-authoring state and comparison workspace` |

Use `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.5`,
`docs/_Registers/Deliverables.csv`, approved `DAG-002`, and the current
coordination readiness surfaces.

## Scope And Objective

Implement deterministic workspace contract records for design-authoring and
comparison review. The slice should represent panel state, operation preview,
state/run browser summaries, comparison rows, graphical overlay descriptors,
constraint/warning presentation, and review-state routing.

Package exclusions remain binding: do not silently supply missing code data,
do not mutate accepted model state, do not execute solver/prover work, and do
not convert comparison or design-knowledge output into professional approval.

## Readiness Evidence

`DEV-001_BLOCKER_QUEUE.csv` marks `DEL-07-08` as `UNBLOCKED` with 21 active
upstreams satisfied and zero blocking upstreams. Local dependency mirror state
is `SYNCHRONIZED_FROM_APPROVED_DAG002_MIRROR_PRESENT`; the local
`Dependencies.csv` is evidence synchronized from approved `DAG-002`, not
independent sequencing authority.

| EdgeID | Upstream | Evidence |
|---|---|---|
| `DAG-002-E0625` | `DEL-00-01` Architecture decision record baseline | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0626` | `DEL-00-02` Repository and module boundary architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0627` | `DEL-00-03` Application service command-query-job model | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0628` | `DEL-00-05` GUI state and interaction architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0629` | `DEL-00-06` Diagnostics, warning, and result-envelope contract | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0630` | `DEL-00-07` API boundary and adapter contract map | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0631` | `DEL-00-08` Layered software test and acceptance strategy | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0840` | `DEL-07-01` 3D viewport and centerline editor | `COMMITTED 4785806` |
| `DAG-002-E0841` | `DEL-07-02` Model tree and property inspector | `COMMITTED 6e0b8f4` |
| `DAG-002-E0842` | `DEL-07-04` Missing-data warning and blocking UX | `COMMITTED 6e0b8f4` |
| `DAG-002-E0843` | `DEL-07-05` Results viewer | `COMMITTED 6e0b8f4` |
| `DAG-002-E0844` | `DEL-13-01` Design knowledge schema and provenance model | `COMMITTED dcdc1ac` |
| `DAG-002-E0845` | `DEL-13-03` Constraint validation engine | `COMMITTED 05878bf` |
| `DAG-002-E0846` | `DEL-13-04` Physical-to-analytical transformation contract | `COMMITTED 24b5717` |
| `DAG-002-E0847` | `DEL-14-01` Immutable model state records | `COMMITTED dcdc1ac` |
| `DAG-002-E0848` | `DEL-14-03` Model-state comparison engine | `COMMITTED 24b5717` |
| `DAG-002-E0849` | `DEL-14-04` Analysis-run comparison engine | `COMMITTED 24b5717` |
| `DAG-002-E0850` | `DEL-14-05` Comparison mapping, tolerance, and export contracts | `COMMITTED 05878bf` |
| `DAG-002-E0851` | `DEL-16-01` Structured model operation schema | `COMMITTED 002263b` |
| `DAG-002-E0852` | `DEL-16-02` Operation validation and diff preview | `COMMITTED c08b0a2` |
| `DAG-002-E0853` | `DEL-16-03` User acceptance and operation audit trail | `COMMITTED 4601724` |

Candidate rows remain excluded from readiness and dispatch authority.

## Applicable Invariants

Apply the architecture basis IDs named in `_CONTEXT.md`: `AB-00-01`,
`AB-00-02`, `AB-00-03`, `AB-00-05`, `AB-00-06`, `AB-00-07`, and
`AB-00-08`. Apply only the applicable constraints; do not copy full `PKG-00`
prose into deliverable artifacts.

Apply the project invariants from `docs/CONTRACT.md`, especially:

- `OPS-K-IP-1`, `OPS-K-IP-2`, `OPS-K-IP-3`
- `OPS-K-DATA-1`, `OPS-K-DATA-2`, `OPS-K-DATA-3`
- `OPS-K-AUTH-1`, `OPS-K-AUTH-2`
- `OPS-K-MECH-2`
- `OPS-K-UNIT-1`
- `OPS-K-REPORT-1`, `OPS-K-PRIV-1`, `OPS-K-PRIV-2`
- `OPS-K-AGENT-1`, `OPS-K-AGENT-2`, `OPS-K-AGENT-3`, `OPS-K-AGENT-4`

## Allowed Write Scope For Later Authorized Implementation

- `core/gui/design_workspace/`
- focused tests such as
  `tests/test_design_authoring_comparison_workspace.py`
- invented GUI fixtures scoped to design-authoring/comparison review
- deliverable-local `MEMORY.md`
- deliverable-local run notes under the `DEL-07-08` folder

Do not edit existing GUI, operation, state, comparison, or design-knowledge
modules except through narrow imports or adapters needed to consume their
public contract records. Do not edit local `Dependencies.csv`,
`_DEPENDENCIES.md`, lifecycle `_STATUS.md`, implementation-evidence
registers, blocker queues, dependency-status projections, aggregate DAG files,
candidate rows, or committed evidence state.

## Tasks For Future Implementation

1. Define workspace records for design knowledge panel state, constraint and
   warning presentation, state/run browser summaries, model-state comparison
   rows, analysis-run comparison rows, graphical overlay descriptors, and
   operation diff-review state.
2. Consume committed design-knowledge, constraint validation, state/run,
   comparison, model-operation, and GUI contract outputs without mutating
   accepted model state.
3. Preserve stable IDs, hashes, provenance, diagnostics, warnings,
   assumptions, review state, privacy classification, tolerance/profile
   references, unmatched classifications, and unresolved `TBD`s.
4. Surface missing data and unavailable comparison/preview inputs as explicit
   findings, not defaults.
5. Add focused invented fixtures, tests, and implementation memory/run notes.

## Acceptance Criteria

- Workspace records are deterministic for the same invented contract inputs.
- Design knowledge, warning, state/run, comparison, overlay, and operation
  review views are represented through inspectable records.
- Proposed operations cannot be represented as accepted model mutations unless
  the accepted-operation contract already records explicit user acceptance.
- Missing data, unmatched comparison rows, unresolved assumptions, and
  unavailable preview inputs remain visible.
- Outputs do not claim certification, sealing, authentication, code
  compliance, professional approval, external validation, or engineering
  acceptance.

## Required Verification For Future Implementation

- Focused design-authoring/comparison workspace tests.
- Adjacent checks where referenced:
  `python3 tests/test_design_knowledge_schema.py`,
  `python3 tests/test_operation_validation_preview.py`,
  `python3 tests/test_operation_audit_trail.py`,
  `python3 tests/test_model_state_schema.py`,
  `python3 tests/test_model_state_comparison.py`,
  `python3 tests/test_analysis_run_comparison.py`,
  `python3 tests/test_comparison_contracts.py`,
  `python3 tests/test_missing_data_warning_ux.py`, and
  `python3 tests/test_results_viewer_contract.py`.
- `git diff --check`.
- Focused scans for protected standards data, private project data,
  proprietary examples, real secrets, and prohibited certification/compliance/
  sealing/professional-approval claims.

## Stop Conditions

Stop before implementation, worker launch, lifecycle/evidence update, blocker
refresh, dependency refresh, aggregate DAG mutation, candidate promotion,
commit, push, protected standards content, private project data, real secrets,
live GUI runtime work, solver/prover execution, hidden accepted-model
mutation, autonomous engineering acceptance, external validation, or
professional/code compliance claims unless separately authorized.
