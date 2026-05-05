---
doc_id: DEV-001-REV05-SEALED-BRIEF-DEL-13-03
doc_kind: coordination.sealed_type2_brief
status: sealed_brief_prepared_dispatch_not_authorized
created: 2026-05-04
prepared_by: ORCHESTRATOR
tranche: DEV-001_REV05_TRANCHE_F
deliverable_id: DEL-13-03
package_id: PKG-13
worker_launch: not_authorized
implementation_lane: contract_first_constraint_validation
source_proposal: execution/_Coordination/DEV-001_REV05_TRANCHE_F_PROPOSAL.md
---

# Sealed Brief - DEL-13-03 Constraint Validation Engine

## Dispatch Boundary

This is a sealed Type 2 implementation brief for future worker dispatch. It
does not authorize worker launch or implementation by itself. Use only after
the human separately approves implementation dispatch for Tranche F sealed
briefs.

The accepted lane is contract-first and provider-neutral. This brief does not
authorize lifecycle/evidence promotion, blocker refresh, dependency refresh,
aggregate DAG mutation, candidate promotion, commit, push, GUI/runtime work,
external prover integration, protected data, private project data, autonomous
accepted-model mutation, or professional/code compliance claims.

## Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-13-03` |
| PackageID | `PKG-13` |
| Name | Constraint validation engine |
| Type | `BACKEND_FEATURE_SLICE` |
| Scope item | `SOW-068` |
| Objective | `OBJ-014` |
| Context envelope | `M` |
| Lifecycle at brief prep | `SEMANTIC_READY` |
| Evidence state at brief prep | `MISSING_EVIDENCE` |
| Deliverable path | `execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-03_Constraint validation engine` |

## Scope And Objective

Implement deterministic validation messages for available design constraints
and missing data.

Acceptance context from `_CONTEXT.md`: this deliverable validates available
design knowledge for connectivity, route conflicts, clearance conflicts,
support-zone conflicts, slope/drain/vent conflicts, and missing required data
with provenance-aware messages. It must not infer hidden owner standards,
protected code requirements, protected owner/project data, or final engineering
acceptance logic.

## Readiness Evidence

`DEV-001_BLOCKER_QUEUE.csv` marks `DEL-13-03` as `UNBLOCKED` with 12 active
upstreams satisfied and zero blocking upstreams. Local dependency mirror state
is `SYNCHRONIZED_FROM_APPROVED_DAG002_MIRROR_PRESENT`; the local
`Dependencies.csv` is evidence synchronized from approved `DAG-002`, not
independent sequencing authority.

| EdgeID | Upstream | Evidence |
|---|---|---|
| `DAG-002-E0653` | `DEL-00-01` Architecture decision record baseline | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0654` | `DEL-00-02` Repository and module boundary architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0655` | `DEL-00-03` Application service command-query-job model | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0656` | `DEL-00-04` Persistence and schema versioning architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0657` | `DEL-00-06` Diagnostics, warning, and result-envelope contract | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0658` | `DEL-00-07` API boundary and adapter contract map | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0659` | `DEL-00-08` Layered software test and acceptance strategy | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0767` | `DEL-13-01` Design knowledge schema and provenance model | `COMMITTED dcdc1ac` |
| `DAG-002-E0768` | `DEL-13-02` Constraint entity and provenance model | `COMMITTED 002263b` |
| `DAG-002-E0769` | `DEL-02-02` Unit system and dimensional-analysis core contract | `COMMITTED a458cba` |
| `DAG-002-E0770` | `DEL-04-06` Solver diagnostics and singularity detection | `COMMITTED fdb0252` |
| `DAG-002-E0771` | `DEL-02-05` Project persistence and round-trip serialization | `COMMITTED 742016e` |

Candidate rows remain excluded from readiness and dispatch authority.

## Applicable Invariants

Apply the architecture basis IDs named in `_CONTEXT.md`: `AB-00-01`,
`AB-00-02`, `AB-00-03`, `AB-00-04`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.
Apply only the applicable constraints; do not copy full `PKG-00` prose into
deliverable artifacts.

Apply the project invariants from `docs/CONTRACT.md`, especially:

- `OPS-K-IP-1`, `OPS-K-IP-2`, `OPS-K-IP-3`
- `OPS-K-DATA-1`, `OPS-K-DATA-2`, `OPS-K-DATA-3`
- `OPS-K-AUTH-1`, `OPS-K-AUTH-2`
- `OPS-K-UNIT-1`
- `OPS-K-PRIV-1`
- `OPS-K-AGENT-1`, `OPS-K-AGENT-2`, `OPS-K-AGENT-3`, `OPS-K-AGENT-4`

## Allowed Write Scope For Later Authorized Implementation

- New constraint-validation module under `core/constraints/validation/`
- `tests/test_constraint_validation.py`, or crate-local tests if a later
  implementation gate explicitly chooses a Rust crate
- deliverable-local `MEMORY.md`
- deliverable-local run notes under the `DEL-13-03` folder

Do not edit `docs/SPEC.md` or `docs/TYPES.md` from a parallel worker. Shared
documentation integration, if needed, belongs to ORCHESTRATOR after
implementation return or to a later closeout gate with explicit scope.

Do not edit local `Dependencies.csv`, `_DEPENDENCIES.md`, lifecycle
`_STATUS.md`, implementation-evidence registers, blocker queues,
dependency-status projections, aggregate DAG files, candidate rows, or
committed evidence state.

## Tasks For Future Implementation

1. Define a deterministic validation module for constraint records and
   associated design-knowledge references.
2. Validate only available user/project/import/agent/source-supplied constraint
   data; missing data must become explicit findings rather than defaults.
3. Cover connectivity, route conflict, clearance, no-go volume, support-zone,
   slope, drain, vent, access, equipment-interface, and missing-required-data
   classes where the upstream constraint schema represents them.
4. Emit provenance-aware diagnostics with stable codes, severity, affected
   references, source references, messages, and remediation text.
5. Preserve unit metadata checks for unit-bearing parameters without performing
   uncontrolled unit conversion or inventing engineering tolerances.
6. Keep validation output as decision-support diagnostics, not hidden report
   prose, agent acceptance text, code compliance, or professional approval.
7. Add focused tests and record implementation memory/run notes.

## Acceptance Criteria

- Validation results are deterministic for the same input records.
- Missing required data, missing provenance, missing unit metadata, invalid
  references, unsupported constraint classes, and unresolved assumptions are
  surfaced explicitly.
- Diagnostics preserve source/provenance and affected-reference information.
- No hidden owner standards, protected code criteria, proprietary project
  constraints, commercial catalog values, or automatic engineering defaults
  are introduced.
- Outputs do not claim certification, sealing, authentication, code
  compliance, professional approval, or engineering acceptance.

## Required Verification For Future Implementation

- Focused constraint-validation tests added by this deliverable.
- Adjacent checks where referenced:
  `python3 tests/test_constraint_schema.py`,
  `python3 tests/test_design_knowledge_schema.py`,
  `python3 tests/test_units_schema.py`,
  `python3 tests/test_persistence_schema.py`, and any selected diagnostics
  checks.
- `git diff --check`
- focused scans for protected standards data, private project data,
  proprietary examples, real secrets, and prohibited certification/compliance/
  sealing/professional-approval claims.

## Stop Conditions

Stop before implementation, worker launch, lifecycle/evidence update, blocker
refresh, dependency refresh, aggregate DAG mutation, candidate promotion,
commit, push, protected standards content, private project data, real secrets,
GUI runtime work, external prover integration, direct accepted-model mutation,
or professional/code compliance claims unless separately authorized.
