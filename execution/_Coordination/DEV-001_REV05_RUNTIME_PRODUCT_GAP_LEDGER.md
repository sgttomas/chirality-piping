---
doc_id: DEV-001-REV05-RUNTIME-PRODUCT-GAP-LEDGER
doc_kind: coordination.runtime_product_gap_ledger
status: proposal_only_complete
created: 2026-05-09
source_acceptance_commit: c0303c4
source_candidate_reconciliation: execution/_Reconciliation/Reconciliation_Run_Summary_2026-05-09_DEV001_REV05_CANDIDATE_EDGE_RECONCILIATION.md
source_release_plan: execution/_Coordination/DEV-001_REV05_RELEASE_READINESS_PLANNING.md
graph_authority: execution/_DAG/DAG-002
full_runtime_claim: not_claimed
product_readiness_claim: not_claimed
graph_mutation: not_authorized_not_performed
candidate_promotion: not_authorized_not_performed
dependency_mirror_refresh: not_authorized_not_performed
lifecycle_evidence_change: not_authorized_not_performed
type2_dispatch: not_performed
---

# DEV-001 Rev 0.5 Runtime/Product Gap Ledger

## Purpose

This ledger identifies product assembly and runtime integration gaps remaining
after accepted DEV-001 revision `0.5` bounded implementation-evidence closure.
It is a planning surface only.

DEV-001 evidence closure proves committed bounded deliverable evidence, not a
complete runnable product, full GUI runtime, live solver/prover workflow,
release package, or professional acceptance workflow.

## Gap Ledger

| Gap ID | Gap | Evidence already present | Missing decision or work | Route |
|---|---|---|---|---|
| RPG-01 | Shared expression architecture | `DEL-05-02` load-case algebra and `DEL-06-02` evaluator are committed separately; `DAG-002-E0616` retire-recommended. | Decide whether to keep separate evaluators or authorize a shared expression grammar/service. | Architecture decision or product integration scope. |
| RPG-02 | Result viewer/export envelope compatibility | `DEL-07-05` viewer and `DEL-08-04` export format are committed; `DAG-002-E0617` retire-recommended. | Define product-level compatibility tests between viewer result envelopes and export schemas. | Product integration plan. |
| RPG-03 | FEA handoff/result export compatibility | `DEL-10-03` handoff contract and `DEL-08-04` result export are committed; `DAG-002-E0618` retire-recommended. | Decide whether handoff packages embed/refer to result exports or remain separate. | Interop architecture decision. |
| RPG-04 | Adapter threat-model feedback | `DEL-10-02` adapter framework and `DEL-12-05` threat model are committed; `DAG-002-E0619` retire-recommended. | Decide if adapter-specific threat-model addendum is required before release. | Release-security review or scoped doc update. |
| RPG-05 | CI/release gate feedback | `DEL-09-05` gates and `DEL-10-04` release skeleton are committed; `DAG-002-E0620` retire-recommended. | Select CI provider, release matrix, thresholds, waiver owners, and evidence retention. | Release-readiness route. |
| RPG-06 | Nonlinear diagnostic refinement | `DEL-04-04` nonlinear supports and `DEL-04-06` diagnostics are committed; `DAG-002-E0622` retire-recommended. | Decide whether nonlinear diagnostic classes need additional regression or warning taxonomy. | Solver validation gap. |
| RPG-07 | Evaluator release security | `DEL-06-02` evaluator and `DEL-12-05` threat model are committed; `DAG-002-E0623` retained non-gating. | Decide hardening, addendum, waiver, or retirement before release claim. | Release-security review. |
| RPG-08 | Shared job orchestration | `DEL-07-07` solve UX and `DEL-10-05` headless runner are committed; `DAG-002-E0624` retire-recommended. | Define common runtime job/progress/cancel orchestration if product runtime is pursued. | Application-service runtime assembly plan. |
| RPG-09 | Full GUI desktop runtime | PKG-07 GUI contract slices are committed. | Build/verify actual desktop shell, visual styling, accessibility conformance, persistence, and live interactions. | Scope-change or new product-assembly initiative. |
| RPG-10 | Live solver/prover execution | Solver, handoff, headless, and external boundary slices are committed. | Integrate live solve/prover execution paths, external tool adapters, result ingestion, and failure handling. | Scope-change with validation/security boundaries. |
| RPG-11 | Persistence/container runtime | Persistence, model state, audit, and private data controls are committed as bounded contracts. | Decide physical project package/container, migrations, encryption/key management, and runtime storage behavior. | Architecture/product scope route. |
| RPG-12 | Reports and public output readiness | Report, result export, protected-content linter, notice, and report-section slices are committed. | Final report layout, release artifact scans, public output review, and professional-boundary wording need acceptance. | Release/product review route. |
| RPG-13 | External integrations | Adapter, handoff, target mapping, export workflow, and external boundary metadata exist. | Concrete target formats, commercial parser behavior, private target data policy, and external solver invocation remain unselected. | Scope-change with IP/security review. |
| RPG-14 | Quarantined Chirality corpus | Corpus remains read-only reference only. | Decide if any architecture or runtime concept should be promoted into active OpenPipeStress scope. | Scope-change or architecture decision. |

## Priority Ordering

Recommended planning order if the project authority wants a product/release
roadmap:

1. Decide release target shape: documentation/evidence archive, headless/CLI,
   GUI runtime, or external integration.
2. Resolve release-security items: `RPG-04`, `RPG-07`, and private/protected
   data checks.
3. Resolve runtime spine: `RPG-08`, `RPG-09`, `RPG-10`, and `RPG-11`.
4. Resolve data/output compatibility: `RPG-02`, `RPG-03`, and `RPG-12`.
5. Resolve external integration and Chirality corpus promotion only through
   explicit scope-change routes.

## Candidate Edge Handling

The gap ledger does not require candidate promotion. The candidate rows are
better treated as planning signals:

- `E0616`, `E0617`, `E0618`, `E0624`: product integration checks.
- `E0619`, `E0620`, `E0622`: feedback loops that should not be promoted as
  prerequisite edges because they risk or create candidate-layer SCC warnings.
- `E0623`: release-security review input.

## Recommended Next Gate

```text
APPROVE: prepare proposal-only DEV-001 revision 0.5 scope-change triage from
the release-readiness plan and runtime/product gap ledger. Classify graph
cleanup, release work, full runtime/product work, external integrations,
professional-authority work, and Chirality corpus promotion routes. Do not
dispatch Type 2 work, mutate DAG-002, promote candidate edges, refresh
dependency mirrors, or change lifecycle/evidence state.
```
