---
doc_id: DEV-001-REV05-SEALED-BRIEF-DEL-07-02
doc_kind: coordination.sealed_type2_brief
status: sealed_brief_prepared_dispatch_not_authorized
created: 2026-05-07
prepared_by: ORCHESTRATOR
tranche: DEV-001_REV05_TRANCHE_L
deliverable_id: DEL-07-02
package_id: PKG-07
worker_launch: not_authorized
implementation_lane: gui_model_tree_property_inspector
source_assessment: execution/_Coordination/DEV-001_REV05_POST_TRANCHE_K_NEXT_STEP_ASSESSMENT.md
---

# Sealed Brief - DEL-07-02 Model Tree And Property Inspector

## Dispatch Boundary

This sealed Type 2 implementation brief is prepared for later bounded worker
dispatch only. It does not authorize worker launch, lifecycle or evidence
promotion, blocker refresh, dependency refresh, aggregate DAG mutation,
candidate promotion, commit, push, final desktop shell integration, live GUI
runtime packaging, protected data, private project data, or professional/code
compliance claims.

## Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-07-02` |
| PackageID | `PKG-07` |
| Name | Model tree and property inspector |
| Type | `UX_UI_SLICE` |
| Scope items | `SOW-020`, `SOW-021` |
| Objectives | `OBJ-006` |
| Context envelope | `M` |
| Lifecycle at brief prep | `SEMANTIC_READY` |
| Evidence state at brief prep | `MISSING_EVIDENCE` |
| Deliverable path | `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector` |

## Scope And Objective

Implement deterministic model-tree and property-inspector contract behavior for
selected model entities. The slice should expose navigation nodes, selected
entity identity, editable/read-only property descriptors, validation state,
source/provenance indicators, unit metadata, and persistence-round-trip
affordances without mutating solver/domain records directly or inventing
missing engineering data.

Package exclusions remain binding: do not silently supply missing code data,
do not implement final desktop shell presentation, do not create a production
persistence service, and do not claim professional review, certification,
sealing, code compliance, or engineering acceptance.

## Readiness Evidence

`DEV-001_BLOCKER_QUEUE.csv` marks `DEL-07-02` as `UNBLOCKED` with 10 active
upstreams satisfied and zero blocking upstreams. Local dependency mirror state
is synchronized from approved `DAG-002`.

Active upstream basis:

- `ARCHITECTURE_BASELINE`: `DEL-00-01`, `DEL-00-02`, `DEL-00-03`,
  `DEL-00-05`, `DEL-00-06`, `DEL-00-07`, `DEL-00-08`.
- `COMMITTED 7b256f3`: `DEL-02-01` Canonical domain model schema.
- `COMMITTED 742016e`: `DEL-02-05` Project persistence and round-trip serialization.
- `COMMITTED f0fdeac`: `DEL-03-02` Pipe section and component library schema.

Candidate rows remain excluded from readiness and dispatch authority.

## Applicable Invariants

Apply architecture basis IDs from `_CONTEXT.md`: `AB-00-01`, `AB-00-02`,
`AB-00-03`, `AB-00-05`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.

Apply the project invariants from `docs/CONTRACT.md`, especially the IP/data,
authorization, unit, report/provenance, privacy, and agent-boundary controls.

## Allowed Write Scope For Later Authorized Implementation

- `core/gui/model_tree/`
- `tests/test_model_tree_property_inspector.py`
- invented GUI fixtures scoped to model-tree/property-inspector behavior
- deliverable-local `MEMORY.md`
- deliverable-local run notes under the `DEL-07-02` folder

Do not edit local `Dependencies.csv`, `_DEPENDENCIES.md`, lifecycle
`_STATUS.md`, implementation-evidence registers, blocker queues,
dependency-status projections, aggregate DAG files, candidate rows, or
committed evidence state.

## Tasks For Future Implementation

1. Define model-tree node records for projects, model entities, materials,
   components, loads, results, reports, warnings, and provenance placeholders
   using stable IDs and deterministic ordering.
2. Define property-inspector records for selected entities, including field
   labels, editability, unit metadata, source/provenance status, validation
   messages, privacy classification, and unresolved `TBD` values.
3. Preserve round-trip/persistence metadata without implementing production
   storage or silently correcting missing values.
4. Add invented fixtures, focused tests, and implementation memory/run notes.

## Acceptance Criteria

- Model-tree and property-inspector records are deterministic for the same
  invented fixture set.
- Missing required values remain explicit diagnostics, limitations, or
  unresolved `TBD`s.
- Units, provenance, source classification, privacy labels, and validation
  status are preserved where available.
- No protected standards text, private project data, private libraries, real
  secrets, or professional acceptance claims appear in fixtures or outputs.

## Required Verification For Future Implementation

- Focused `DEL-07-02` GUI contract tests.
- Adjacent model schema, persistence, component schema, units, analysis status,
  viewport, and redaction/export-control checks where applicable.
- `git diff --check`.
- Focused scans for protected standards data, private project data, secrets,
  and prohibited certification/compliance/sealing/professional-approval claims.

## Stop Conditions

Stop before implementation outside the allowed write scope, lifecycle/evidence
update, blocker refresh, dependency refresh, aggregate DAG mutation, candidate
promotion, commit, push, protected/private data use, production persistence,
live desktop shell integration, or professional/code-compliance claims unless
separately authorized.
