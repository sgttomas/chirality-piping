---
doc_id: DEV-001-DISPATCH-DEL-10-05
doc_kind: coordination.dispatch_brief
status: sealed_brief_prepared
created: 2026-05-02
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-10-05
package_id: PKG-10
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-10-05 Headless CLI And Structured I/O Analysis Runner

**Dispatch status:** implemented in working tree on 2026-05-02 after separate
human approval from the sealed brief.
**Coordination mode:** `FULL_GRAPH`
**Graph authority:** `execution/_DAG/DAG-001/DependencyEdges.csv`
**Implementation threshold:** upstream `COMMITTED` evidence

## Dispatch Decision

The human project authority authorized one bounded ORCHESTRATOR action:

- prepare `execution/_Coordination/DEV-001_DISPATCH_DEL-10-05.md`
  for `DEL-10-05 - Headless CLI and structured I/O analysis runner`.

This authorization prepares the implementation brief only. It does not
authorize implementation, lifecycle transition, implementation-evidence
registration, dependency-register edits, blocker-queue refresh, `DAG-001`
changes, candidate-edge promotion, staging, commit, or broad DAG execution.

The eventual implementation scope should be constrained to an early
schema-first headless runner contract/support surface for non-GUI solve,
validation, regression, and result-export workflows. It may define a bounded
runner contract, add a small headless-runner support module, update focused
schema/type/spec documentation, and add deterministic tests. It must not
choose final CLI command syntax, package manager scripts, CI provider,
release matrix, public API transport, external import/export formats, GUI
runtime behavior, report rendering behavior, adapter implementations, local
FEA handoff packaging, protected standards content, private engineering data,
real secrets, or professional/code-compliance claims.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-10-05` |
| PackageID | `PKG-10` |
| Name | Headless CLI and structured I/O analysis runner |
| Type | `BACKEND_FEATURE_SLICE` |
| Scope items | `SOW-054`, `SOW-032` |
| Objectives | `OBJ-008`, `OBJ-009`, `OBJ-012` |
| Context envelope | `M` |
| Deliverable path | `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-05_Headless CLI and structured I-O analysis runner` |
| Current lifecycle | `SEMANTIC_READY` |

## Approved DAG Preconditions

Active upstream dependencies from `DAG-001`:

| EdgeID | Upstream | Dependency type | Implementation-readiness satisfaction |
|---|---|---|---|
| `DAG-001-E0322` - `DAG-001-E0328` | Applicable `PKG-00` architecture basis | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DAG-001-E0565` | `DEL-10-01` Public API and plugin boundary | `SERVICE_API` | `COMMITTED` evidence `53cc3d6` |
| `DAG-001-E0566` | `DEL-02-05` Project persistence and round-trip serialization | `PERSISTENCE_CONTRACT` | `COMMITTED` evidence `742016e` |
| `DAG-001-E0567` | `DEL-04-01` 3D frame stiffness kernel | `SOLVER_PREDECESSOR` | `COMMITTED` evidence `1506cc0` |
| `DAG-001-E0568` | `DEL-05-01` Primitive load case engine | `LOAD_STRESS_PREDECESSOR` | `COMMITTED` evidence `e3c9695` |
| `DAG-001-E0569` | `DEL-08-04` Result export format | `REPORTING_PREDECESSOR` | `COMMITTED` evidence `3e33ea4` |
| `DAG-001-E0570` | `DEL-08-02` Audit manifest and model hash | `REPORTING_PREDECESSOR` | `COMMITTED` evidence `061f1af` |

Implementation-readiness queue state at dispatch preparation:

- `DEL-10-05` is `UNBLOCKED`.
- `DEL-10-05` has `MISSING_EVIDENCE`; it is not yet recorded as implemented.
- Candidate edges are excluded.

Downstream impact at dispatch preparation if later implemented and committed:

- `DEL-10-05` currently blocks `DEL-10-04 - Build, packaging, and CI/CD
  pipeline` in the active implementation-readiness queue.
- Candidate edge `DAG-001-E0624` mentions possible coupling from `DEL-07-07`
  to `DEL-10-05`; that edge remains non-gating and must not affect dispatch
  readiness unless later reconciled and promoted.

## Applicable Architecture Basis

Applicable basis IDs from `SCA-001`: `AB-00-01`, `AB-00-02`, `AB-00-03`,
`AB-00-04`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.

Resolved baseline to preserve: ADR traceability; layer/module responsibilities;
schema-first command/query/job/result-envelope boundaries; JSON Schema 2020-12
contracts; canonical JSON/JCS-compatible hash basis where JSON payloads are
hashed; diagnostics/result-envelope warning classes; API/plugin/adapter
no-bypass boundaries; and layered Cargo/Vitest/Playwright/validation/
protected-content/security gates where applicable.

The headless runner must preserve the `DEL-10-01` public API/plugin boundary,
`DEL-02-05` persistence and round-trip contract, `DEL-04-01` solver-kernel
boundary, `DEL-05-01` primitive-load workflow boundary, `DEL-08-02` audit
manifest/hash basis, and `DEL-08-04` result-export envelope. Exact command
names, package-manager scripts, process invocation behavior, public transport,
external import/export formats, CI provider, coverage thresholds, release
matrix, physical project package/container, and local FEA package structure
remain `TBD` unless separately approved by the human project authority.

## Explicit Write Scope

The bounded implementation write scope, if separately authorized, is limited
to:

- `schemas/headless_runner.schema.yaml`
- `core/runner/headless/`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `tests/test_headless_runner_contract.py`
- `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-05_Headless CLI and structured I-O analysis runner/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-10-05.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch. In particular, do not edit GUI
code, report renderer code, adapter implementations, local FEA handoff
artifacts, package manifests, CI workflows, release scripts, public API
transport bindings, solver internals outside the bounded runner interface,
`DAG-001`, candidate edges, deliverable-local `Dependencies.csv`,
`DEV-001_IMPLEMENTATION_EVIDENCE.csv`, or `DEV-001_BLOCKER_QUEUE.*` during
implementation unless separately authorized.

## Applicable Invariants

- `OPS-K-IP-1`, `OPS-K-IP-2`, `OPS-K-IP-3`
- `OPS-K-DATA-1`, `OPS-K-DATA-2`, `OPS-K-DATA-3`
- `OPS-K-AUTH-1`, `OPS-K-AUTH-2`
- `OPS-K-MECH-1`, `OPS-K-MECH-2`
- `OPS-K-UNIT-1`
- `OPS-K-SOLVER-1`, `OPS-K-SOLVER-2`
- `OPS-K-RULE-2`, `OPS-K-RULE-3`
- `OPS-K-REPORT-1`, `OPS-K-REPORT-2`
- `OPS-K-PRIV-1`, `OPS-K-PRIV-2`
- `OPS-K-AGENT-1`, `OPS-K-AGENT-2`, `OPS-K-AGENT-3`, `OPS-K-AGENT-4`

## Acceptance Criteria

This bounded item is acceptable for implementation closeout when:

- `schemas/headless_runner.schema.yaml` exists, remains strict JSON syntax
  parseable by Python `json`, and is traceable to `DEL-10-05`, `PKG-10`,
  `SOW-054`, `SOW-032`, `OBJ-008`, `OBJ-009`, and `OBJ-012`.
- The contract defines a schema-first headless runner request/result envelope
  suitable for non-GUI solve execution, benchmark automation, regression
  execution, audit-manifest capture, and result-export handoff without
  choosing final CLI command syntax, package scripts, public transport, CI
  provider, release matrix, external adapter formats, local FEA package
  formats, or GUI/report runtime behavior.
- The contract requires runner operations to route through command/job/result
  boundaries consistent with `DEL-10-01`, including operation identity,
  model/project reference, unit-system reference, load-case or combination
  basis, requested operation class, job/progress/cancellation metadata,
  diagnostics, result-envelope reference, audit manifest/hash references, and
  privacy/provenance controls.
- Structured inputs remain unit-aware and deterministic. Missing solve-required
  or rule-check-required values produce structured diagnostics/finding states
  rather than silent defaults.
- Runner outputs align with `schemas/results.schema.yaml` and the
  `core/reporting/result_export/` boundary for result export compatibility;
  they do not introduce an ad hoc output shape for regression or downstream
  tooling.
- Audit and reproducibility fields preserve `DEL-08-02` manifest/hash posture,
  including model/run identity, solver/version basis, input manifest reference,
  checksum/canonicalization metadata where JSON payloads are hashed, and clear
  `TBD` handling for non-JSON/binary assets.
- Diagnostics use structured fields compatible with the architecture baseline:
  code, class, severity, source, affected object, message, remediation, and
  provenance.
- Runner fixtures, examples, or test payloads, if added, use invented or
  otherwise permitted data only and do not embed protected standards text,
  protected tables, copied code formulas, material allowables, SIF/flexibility
  tables, protected dimensional tables, proprietary vendor data, private
  project data, private rule-pack payloads, real secrets, or real user paths.
- The runner preserves the code-neutral authority boundary: mechanics solve,
  user-rule checking, and human professional acceptance are distinct; runner
  outputs do not emit automatic `CODE_COMPLIANT`, certification, sealing,
  approval, endorsement, authentication, or professional-reliance statuses.
- The support module, if implemented, provides bounded in-memory runner
  contract validation and deterministic envelope construction only. It does not
  parse arbitrary project files, invoke external processes, access the network,
  implement GUI/report/adapter/local-FEA runtime behavior, or mutate filesystem
  state outside explicit caller-provided fixtures.
- Deterministic tests under `tests/test_headless_runner_contract.py` cover
  JSON parseability, metadata traceability, unresolved `TBD` command/CI/
  transport/package decisions, operation categories, unit/dimension
  requirements, diagnostics, result-export compatibility, audit/hash fields,
  privacy/provenance controls, and absence of prohibited professional/
  compliance status terms.
- Deliverable `MEMORY.md` records the work, source basis, verification, and
  remaining open decisions.
- Verification includes `python3 tests/test_headless_runner_contract.py`,
  `python3 tests/test_results_schema.py`, `python3 tests/test_api_boundary_contract.py`,
  `git diff --check`, and focused scans for protected standards content,
  private data, real secrets, external command/process/network commitments,
  package/CI/release commitments, and certification/compliance claim patterns.
- No lifecycle transition, dependency-register edit, candidate-edge change,
  blocker-queue refresh, implementation-evidence registration, protected
  standards data, proprietary engineering value, real private data, real
  secret, final CLI command syntax, package manifest/script change, CI provider
  choice, release matrix choice, transport choice, external format choice,
  GUI/report/adapter/local-FEA implementation, or professional/security/
  code-compliance claim occurs unless separately authorized.

## Dispatch Task Shape

```markdown
PURPOSE: Implement the sealed OpenPipeStress deliverable within explicit scope.
RequestedBy: WORKING_ITEMS

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-05_Headless CLI and structured I-O analysis runner
DeliverablePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-05_Headless CLI and structured I-O analysis runner
TaskProfile: interop-build

DeliverableID: DEL-10-05
PackageID: PKG-10

Tasks:
  - Implement only the artifacts authorized for DEL-10-05.
  - Add schemas/headless_runner.schema.yaml as a strict-JSON JSON Schema
    2020-12 contract for the schema-first headless runner request/result
    envelope.
  - Add core/runner/headless/ as a bounded support module for in-memory runner
    contract validation and deterministic envelope construction.
  - Add deterministic contract tests under tests/test_headless_runner_contract.py.
  - Update focused docs/SPEC.md and docs/TYPES.md sections only as needed for
    the headless runner boundary.
  - Preserve all applicable contract invariants, architecture-basis
    constraints, and upstream DEL-10-01/02-05/04-01/05-01/08-02/08-04
    contracts.
  - Keep final CLI command names, package scripts, CI provider, release matrix,
    public transport, external import/export formats, physical project
    container, local FEA package structure, GUI/report runtime behavior, and
    adapter implementation as TBD unless already approved by a human ruling.
  - Do not implement process execution, network access, filesystem mutation
    outside explicit caller-provided fixtures, GUI code, report renderer code,
    adapter code, local FEA code, package manifests, CI workflows, release
    scripts, candidate-edge changes, dependency-register edits, or
    protected/private content.

Return:
  - Files changed.
  - Tests, scans, and validation commands run.
  - Headless runner contract summary.
  - Remaining TBDs and blocked decisions.
```

## Current Stop Point

Human project authority later authorized implementation from this sealed brief
only. Implementation stayed within the approved write scope:

- `schemas/headless_runner.schema.yaml`
- `core/runner/headless/`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `tests/test_headless_runner_contract.py`
- `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-05_Headless CLI and structured I-O analysis runner/MEMORY.md`
- this dispatch brief
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

Implementation summary:

- Added `schemas/headless_runner.schema.yaml` as the strict-JSON JSON Schema
  2020-12 contract for schema-first headless runner request/result envelopes.
- Added `core/runner/headless/` as a bounded Rust support crate for in-memory
  runner request/result validation.
- Added `tests/test_headless_runner_contract.py` structural checks for schema
  metadata, required request/result categories, unresolved `TBD` command/CI/
  transport/package decisions, result-export compatibility, audit/hash fields,
  privacy/provenance controls, and professional-boundary exclusions.
- Updated `docs/SPEC.md` and `docs/TYPES.md` with the headless runner
  boundary.
- Added deliverable `MEMORY.md`.

Human project authority later authorized lifecycle/evidence/dependency-mirror
alignment and blocker queue refresh for the implemented `DEL-10-05` work.
Alignment stayed within the closeout scope:

- `DEL-10-05` lifecycle display state set to `CHECKING`.
- `DEL-10-05` local dependency rows `DAG-001-E0565` through `DAG-001-E0570`
  marked `SATISFIED` from committed upstream implementation evidence.
- `DEL-10-05` implementation and closeout alignment were committed as
  `9de5e9b core: add headless runner contract`.
- Human project authority authorized post-commit evidence promotion and
  CHANGE gate handling.
- `DEV-001_IMPLEMENTATION_EVIDENCE.csv` now records `DEL-10-05` as
  `COMMITTED` evidence for commit `9de5e9b`.
- `DEV-001_BLOCKER_QUEUE.*` was rebuilt at 64 unblocked / 9 blocked.
  `DEL-10-04` no longer waits on `DEL-10-05`; it still waits on `DEL-09-05`
  and `DEL-08-05`.
- Aggregate `DAG-001` was not edited; candidate edge `DAG-001-E0624` remains
  non-gating.

Candidate-edge promotion and broad DAG execution require a separate human
approval gate.
