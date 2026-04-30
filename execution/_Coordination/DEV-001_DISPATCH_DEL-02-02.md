---
doc_id: DEV-001-DISPATCH-DEL-02-02
doc_kind: coordination.dispatch_brief
status: launched_single_bounded_item
created: 2026-04-30
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-02-02
package_id: PKG-02
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-02-02

## Dispatch Decision

The human project authority authorized exactly one bounded DEV-001 item:
`DEL-02-02 - Unit system and dimensional-analysis core contract`.

This dispatch is not broad fan-out. It does not authorize lifecycle
transitions, candidate-edge promotion, blocker queue refresh, or work on any
other deliverable.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-02-02` |
| PackageID | `PKG-02` |
| Name | Unit system and dimensional-analysis core contract |
| Type | `BACKEND_FEATURE_SLICE` |
| Scope items | `SOW-025` |
| Objectives | `OBJ-001`, `OBJ-012` |
| Context envelope | `M` |
| Deliverable path | `execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract` |
| Current lifecycle | `SEMANTIC_READY` |

## Approved DAG Preconditions

Active upstream dependencies from `DAG-001`:

| Target | Dependency type | Required maturity | Satisfaction |
|---|---|---|---|
| `DEL-00-01` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-02` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-03` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-04` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-06` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-07` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-00-08` | `ARCHITECTURE_BASIS` | `SEMANTIC_READY` | `SATISFIED` |
| `DEL-02-01` | `UNIT_CONTRACT` | `SEMANTIC_READY` | `SATISFIED` |

No `CANDIDATE` edge currently gates `DEL-02-02`.

## Applicable Architecture Basis

- `AB-00-01` - ADR and decision-record discipline.
- `AB-00-02` - layer/module responsibilities and no-bypass dependency rules.
- `AB-00-03` - command/query/job/result envelope boundaries and distinct mechanics/rule/human states.
- `AB-00-04` - deterministic, versioned, unit-aware, provenance-preserving, schema-governed JSON persistence compatibility and JCS-compatible hash basis where JSON payload hashes are used.
- `AB-00-06` - diagnostics/result-envelope fields, warning classes, and no certification/compliance claims.
- `AB-00-07` - public/internal API boundary and adapter/plugin validation constraints.
- `AB-00-08` - layered schema, unit, validation, and protected-content/provenance gates.

Resolved baseline to preserve: Rust core/application services; JSON Schema
2020-12 contracts; schema-first command/query/job/result-envelope compatibility;
canonical JSON/JCS-compatible hash basis where JSON payloads are hashed; and
Cargo/Vitest/Playwright/validation/protected-content test gates as applicable.

Remaining implementation-level TBDs are not resolved by this dispatch: exact
dependency versions, solver numerical library, rule expression grammar/library,
public API transport, import/export format list, CI provider/coverage
thresholds, and physical project package/container.

## Applicable Contract Invariants

Minimum invariants carried into this sealed item:

- `OPS-K-IP-1` through `OPS-K-IP-3` - protected standards/code/proprietary data stay out of public content.
- `OPS-K-DATA-1` through `OPS-K-DATA-3` - code-specific values are user supplied, missing values are findings, and reliance-affecting records carry provenance.
- `OPS-K-AUTH-1` and `OPS-K-AUTH-2` - no certification, sealing, approval, or reliance claims by software/agents.
- `OPS-K-UNIT-1` - calculations, formulas, imported values, and exports must be unit-aware and dimensionally checked.
- `OPS-K-MECH-2` - mechanics solve, user rule checks, and professional approval remain distinct.
- `OPS-K-RULE-2` and `OPS-K-RULE-3` - rule packs remain sandboxed, unit-aware, versioned, checksummed, source-noted, and public/private marked.
- `OPS-K-AGENT-1` through `OPS-K-AGENT-4` - no invented values, conflicts surfaced, sealed execution, draft until human accepted.

## Explicit Write Scope

Authorized product and evidence targets for this bounded item:

- `schemas/units.schema.yaml`
- `core/units/README.md`
- `docs/SPEC.md`
- `tests/test_units_schema.py`
- `execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-02-02.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch.

## Acceptance Criteria

This bounded item is acceptable for closeout when:

- `schemas/units.schema.yaml` remains strict JSON syntax using JSON Schema 2020-12.
- The schema covers unit systems, dimensions, quantities, conversions, dimension checks, diagnostics, operation rules, test obligations, and open decisions.
- Unit-bearing quantities require explicit unit reference, dimension identifier, missing-unit behavior, and provenance.
- Dimensionless, ratio, percentage, coefficient, and unit-bearing classifications remain explicit.
- Conversion declarations require provenance, review status, transform kind, and visible unsupported/TBD handling.
- Unit diagnostics carry code/class/severity/source/affected references/message fields and preserve the no certification/compliance boundary.
- Operation rules expose compatible-dimension behavior for addition, subtraction, comparison, conversion, multiplication, division, power, schema validation, import/export validation, and rule evaluation.
- Test obligations preserve gated deterministic conversion tests until conversion constants, factor representation, and tolerances are approved.
- Documentation in `core/units/README.md` and `docs/SPEC.md` records the unit contract, adapter/rule obligations, storage convention, diagnostics, tests, and remaining TBDs.
- Focused stdlib tests pass for `tests/test_units_schema.py` and relevant existing schema tests.
- No lifecycle state transition, dependency-register edit, candidate-edge change, or blocker-queue refresh occurs.
