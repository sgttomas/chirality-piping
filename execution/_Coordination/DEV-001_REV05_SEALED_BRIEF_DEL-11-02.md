---
doc_id: DEV-001-REV05-SEALED-BRIEF-DEL-11-02
doc_kind: coordination.sealed_type2_brief
status: sealed_brief_prepared_dispatch_not_authorized
created: 2026-05-03
prepared_by: ORCHESTRATOR
tranche: DEV-001_REV05_TRANCHE_A
deliverable_id: DEL-11-02
package_id: PKG-11
worker_launch: not_authorized
---

# Sealed Brief - DEL-11-02 Developer Guide For Solver And Rule Packs

## Dispatch Boundary

This is a sealed Type 2 implementation brief for future worker dispatch. It
does not authorize worker launch by itself. Use only after the human separately
approves worker dispatch for Tranche A sealed briefs.

## Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-11-02` |
| PackageID | `PKG-11` |
| Name | Developer guide for solver and rule packs |
| Type | `DOC_UPDATE` |
| Scope items | `SOW-033` |
| Objectives | `OBJ-001,OBJ-002` |
| Context envelope | `M` |
| Lifecycle at brief prep | `SEMANTIC_READY` |
| Deliverable path | `execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-02_Developer guide for solver and rule packs` |

Local `_CONTEXT.md` still names revision `0.4`; this brief overrides it for
dispatch authority. Use `execution/_Decomposition/SOFTWARE_DECOMP.md` revision
`0.5`, `docs/_Registers/Deliverables.csv`, and approved `DAG-002`.

## Readiness Evidence

`DEV-001_BLOCKER_QUEUE.csv` marks `DEL-11-02` as `UNBLOCKED` with 11 active
upstreams satisfied. Local dependency mirror state is
`SYNCHRONIZED_FROM_APPROVED_DAG002_MIRROR_PRESENT`.

| EdgeID | Upstream | Evidence |
|---|---|---|
| `DAG-002-E0334` | `DEL-00-01` Architecture decision record baseline | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0335` | `DEL-00-02` Repository and module boundary architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0336` | `DEL-00-06` Diagnostics, warning, and result-envelope contract | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0337` | `DEL-00-07` API boundary and adapter contract map | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0338` | `DEL-00-08` Layered software test and acceptance strategy | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0580` | `DEL-02-01` Canonical domain model schema | `COMMITTED 7b256f3` |
| `DAG-002-E0581` | `DEL-02-02` Unit system and dimensional-analysis core contract | `COMMITTED a458cba` |
| `DAG-002-E0582` | `DEL-04-01` 3D frame stiffness kernel | `COMMITTED 1506cc0` |
| `DAG-002-E0583` | `DEL-06-01` Rule-pack schema | `COMMITTED 20241f9` |
| `DAG-002-E0584` | `DEL-10-01` Public API and plugin boundary | `COMMITTED 53cc3d6` |
| `DAG-002-E0585` | `DEL-01-02` Copyright and protected-data boundary policy | `COMMITTED 0d729cf` |

Candidate edges are excluded.

## Applicable Invariants

Apply `OPS-K-IP-1`, `OPS-K-IP-2`, `OPS-K-IP-3`, `OPS-K-DATA-1`,
`OPS-K-DATA-2`, `OPS-K-DATA-3`, `OPS-K-AUTH-1`, `OPS-K-MECH-2`,
`OPS-K-RULE-2`, `OPS-K-RULE-3`, `OPS-K-PRIV-1`, `OPS-K-AGENT-1`,
`OPS-K-AGENT-2`, `OPS-K-AGENT-3`, and `OPS-K-AGENT-4`.

Applicable architecture basis IDs: `AB-00-01`, `AB-00-02`, `AB-00-06`,
`AB-00-07`, `AB-00-08`.

## Allowed Write Scope

- `docs/developer_guide/index.md`
- optional `docs/developer_guide/solver_and_rule_packs.md`
- `execution/PKG-11_Documentation, Examples, and Education/1_Working/DEL-11-02_Developer guide for solver and rule packs/MEMORY.md`
- deliverable-local run notes if created under this deliverable folder

Do not edit `docs/SPEC.md`, `docs/TYPES.md`, rule schemas, solver code,
lifecycle `_STATUS.md`, local `Dependencies.csv`, coordination evidence,
blocker queues, aggregate DAG files, or implementation-evidence registers.

## Tasks

1. Create a developer guide entry for solver architecture, rule-pack schema,
   test discipline, and contribution boundaries.
2. Explain code-neutral mechanics versus user-supplied rule checks and private
   data responsibilities.
3. Reference existing schemas/modules at a high level without copying protected
   formulas, standards text, or proprietary engineering values.
4. Update deliverable `MEMORY.md` with scope, evidence, validation, and
   remaining TBDs.

## Acceptance Criteria

- The guide is actionable for contributors and grounded in existing project
  artifacts.
- It preserves the open-mechanics/private-rule-pack boundary.
- It does not include protected standards text, copied tables, proprietary
  examples, or real project data.
- It avoids certification, sealing, professional approval, and code-compliance
  claims.
- It points unresolved implementation choices to `TBD` or existing governance
  decision surfaces.

## Required Verification

- Run a documentation link/path sanity check where practical.
- Run `git diff --check`.
- Run focused scans for protected standards data, private data, real secrets,
  and prohibited certification/compliance/sealing claims in changed files.

## Stop Conditions

Stop and return to ORCHESTRATOR if work requires protected formulas, standards
examples, legal conclusions, schema/code edits outside the allowed scope,
candidate-edge promotion, lifecycle changes, or professional approval claims.
