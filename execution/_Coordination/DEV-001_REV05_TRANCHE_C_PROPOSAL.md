---
doc_id: DEV-001-REV05-TRANCHE-C-PROPOSAL
doc_kind: coordination.tranche_proposal
status: proposal_accepted_implementation_authorized
created: 2026-05-04
prepared_by: ORCHESTRATOR
decomposition_revision: "0.5"
graph_authority: execution/_DAG/DAG-002/
approval_record: execution/_DAG/DAG-002/APPROVAL_RECORD.md
blocker_queue: execution/_Coordination/DEV-001_BLOCKER_QUEUE.csv
human_authorization: proposal_preparation_only_2026-05-04
sealed_briefs_status: prepared
dispatch_authorization: human_authorized_orchestrator_local_execution
worker_launch: not_used
---

# DEV-001 Revision 0.5 Tranche C Proposal

## Boundary

This artifact records a proposal-only Tranche C plan for `DEL-10-04 - Build,
packaging, and CI/CD pipeline` from the current approved `DAG-002` revision
`0.5` implementation-readiness state.

Human authorization received:

```text
APPROVE: prepare proposal-only DEV-001 revision 0.5 Tranche C plan for
DEL-10-04 from the current approved DAG-002 readiness state. Do not prepare
sealed briefs or dispatch implementation.
```

This authorization permits proposal preparation only. It does not prepare a
sealed brief, dispatch workers, run implementation, create CI workflows, create
packaging scripts, edit lifecycle state, update implementation evidence,
refresh blocker queues, refresh dependency mirrors, mutate aggregate DAG
artifacts, promote candidate rows, commit file state, or promote the
quarantined Chirality reference corpus.

Implementation authorization later received:

```text
proceed with implementation
```

Because no live CI provider or workflow write scope was named, ORCHESTRATOR
treated this as acceptance of the proposal and authorization to implement the
recommended provider-neutral release/packaging skeleton lane. Sealed brief:
`execution/_Coordination/DEV-001_REV05_SEALED_BRIEF_DEL-10-04.md`.

## Source Inputs

| Surface | Path / state |
|---|---|
| Decomposition | `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.5` |
| Deliverables register | `docs/_Registers/Deliverables.csv` |
| Invariants | `docs/CONTRACT.md` |
| Active graph | `execution/_DAG/DAG-002/DependencyEdges.csv` approved `ACTIVE` edges only |
| Approval | `execution/_DAG/DAG-002/APPROVAL_RECORD.md` |
| Blocker queue | `execution/_Coordination/DEV-001_BLOCKER_QUEUE.csv`: 73 unblocked, 19 blocked |
| Lifecycle projection | `execution/_Coordination/REV05_LIFECYCLE_STATE_SNAPSHOT.csv` |
| Evidence projection | `execution/_Coordination/DEV-001_REV05_IMPLEMENTATION_EVIDENCE_STATUS.csv` |
| Dependency register status | `execution/_Coordination/DEV-001_REV05_DEPENDENCY_REGISTER_STATUS.csv` |
| Local context | `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-04_Build, packaging, and CI-CD pipeline/` |

Candidate edges are excluded. `PKG-00` architecture-basis edges are satisfied
by the accepted architecture baseline and remain brief-injection context, not
implementation work.

The local `_CONTEXT.md` still names decomposition revision `0.4`. Any future
sealed brief must override that stale local reference with accepted revision
`0.5`, approved `DAG-002`, and this proposal.

## Candidate Screen

Current implementation-unblocked deliverables with `MISSING_EVIDENCE` remain
available as proposal candidates, but only `DEL-10-04` is selected for this
proposal because it was newly unblocked by Tranche B evidence promotion and has
a broad `L` context envelope.

| DeliverableID | Name | Lifecycle | Register state | Proposal disposition |
|---|---|---|---|---|
| `DEL-10-04` | Build, packaging, and CI/CD pipeline | `SEMANTIC_READY` | mirror present | Include as single-item Tranche C proposal. |
| `DEL-07-02` / `DEL-07-03` / `DEL-07-04` / `DEL-07-05` / `DEL-07-07` | GUI slices | `SEMANTIC_READY` | mirror present | Hold for a coordinated GUI tranche with shared app-shell/state ownership. |
| `DEL-11-05` | Contributor tutorial and onboarding | `SEMANTIC_READY` | mirror present | Hold to avoid mixing release automation with contributor education. |
| `DEL-12-04` | Secret and private-library handling | `SEMANTIC_READY` | mirror present | Hold for a later security tranche or explicit coordination with redaction/secret policy. |
| `DEL-13-01` / `DEL-14-01` | New revision `0.5` schema/state surfaces | `OPEN` | mirror pending | Hold until local mirror refresh or equivalent sealed-brief evidence. |

## Proposed Tranche C

Tranche C contains one deliverable.

| DeliverableID | Package | Type | Scope / objectives | Readiness basis |
|---|---|---|---|---|
| `DEL-10-04` | `PKG-10` | `CI_CD_CHANGE` | `SOW-032` / `OBJ-008,OBJ-009` | 11 active upstreams satisfied; local mirror synchronized; evidence missing only for this deliverable. |

Because `DEL-10-04` spans build reproducibility, packaging, CI, release notes,
quality gates, and supply-chain constraints, the recommended tranche size is
one deliverable. Future implementation should be split internally by write
scope and should stop if the task expands into live release automation,
signing, publishing, provider-specific secrets, or final threshold decisions
without a later explicit human ruling.

## Upstream Readiness

`DEL-10-04` active upstreams:

| EdgeID | Upstream | Readiness basis |
|---|---|---|
| `DAG-002-E0315` | `DEL-00-01` Architecture decision record baseline | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0316` | `DEL-00-02` Repository and module boundary architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0317` | `DEL-00-03` Application service command-query-job model | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0318` | `DEL-00-04` Persistence and schema versioning architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0319` | `DEL-00-06` Diagnostics, warning, and result-envelope contract | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0320` | `DEL-00-07` API boundary and adapter contract map | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0321` | `DEL-00-08` Layered software test and acceptance strategy | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0571` | `DEL-09-05` Release quality gate checklist | `COMMITTED` evidence `03344e6` |
| `DAG-002-E0572` | `DEL-10-05` Headless CLI and structured I/O analysis runner | `COMMITTED` evidence `9de5e9b` |
| `DAG-002-E0573` | `DEL-08-05` Report protected-content linter | `COMMITTED` evidence `69adffa` |
| `DAG-002-E0574` | `DEL-12-05` Security threat model | `COMMITTED` evidence `b97121d` |

## Candidate Edge Caveat

`DAG-002-E0620` is retained as a non-gating candidate row from `DEL-09-05` to
`DEL-10-04`. It records that CI implementation may later refine release quality
gate details. It must not be used as readiness authority, blocker authority,
dispatch authority, schedule, priority, or lifecycle evidence.

The approved active readiness direction for this proposal is the opposite:
`DEL-10-04` depends on `DEL-09-05` through `DAG-002-E0571`, and Tranche B has
now supplied `COMMITTED` evidence for `DEL-09-05`.

## Proposed Write Ownership For A Future Brief

No sealed brief is prepared by this proposal. If the human later accepts this
proposal for sealed brief preparation, the future brief should keep the write
scope explicit and choose one implementation lane before dispatch.

Recommended lane:

| Lane | Write scope | Rationale |
|---|---|---|
| Provider-neutral release/packaging skeleton | `docs/BUILD_AND_RELEASE.md`; `docs/RELEASE_NOTES_TEMPLATE.md`; optional `tools/release/` scripts that run local checks without external services; deliverable-local `MEMORY.md` / run notes | Preserves the `CI provider`, release matrix, signing, publishing, and threshold `TBD` decisions while still creating reproducibility and release-evidence structure. |

Provider-specific lane, only after explicit human ruling:

| Lane | Write scope | Additional gate needed |
|---|---|---|
| Live CI workflow skeleton | `.github/workflows/` or another named provider path; package/release scripts; release-note template; deliverable-local `MEMORY.md` / run notes | Human must approve the CI provider, workflow path, external-service boundary, secret policy, and whether workflow files may be committed. |

Shared coordination surfaces remain excluded from future worker write scope
unless a later closeout gate explicitly grants updates:

- `execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv`
- `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md`
- `execution/_Coordination/DEV-001_BLOCKER_QUEUE.csv`
- `execution/_Coordination/REV05_LIFECYCLE_STATE_SNAPSHOT.csv`
- `execution/_Coordination/DEV-001_REV05_IMPLEMENTATION_EVIDENCE_STATUS.csv`
- `execution/_Coordination/DEV-001_REV05_DEPENDENCY_REGISTER_STATUS.csv`
- aggregate `execution/_DAG/DAG-002/*`
- local `Dependencies.csv` mirrors
- lifecycle `_STATUS.md` files, unless a later closeout gate explicitly grants
  lifecycle updates

## Brief Preconditions

Before sealed brief preparation:

1. Human must accept or revise this proposal.
2. Human must choose whether `DEL-10-04` should use the recommended
   provider-neutral lane or a provider-specific live CI lane.
3. If provider-specific, human must name the CI provider and permitted workflow
   path before sealed brief preparation.
4. ORCHESTRATOR must prepare one sealed brief from revision `0.5` registers,
   approved `DAG-002`, current local context, applicable `AB-00-*` rows,
   invariants, acceptance criteria, guardrails, validation expectations, and
   the chosen write scope.
5. The sealed brief must leave final numerical thresholds, supported platform
   matrix, signing, release attestation, publishing, and release authority as
   `TBD` unless separately governed.

## Validation Expectations For A Future Brief

Future sealed-brief validation should require focused checks appropriate to the
chosen lane:

- local documentation path sanity check for referenced repository paths;
- local command dry-run for any provider-neutral scripts;
- applicable existing Python tests, especially contract/schema tests touched by
  the release evidence model;
- applicable focused Cargo tests for headless runner and protected-content
  linter integration if those scripts call them;
- aggregate `DAG-002` schema/audit checks only in closeout if coordination
  state is updated;
- `git diff --check`;
- focused scans for protected standards data, private project data, real
  secrets, signing secrets, tokens, and prohibited certification/compliance/
  sealing claims.

No future brief should require network access, external CI execution, release
publishing, signing/notarization, or platform packaging on unsupported local
hosts unless a later human gate explicitly authorizes that scope.

## Recommendation

Accept this as a single-item Tranche C candidate only if the next intended step
is sealed brief preparation for `DEL-10-04`. The safest next gate is:

```text
APPROVE: accept DEV-001 revision 0.5 Tranche C proposal for DEL-10-04 and
prepare one sealed brief using the provider-neutral release/packaging skeleton
lane. Do not dispatch implementation.
```

If the human wants a live CI workflow, replace the provider-neutral lane with a
named CI provider and explicit workflow write scope in the approval.
