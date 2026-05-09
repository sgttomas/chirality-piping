---
doc_id: DEV-001-REV05-SEALED-BRIEF-DEL-07-03
doc_kind: coordination.sealed_type2_brief
status: sealed_brief_prepared_dispatch_not_authorized
created: 2026-05-07
prepared_by: ORCHESTRATOR
tranche: DEV-001_REV05_TRANCHE_L
deliverable_id: DEL-07-03
package_id: PKG-07
worker_launch: not_authorized
implementation_lane: gui_material_component_rule_pack_editors
source_assessment: execution/_Coordination/DEV-001_REV05_POST_TRANCHE_K_NEXT_STEP_ASSESSMENT.md
---

# Sealed Brief - DEL-07-03 Material, Component, And Rule-Pack Editors

## Dispatch Boundary

This sealed Type 2 implementation brief is prepared for later bounded worker
dispatch only. It does not authorize worker launch, lifecycle/evidence
promotion, blocker refresh, dependency refresh, aggregate DAG mutation,
candidate promotion, commit, push, protected data, private project data, live
desktop shell packaging, or professional/code-compliance claims.

`DEL-07-03` has context envelope `L` / `WATCH`; if implementation scope grows
beyond pure editor contract behavior, stop and escalate before broadening.

## Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-07-03` |
| PackageID | `PKG-07` |
| Name | Material, component, and rule-pack editors |
| Type | `UX_UI_SLICE` |
| Scope item | `SOW-021` |
| Objectives | `OBJ-006` |
| Context envelope | `L` / `WATCH` |
| Lifecycle at brief prep | `SEMANTIC_READY` |
| Evidence state at brief prep | `MISSING_EVIDENCE` |
| Deliverable path | `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-03_Material, component, and rule-pack editors` |

## Scope And Objective

Implement deterministic editor contract behavior for user/private materials,
components, and rule-pack references. The slice should represent editor field
definitions, validation states, provenance/source indicators, private-library
references, rule-pack checksum/lifecycle cues, and save/cancel intent records
without storing private payloads or bypassing domain/rule-pack schemas.

## Readiness Evidence

`DEV-001_BLOCKER_QUEUE.csv` marks `DEL-07-03` as `UNBLOCKED` with 13 active
upstreams satisfied and zero blocking upstreams.

Active upstream basis:

- `ARCHITECTURE_BASELINE`: `DEL-00-01`, `DEL-00-02`, `DEL-00-03`,
  `DEL-00-05`, `DEL-00-06`, `DEL-00-07`, `DEL-00-08`.
- `COMMITTED 3793e87`: `DEL-03-01` Material library schema with provenance.
- `COMMITTED f0fdeac`: `DEL-03-02` Pipe section and component library schema.
- `COMMITTED 4d880b3`: `DEL-03-07` Public/private library import provenance checker.
- `COMMITTED 20241f9`: `DEL-06-01` Rule-pack schema.
- `COMMITTED ad270f6`: `DEL-06-04` Private rule-pack lifecycle and checksum handling.
- `COMMITTED 84e0a73`: `DEL-12-01` Local-first storage and private data paths.

Candidate rows remain excluded from readiness and dispatch authority.

## Applicable Invariants

Apply architecture basis IDs from `_CONTEXT.md`: `AB-00-01`, `AB-00-02`,
`AB-00-03`, `AB-00-05`, `AB-00-06`, `AB-00-07`, and `AB-00-08`. Apply
`docs/CONTRACT.md` IP/data, authorization, unit, provenance, privacy,
protected-content, and agent-boundary controls.

## Allowed Write Scope For Later Authorized Implementation

- `core/gui/editors/`
- `tests/test_gui_editors_contract.py`
- invented GUI fixtures scoped to editor behavior
- deliverable-local `MEMORY.md`
- deliverable-local run notes under the `DEL-07-03` folder

Do not edit lifecycle/evidence/blocker/dependency/DAG/candidate state.

## Tasks For Future Implementation

1. Define material, component, and rule-pack editor state records with field
   descriptors, unit metadata, provenance/source state, private/public library
   classification, and validation messages.
2. Represent rule-pack reference lifecycle and checksum cues without copying
   private rule-pack payloads.
3. Represent save/cancel/apply intents as bounded command descriptors, not as
   production persistence or mutation workflows.
4. Add invented fixtures, focused tests, and implementation memory/run notes.

## Acceptance Criteria

- Editor records are deterministic and schema-aligned.
- Private/protected payloads are never embedded in public fixtures.
- Invalid/missing values remain explicit diagnostics or unresolved `TBD`s.
- Outputs do not claim certification, sealing, code compliance, professional
  approval, or engineering acceptance.

## Required Verification For Future Implementation

- Focused `DEL-07-03` GUI editor tests.
- Adjacent material/component schema, rule-pack schema/lifecycle,
  library-provenance, local-first storage, units, and redaction/export-control
  checks where applicable.
- `git diff --check`.
- Focused protected/private/secret/prohibited-claim scans.

## Stop Conditions

Stop before implementing broad editor runtime, production persistence,
protected/private payload handling, lifecycle/evidence updates, blocker or DAG
changes, candidate promotion, commit, push, or professional/code-compliance
claims unless separately authorized.
