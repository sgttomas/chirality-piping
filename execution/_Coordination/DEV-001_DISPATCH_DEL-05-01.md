---
doc_id: DEV-001-DISPATCH-DEL-05-01
doc_kind: coordination.dispatch_brief
status: launched_single_bounded_item
created: 2026-05-01
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-05-01
package_id: PKG-05
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-05-01

## Dispatch Decision

The human project authority authorized one bounded DAG-ready item:

- `DEL-05-01 - Primitive load case engine`

The human project authority later authorized implementation of this single
bounded item. This does not authorize lifecycle state changes, dependency
register edits, implementation evidence updates, blocker queue refreshes, or
commits.

The eventual implementation scope should be deliberately constrained to a
code-neutral primitive load engine: explicit load definitions for weight,
pressure, thermal, imposed displacement, hydrotest, wind, seismic, and
occasional categories; mechanics-only application hooks into existing solver
boundaries; deterministic validation findings; and tests. It does not authorize
code-specific load combinations, proprietary standards data, stress recovery,
rule-pack checks, GUI work, headless runner work, or professional/code
compliance claims.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-05-01` |
| PackageID | `PKG-05` |
| Name | Primitive load case engine |
| Type | `BACKEND_FEATURE_SLICE` |
| Scope items | `SOW-013` |
| Objectives | `OBJ-003` |
| Context envelope | `M` |
| Deliverable path | `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine` |
| Current lifecycle | `SEMANTIC_READY` |

## Approved DAG Preconditions

Active upstream dependencies from `DAG-001`:

| Target | Dependency type | Implementation-readiness satisfaction |
|---|---|---|
| `DEL-00-01` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-02` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-03` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-06` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-00-08` | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DEL-02-01` | `DOMAIN_MODEL` | `COMMITTED` evidence `7b256f3` |
| `DEL-02-02` | `UNIT_CONTRACT` | `COMMITTED` evidence `a458cba` |
| `DEL-04-01` | `SOLVER_PREDECESSOR` | `COMMITTED` evidence `1506cc0` |
| `DEL-04-03` | `SOLVER_PREDECESSOR` | `COMMITTED` evidence `d227a27` |

Current implementation-readiness queue state:

- `DEL-05-01` is `UNBLOCKED`.
- `DEL-05-01` has `MISSING_EVIDENCE` for its own implementation.
- Candidate edges are excluded.
- `DEL-05-01` is currently a missing upstream for `DEL-05-02`,
  `DEL-05-03`, `DEL-05-05`, `DEL-07-07`, `DEL-09-01`, and `DEL-10-05`.

## Applicable Architecture Basis

- `AB-00-01` - ADR and decision-record discipline.
- `AB-00-02` - layer/module responsibilities and no-bypass dependency rules.
- `AB-00-03` - schema-first command/query/job/result-envelope service
  boundaries.
- `AB-00-06` - diagnostics/result-envelope fields, warning classes, and no
  certification/compliance claims.
- `AB-00-08` - layered validation, provenance, protected-content, and
  acceptance gates.

Resolved baseline to preserve: primitive loads are mechanics inputs consumed by
solver/application-service boundaries, remain unit-aware, and must report
missing or invalid solve-required load data through deterministic findings
rather than silent defaults.

Remaining implementation-level TBDs are not resolved by this dispatch:
canonical calculation unit basis, conversion constants, solver numerical
library, final result-envelope integration, concrete application-service API,
load-case storage representation, wind/seismic dynamic treatment, and
production tolerance policy.

## Explicit Write Scope

The bounded implementation write scope is limited to:

- `core/loads/primitive_loads/.gitignore`
- `core/loads/primitive_loads/Cargo.toml`
- `core/loads/primitive_loads/README.md`
- `core/loads/primitive_loads/src/lib.rs`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-05-01.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch. In particular, do not edit
`DAG-001`, candidate edges, deliverable-local `Dependencies.csv`,
`DEV-001_IMPLEMENTATION_EVIDENCE.csv`, or `DEV-001_BLOCKER_QUEUE.*` during
implementation unless separately authorized.

## Acceptance Criteria

This bounded item is acceptable for implementation closeout when:

- The primitive load module defines mechanics-only inputs for weight, pressure,
  thermal expansion, imposed displacement, hydrotest, wind, seismic, and
  occasional primitive load categories without adding protected standards data,
  code-specific combinations, public catalog defaults, or rule-pack checks.
- The module preserves the six-DOF frame and support boundaries from
  `DEL-04-01` and `DEL-04-03` without changing frame-kernel or linear-support
  behavior.
- Load definitions are unit-aware by construction or carry explicit dimension
  intent that the existing unit contract can validate.
- Missing solve-required load data and invalid numeric input produce
  deterministic findings rather than silent defaults.
- Load application produces deterministic nodal, element, or boundary-condition
  contributions suitable for later load-case algebra and stress recovery
  consumers, while leaving code-specific combinations outside this scope.
- Tests cover each primitive load category at the mechanics-boundary level,
  invalid/missing data findings, deterministic application behavior, and
  preservation of solver/support boundary assumptions.
- No lifecycle transition, dependency-register edit, candidate-edge change,
  blocker-queue refresh, protected standards data, proprietary engineering
  value, or professional/code-compliance claim occurs unless separately
  authorized.
