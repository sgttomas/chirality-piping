# DEV-001 Dispatch - DEL-08-02 Audit Manifest and Model Hash

**Dispatch status:** approved by human project authority after `DEL-06-03`
closeout on 2026-05-02
**Coordination mode:** `FULL_GRAPH`
**Graph authority:** `execution/_DAG/DAG-001/DependencyEdges.csv`
**Implementation threshold:** upstream `COMMITTED` evidence

## Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-08-02` |
| PackageID | `PKG-08` |
| Name | Audit manifest and model hash |
| Type | `BACKEND_FEATURE_SLICE` |
| Register objective | Implement canonical input manifest, model hash, solver version stamp, and rule-pack checksum capture. |
| Anticipated artifacts | audit manifest; hash tests |

## Authorized Write Scope

- `core/reporting/audit_manifest/`
- `docs/SPEC.md`
- `docs/TYPES.md`
- `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-02_Audit manifest and model hash/MEMORY.md`
- `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-02_Audit manifest and model hash/_STATUS.md`
- `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-02_Audit manifest and model hash/Dependencies.csv`
- `execution/_Coordination/DEV-001_DISPATCH_DEL-08-02.md`
- `execution/_Coordination/NEXT_INSTANCE_STATE.md`

Implementation-evidence, blocker-queue, staging, and commit changes are part
of the approved closeout path for this bounded item.

## Active Upstream Dependencies

| EdgeID | Upstream | Evidence |
|---|---|---|
| `DAG-001-E0246` - `DAG-001-E0252` | Applicable `PKG-00` architecture basis | Satisfied by accepted architecture baseline |
| `DAG-001-E0512` | `DEL-02-05` Project persistence and round-trip serialization | `COMMITTED` |
| `DAG-001-E0513` | `DEL-06-04` Private rule-pack lifecycle and checksum handling | `COMMITTED` |

## Applicable Invariants

- `OPS-K-IP-1`, `OPS-K-IP-2`, `OPS-K-IP-3`
- `OPS-K-DATA-1`, `OPS-K-DATA-2`, `OPS-K-DATA-3`
- `OPS-K-AUTH-1`, `OPS-K-AUTH-2`
- `OPS-K-UNIT-1`, `OPS-K-RULE-3`, `OPS-K-REPORT-1`
- `OPS-K-PRIV-1`, `OPS-K-AGENT-1`, `OPS-K-AGENT-3`

## Acceptance Criteria

- Implement bounded audit-manifest/model-hash support with canonical JSON hash
  basis and separate non-JSON asset hashes.
- Capture solver version stamp, unit-system reference, rule-pack checksum
  references, privacy/redaction metadata, and manifest findings.
- Missing model hash, solver version, unit reference, rule-pack checksum, asset
  hash, or required provenance surfaces as explicit findings.
- Public artifacts do not include protected standards data, private rule-pack
  payloads, proprietary engineering values, or professional/code-compliance
  claims.
- Focused Cargo tests pass.

## Exclusions

- No physical project container selection.
- No arbitrary JSON text parser or external canonicalization dependency.
- No private storage, encryption, access control, or secret handling.
- No report renderer, API transport, CLI runner, GUI workflow, or final
  result-envelope integration.
