---
doc_id: DEV-001-REV05-SEALED-BRIEF-TP-MAC-01-F
doc_kind: coordination.sealed_product_assembly_brief
status: sealed_brief_prepared_dispatch_not_authorized
created: 2026-05-10
prepared_by: ORCHESTRATOR
tranche: TP-MAC-01
unit_id: TP-MAC-01-F
unit_name: Agentic proposal panel
primary_deliverables: DEL-16-01, DEL-16-02, DEL-16-03, DEL-16-04, DEL-07-08
worker_launch: not_authorized
implementation_dispatch: not_authorized
source_plan: execution/_Coordination/DEV-001_REV05_MACOS_DESKTOP_PRODUCT_ASSEMBLY_TRANCHE_PLAN.md
---

# Sealed Brief - TP-MAC-01-F Agentic Proposal Panel

## Dispatch Boundary

This brief is prepared for later bounded implementation only. It does not
authorize worker launch, lifecycle/evidence changes, graph mutation,
dependency refresh, autonomous accepted-model mutation, professional approval,
code compliance, external validation, protected data, or private project data.

## Product Slice

Implement a bounded agentic proposal panel for the technical preview. The panel
may draft a structured proposed operation against the invented model fixture,
show rationale and diff/validation output, and require explicit user acceptance
before any mutation. Initial implementation may keep mutation disabled and
record the proposal as review-only if accepted persistence mutation is not
ready.

## PKG/DEL Basis

| Package | Deliverable | Use in this brief |
|---|---|---|
| `PKG-16` | `DEL-16-01` Structured model operation schema | Proposed operation data shape. |
| `PKG-16` | `DEL-16-02` Operation validation and diff preview | Validation and diff preview before acceptance. |
| `PKG-16` | `DEL-16-03` User acceptance and operation audit trail | Explicit acceptance boundary and audit metadata. |
| `PKG-16` | `DEL-16-04` Agent rationale and professional-boundary controls | Rationale, assumptions, and prohibited-claim guardrails. |
| `PKG-07` | `DEL-07-08` Design-authoring state and comparison workspace | UI placement and review workflow. |

Applicable architecture basis: `AB-00-03`, `AB-00-05`, `AB-00-06`,
`AB-00-07`, `AB-00-08`.

## Allowed Write Scope

- `core/product_preview/`
- `apps/desktop/src/features/agent-proposals/`
- `tests/product_preview/`
- invented proposal fixtures under `fixtures/product_preview/`

Do not edit external agent SDK/provider integrations, private credentials,
accepted model mutation engines outside preview scope, dependency mirrors,
evidence registers, lifecycle files, or `DAG-002`.

## Required Output

- Proposal UI panel with fields for prompt/context, proposed operation,
  rationale, affected entity IDs, assumptions, validation status, and diff
  preview.
- Deterministic local proposal examples using invented data. No network model
  provider or external agent service is required or authorized.
- Guardrail text and data model ensuring proposals are drafts until accepted
  by a user.
- Tests for proposal validation, diff preview, audit boundary, and prohibited
  professional/code-compliance wording.

## Acceptance Criteria

- A user can generate or load a sample proposed operation for the invented
  model.
- The proposal shows rationale, affected IDs, assumptions, validation result,
  and diff preview.
- The UI and data model cannot silently mutate accepted model state.
- User acceptance, if represented, is explicit and audit-recorded; if mutation
  is disabled, that limitation is clear.
- The panel does not claim engineering approval, certification, sealing, code
  compliance, external validation, or professional reliance.

## Required Verification

```text
python3 -m pytest tests/product_preview tests/test_model_operation_schema.py tests/test_operation_validation_preview.py tests/test_operation_audit_trail.py tests/test_agent_rationale_boundary.py tests/test_design_authoring_comparison_workspace.py
npm run build --workspace apps/desktop
git diff --check
```

## Stop Conditions

Stop before adding external agent/provider integration, autonomous engineering
decisions, hidden accepted-model mutation, private data access, protected data,
or professional/release claims.
