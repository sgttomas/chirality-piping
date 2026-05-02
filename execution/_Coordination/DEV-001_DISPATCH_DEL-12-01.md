---
doc_id: DEV-001-DISPATCH-DEL-12-01
doc_kind: coordination.dispatch_brief
status: sealed_brief_prepared
created: 2026-05-02
prepared_by: ORCHESTRATOR
active_plan: plans/DEV-001_PRODUCT_DEVELOPMENT_DISPATCH_PLAN.md
accepted_dag: execution/_DAG/DAG-001/
approval_record: execution/_DAG/DAG-001/APPROVAL_RECORD.md
deliverable_id: DEL-12-01
package_id: PKG-12
blocker_computation: enabled_active_edges_only
candidate_edges: excluded
write_scope: explicit_bounded_targets
---

# DEV-001 Dispatch - DEL-12-01 Local-First Storage And Private Data Paths

**Dispatch status:** sealed dispatch brief prepared on 2026-05-02.
**Coordination mode:** `FULL_GRAPH`
**Graph authority:** `execution/_DAG/DAG-001/DependencyEdges.csv`
**Implementation threshold:** upstream `COMMITTED` evidence

## Dispatch Decision

The human project authority authorized one bounded ORCHESTRATOR action:

- prepare `execution/_Coordination/DEV-001_DISPATCH_DEL-12-01.md`
  for `DEL-12-01 - Local-first storage and private data paths`.

This authorization prepares the implementation brief only. It does not
authorize implementation, lifecycle transition, implementation-evidence
registration, dependency-register edits, blocker-queue refresh, `DAG-001`
changes, candidate-edge promotion, staging, commit, or broad DAG execution.

The eventual implementation scope should be deliberately constrained to a
local-first storage policy and deterministic policy tests that define symbolic
private-data path classes and repository-leakage prohibitions. It does not
authorize runtime storage code, filesystem writes, product config schemas,
physical project package/container selection, OS-specific directory selection,
cloud sync, cloud storage, encryption claims, secret storage implementation,
private data creation, real private paths, GUI behavior, adapter behavior,
plugin behavior, protected standards content, real secrets, or professional/
code-compliance/security certification claims.

## Deliverable Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-12-01` |
| PackageID | `PKG-12` |
| Name | Local-first storage and private data paths |
| Type | `SECURITY_CONTROL` |
| Scope items | `SOW-029` |
| Objectives | `OBJ-010` |
| Context envelope | `M` |
| Deliverable path | `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-01_Local-first storage and private data paths` |
| Current lifecycle | `SEMANTIC_READY` |

## Approved DAG Preconditions

Active upstream dependencies from `DAG-001`:

| EdgeID | Upstream | Dependency type | Implementation-readiness satisfaction |
|---|---|---|---|
| `DAG-001-E0354` - `DAG-001-E0360` | Applicable `PKG-00` architecture basis | `ARCHITECTURE_BASIS` | Satisfied by accepted architecture baseline |
| `DAG-001-E0601` | `DEL-02-05` Project persistence and round-trip serialization | `SECURITY_PREDECESSOR` | `COMMITTED` evidence `742016e` |
| `DAG-001-E0602` | `DEL-12-05` Security threat model | `SECURITY_PREDECESSOR` | `COMMITTED` evidence `b97121d` |
| `DAG-001-E0603` | `DEL-01-02` Copyright and protected-data boundary policy | `SECURITY_PREDECESSOR` | `COMMITTED` evidence `0d729cf` |

Current implementation-readiness queue state:

- `DEL-12-01` is `UNBLOCKED`.
- `DEL-12-01` has `MISSING_EVIDENCE`; it is not implemented.
- Candidate edges are excluded.

Downstream impact if later implemented and committed:

- `DEL-12-01` currently blocks `DEL-07-03`, `DEL-10-02`, `DEL-12-02`, and
  `DEL-12-04` in the active implementation-readiness queue.

## Applicable Architecture Basis

Applicable basis IDs from `SCA-001`: `AB-00-01`, `AB-00-02`, `AB-00-03`,
`AB-00-04`, `AB-00-06`, `AB-00-07`, and `AB-00-08`.

Resolved baseline to preserve: ADR traceability; layer/module responsibilities
for GUI, application services, storage, schemas, validation, adapters, and
tests; versioned, unit-aware, provenance-preserving, schema-governed,
migration-aware, round-trip testable persistence; canonical JSON/JCS-compatible
hash basis where JSON payloads are hashed; diagnostics/result-envelope warning
classes; API/plugin no-bypass boundaries; and layered validation/protected-
content/security gates where applicable.

The local-first storage design must preserve the `DEL-12-05` threat-model
posture and the `DEL-12-03` telemetry posture: private project data, private
rule packs, private material/component libraries, reports, diagnostics, paths,
and secrets remain user controlled; telemetry remains disabled by default; and
cloud storage or cloud sync is out of MVP unless separately approved.

Exact physical project package/container, OS-specific roots, application data
directories, migration framework, encryption, secret storage, key management,
redaction workflow, import/export formats, and runtime storage implementation
remain `TBD` unless separately approved by the human project authority.

## Explicit Write Scope

The bounded implementation write scope, if separately authorized, is limited
to:

- `docs/security/local_first_storage_policy.md`
- `tests/security/test_local_first_storage_policy.py`
- `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-01_Local-first storage and private data paths/MEMORY.md`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-12-01.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

No other files are authorized by this dispatch. In particular, do not edit
runtime storage code, persistence schemas, product config schemas, GUI code,
plugin manifests, adapter code, import/export code, telemetry code, secret
storage code, encryption code, `DAG-001`, candidate edges, deliverable-local
`Dependencies.csv`, `DEV-001_IMPLEMENTATION_EVIDENCE.csv`, or
`DEV-001_BLOCKER_QUEUE.*` during implementation unless separately authorized.

## Applicable Invariants

- `OPS-K-IP-1`, `OPS-K-IP-2`, `OPS-K-IP-3`
- `OPS-K-DATA-1`, `OPS-K-DATA-2`, `OPS-K-DATA-3`
- `OPS-K-AUTH-1`, `OPS-K-AUTH-2`
- `OPS-K-RULE-2`, `OPS-K-RULE-3`
- `OPS-K-REPORT-2`
- `OPS-K-PRIV-1`, `OPS-K-PRIV-2`
- `OPS-K-AGENT-1`, `OPS-K-AGENT-2`, `OPS-K-AGENT-3`, `OPS-K-AGENT-4`

## Acceptance Criteria

This bounded item is acceptable for implementation closeout when:

- `docs/security/local_first_storage_policy.md` exists and is traceable to
  `DEL-12-01`, `PKG-12`, `SOW-029`, and `OBJ-010`.
- The policy states that private project models, private rule packs, private
  material/component libraries, owner standards, company design bases,
  credentials, secrets, diagnostics, reports, and generated outputs remain
  local-first and user controlled by default.
- The policy states that public repository paths are not default durable
  storage locations for private project, rule-pack, material, component,
  report, diagnostic, owner-standard, credential, secret, or proprietary data.
- The policy defines symbolic path classes, not real user paths or OS-specific
  root choices, for at least: public repository content, public examples,
  user project packages, private user libraries, private rule packs, reports,
  diagnostics/support bundles, imports, exports, caches, and secrets.
- The policy states that absent, unknown, or unresolved storage choices remain
  explicit `TBD`, warning, or finding states rather than silent defaults.
- The policy preserves the persistence baseline: versioned, schema-governed,
  provenance-preserving, migration-aware, round-trip testable persistence, with
  canonical JSON/JCS-compatible hashes where JSON payloads are hashed.
- The policy states that cloud storage, cloud sync, cloud service behavior,
  physical project package/container selection, OS-specific roots, encryption,
  secret storage, key management, and runtime storage implementation remain
  `TBD` unless separately approved.
- The policy states that plugins, adapters, import/export paths, reports,
  telemetry, CLI runners, diagnostics, and application services cannot bypass
  private/public path classification, provenance, protected-content checks,
  diagnostics, sandboxing, or report/export controls.
- Tests under `tests/security/test_local_first_storage_policy.py` provide
  deterministic policy checks for traceability, local-first default behavior,
  repository-leakage prohibition, symbolic path classes, required `TBD`
  decisions, no-bypass language, and absence of real paths, secrets, cloud
  commitments, runtime storage code, and product dependency additions.
- Deliverable `MEMORY.md` records the work, source basis, verification, and
  remaining open decisions.
- Verification includes `git diff --check` and focused scans for prohibited
  protected-content, private-data, real-secret, real-path, cloud-commitment,
  and certification/compliance claim patterns.
- No lifecycle transition, dependency-register edit, candidate-edge change,
  blocker-queue refresh, implementation-evidence registration, protected
  standards data, proprietary engineering value, real private data, real
  secret, real path, cloud storage implementation, runtime storage behavior,
  encryption claim, or professional/security/code-compliance claim occurs
  unless separately authorized.

## Dispatch Task Shape

```markdown
PURPOSE: Implement the sealed OpenPipeStress deliverable within explicit scope.
RequestedBy: WORKING_ITEMS

ScopePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-01_Local-first storage and private data paths
DeliverablePath: /Users/ryan/ai-env/projects/chirality-piping/execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-01_Local-first storage and private data paths
TaskProfile: security-privacy

DeliverableID: DEL-12-01
PackageID: PKG-12

Tasks:
  - Implement only the artifacts authorized for DEL-12-01.
  - Create docs/security/local_first_storage_policy.md as the public local-first
    storage and private path policy.
  - Add deterministic policy tests under
    tests/security/test_local_first_storage_policy.py without adding runtime
    storage behavior, filesystem writes, schema edits, cloud behavior, or
    product dependencies.
  - Preserve all applicable contract invariants, architecture-basis
    constraints, DEL-12-05 threat-model posture, and DEL-12-03 telemetry
    posture.
  - Keep physical project package/container, OS-specific roots, application
    data directories, migration framework, encryption, secret storage, key
    management, redaction workflow, import/export formats, and runtime storage
    implementation as TBD unless already approved by a human ruling.
  - Do not implement storage code, create real private paths, add cloud sync,
    edit schemas, change plugin/runtime behavior, promote candidate edges, or
    introduce protected/private content.

Return:
  - Files changed.
  - Tests, scans, and validation commands run.
  - Local-first/private-path policy summary.
  - Remaining TBDs and blocked decisions.
```

## Current Stop Point

Stop after sealed dispatch brief preparation. Implementation requires a
separate human approval gate from this brief.
