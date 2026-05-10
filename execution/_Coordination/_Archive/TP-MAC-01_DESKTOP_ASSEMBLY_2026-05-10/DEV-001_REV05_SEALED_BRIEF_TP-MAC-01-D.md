---
doc_id: DEV-001-REV05-SEALED-BRIEF-TP-MAC-01-D
doc_kind: coordination.sealed_product_assembly_brief
status: sealed_brief_prepared_dispatch_not_authorized
created: 2026-05-10
prepared_by: ORCHESTRATOR
tranche: TP-MAC-01
unit_id: TP-MAC-01-D
unit_name: Design knowledge and diagnostics
primary_deliverables: DEL-13-01, DEL-13-02, DEL-13-03, DEL-04-06, DEL-07-04
worker_launch: not_authorized
implementation_dispatch: not_authorized
source_plan: execution/_Coordination/DEV-001_REV05_MACOS_DESKTOP_PRODUCT_ASSEMBLY_TRANCHE_PLAN.md
---

# Sealed Brief - TP-MAC-01-D Design Knowledge And Diagnostics

## Dispatch Boundary

This brief is prepared for later bounded implementation only. It does not
authorize worker launch, lifecycle/evidence changes, dependency refresh, graph
mutation, protected data, private owner standards, live professional
acceptance, or code-compliance claims.

## Product Slice

Add the preview design-knowledge and diagnostics layer: a GUI panel and core
feed that exposes invented endpoints, routing context, constraints,
assumptions, warnings, and unsupported/missing-data diagnostics.

## PKG/DEL Basis

| Package | Deliverable | Use in this brief |
|---|---|---|
| `PKG-13` | `DEL-13-01` Design knowledge schema and provenance model | Design knowledge records and provenance display. |
| `PKG-13` | `DEL-13-02` Constraint entity and provenance model | Constraint/warning data shape. |
| `PKG-13` | `DEL-13-03` Constraint validation engine | Deterministic constraint diagnostics. |
| `PKG-04` | `DEL-04-06` Solver diagnostics and singularity detection | Mechanics/solver diagnostic classes and warning semantics. |
| `PKG-07` | `DEL-07-04` Missing-data warning and blocking UX | GUI presentation of missing data and blocking states. |

Applicable architecture basis: `AB-00-03`, `AB-00-05`, `AB-00-06`,
`AB-00-07`, `AB-00-08`.

## Allowed Write Scope

- `core/product_preview/`
- `apps/desktop/src/features/knowledge/`
- `apps/desktop/src/features/diagnostics/`
- `tests/product_preview/`
- additions to `fixtures/product_preview/` for invented design-knowledge and
  constraint records

Do not edit solver implementation, canonical schemas, dependency mirrors,
evidence registers, or lifecycle files.

## Required Output

- Invented design-knowledge records with endpoints, routing corridor or zone
  examples, supportable zone examples, assumptions, source/provenance fields,
  and non-engineering notices.
- Constraint/diagnostic feed for the preview model.
- GUI panel that displays design knowledge, warnings, missing-data states, and
  unsupported behavior without hiding unresolved assumptions.
- Tests proving diagnostics are deterministic for the same fixture.

## Acceptance Criteria

- Design knowledge and constraint messages are visible in the desktop preview.
- Every nontrivial design-knowledge record carries provenance or an explicit
  invented/source note.
- Missing solve-required or rule-check-required data appears as a finding, not
  a default.
- Unsupported behavior is explicitly labeled without implying engineering
  approval.
- No owner standards, protected criteria, protected tables, or private project
  values are introduced.

## Required Verification

```text
python3 -m pytest tests/product_preview tests/test_design_knowledge_schema.py tests/test_constraint_schema.py tests/test_constraint_validation.py tests/test_missing_data_warning_ux.py
npm run build --workspace apps/desktop
git diff --check
```

## Stop Conditions

Stop before adding protected routing/design criteria, owner standards, automatic
approval logic, hidden data repair, or release/professional claims.
