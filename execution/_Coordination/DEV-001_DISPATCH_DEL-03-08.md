---
doc_id: DEV-001-DISPATCH-DEL-03-08
doc_kind: coordination.dispatch_brief
status: launched_single_bounded_item
created: 2026-05-01
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-03-08
package_id: PKG-03
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-03-08

## Dispatch Decision

The human project authority authorized exactly one bounded DAG item:
`DEL-03-08 - Pipe section property and mass-property calculator`.

This dispatch is not broad fan-out. It does not authorize lifecycle
transitions, candidate-edge promotion, dependency-register edits, blocker queue
refresh, or work on any other deliverable.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-03-08` |
| PackageID | `PKG-03` |
| Name | Pipe section property and mass-property calculator |
| Type | `BACKEND_FEATURE_SLICE` |
| Scope items | `SOW-051`, `SOW-018` |
| Objectives | `OBJ-004`, `OBJ-012` |
| Context envelope | `M` |
| Deliverable path | `execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-08_Pipe section property and mass-property calculator` |
| Current lifecycle | `SEMANTIC_READY` |

## Approved DAG Preconditions

Active upstream dependencies from `DAG-001`:

| Target | Dependency type | Implementation-readiness satisfaction |
|---|---|---|
| `DEL-00-01` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-02` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-04` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-06` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-07` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-08` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-03-01` | `DOMAIN_MODEL` | `COMMITTED` evidence `3793e87` |
| `DEL-03-02` | `DOMAIN_MODEL` | `COMMITTED` evidence `f0fdeac` |
| `DEL-02-02` | `UNIT_CONTRACT` | `COMMITTED` evidence `a458cba` |

No `CANDIDATE` edge currently gates `DEL-03-08`.

## Applicable Architecture Basis

- `AB-00-01` - ADR and decision-record discipline.
- `AB-00-02` - layer/module responsibilities and no-bypass dependency rules.
- `AB-00-04` - JSON Schema 2020-12, canonical JSON, and versioned
  persistence/hash basis.
- `AB-00-06` - diagnostics/result-envelope fields, warning classes, and no
  certification/compliance claims.
- `AB-00-07` - schema-first API/adapter boundaries and no-bypass provenance,
  units, diagnostics, and data-boundary constraints.
- `AB-00-08` - layered validation, provenance, protected-content, and
  acceptance gates.

Resolved baseline to preserve: pipe section calculations must consume
user-entered unit-bearing dimensions and density inputs, preserve provenance in
calculated outputs, and emit diagnostics for missing, incompatible, or invalid
inputs.

Remaining implementation-level TBDs are not resolved by this dispatch:
approved unit catalog/conversion constants, public pipe section source
catalogs, physical project package/container, GUI/editor presentation, and
solver consumption policy.

## Applicable Contract Invariants

- `OPS-K-IP-1` through `OPS-K-IP-3` - no protected standards/proprietary data,
  provenance required, suspected protected content quarantined.
- `OPS-K-DATA-1` through `OPS-K-DATA-3` - code-specific and catalog values are
  user-supplied/private unless lawfully imported; missing values are explicit
  findings; material/component data carries provenance.
- `OPS-K-UNIT-1` - calculations are unit-aware and dimensionally checked.
- `OPS-K-MECH-1` and `OPS-K-MECH-2` - mechanics support remains distinct from
  rule checks and professional approval.
- `OPS-K-AUTH-1` - no certification, sealing, approval, authentication, legal
  acceptance, or code-compliance claims for reliance.
- `OPS-K-AGENT-1` through `OPS-K-AGENT-4` - no invented engineering values,
  conflicts surfaced, sealed execution, draft until human accepted.

## Explicit Write Scope

Authorized product and evidence targets for this bounded item:

- `core/section_properties/calculator.py`
- `core/section_properties/README.md`
- `tests/test_section_properties.py`
- `tests/test_component_section_schema.py`
- `schemas/section.schema.yaml`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-08_Pipe section property and mass-property calculator/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-03-08.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch.

## Acceptance Criteria

This bounded item is acceptable for closeout when:

- A pipe section property calculator derives circular pipe section properties
  from user-entered outside diameter, wall thickness, and corrosion allowance.
- Mass-per-length calculations use only supplied material, contents, and
  insulation density inputs.
- The calculator does not provide pipe schedule tables, material defaults,
  catalog values, unit conversion constants, protected dimensional tables,
  proprietary values, or code-specific values.
- Missing, incompatible, or invalid inputs produce blocking diagnostics rather
  than silent defaults.
- Tests cover normal section properties, corrosion allowance, mass components,
  missing dimensions, mixed units, invalid geometry, and schema-like quantity
  metadata.
- Documentation identifies the calculator and its boundaries without claiming
  engineering approval, legal acceptance, code compliance, public data
  suitability, or catalog completeness.
- No lifecycle state transition, dependency-register edit, candidate-edge
  change, or blocker-queue refresh occurs.
