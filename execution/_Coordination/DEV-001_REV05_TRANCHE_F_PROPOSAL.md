---
doc_id: DEV-001-REV05-TRANCHE-F-PROPOSAL
doc_kind: coordination.tranche_proposal
status: sealed_briefs_prepared_dispatch_not_authorized
created: 2026-05-04
prepared_by: ORCHESTRATOR
decomposition_revision: "0.5"
graph_authority: execution/_DAG/DAG-002/
approval_record: execution/_DAG/DAG-002/APPROVAL_RECORD.md
blocker_queue: execution/_Coordination/DEV-001_BLOCKER_QUEUE.csv
source_assessment: execution/_Coordination/DEV-001_REV05_POST_TRANCHE_E_NEXT_STEP_ASSESSMENT.md
human_authorization: proposal_preparation_only_2026-05-04
accepted_for_brief_preparation: 2026-05-04
sealed_briefs_status: prepared
dispatch_authorization: not_authorized
---

# DEV-001 Revision 0.5 Tranche F Proposal

## Boundary

This artifact records a proposal-only DEV-001 revision `0.5` Tranche F plan
for `DEL-13-03`, `DEL-14-05`, and `DEL-15-01` from the current approved
`DAG-002` implementation-readiness state and the committed Tranche E evidence.

Human authorization received:

```text
APPROVE: prepare a proposal-only DEV-001 revision 0.5 Tranche F proposal for
DEL-13-03, DEL-14-05, and DEL-15-01 from the current approved DAG-002 readiness
state. Do not prepare sealed briefs or dispatch implementation.
```

This authorization permits proposal preparation only. It does not prepare
sealed briefs, dispatch workers, run implementation, change lifecycle state,
update implementation evidence, refresh dependency mirrors, recompute the
blocker queue, mutate aggregate DAG artifacts, promote candidate rows, commit
file state, push, run live CI/signing/publishing, claim professional acceptance,
start autonomous mutation workflow, or promote the quarantined Chirality
reference corpus.

The human project authority later accepted this proposal for sealed brief
preparation:

```text
APPROVE: accept DEV-001 revision 0.5 Tranche F proposal and prepare sealed
briefs for DEL-13-03, DEL-14-05, and DEL-15-01 using the contract-first
provider-neutral validation/comparison/handoff lane. Do not dispatch
implementation.
```

Prepared sealed briefs:

- `execution/_Coordination/DEV-001_REV05_SEALED_BRIEF_DEL-13-03.md`
- `execution/_Coordination/DEV-001_REV05_SEALED_BRIEF_DEL-14-05.md`
- `execution/_Coordination/DEV-001_REV05_SEALED_BRIEF_DEL-15-01.md`

The later authorization permits sealed brief preparation only. It does not
dispatch workers, run implementation, change lifecycle state, update
implementation evidence, refresh dependency mirrors, recompute the blocker
queue, mutate aggregate DAG artifacts, promote candidate rows, commit file
state, push, run live CI/signing/publishing, claim professional acceptance,
start autonomous mutation workflow, or promote the quarantined Chirality
reference corpus.

## Source Inputs

| Surface | Path / state |
|---|---|
| Decomposition | `execution/_Decomposition/SOFTWARE_DECOMP.md` revision `0.5` |
| Deliverables register | `docs/_Registers/Deliverables.csv` |
| Invariants | `docs/CONTRACT.md` |
| Active graph | `execution/_DAG/DAG-002/DependencyEdges.csv` approved `ACTIVE` edges only |
| Approval | `execution/_DAG/DAG-002/APPROVAL_RECORD.md` |
| Source assessment | `execution/_Coordination/DEV-001_REV05_POST_TRANCHE_E_NEXT_STEP_ASSESSMENT.md` |
| Blocker queue | `execution/_Coordination/DEV-001_BLOCKER_QUEUE.csv`: 79 unblocked, 13 blocked after closeout preparation |
| Lifecycle projection | `execution/_Coordination/REV05_LIFECYCLE_STATE_SNAPSHOT.csv`: 64 `CHECKING`, 28 `SEMANTIC_READY`, 0 `OPEN` after closeout preparation |
| Evidence projection | `execution/_Coordination/DEV-001_REV05_IMPLEMENTATION_EVIDENCE_STATUS.csv`: 61 `COMMITTED`, 3 `WORKING_TREE`, 8 `ARCHITECTURE_BASELINE`, 20 `MISSING_EVIDENCE` after closeout preparation |
| Dependency register status | `execution/_Coordination/DEV-001_REV05_DEPENDENCY_REGISTER_STATUS.csv`: 84 synchronized non-`PKG-00` mirrors, 8 `PKG-00` exemptions |
| Local contexts | `execution/PKG-13_Physical Design Knowledge and Constraint Engine/1_Working/DEL-13-03_Constraint validation engine/`; `execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-05_Comparison mapping, tolerance, and export contracts/`; `execution/PKG-15_Handoff and External Prover Workflow/1_Working/DEL-15-01_Canonical handoff package schema and manifest/` |

Candidate edges are excluded. `PKG-00` architecture-basis edges are satisfied by
the accepted architecture baseline and remain brief-injection context, not
implementation work.

## Candidate Screen

The post-Tranche E assessment screened ten implementation-unblocked
`MISSING_EVIDENCE` deliverables. This proposal selects the three SCA-002
foundation surfaces newly unblocked by the Tranche E `COMMITTED` evidence
promotion.

| DeliverableID | Name | Lifecycle | Register state | Proposal disposition |
|---|---|---|---|---|
| `DEL-07-02` | Model tree and property inspector | `SEMANTIC_READY` | mirror present | Hold for a coordinated GUI tranche with explicit app-shell/state lane. |
| `DEL-07-03` | Material, component, and rule-pack editors | `SEMANTIC_READY` | mirror present | Hold for a coordinated GUI/editor tranche; context envelope is `L`. |
| `DEL-07-04` | Missing-data warning and blocking UX | `SEMANTIC_READY` | mirror present | Hold for coordinated GUI diagnostics work. |
| `DEL-07-05` | Results viewer | `SEMANTIC_READY` | mirror present | Hold for coordinated GUI results work; context envelope is `L`. |
| `DEL-07-07` | Solve execution UX: progress, cancellation, and diagnostics | `SEMANTIC_READY` | mirror present | Hold for coordinated GUI job/progress work. |
| `DEL-11-05` | Contributor tutorial and onboarding | `SEMANTIC_READY` | mirror present | Defer; useful leaf item but does not unblock implementation consumers. |
| `DEL-12-04` | Secret and private-library handling | `SEMANTIC_READY` | mirror present | Defer unless a security tranche is selected. |
| `DEL-13-03` | Constraint validation engine | `SEMANTIC_READY` | mirror present | Include in Tranche F. |
| `DEL-14-05` | Comparison mapping, tolerance, and export contracts | `SEMANTIC_READY` | mirror present | Include in Tranche F. |
| `DEL-15-01` | Canonical handoff package schema and manifest | `SEMANTIC_READY` | mirror present | Include in Tranche F. |

## Proposed Tranche F

Tranche F contains three deliverables. All are implementation-unblocked from
approved active `DAG-002` edges, have synchronized local dependency mirrors,
and can be assigned disjoint contract-first write scopes.

| DeliverableID | Package | Type | Scope / objectives | Readiness basis |
|---|---|---|---|---|
| `DEL-13-03` | `PKG-13` | `BACKEND_FEATURE_SLICE` | `SOW-068` / `OBJ-014` | 12 active upstreams satisfied; local mirror synchronized; evidence missing only for this deliverable. |
| `DEL-14-05` | `PKG-14` | `API_CONTRACT` | `SOW-073` / `OBJ-016` | 11 active upstreams satisfied; local mirror synchronized; evidence missing only for this deliverable. |
| `DEL-15-01` | `PKG-15` | `API_CONTRACT` | `SOW-074` / `OBJ-017` | 13 active upstreams satisfied; local mirror synchronized; evidence missing only for this deliverable. |

Recommended implementation lane for any later accepted brief preparation:
contract-first, provider-neutral validation/comparison/handoff foundations.

The lane should:

- create a deterministic constraint-validation module shape without inferring
  hidden owner standards, protected code requirements, proprietary project
  criteria, or final engineering acceptance logic;
- define comparison mapping, unmatched classification, tolerance, and export
  contracts without comprehensive commercial-prover ingestion or external
  validation decisions;
- define a canonical handoff package and manifest contract that carries hashes,
  units, entity IDs, library/rule references, unresolved assumptions, warnings,
  target mapping metadata, unsupported-target flags, and provenance without
  creating professional approval records.

The lane must not create live GUI state, external prover execution,
commercial-tool parsers, private storage behavior, physical project
package/container behavior, signing/publishing behavior, autonomous accepted
model mutation, or professional/code-compliance acceptance logic.

## Upstream Readiness

`DEL-13-03` active upstreams:

| EdgeID | Upstream | Readiness basis |
|---|---|---|
| `DAG-002-E0653` | `DEL-00-01` Architecture decision record baseline | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0654` | `DEL-00-02` Repository and module boundary architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0655` | `DEL-00-03` Application service command-query-job model | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0656` | `DEL-00-04` Persistence and schema versioning architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0657` | `DEL-00-06` Diagnostics, warning, and result-envelope contract | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0658` | `DEL-00-07` API boundary and adapter contract map | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0659` | `DEL-00-08` Layered software test and acceptance strategy | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0767` | `DEL-13-01` Design knowledge schema and provenance model | `COMMITTED` evidence `dcdc1ac` |
| `DAG-002-E0768` | `DEL-13-02` Constraint entity and provenance model | `COMMITTED` evidence `002263b` |
| `DAG-002-E0769` | `DEL-02-02` Unit system and dimensional-analysis core contract | `COMMITTED` evidence `a458cba` |
| `DAG-002-E0770` | `DEL-04-06` Solver diagnostics and singularity detection | `COMMITTED` evidence `fdb0252` |
| `DAG-002-E0771` | `DEL-02-05` Project persistence and round-trip serialization | `COMMITTED` evidence `742016e` |

`DEL-14-05` active upstreams:

| EdgeID | Upstream | Readiness basis |
|---|---|---|
| `DAG-002-E0695` | `DEL-00-01` Architecture decision record baseline | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0696` | `DEL-00-02` Repository and module boundary architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0697` | `DEL-00-03` Application service command-query-job model | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0698` | `DEL-00-04` Persistence and schema versioning architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0699` | `DEL-00-06` Diagnostics, warning, and result-envelope contract | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0700` | `DEL-00-07` API boundary and adapter contract map | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0701` | `DEL-00-08` Layered software test and acceptance strategy | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0788` | `DEL-14-01` Immutable model state records | `COMMITTED` evidence `dcdc1ac` |
| `DAG-002-E0789` | `DEL-14-02` Analysis run records | `COMMITTED` evidence `002263b` |
| `DAG-002-E0790` | `DEL-08-04` Result export format | `COMMITTED` evidence `3e33ea4` |
| `DAG-002-E0791` | `DEL-02-02` Unit system and dimensional-analysis core contract | `COMMITTED` evidence `a458cba` |

`DEL-15-01` active upstreams:

| EdgeID | Upstream | Readiness basis |
|---|---|---|
| `DAG-002-E0702` | `DEL-00-01` Architecture decision record baseline | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0703` | `DEL-00-02` Repository and module boundary architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0704` | `DEL-00-03` Application service command-query-job model | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0705` | `DEL-00-04` Persistence and schema versioning architecture | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0706` | `DEL-00-06` Diagnostics, warning, and result-envelope contract | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0707` | `DEL-00-07` API boundary and adapter contract map | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0708` | `DEL-00-08` Layered software test and acceptance strategy | `ARCHITECTURE_BASELINE` |
| `DAG-002-E0799` | `DEL-10-03` Local FEA handoff data contract | `COMMITTED` evidence `abdecbd` |
| `DAG-002-E0800` | `DEL-08-04` Result export format | `COMMITTED` evidence `3e33ea4` |
| `DAG-002-E0801` | `DEL-08-02` Audit manifest and model hash | `COMMITTED` evidence `061f1af` |
| `DAG-002-E0802` | `DEL-14-01` Immutable model state records | `COMMITTED` evidence `dcdc1ac` |
| `DAG-002-E0803` | `DEL-14-02` Analysis run records | `COMMITTED` evidence `002263b` |
| `DAG-002-E0804` | `DEL-02-01` Canonical domain model schema | `COMMITTED` evidence `7b256f3` |

## Downstream Impact

If a later implementation and evidence-promotion gate completed all three
Tranche F deliverables as `COMMITTED`, the current blocker queue indicates
these direct downstream effects:

| Proposed deliverable | Direct active downstream consumers | Currently newly unblockable by this tranche |
|---|---|---|
| `DEL-13-03` | `DEL-13-04`, `DEL-16-02`, `DEL-07-08` | `DEL-13-04` |
| `DEL-14-05` | `DEL-14-03`, `DEL-14-04`, `DEL-15-02`, `DEL-15-03`, `DEL-16-02`, `DEL-07-08`, `DEL-08-06` | `DEL-14-03`, `DEL-14-04` |
| `DEL-15-01` | `DEL-15-02`, `DEL-15-03`, `DEL-15-04`, `DEL-08-06` | none by itself; it removes one blocker from each listed downstream consumer |

Planning simulation only: if `DEL-13-03`, `DEL-14-05`, and `DEL-15-01` were
later implemented and promoted to `COMMITTED`, the queue would become 82
unblocked / 10 blocked, with `DEL-13-04`, `DEL-14-03`, and `DEL-14-04`
newly implementation-ready. This proposal does not run that promotion or
refresh the blocker queue.

## Write Ownership In Prepared Briefs

The sealed briefs prepared from this proposal keep write scope disjoint.

| DeliverableID | Prepared brief write scope | Explicit exclusions |
|---|---|---|
| `DEL-13-03` | New constraint-validation module under a single brief-named path, recommended as `core/constraints/validation/`; focused validation diagnostics tests, recommended as `tests/test_constraint_validation.py` or crate-local tests if a Rust crate is selected; deliverable-local `MEMORY.md` / run notes | No hidden owner standards, no protected code requirements, no proprietary project constraints, no automatic engineering defaults, no final engineering acceptance logic, no direct accepted-model mutation, no parallel worker edits to shared docs. |
| `DEL-14-05` | `schemas/comparison_mapping.schema.json`; `schemas/comparison_tolerance.schema.json`; comparison export contract tests, recommended as `tests/test_comparison_contracts.py`; optional export-contract stubs only if the future sealed brief names exact paths; deliverable-local `MEMORY.md` / run notes | No comprehensive commercial prover output ingestion, no external validation decision, no hard-coded tolerance defaults unless human-governed, no result-delta engine implementation, no parallel worker edits to shared docs. |
| `DEL-15-01` | `schemas/handoff_package.schema.json`; handoff manifest schema/tests, recommended as `tests/test_handoff_package_schema.py`; deliverable-local `MEMORY.md` / run notes | No target-specific commercial-tool parser, no professional approval record, no physical package/container finalization unless separately governed, no private payloads, no parallel worker edits to shared docs. |

The prepared sealed briefs leave shared `docs/SPEC.md` and `docs/TYPES.md`
integration to ORCHESTRATOR after implementation return or to a later closeout
gate.

Shared coordination surfaces remain excluded from future worker write scope
unless a later REVIEW/AUDIT/CHANGE closeout gate explicitly grants updates:

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
- candidate edge status

## Brief Preconditions

Sealed brief preparation preconditions:

1. Completed: human accepted this proposal and authorized sealed brief
   preparation only.
2. Completed: ORCHESTRATOR prepared one sealed brief per deliverable from revision
   `0.5` registers, approved `DAG-002`, current local context, applicable
   `AB-00-*` rows, invariants, acceptance criteria, guardrails, validation
   expectations, and the write ownership above.
3. Completed: each sealed brief states that the local dependency mirror is evidence
   synchronized from approved `DAG-002`, not independent sequencing authority.
4. Completed: each sealed brief keeps `PKG-00` architecture-basis content as
   applicable constraints and must not copy full `PKG-00` prose into
   deliverable artifacts.
5. Completed: any exact module path not already named by a local context is named in
   the sealed brief before implementation starts.

No prerequisite dependency mirror refresh is required for the three selected
deliverables because their local mirrors are already synchronized from approved
`DAG-002`.

## Validation Expectations In Prepared Briefs

Prepared sealed briefs require focused validation plus tranche-level checks:

- `DEL-13-03`: deterministic constraint-validation tests covering available
  design constraints, missing required data, diagnostics severity, provenance,
  unit references where relevant, and no professional/code-compliance claims;
  adjacent constraint, design-knowledge, unit, diagnostics, and persistence
  checks where referenced.
- `DEL-14-05`: JSON/schema parse checks for comparison mapping and tolerance
  schemas; focused comparison contract tests for stable IDs, manual mappings,
  unmatched classifications, unit-normalized tolerance metadata, JSON/CSV
  export semantics, and professional-boundary notices; adjacent model-state,
  analysis-run, result-export, and unit checks where referenced.
- `DEL-15-01`: JSON/schema parse check for
  `schemas/handoff_package.schema.json`; focused handoff manifest tests for
  hashes, units, entity IDs, library/rule references, unresolved assumptions,
  warnings, target mapping metadata, unsupported-target flags, provenance, and
  professional-boundary notices; adjacent local FEA handoff, result export,
  audit/hash, model-state, analysis-run, model, and unit checks where
  referenced.
- Documentation path sanity checks for any touched docs.
- `git diff --check`.
- Focused scans for protected standards data, owner/project data, private
  payloads, proprietary examples, real secrets, and prohibited certification/
  compliance/sealing/professional-approval claims.
- No candidate-edge use.
- No lifecycle, evidence, blocker, dependency-register, aggregate-DAG, or
  candidate-edge update by workers.

## Stop Conditions

After this working-tree closeout preparation, stop before commit,
`COMMITTED` evidence promotion, dependency refresh, candidate promotion,
aggregate DAG mutation, push, live CI/signing/publishing, GUI runtime work,
external prover integration, commercial-tool parser work, protected standards
content, private project data, real secrets, autonomous accepted-model
mutation, or professional/code compliance claims unless separately authorized.

## Implementation Outputs

| DeliverableID | Working-tree outputs |
|---|---|
| `DEL-13-03` | `core/constraints/validation/`; `tests/test_constraint_validation.py`; deliverable `MEMORY.md`; deliverable run note |
| `DEL-14-05` | `schemas/comparison_mapping.schema.json`; `schemas/comparison_tolerance.schema.json`; `tests/test_comparison_contracts.py`; deliverable `MEMORY.md` |
| `DEL-15-01` | `schemas/handoff_package.schema.json`; `tests/test_handoff_package_schema.py`; deliverable `MEMORY.md` |

## Review/Audit Closeout

Closeout record:

- `execution/_Coordination/DEV-001_REV05_TRANCHE_F_REVIEW_AUDIT_CLOSEOUT.md`

Prepared closeout state:

| Fact | State |
|---|---:|
| `DEL-13-03` lifecycle/evidence | `CHECKING` / `WORKING_TREE` |
| `DEL-14-05` lifecycle/evidence | `CHECKING` / `WORKING_TREE` |
| `DEL-15-01` lifecycle/evidence | `CHECKING` / `WORKING_TREE` |
| Implementation evidence records | 64 |
| Committed implementation evidence records | 61 |
| Working-tree evidence records | 3 |
| Blocker queue | 79 unblocked / 13 blocked |
| Newly unblocked by closeout | None; `WORKING_TREE` evidence does not satisfy the `COMMITTED` threshold. |

The queue was regenerated from approved active `DAG-002` edges under the
unchanged `COMMITTED` threshold after recording `WORKING_TREE` evidence.

## Recommendation

The proposal has been accepted, the briefs are prepared, worker implementation
is complete, and post-worker REVIEW/AUDIT closeout preparation is complete.
The next gated action, if accepted, is CHANGE commit and evidence promotion:

```text
APPROVE: CHANGE commit DEV-001 revision 0.5 Tranche F working-tree
implementation and closeout patch, then promote DEL-13-03, DEL-14-05, and
DEL-15-01 implementation evidence from WORKING_TREE to COMMITTED using the
resulting commit hash and rebuild the blocker queue. Commit the promotion
handoff. Do not push.
```
