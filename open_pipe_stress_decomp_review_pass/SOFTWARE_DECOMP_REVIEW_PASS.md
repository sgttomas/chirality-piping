---
doc_id: OPS-SOFTWARE-DECOMP-REVIEW-PASS-001
doc_kind: review.software_decomp
status: draft_for_human_decision
created: 2026-04-30
reviewed_candidate: docs/_Decomposition/SOFTWARE_DECOMP.md revision 0.1 Draft
proposed_revision: 0.2-proposed
---

# SOFTWARE_DECOMP Formal Review Pass — OpenPipeStress

## Verdict

**Do not confirm the existing candidate decomposition as the accepted downstream basis as-is.**

The candidate decomposition is structurally sound and close to acceptable, but the formal pass found PRD-derived gaps and a small register consistency defect. This review therefore proposes a bounded **v0.2 amendment** rather than final acceptance.

No PREPARATION or Type 2 execution should proceed until the human project authority either accepts this amendment or rules on the proposed changes.

## Evidence reviewed

- `docs/INIT.md`
- `docs/AGENTS.md`
- `docs/INTENT.md`
- `docs/PRD.md`
- `docs/DIRECTIVE.md`
- `docs/TYPES.md`
- `docs/CONTRACT.md`
- `docs/SPEC.md`
- `docs/IP_AND_DATA_BOUNDARY.md`
- `docs/VALIDATION_STRATEGY.md`
- `docs/_Decomposition/SOFTWARE_DECOMP.md`
- `docs/_Registers/ScopeLedger.csv`
- `docs/_Registers/Deliverables.csv`
- `docs/_Registers/ContextBudgetQA.csv`
- `AGENT_SOFTWARE_DECOMP.md`

## Gate-by-gate review

|Gate|Result|Notes|
|---|---|---|
|Gate 1 — Intake|PASS with note|The intake reflects the product concept: open mechanics, private code data, centerline analysis, human responsibility.|
|Gate 2 — SSOW|REVISE|The SSOW captures most scope but misses several PRD must/exit-criteria items: project persistence, pipe section property calculations, concentrated loads, solver diagnostics, headless structured I/O, and GUI solve execution controls.|
|Gate 3 — Objectives|REVISE|Existing objectives are meaningful, but unit-safe deterministic data flow is load-bearing enough to deserve explicit `OBJ-012`.|
|Gate 4 — Packages|PASS with amendment|The 12 flat packages remain valid. No new package is required; added scope fits existing packages.|
|Gate 5 — Deliverables|REVISE|Six bounded deliverables are proposed. No existing deliverable is deleted or renumbered.|
|Gate 6 — Coverage/context budget|REVISE|The existing candidate has no unassigned scope and no XL deliverables, but five scope-ledger rows have `ObjectiveIDs=TBD`. Proposed v0.2 resolves those and adds six medium deliverables.|
|Gate 7 — Publish|BLOCKED PENDING HUMAN|Human must approve or reject the v0.2 amendment before downstream execution.|

## Structural validation findings

|Check|Candidate v0.1 result|Disposition|
|---|---:|---|
|Scope items|49|Valid but incomplete against PRD review.|
|Packages|12|Valid; flat, domain-based, no nesting.|
|Deliverables|59|Valid shape; several PRD-derived deliverables missing.|
|Objective count|11|Valid shape; add `OBJ-012` for unit-safe deterministic data flow.|
|Unassigned scope items|0|Pass.|
|Scope items without deliverable mapping|0|Pass.|
|XL deliverables|0|Pass.|
|Scope-ledger rows with `ObjectiveIDs=TBD`|5|Revise: SOW-013, SOW-014, SOW-015, SOW-025, SOW-041.|

## Required amendments

### A. Scope amendments

Add six PRD-derived `IN` scope items:

|ScopeItemID|Package|Summary|Basis|
|---|---|---|---|
|SOW-050|PKG-02|Project persistence and deterministic round-trip serialization.|PRD FR-001; Appendix A.|
|SOW-051|PKG-03|Pipe section and mass-property calculations from user-entered dimensions/materials.|PRD FR-005.|
|SOW-052|PKG-05|Concentrated forces, concentrated moments, and distributed user loads.|PRD FR-007; load requirements.|
|SOW-053|PKG-04|Singular/ill-conditioned/nonconverged solver diagnostics.|PRD §11.8, §20, R1 exit criteria.|
|SOW-054|PKG-10|Headless CLI or structured I/O runner for early validation and automation.|PRD R0/R1 release strategy.|
|SOW-055|PKG-07|GUI background solve execution, progress, cancellation, and diagnostics presentation.|PRD §20–§21, R2 GUI expectations.|

### B. Objective amendment

Add:

|ObjectiveID|Statement|
|---|---|
|OBJ-012|Ensure unit-safe, deterministic, and reproducible model data flow across persistence, solving, rule evaluation, automation, and reporting.|

Also repair the existing scope ledger objective mappings:

|ScopeItemID|From|To|
|---|---|---|
|SOW-013|TBD|OBJ-003|
|SOW-014|TBD|OBJ-003, OBJ-005|
|SOW-015|TBD|OBJ-003|
|SOW-025|TBD|OBJ-012|
|SOW-041|TBD|OBJ-001, OBJ-012|

### C. Package-by-package deliverable amendments

|Package|Amendment|
|---|---|
|PKG-01|No deliverable change. Existing governance/IP/professional-boundary package remains valid.|
|PKG-02|Add `DEL-02-05 Project persistence and round-trip serialization` for PRD FR-001. Update `DEL-02-02` to support `OBJ-012`.|
|PKG-03|Add `DEL-03-08 Pipe section property and mass-property calculator` for PRD FR-005.|
|PKG-04|Add `DEL-04-06 Solver diagnostics and singularity detection` for singular/conditioning/nonconvergence reporting.|
|PKG-05|Add `DEL-05-05 Concentrated and distributed user load application` for PRD FR-007 concentrated loads.|
|PKG-06|No deliverable change. Rule-pack package remains valid.|
|PKG-07|Add `DEL-07-07 Solve execution UX: progress, cancellation, and diagnostics` for GUI responsiveness and reviewability.|
|PKG-08|No new deliverable. Update `DEL-08-02` to support `OBJ-012` because audit manifest/model hash participates in deterministic reproducibility.|
|PKG-09|No deliverable change. Validation package remains valid.|
|PKG-10|Add `DEL-10-05 Headless CLI and structured I/O analysis runner` for R0/R1 automation and non-GUI validation execution.|
|PKG-11|No deliverable change. Documentation/examples package remains valid.|
|PKG-12|No deliverable change. Security/privacy package remains valid.|

### D. Open issues to add

|OpenIssueID|Issue|
|---|---|
|OI-009|Dynamic analysis modules such as modal or response-spectrum analysis are PRD Could/post-MVP items and are not decomposed for current execution. Promote through scope change if desired.|
|OI-010|Private rule-pack encryption default requires human/security decision.|
|OI-011|Project persistence file format and versioning scheme remain TBD.|

### E. INIT.md process note

`docs/INIT.md` currently tells an agent to select a `DEL-XX-YY` and execute after reading the docs. That is premature while the decomposition is still a draft candidate. Amend the next-step language to say: run/confirm SOFTWARE_DECOMP first; after human Gate 7 acceptance, run PREPARATION; only after PREPARATION should Type 2 execution begin.

## Proposed v0.2 telemetry

|Metric|v0.1 Candidate|v0.2 Proposed|
|---|---:|---:|
|ScopeItemCount|49|55|
|PackageCount|12|12|
|DeliverableCount|59|65|
|ObjectiveCount|11|12|
|UnassignedScopeItems|0|0|
|ScopeItemsWithoutDeliverableMapping|0|0|
|UnmappedObjectives|0|0|
|ContextEnvelopeCounts|S=8, M=42, L=9, XL=0|S=8, M=48, L=9, XL=0|
|OpenIssues|8|11|

## Output artifacts

This review pass produced:

- `_Proposed_Amendment/SOFTWARE_DECOMP_v0_2_proposed.md`
- `_Proposed_Amendment/ScopeLedger_v0_2_proposed.csv`
- `_Proposed_Amendment/Deliverables_v0_2_proposed.csv`
- `_Proposed_Amendment/ContextBudgetQA_v0_2_proposed.csv`

## Human decision requested

Please choose one:

1. **Accept v0.2 amendment** as the current downstream SOFTWARE_DECOMP basis.
2. **Accept v0.1 as-is** despite the review findings.
3. **Revise the proposed amendment** by adding/removing specific scope items or deliverables.

Until you choose, the accepted downstream basis remains unconfirmed.
