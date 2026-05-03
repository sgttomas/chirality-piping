---
doc_id: DAG-002-AUDIT
doc_kind: coordination.dag_audit
status: generated_unapproved_proposal
created: 2026-05-03
decomposition_revision: "0.5"
---

# DAG-002 Proposal Audit

## Authority

`DAG-002` is an unapproved revision `0.5` proposal snapshot. It does not approve a graph, enable blocker computation, authorize lifecycle transitions, refresh deliverable-local dependency mirrors, or dispatch Type 2 work.

## Counts

| Fact | State |
|---|---|
| Deliverable nodes | 92 |
| Packages represented | 17 |
| Historical DAG-001 nodes carried forward | 73 |
| Revision 0.5 nodes added to proposal | 19 |
| Active edges carried forward as proposal rows | 615 |
| Candidate edges carried forward as non-gating proposal rows | 9 |
| Invalid endpoints | 0 |
| Self dependencies | 0 |
| Duplicate active directed edges | 0 |
| Bidirectional active pairs | 0 |
| Active SCCs | 0 |
| Active plus candidate SCCs | 4 |
| Topological waves from active proposal edges | 12 |

## Revision 0.5 Additions

The proposal node register now includes all 19 accepted revision `0.5` nodes absent from `DAG-001`: `DEL-07-08`, `DEL-08-06`, `DEL-13-01`, `DEL-13-02`, `DEL-13-03`, `DEL-13-04`, `DEL-14-01`, `DEL-14-02`, `DEL-14-03`, `DEL-14-04`, `DEL-14-05`, `DEL-15-01`, `DEL-15-02`, `DEL-15-03`, `DEL-15-04`, `DEL-16-01`, `DEL-16-02`, `DEL-16-03`, `DEL-16-04`.

No new active dependencies were inferred for those nodes in this alignment pass. The new-scope dependency questions remain in `DAG-002_CandidateQuestionWorklist.csv` for a later graph-authoring decision.

## Review Flags

- `DEL-01-04` and `DEL-02-01` retain committed historical evidence by ID, but require targeted revision `0.5` scope review before completeness reliance.
- Candidate rows remain non-gating and must not affect blocker queues, wave placement, schedule, staffing, priority, or dispatch readiness.
- `docs/_ScopeChange/chirality-app-docs/` remains quarantined reference material and created no DAG edges.

## Machine Summary

Detailed counts are recorded in `DAG_Audit.json`.

