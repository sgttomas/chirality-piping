---
doc_id: DEV-001-REV05-SEALED-BRIEF-TP-MAC-01-C
doc_kind: coordination.sealed_product_assembly_brief
status: sealed_brief_prepared_dispatch_not_authorized
created: 2026-05-10
prepared_by: ORCHESTRATOR
tranche: TP-MAC-01
unit_id: TP-MAC-01-C
unit_name: Application service bridge
primary_deliverables: DEL-02-01, DEL-02-02, DEL-02-05, DEL-10-05
worker_launch: not_authorized
implementation_dispatch: not_authorized
source_plan: execution/_Coordination/DEV-001_REV05_MACOS_DESKTOP_PRODUCT_ASSEMBLY_TRANCHE_PLAN.md
---

# Sealed Brief - TP-MAC-01-C Application Service Bridge

## Dispatch Boundary

This brief is prepared for later bounded implementation only. It does not
authorize worker launch, lifecycle/evidence changes, blocker refresh,
dependency refresh, graph mutation, release work, protected data, private
project data, or professional/code-compliance claims.

## Product Slice

Implement a local command/query bridge that lets the desktop preview load,
validate, save, and query the invented preview model through schema-backed
records and structured result envelopes.

## PKG/DEL Basis

| Package | Deliverable | Use in this brief |
|---|---|---|
| `PKG-02` | `DEL-02-01` Canonical domain model schema | Model entity contracts and physical-model source-of-truth boundary. |
| `PKG-02` | `DEL-02-02` Unit system and dimensional-analysis core contract | Unit metadata and no-silent-default behavior. |
| `PKG-02` | `DEL-02-05` Project persistence and round-trip serialization | Local JSON persistence, schema versioning, and deterministic round-trip expectations. |
| `PKG-10` | `DEL-10-05` Headless CLI and structured I/O analysis runner | Structured command/query/result envelopes and local execution discipline. |

Applicable architecture basis: `AB-00-02`, `AB-00-03`, `AB-00-04`,
`AB-00-06`, `AB-00-07`, `AB-00-08`.

## Allowed Write Scope

- `core/product_preview/`
- `core/application_service/`
- `apps/desktop/src-tauri/src/`
- `tests/product_preview/`
- narrow generated or handwritten TypeScript/Rust bridge types under
  `apps/desktop/src/` only where required to call the service

Do not edit GUI feature panels owned by other `TP-MAC-01` briefs except for
narrow integration imports. Do not edit schemas unless a mismatch is found and
escalated for a separate scope decision.

## Required Output

- Local preview service API for:
  - load invented model;
  - validate schema/version/unit metadata;
  - save or round-trip model JSON in a local preview path;
  - query model tree/property data;
  - return structured diagnostics and status envelopes.
- Tauri command bindings or equivalent local bridge from the desktop app to
  the preview service.
- Focused tests for load/validate/round-trip/query behavior.

## Acceptance Criteria

- The invented preview model can be loaded through the desktop bridge.
- Invalid or incomplete input returns explicit diagnostics, not silent fixes.
- Round-trip behavior preserves stable IDs and schema version fields.
- Result envelopes distinguish data validation, mechanics readiness, rule-check
  readiness, and professional acceptance as separate states.
- No private path, secret, or protected standards data is introduced.

## Required Verification

```text
python3 -m pytest tests/product_preview tests/test_model_schema.py tests/test_units_schema.py tests/test_persistence_schema.py tests/test_headless_runner_contract.py
npm run build --workspace apps/desktop
git diff --check
```

## Stop Conditions

Stop before changing canonical schemas, adding cloud or remote service
behavior, accepting hidden model mutations, introducing real private data, or
claiming release/professional readiness.
