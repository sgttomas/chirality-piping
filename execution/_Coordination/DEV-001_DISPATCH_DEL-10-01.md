---
doc_id: DEV-001-DISPATCH-DEL-10-01
doc_kind: coordination.dispatch_brief
status: sealed_brief_prepared
created: 2026-05-02
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-10-01
package_id: PKG-10
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-10-01 Public API And Plugin Boundary

**Dispatch status:** implemented in working tree on 2026-05-02 after separate
human approval gate.
**Coordination mode:** `FULL_GRAPH`
**Graph authority:** `execution/_DAG/DAG-001/DependencyEdges.csv`
**Implementation threshold:** upstream `COMMITTED` evidence

## Dispatch Decision

The human project authority authorized one bounded ORCHESTRATOR action:

- prepare `execution/_Coordination/DEV-001_DISPATCH_DEL-10-01.md`
  for `DEL-10-01 - Public API and plugin boundary`.

This authorization prepares the implementation brief only. It does not
authorize implementation, lifecycle transition, implementation-evidence
registration, dependency-register edits, blocker-queue refresh, `DAG-001`
changes, candidate-edge promotion, staging, commit, or broad DAG execution.

The eventual implementation scope should be deliberately constrained to the
existing public API/plugin boundary contract surfaces and deterministic
contract checks. It may harden `api/api_boundary_contract.yaml`,
`docs/architecture/plugin_boundary.md`, and focused tests for the API boundary.
It does not authorize public transport selection, endpoint syntax, OpenAPI
transport binding, plugin runtime/loading/signing implementation, adapter
implementation, headless runner implementation, external import/export format
selection, product package manifests, network/filesystem/process access
grants, protected standards content, private engineering data, or professional/
code-compliance/security certification claims.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-10-01` |
| PackageID | `PKG-10` |
| Name | Public API and plugin boundary |
| Type | `API_CONTRACT` |
| Scope items | `SOW-030` |
| Objectives | `OBJ-009` |
| Context envelope | `M` |
| Deliverable path | `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-01_Public API and plugin boundary` |
| Current lifecycle | `SEMANTIC_READY` |

## Approved DAG Preconditions

Active upstream dependencies from `DAG-001`:

| EdgeID | Upstream | Dependency type | Implementation-readiness satisfaction |
|---|---|---|---|
| `DAG-001-E0294` - `DAG-001-E0300` | Applicable `PKG-00` architecture basis | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DAG-001-E0552` | `DEL-02-04` Plugin and extension domain contracts | `SERVICE_API` | `COMMITTED` evidence `ef44f4c` |
| `DAG-001-E0553` | `DEL-02-01` Canonical domain model schema | `SCHEMA_CONTRACT` | `COMMITTED` evidence `7b256f3` |
| `DAG-001-E0554` | `DEL-02-02` Unit system and dimensional-analysis core contract | `UNIT_CONTRACT` | `COMMITTED` evidence `a458cba` |
| `DAG-001-E0555` | `DEL-02-03` Code-neutral analysis boundary model | `DOMAIN_MODEL` | `COMMITTED` evidence `abc1306` |

Current implementation-readiness queue state:

- `DEL-10-01` is `UNBLOCKED`.
- `DEL-10-01` has `MISSING_EVIDENCE`; it is not yet recorded as implemented.
- Candidate edges are excluded.

Downstream impact if later implemented and committed:

- `DEL-10-01` currently blocks `DEL-10-02`, `DEL-10-03`, `DEL-10-05`, and
  `DEL-11-02` in the active implementation-readiness queue.

## Applicable Architecture Basis

Applicable basis IDs from `SCA-001`: `AB-00-01`, `AB-00-02`, `AB-00-03`,
`AB-00-04`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.

Resolved baseline to preserve: ADR traceability; layer/module responsibilities
for GUI, application services, domain core, solver, storage, reporting,
private libraries, plugins, adapters, schemas, validation, and tests;
schema-first command/query/job/result-envelope concepts; JSON Schema 2020-12
contracts; canonical JSON/JCS-compatible hash basis where JSON payloads are
hashed; diagnostics/result-envelope warning classes; API/plugin/adapter
no-bypass boundaries; and layered validation/protected-content/security gates
where applicable.

The public API/plugin boundary must preserve the `DEL-02-04` extension-domain
contract, `DEL-02-01` canonical model schema posture, `DEL-02-02` unit
contract, and `DEL-02-03` mechanics/user-rule/human-authority separation.
Exact public transport, endpoint syntax, OpenAPI file generation, plugin
runtime/loading/signing/isolation, permission grant persistence, external
format list, code-generation tooling, runtime adapter behavior, and API
stability level remain `TBD` unless separately approved by the human project
authority.

## Explicit Write Scope

The bounded implementation write scope, if separately authorized, is limited
to:

- `api/api_boundary_contract.yaml`
- `docs/architecture/plugin_boundary.md`
- `tests/test_api_boundary_contract.py`
- `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-01_Public API and plugin boundary/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-10-01.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch. In particular, do not edit
runtime API server code, GUI code, CLI code, adapter implementations, plugin
loader/runtime code, package manifests, persistence schemas, rule-pack schemas,
OpenAPI transport bindings, `DAG-001`, candidate edges, deliverable-local
`Dependencies.csv`, `DEV-001_IMPLEMENTATION_EVIDENCE.csv`, or
`DEV-001_BLOCKER_QUEUE.*` during implementation unless separately authorized.

## Applicable Invariants

- `OPS-K-IP-1`, `OPS-K-IP-2`, `OPS-K-IP-3`
- `OPS-K-DATA-1`, `OPS-K-DATA-2`, `OPS-K-DATA-3`
- `OPS-K-AUTH-1`, `OPS-K-AUTH-2`
- `OPS-K-MECH-1`, `OPS-K-MECH-2`
- `OPS-K-UNIT-1`
- `OPS-K-RULE-2`, `OPS-K-RULE-3`
- `OPS-K-REPORT-1`, `OPS-K-REPORT-2`
- `OPS-K-PRIV-1`, `OPS-K-PRIV-2`
- `OPS-K-AGENT-1`, `OPS-K-AGENT-2`, `OPS-K-AGENT-3`, `OPS-K-AGENT-4`

## Acceptance Criteria

This bounded item is acceptable for implementation closeout when:

- `api/api_boundary_contract.yaml` exists, remains strict JSON syntax parseable
  by Python `json`, and is traceable to `DEL-10-01`, `PKG-10`, `SOW-030`, and
  `OBJ-009`.
- The API boundary contract declares schema-first command, query, job,
  diagnostics, result-envelope, provenance, privacy, permission, checksum,
  plugin-manifest, and no-bypass categories without choosing final public
  transport or endpoint syntax.
- The contract keeps `public_transport_protocol`, external import/export
  format list, API stability/versioning policy, runtime plugin mechanism, and
  OpenAPI transport binding as `TBD` unless separately approved.
- The contract requires mutating operations to route through governed command
  boundaries, read-only operations through queries, and long-running solve,
  report, export, or validation work through jobs with progress/cancellation
  and final result envelopes.
- The contract preserves model import/export guards for schema validation,
  unit validation, provenance, redistribution/private-public classification,
  protected-content screening, diagnostics, checksums, and result envelopes.
- The contract preserves rule-pack hook controls for sandboxed deterministic
  evaluation, unit awareness, required-input diagnostics, rule-pack identity,
  version, checksum, source notice, redistribution status, and private/public
  marking.
- The contract and `docs/architecture/plugin_boundary.md` state that plugins
  and adapters cannot bypass units, provenance, privacy, protected-content
  checks, diagnostics, result envelopes, rule sandboxing, persistence/hash
  controls, report controls, or human-acceptance boundaries.
- The contract and docs preserve the code-neutral authority boundary:
  mechanics solve, user-rule checking, and human professional acceptance are
  distinct; API outputs do not emit automatic `CODE_COMPLIANT`,
  certification, sealing, approval, endorsement, authentication, or
  professional-reliance claims.
- Tests under `tests/test_api_boundary_contract.py` provide deterministic
  checks for JSON parseability, metadata traceability, `TBD` transport/format
  decisions, operation categories, command/query/job registry coverage,
  no-bypass controls, privacy/provenance/checksum/result-envelope requirements,
  and absence of prohibited professional/compliance status terms.
- Deliverable `MEMORY.md` records the work, source basis, verification, and
  remaining open decisions.
- Verification includes `python3 tests/test_api_boundary_contract.py`,
  `python3 tests/test_plugin_manifest_schema.py`, `git diff --check`, and
  focused scans for protected standards content, private data, real secrets,
  transport/runtime commitments, and certification/compliance claim patterns.
- No lifecycle transition, dependency-register edit, candidate-edge change,
  blocker-queue refresh, implementation-evidence registration, protected
  standards data, proprietary engineering value, real private data, real
  secret, runtime API/plugin/adapter implementation, transport choice, external
  format choice, or professional/security/code-compliance claim occurs unless
  separately authorized.

## Dispatch Task Shape

```markdown
PURPOSE: Implement the sealed OpenPipeStress deliverable within explicit scope.
RequestedBy: WORKING_ITEMS

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-01_Public API and plugin boundary
DeliverablePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-01_Public API and plugin boundary
TaskProfile: interop-build

DeliverableID: DEL-10-01
PackageID: PKG-10

Tasks:
  - Implement only the artifacts authorized for DEL-10-01.
  - Harden api/api_boundary_contract.yaml as the strict-JSON public API/plugin
    boundary contract.
  - Harden docs/architecture/plugin_boundary.md as the human-readable public
    API/plugin boundary companion.
  - Add deterministic contract tests under tests/test_api_boundary_contract.py.
  - Preserve all applicable contract invariants, architecture-basis
    constraints, and upstream DEL-02-01/02/03/04 contracts.
  - Keep public transport, endpoint syntax, OpenAPI transport binding, plugin
    runtime/loading/signing/isolation, permission grant persistence, external
    format list, code generation, adapter behavior, and API stability level as
    TBD unless already approved by a human ruling.
  - Do not implement API server/runtime code, plugin loader/runtime code,
    adapter code, CLI code, network/filesystem/process permissions, package
    manifests, candidate-edge changes, or protected/private content.

Return:
  - Files changed.
  - Tests, scans, and validation commands run.
  - Public API/plugin boundary contract summary.
  - Remaining TBDs and blocked decisions.
```

## Current Stop Point

Human project authority later authorized implementation from this sealed brief
only. Implementation stayed within the approved write scope:

- `api/api_boundary_contract.yaml`
- `docs/architecture/plugin_boundary.md`
- `tests/test_api_boundary_contract.py`
- `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-01_Public API and plugin boundary/MEMORY.md`
- this dispatch brief
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

Implementation summary:

- Hardened the strict-JSON public API boundary contract metadata and operation
  registry.
- Kept public transport protocol, endpoint syntax, OpenAPI transport binding,
  external format list, plugin runtime/loading/signing/isolation, permission
  grant persistence, API stability level, and code-generation tooling as
  explicit `TBD` decisions.
- Added validation-test execution as a governed job/boundary category.
- Expanded no-bypass controls for diagnostics, persistence/hash controls,
  report controls, and human-acceptance boundaries.
- Updated the plugin-boundary documentation to match the machine-readable
  contract.
- Added deterministic stdlib tests for the API boundary contract.
- No lifecycle transition, dependency-register edit, candidate-edge change,
  blocker-queue refresh, implementation-evidence registration, public
  transport choice, endpoint syntax, OpenAPI transport binding, plugin runtime,
  adapter implementation, external format choice, protected standards data,
  proprietary engineering value, real private data, real secret, or
  professional/security/code-compliance claim occurred.

Verification:

- `python3 tests/test_api_boundary_contract.py` passed.
- `python3 tests/test_plugin_manifest_schema.py` passed.
- `git diff --check` passed before handoff update.

Human project authority later authorized lifecycle/evidence/local
dependency-register alignment and blocker-queue refresh.

Closeout summary:

- `DEL-10-01` lifecycle moved to `CHECKING`.
- `DEL-10-01` local dependency rows `DAG-001-E0552` through `DAG-001-E0555`
  were annotated as `SATISFIED` from committed upstream evidence.
- `DEV-001_IMPLEMENTATION_EVIDENCE.csv` records `DEL-10-01` as
  `WORKING_TREE`; because the implementation is not committed, this does not
  satisfy downstream implementation blockers.
- `DEV-001_BLOCKER_QUEUE.*` was refreshed and remains 60 unblocked / 13
  blocked. `DEL-10-02`, `DEL-10-03`, `DEL-10-05`, and `DEL-11-02` still show
  `DEL-10-01` as a missing committed upstream provider.
- `DAG-001` schema and strict audit passed; no aggregate graph, candidate edge,
  or ordering change was required.

Closeout verification:

- `python3 tools/coordination/build_dev001_blocker_queue.py --generated-date
  2026-05-02` passed.
- `python3 tools/validation/validate_dependencies_schema.py
  execution/_DAG/DAG-001/DependencyEdges.csv` passed.
- `python3 tools/validation/validate_dependencies_schema.py
  "execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-01_Public API and plugin boundary/Dependencies.csv"`
  passed.
- `python3 tools/coordination/audit_dag.py --strict --dag-dir
  execution/_DAG/DAG-001` passed.
- `pytest tools/coordination` passed.
- `git diff --check` passed.

Current stop point: staging and commit require a separate human gate. After
commit, `DEL-10-01` evidence should be promoted from `WORKING_TREE` to
`COMMITTED` with the commit hash and the blocker queue should be refreshed.
