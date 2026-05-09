---
doc_id: DEV-001-REV05-SEALED-BRIEF-DEL-07-04
doc_kind: coordination.sealed_type2_brief
status: sealed_brief_prepared_dispatch_not_authorized
created: 2026-05-07
prepared_by: ORCHESTRATOR
tranche: DEV-001_REV05_TRANCHE_L
deliverable_id: DEL-07-04
package_id: PKG-07
worker_launch: not_authorized
implementation_lane: gui_missing_data_warning_blocking_ux
source_assessment: execution/_Coordination/DEV-001_REV05_POST_TRANCHE_K_NEXT_STEP_ASSESSMENT.md
---

# Sealed Brief - DEL-07-04 Missing-Data Warning And Blocking UX

## Dispatch Boundary

This sealed Type 2 implementation brief is prepared for later bounded worker
dispatch only. It does not authorize worker launch, lifecycle/evidence
promotion, blocker refresh, dependency refresh, aggregate DAG mutation,
candidate promotion, commit, push, protected data, private project data, live
GUI runtime packaging, or professional/code-compliance claims.

## Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-07-04` |
| PackageID | `PKG-07` |
| Name | Missing-data warning and blocking UX |
| Type | `UX_UI_SLICE` |
| Scope item | `SOW-022` |
| Objectives | `OBJ-006`, `OBJ-011` |
| Context envelope | `M` |
| Lifecycle at brief prep | `SEMANTIC_READY` |
| Evidence state at brief prep | `MISSING_EVIDENCE` |
| Deliverable path | `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-04_Missing-data warning and blocking UX` |

## Scope And Objective

Implement deterministic GUI warning/blocking contract behavior for
solve-required, code-check-required, provenance-missing, assumption,
incomplete-data, diagnostic, and professional-boundary states. The slice should
make blocking reasons visible without silently supplying data or turning
software diagnostics into professional acceptance.

## Readiness Evidence

`DEV-001_BLOCKER_QUEUE.csv` marks `DEL-07-04` as `UNBLOCKED` with 11 active
upstreams satisfied and zero blocking upstreams.

Active upstream basis:

- `ARCHITECTURE_BASELINE`: `DEL-00-01`, `DEL-00-02`, `DEL-00-03`,
  `DEL-00-05`, `DEL-00-06`, `DEL-00-07`, `DEL-00-08`.
- `COMMITTED abc1306`: `DEL-02-03` Code-neutral analysis boundary model.
- `COMMITTED dbaf21e`: `DEL-05-04` Analysis status semantics.
- `COMMITTED c075522`: `DEL-06-03` Required-input completeness checker.
- `COMMITTED fdb0252`: `DEL-04-06` Solver diagnostics and singularity detection.

Candidate rows remain excluded from readiness and dispatch authority.

## Applicable Invariants

Apply architecture basis IDs from `_CONTEXT.md`: `AB-00-01`, `AB-00-02`,
`AB-00-03`, `AB-00-05`, `AB-00-06`, `AB-00-07`, and `AB-00-08`. Apply
`docs/CONTRACT.md` IP/data, authorization, unit, diagnostics, reporting,
privacy, and agent-boundary controls.

## Allowed Write Scope For Later Authorized Implementation

- `core/gui/warnings/`
- `tests/test_missing_data_warning_ux.py`
- invented GUI fixtures scoped to warning/blocking states
- deliverable-local `MEMORY.md`
- deliverable-local run notes under the `DEL-07-04` folder

Do not edit lifecycle/evidence/blocker/dependency/DAG/candidate state.

## Tasks For Future Implementation

1. Define warning and blocking-state records for solve-required,
   rule-check-required, provenance-missing, assumption, incomplete data,
   diagnostics, and professional-boundary conditions.
2. Define deterministic severity, blocking, target, remediation-text, and
   source/provenance fields.
3. Preserve analysis status and completeness-check distinctions without
   replacing them with GUI-only pass/fail states.
4. Add invented fixtures, focused tests, and implementation memory/run notes.

## Acceptance Criteria

- Warning records distinguish blocking from non-blocking states.
- Missing required data remains explicit and is never auto-filled.
- Diagnostics and completeness states retain source/provenance context.
- Outputs do not claim certification, sealing, code compliance, professional
  approval, or engineering acceptance.

## Required Verification For Future Implementation

- Focused `DEL-07-04` warning/blocking UX tests.
- Adjacent analysis boundary, analysis status, completeness checker, solver
  diagnostics, units, and redaction/export-control checks where applicable.
- `git diff --check`.
- Focused protected/private/secret/prohibited-claim scans.

## Stop Conditions

Stop before implementing live solver/rule execution, hidden data repair,
professional approval logic, lifecycle/evidence updates, blocker or DAG
changes, candidate promotion, commit, push, or protected/private data use
unless separately authorized.
