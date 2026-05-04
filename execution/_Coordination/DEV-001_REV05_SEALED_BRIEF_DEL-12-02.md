---
doc_id: DEV-001-REV05-SEALED-BRIEF-DEL-12-02
doc_kind: coordination.sealed_type2_brief
status: sealed_brief_prepared_dispatch_not_authorized
created: 2026-05-03
prepared_by: ORCHESTRATOR
tranche: DEV-001_REV05_TRANCHE_A
deliverable_id: DEL-12-02
package_id: PKG-12
worker_launch: not_authorized
---

# Sealed Brief - DEL-12-02 Private Data Redaction And Export Controls

## Dispatch Boundary

This is a sealed Type 2 implementation brief for future worker dispatch. It
does not authorize worker launch by itself. Use only after the human separately
approves worker dispatch for Tranche A sealed briefs.

## Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-12-02` |
| PackageID | `PKG-12` |
| Name | Private data redaction and export controls |
| Type | `SECURITY_CONTROL` |
| Scope items | `SOW-040` |
| Objectives | `OBJ-010` |
| Context envelope | `M` |
| Lifecycle at brief prep | `SEMANTIC_READY` |
| Deliverable path | `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-02_Private data redaction and export controls` |

Local `_CONTEXT.md` still names revision `0.4`; this brief overrides it for
dispatch authority. Use `execution/_Decomposition/SOFTWARE_DECOMP.md` revision
`0.5`, `docs/_Registers/Deliverables.csv`, and approved `DAG-002`.

## Readiness Evidence

`DEV-001_BLOCKER_QUEUE.csv` marks `DEL-12-02` as `UNBLOCKED` with 13 active
upstreams satisfied. Local dependency mirror state is
`SYNCHRONIZED_FROM_APPROVED_DAG002_MIRROR_PRESENT`.

| EdgeID | Upstream | Evidence |
|---|---|---|
| `DAG-002-E0361` | `DEL-00-01` Architecture decision record baseline | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0362` | `DEL-00-02` Repository and module boundary architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0363` | `DEL-00-03` Application service command-query-job model | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0364` | `DEL-00-04` Persistence and schema versioning architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0365` | `DEL-00-06` Diagnostics, warning, and result-envelope contract | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0366` | `DEL-00-07` API boundary and adapter contract map | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0367` | `DEL-00-08` Layered software test and acceptance strategy | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0610` | `DEL-12-05` Security threat model | `COMMITTED b97121d` |
| `DAG-002-E0611` | `DEL-12-01` Local-first storage and private data paths | `COMMITTED 84e0a73` |
| `DAG-002-E0612` | `DEL-08-01` Calculation report generator | `COMMITTED 9e21716` |
| `DAG-002-E0613` | `DEL-08-04` Result export format | `COMMITTED 3e33ea4` |
| `DAG-002-E0614` | `DEL-06-04` Private rule-pack lifecycle and checksum handling | `COMMITTED ad270f6` |
| `DAG-002-E0615` | `DEL-03-07` Public/private library import provenance checker | `COMMITTED 4d880b3` |

Candidate edges are excluded.

## Applicable Invariants

Apply `OPS-K-IP-1`, `OPS-K-IP-2`, `OPS-K-IP-3`, `OPS-K-DATA-1`,
`OPS-K-DATA-2`, `OPS-K-DATA-3`, `OPS-K-AUTH-1`, `OPS-K-REPORT-1`,
`OPS-K-PRIV-1`, `OPS-K-PRIV-2`, `OPS-K-AGENT-1`, `OPS-K-AGENT-2`,
`OPS-K-AGENT-3`, and `OPS-K-AGENT-4`.

Applicable architecture basis IDs: `AB-00-01`, `AB-00-02`, `AB-00-03`,
`AB-00-04`, `AB-00-06`, `AB-00-07`, `AB-00-08`.

## Allowed Write Scope

- `schemas/redaction_export_controls.schema.yaml`
- `core/security/redaction/`
- `tests/security/test_redaction_export_controls.py`
- `docs/security/redaction_export_controls.md`
- `execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-02_Private data redaction and export controls/MEMORY.md`
- deliverable-local run notes if created under this deliverable folder

Do not edit cloud service code, secret material, real private data, destructive
quarantine movement, lifecycle `_STATUS.md`, local `Dependencies.csv`,
coordination evidence, blocker queues, aggregate DAG files, or
implementation-evidence registers.

## Tasks

1. Define redaction/export-control schema and bounded support logic for public
   report/export safeguards.
2. Classify fields or records as public, private, redacted, omitted, or
   warning-only based on explicit metadata, never hidden engineering guesses.
3. Add tests for private rule/material/component/report/export values using
   invented fixtures only.
4. Document redaction/export behavior and update deliverable `MEMORY.md`.

## Acceptance Criteria

- Controls default to protecting private project, material, component,
  rule-pack, owner-standard, company design-basis, path, and secret-like data.
- Missing provenance or unknown redistribution status produces explicit
  warnings or redaction behavior, not silent export.
- Reports/exports do not include protected standards text, copied standards
  tables, proprietary values, real secrets, or private examples.
- The implementation is local-first and does not introduce cloud transmission.
- No automatic professional approval, certification, sealing, or code
  compliance claims are introduced.

## Required Verification

- Run `python3 tests/security/test_redaction_export_controls.py`.
- Run schema JSON validation for `schemas/redaction_export_controls.schema.yaml`.
- Run `git diff --check`.
- Run focused scans for protected standards data, private data, real secrets,
  and prohibited certification/compliance/sealing claims in changed files.

## Stop Conditions

Stop and return to ORCHESTRATOR if work requires real private data, real
secrets, cloud export behavior, destructive quarantine operations, legal
conclusions, local dependency edits, candidate-edge promotion, lifecycle
changes, or professional approval claims.
