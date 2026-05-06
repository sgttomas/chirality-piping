---
doc_id: DEV-001-REV05-SEALED-BRIEF-DEL-15-02
doc_kind: coordination.sealed_type2_brief
status: sealed_brief_prepared_dispatch_not_authorized
created: 2026-05-06
prepared_by: ORCHESTRATOR
tranche: DEV-001_REV05_TRANCHE_H
deliverable_id: DEL-15-02
package_id: PKG-15
worker_launch: not_authorized
implementation_lane: contract_first_target_mapping_unsupported_behavior
source_proposal: execution/_Coordination/DEV-001_REV05_TRANCHE_H_PROPOSAL.md
---

# Sealed Brief - DEL-15-02 Target Mapping And Unsupported-Behavior Contract

## Dispatch Boundary

This is a sealed Type 2 implementation brief for future worker dispatch. It
does not authorize worker launch or implementation by itself. Use only after
the human separately approves implementation dispatch for Tranche H sealed
briefs.

The accepted lane is contract-first, provider-neutral target mapping and
unsupported-behavior disclosure. This brief does not authorize lifecycle or
evidence promotion, blocker refresh, dependency refresh, aggregate DAG
mutation, candidate promotion, commit, push, target-specific commercial parser
behavior, external solver execution, protected data, private project data, or
professional/code-compliance claims.

## Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-15-02` |
| PackageID | `PKG-15` |
| Name | Target mapping and unsupported-behavior contract |
| Type | `API_CONTRACT` |
| Scope item | `SOW-074` |
| Objective | `OBJ-017` |
| Context envelope | `M` |
| Lifecycle at brief prep | `SEMANTIC_READY` |
| Evidence state at brief prep | `MISSING_EVIDENCE` |
| Deliverable path | `execution/PKG-15_Handoff and External Prover Workflow/1_Working/DEL-15-02_Target mapping and unsupported-behavior contract` |

## Scope And Objective

Define target mapping metadata and unsupported or approximate behavior flags
for handoff exports. The contract must preserve handoff context such as model
hashes, units manifests, entity IDs, library/rule references, unresolved
assumptions, warnings, target mapping metadata, and unsupported-target flags.

Package exclusions remain binding: do not force a prover-status lifecycle,
generate professional approval records, implement target-specific commercial
parsers, or imply downstream validation/certification.

## Readiness Evidence

`DEV-001_BLOCKER_QUEUE.csv` marks `DEL-15-02` as `UNBLOCKED` with 13 active
upstreams satisfied and zero blocking upstreams. Local dependency mirror state
is `SYNCHRONIZED_FROM_APPROVED_DAG002_MIRROR_PRESENT`; the local
`Dependencies.csv` is evidence synchronized from approved `DAG-002`, not
independent sequencing authority.

| EdgeID | Upstream | Evidence |
|---|---|---|
| `DAG-002-E0709` | `DEL-00-01` Architecture decision record baseline | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0710` | `DEL-00-02` Repository and module boundary architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0711` | `DEL-00-03` Application service command-query-job model | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0712` | `DEL-00-04` Persistence and schema versioning architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0713` | `DEL-00-06` Diagnostics, warning, and result-envelope contract | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0714` | `DEL-00-07` API boundary and adapter contract map | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0715` | `DEL-00-08` Layered software test and acceptance strategy | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0805` | `DEL-15-01` Canonical handoff package schema and manifest | `COMMITTED 05878bf` |
| `DAG-002-E0806` | `DEL-10-02` Import/export adapter framework | `COMMITTED be29df7` |
| `DAG-002-E0807` | `DEL-10-03` Local FEA handoff data contract | `COMMITTED abdecbd` |
| `DAG-002-E0808` | `DEL-12-02` Private data redaction and export controls | `COMMITTED abdecbd` |
| `DAG-002-E0809` | `DEL-13-04` Physical-to-analytical transformation contract | `COMMITTED 24b5717` |
| `DAG-002-E0810` | `DEL-14-05` Comparison mapping, tolerance, and export contracts | `COMMITTED 05878bf` |

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
- `OPS-K-REPORT-1`, `OPS-K-REPORT-2`
- `OPS-K-PRIV-1`
- `OPS-K-AGENT-1`, `OPS-K-AGENT-2`, `OPS-K-AGENT-3`, `OPS-K-AGENT-4`

## Allowed Write Scope For Later Authorized Implementation

- Target mapping / unsupported-behavior schema or contract file under
  `schemas/` or a narrow `core/handoff/target_mapping/` path named in the
  implementation gate
- Unsupported behavior taxonomy fixture or module, if scoped to this
  deliverable
- Focused tests such as `tests/test_target_mapping_contract.py`
- deliverable-local `MEMORY.md`
- deliverable-local run notes under the `DEL-15-02` folder

Do not edit `docs/SPEC.md` or `docs/TYPES.md` from a parallel worker. Shared
documentation integration, if needed, belongs to ORCHESTRATOR after
implementation return or to a later closeout gate with explicit scope.

Do not edit local `Dependencies.csv`, `_DEPENDENCIES.md`, lifecycle
`_STATUS.md`, implementation-evidence registers, blocker queues,
dependency-status projections, aggregate DAG files, candidate rows, or
committed evidence state.

## Tasks For Future Implementation

1. Define target mapping metadata for handoff exports.
2. Define unsupported-target and approximate-behavior disclosure records.
3. Preserve links to handoff package context: model hash, units manifest,
   entity IDs, library/rule references, unresolved assumptions, warnings, and
   private/redaction boundaries.
4. Emit explicit diagnostics or unsupported records instead of silently
   replacing missing or unsupported values with defaults.
5. Keep exact target list, target-specific mapping strategy, and commercial
   parser behavior out of scope unless a later human gate authorizes them.
6. Add focused contract tests and record implementation memory/run notes.

## Acceptance Criteria

- Target mapping metadata and unsupported/approximate behavior records are
  represented deterministically.
- Unsupported or approximate behavior is explicit and traceable to affected
  entities, assumptions, warnings, or target capability references.
- Unit-bearing mapped values are not represented without unit/dimension
  metadata or diagnostics.
- Private data, rule-pack payloads, protected standards content, and
  proprietary target data are not copied into public artifacts.
- Outputs do not claim certification, sealing, authentication, code
  compliance, professional approval, external validation, or engineering
  acceptance.

## Required Verification For Future Implementation

- Focused target mapping / unsupported behavior contract tests.
- Adjacent checks where referenced:
  `python3 tests/test_handoff_package_schema.py`,
  `python3 tests/test_comparison_contracts.py`,
  `python3 tests/test_results_schema.py`, and
  `python3 tests/test_units_schema.py`.
- `git diff --check`
- focused scans for protected standards data, private project data,
  proprietary examples, real secrets, and prohibited certification/compliance/
  sealing/professional-approval claims.

## Stop Conditions

Stop before implementation, worker launch, lifecycle/evidence update, blocker
refresh, dependency refresh, aggregate DAG mutation, candidate promotion,
commit, push, protected standards content, private project data, real secrets,
target-specific parser behavior, external solver execution, commercial-prover
validation, direct accepted-model mutation, or professional/code compliance
claims unless separately authorized.

