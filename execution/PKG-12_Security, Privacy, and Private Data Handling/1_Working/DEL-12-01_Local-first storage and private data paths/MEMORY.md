# Memory: DEL-12-01 Local-first storage and private data paths

## Current Session

2026-05-02 - Implemented from sealed dispatch brief
`execution/_Coordination/DEV-001_DISPATCH_DEL-12-01.md`.

## Decisions And Rulings

- Human project authority authorized implementation from the sealed brief only
  after brief preparation commit `2c3dcae`.
- Implementation stayed within the approved write scope:
  `docs/security/local_first_storage_policy.md`,
  `tests/security/test_local_first_storage_policy.py`, this `MEMORY.md`, the
  dispatch brief, and `NEXT_INSTANCE_STATE.md`.
- No lifecycle transition, implementation-evidence registration,
  dependency-register edit, blocker-queue refresh, `DAG-001` change,
  candidate-edge promotion, runtime storage code, filesystem write behavior,
  schema edit, product config edit, physical project package/container
  selection, OS-specific root selection, cloud sync/storage, secret storage,
  encryption implementation, plugin/runtime behavior change, or adapter
  behavior change was performed.

## Source Basis

- `execution/_Coordination/DEV-001_DISPATCH_DEL-12-01.md`
- `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-01_Local-first storage and private data paths/_CONTEXT.md`
- `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-01_Local-first storage and private data paths/Specification.md`
- `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-01_Local-first storage and private data paths/Guidance.md`
- `docs/CONTRACT.md`
- `docs/IP_AND_DATA_BOUNDARY.md`
- `docs/security/threat_model.md`
- `docs/security/telemetry_policy.md`

## Implementation Notes

- Added `docs/security/local_first_storage_policy.md` as the public
  local-first storage and private path policy.
- Defined local-first, user-controlled defaults for private project models,
  private rule packs, private material/component libraries, owner standards,
  company design bases, credentials, secrets, diagnostics, reports, generated
  outputs, and other user-owned engineering data.
- Defined symbolic path classes for public repository content, public examples,
  user project packages, private libraries, private rule packs, reports,
  diagnostics/support bundles, imports, exports, local caches, and secret
  references.
- Prohibited public repository paths as default durable storage for private
  project, rule-pack, material, component, report, diagnostic, owner-standard,
  credential, secret, or proprietary data.
- Preserved the persistence baseline: versioned, schema-governed,
  unit-aware, provenance-preserving, migration-aware, round-trip testable
  persistence with canonical JSON/JCS-compatible hashes where JSON payloads
  are hashed.
- Kept physical project package/container, OS-specific roots, application data
  directories, migration framework, encryption, secret storage, key
  management, redaction workflow, import/export formats, cloud exception
  workflow, and runtime storage implementation as `TBD`.

## Verification

- `python3 -m pytest tests/security/test_local_first_storage_policy.py` passed.
- `git diff --check` passed.
- Focused protected-content/prohibited-claim/real-secret/real-path/cloud-
  commitment scan found only guardrail and exclusion wording in the local-first
  storage policy, tests, dispatch brief, memory, and state.

## Remaining TBDs

- Physical project package/container.
- OS-specific roots and application data directories.
- Migration framework and concrete persistence service.
- Encryption, secret storage, and key management.
- Redaction workflow and export staging behavior.
- Private-library registry and secret/private-library handling.
- Import/export formats and adapter behavior.
- Cloud exception workflow.
- Lifecycle/evidence/local dependency-register alignment and blocker-queue
  refresh for `DEL-12-01`, if later authorized.
