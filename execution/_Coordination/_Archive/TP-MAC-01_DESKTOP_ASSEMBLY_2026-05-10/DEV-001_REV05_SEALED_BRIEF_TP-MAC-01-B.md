---
doc_id: DEV-001-REV05-SEALED-BRIEF-TP-MAC-01-B
doc_kind: coordination.sealed_product_assembly_brief
status: sealed_brief_prepared_dispatch_not_authorized
created: 2026-05-10
prepared_by: ORCHESTRATOR
tranche: TP-MAC-01
unit_id: TP-MAC-01-B
unit_name: 3D model workspace
primary_deliverables: DEL-07-01, DEL-07-02, DEL-07-08
worker_launch: not_authorized
implementation_dispatch: not_authorized
source_plan: execution/_Coordination/DEV-001_REV05_MACOS_DESKTOP_PRODUCT_ASSEMBLY_TRANCHE_PLAN.md
---

# Sealed Brief - TP-MAC-01-B 3D Model Workspace

## Dispatch Boundary

This brief is prepared for later bounded implementation only. It does not
authorize worker launch, lifecycle/evidence changes, `DAG-002` mutation,
dependency refresh, release creation, protected data, hidden accepted-model
mutation, live solver/prover execution, or professional/code-compliance claims.

## Product Slice

Implement the first interactive desktop model workspace: a Three.js centerline
pipe view backed by an invented model fixture, with synchronized model tree and
read-only property inspector.

## PKG/DEL Basis

| Package | Deliverable | Use in this brief |
|---|---|---|
| `PKG-07` | `DEL-07-01` 3D viewport and centerline editor | 3D centerline geometry, selection, and component/support visual conventions. |
| `PKG-07` | `DEL-07-02` Model tree and property inspector | Tree, selection state, and property inspector records. |
| `PKG-07` | `DEL-07-08` Design-authoring state and comparison workspace | Workspace composition and design-review panels. |

Adjacent fixture/schema consumption: `PKG-03` invented component/section
fixtures may be consumed as data, but no protected catalogs or owner standards
may be added.

Applicable architecture basis: `AB-00-02`, `AB-00-03`, `AB-00-05`,
`AB-00-06`, `AB-00-07`, `AB-00-08`.

## Allowed Write Scope

- `apps/desktop/src/`
- `apps/desktop/src/features/model-workspace/`
- `apps/desktop/src/features/viewport/`
- `apps/desktop/src/features/model-tree/`
- `fixtures/product_preview/`
- focused UI tests under `apps/desktop/`

Do not edit Tauri backend commands, `core/product_preview/`, solver modules,
dependency mirrors, evidence registers, or lifecycle files unless separately
authorized by another `TP-MAC-01` brief.

## Required Output

- Invented product-preview piping model fixture with stable IDs, units,
  nodes, pipe segments, supports, simple components, and provenance notices.
- Three.js renderer for centerline pipe geometry, nodes, supports, and simple
  component markers.
- Model tree synchronized with 3D selection.
- Read-only property inspector for selected entity details.
- Stable layout slots for design knowledge, diagnostics, solve status, and
  agent proposals, even if those panels are populated by other units.

## Acceptance Criteria

- The 3D canvas is nonblank and visibly renders the invented pipe model.
- Selecting a model tree row highlights or identifies the corresponding 3D
  entity and updates the property inspector.
- The fixture contains only invented data and no standards-derived values.
- The renderer does not silently infer missing engineering values.
- Text and UI elements do not overlap in normal desktop viewport.

## Required Verification

```text
npm run build --workspace apps/desktop
npm test --workspace apps/desktop
python3 -m pytest tests/test_viewport_editor_contract.py tests/test_model_tree_property_inspector.py
git diff --check
```

Final review should include a screenshot or Playwright/browser verification of
the nonblank Three.js scene.

## Stop Conditions

Stop before adding live solve execution, accepted model mutation, protected
data, external tool integration, or release/professional claims.
