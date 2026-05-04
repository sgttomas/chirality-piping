---
doc_id: DEV-001-REV05-SEALED-BRIEF-DEL-10-04
doc_kind: coordination.sealed_type2_brief
status: sealed_brief_prepared_implementation_authorized
created: 2026-05-04
prepared_by: ORCHESTRATOR
tranche: DEV-001_REV05_TRANCHE_C
deliverable_id: DEL-10-04
package_id: PKG-10
worker_launch: not_used_orchestrator_local_execution
implementation_lane: provider_neutral_release_packaging_skeleton
---

# Sealed Brief - DEL-10-04 Build, Packaging, and CI/CD Pipeline

## Dispatch Boundary

This is a sealed Type 2 implementation brief for DEV-001 revision `0.5`
Tranche C. The human project authority accepted the Tranche C proposal and
authorized implementation with:

```text
proceed with implementation
```

Because no live CI provider, workflow path, signing policy, publishing target,
or platform release matrix was named, ORCHESTRATOR is executing the
provider-neutral release/packaging skeleton lane recommended by
`execution/_Coordination/DEV-001_REV05_TRANCHE_C_PROPOSAL.md`.

ORCHESTRATOR is executing this brief locally. No spawned worker agent is used.

## Identity

| Field | Value |
|---|---|
| DeliverableID | `DEL-10-04` |
| PackageID | `PKG-10` |
| Name | Build, packaging, and CI/CD pipeline |
| Type | `CI_CD_CHANGE` |
| Scope items | `SOW-032` |
| Objectives | `OBJ-008,OBJ-009` |
| Context envelope | `L` |
| Lifecycle at brief prep | `SEMANTIC_READY` |
| Deliverable path | `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-04_Build, packaging, and CI-CD pipeline` |

Local `_CONTEXT.md` still names revision `0.4`; this brief overrides it for
dispatch authority. Use `execution/_Decomposition/SOFTWARE_DECOMP.md` revision
`0.5`, `docs/_Registers/Deliverables.csv`, approved `DAG-002`, and
`execution/_Coordination/DEV-001_REV05_TRANCHE_C_PROPOSAL.md`.

## Readiness Evidence

`DEV-001_BLOCKER_QUEUE.csv` marks `DEL-10-04` as `UNBLOCKED` with 11 active
upstreams satisfied. Local dependency mirror state is
`SYNCHRONIZED_FROM_APPROVED_DAG002_MIRROR_PRESENT`.

| EdgeID | Upstream | Evidence |
|---|---|---|
| `DAG-002-E0315` | `DEL-00-01` Architecture decision record baseline | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0316` | `DEL-00-02` Repository and module boundary architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0317` | `DEL-00-03` Application service command-query-job model | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0318` | `DEL-00-04` Persistence and schema versioning architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0319` | `DEL-00-06` Diagnostics, warning, and result-envelope contract | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0320` | `DEL-00-07` API boundary and adapter contract map | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0321` | `DEL-00-08` Layered software test and acceptance strategy | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0571` | `DEL-09-05` Release quality gate checklist | `COMMITTED 03344e6` |
| `DAG-002-E0572` | `DEL-10-05` Headless CLI and structured I/O analysis runner | `COMMITTED 9de5e9b` |
| `DAG-002-E0573` | `DEL-08-05` Report protected-content linter | `COMMITTED 69adffa` |
| `DAG-002-E0574` | `DEL-12-05` Security threat model | `COMMITTED b97121d` |

`DAG-002-E0620` is a retained candidate row from `DEL-09-05` to `DEL-10-04`.
It is excluded from readiness, blocker, dispatch, and lifecycle authority.

## Applicable Invariants

Apply `OPS-K-IP-1`, `OPS-K-IP-2`, `OPS-K-IP-3`, `OPS-K-DATA-1`,
`OPS-K-DATA-2`, `OPS-K-DATA-3`, `OPS-K-AUTH-1`, `OPS-K-AUTH-2`,
`OPS-K-MECH-2`, `OPS-K-UNIT-1`, `OPS-K-SOLVER-1`, `OPS-K-SOLVER-2`,
`OPS-K-RULE-1`, `OPS-K-RULE-2`, `OPS-K-RULE-3`, `OPS-K-REPORT-1`,
`OPS-K-REPORT-2`, `OPS-K-PRIV-1`, `OPS-K-PRIV-2`, `OPS-K-GOV-3`,
`OPS-K-GOV-4`, `OPS-K-AGENT-1`, `OPS-K-AGENT-2`, `OPS-K-AGENT-3`, and
`OPS-K-AGENT-4`.

Applicable architecture basis IDs: `AB-00-01`, `AB-00-02`, `AB-00-03`,
`AB-00-04`, `AB-00-06`, `AB-00-07`, `AB-00-08`.

## Allowed Write Scope

- `docs/BUILD_AND_RELEASE.md`
- `docs/RELEASE_NOTES_TEMPLATE.md`
- `tools/release/check_release_readiness.py`
- focused tests for the provider-neutral release-readiness script
- `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-04_Build, packaging, and CI-CD pipeline/MEMORY.md`
- deliverable-local run notes under the `DEL-10-04` folder

Do not edit `.github/`, live CI workflow files, package-manager manifests,
release signing or publishing configuration, lifecycle `_STATUS.md`, local
`Dependencies.csv`, `_DEPENDENCIES.md`, coordination implementation-evidence
registers, blocker queues, dependency-status projections, aggregate DAG files,
candidate rows, or committed evidence state.

## Tasks

1. Create a provider-neutral build and release guide that records reproducible
   local build/test evidence, release artifact expectations, package-matrix
   placeholders, and external-service boundaries.
2. Create a release-notes template for software-quality evidence, known
   limitations, data-boundary notices, and professional-boundary notices.
3. Add a local release-readiness script that discovers repository check
   surfaces and can dry-run or execute local checks without network access,
   signing, publishing, or CI-provider integration.
4. Add focused tests for the release-readiness script.
5. Update deliverable `MEMORY.md` and run notes with scope, evidence,
   validation, non-actions, and remaining `TBD` items.

## Acceptance Criteria

- Build/release documentation is provider-neutral and does not name a live CI
  provider as selected.
- The local readiness script has a dry-run mode, does not use `shell=True`,
  does not write release artifacts, and does not access external services.
- The script discovers existing Cargo manifests and repository validation
  surfaces without assuming a root Cargo workspace or root JavaScript package.
- Release notes explicitly separate software-quality evidence from
  professional engineering acceptance.
- Final CI provider, coverage thresholds, performance thresholds, release
  matrix, signing, notarization, attestation, publishing, and maintainer quorum
  remain `TBD`.
- No protected standards content, proprietary data, private project data, real
  secrets, or automatic certification/compliance/sealing claims are introduced.

## Required Verification

- Run a documentation path sanity check for referenced repository paths.
- Run the provider-neutral release-readiness script in dry-run mode.
- Run focused tests for the release-readiness script.
- Run `git diff --check`.
- Run focused scans for protected standards data, private data, real secrets,
  and prohibited certification/compliance/sealing claims in changed files.

## Stop Conditions

Stop and return to ORCHESTRATOR if work requires live CI workflow edits,
external release services, signing/notarization keys, publishing credentials,
final platform matrix decisions, final numerical thresholds, protected
standards content, real project data, candidate-edge promotion, dependency
register edits, lifecycle changes, implementation-evidence promotion, or
professional approval claims.
