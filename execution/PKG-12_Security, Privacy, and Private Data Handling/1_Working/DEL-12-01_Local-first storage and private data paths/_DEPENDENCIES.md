# Dependencies: DEL-12-01 Local-first storage and private data paths

## Coordination

- Coordination Mode: DECLARED
- Dependency extraction mode: `UPDATE`
- Strictness: `CONSERVATIVE`
- Consumer context: `RECONCILIATION`
- Decomposition path: `docs/_Decomposition/SOFTWARE_DECOMP.md`

## Declared Upstream

- Human-owned declarations were not supplied.

## Declared Downstream

- Human-owned declarations were not supplied.

## Extracted Dependency Register

| DependencyClass | Count | Notes |
|---|---:|---|
| ANCHOR | 3 | Parent package anchor plus SOW-029 and OBJ-010 traces. |
| EXECUTION | 3 | Local `_CONTEXT.md` prerequisite, AB-00-04 persistence constraint, and downstream DEL-12-04 handoff. |
| ACTIVE | 6 | All extracted rows are current for this setup run. |
| RETIRED | 0 | None. |

Canonical register: `Dependencies.csv` v3.1.

## Run Notes

- `SOURCE_DOCS=AUTO`: scanned recreated production documents plus deliverable-local metadata.
- `ANCHOR_DOC=AUTO`: anchors resolved from `_CONTEXT.md`, register rows, and decomposition entries.
- `EXECUTION_DOC_ORDER=AUTO`: execution cues extracted from `Specification.md`, `Guidance.md`, and `Procedure.md`.
- `[INFO] Parent anchor found`: one active `IMPLEMENTS_NODE` anchor is present.
- `[INFO] Physical project package/container remains TBD`: no dependency row resolves that implementation choice.
- `[INFO] No cloud dependency extracted`: cloud services remain out of MVP unless separately approved.
- `[INFO] No protected data dependency extracted`: no protected standards, proprietary values, or real private data were used.

## Run History

| Date | Agent/Skill | Mode | Result | Notes |
|---|---|---|---|---|
| 2026-04-30 | TASK + dependency-extract | UPDATE | SUCCESS | Rebuilt v3.1 register after recreating four documents and semantic artifacts. |

## Lifecycle Summary

| Field | Value |
|---|---|
| Register schema | v3.1 |
| Active rows | 6 |
| Retired rows | 0 |
| Parent anchors | 1 |
| Trace anchors | 2 |
| Execution edges | 3 |
| Current deliverable state | SEMANTIC_READY after setup validation |

## Downstream Handoff Notes

- DEL-12-04 should verify whether the symbolic private path classes here are sufficient before defining secret or private-library handling.
- DEL-12-02 should consume the public/private path distinction when redaction/export controls are implemented.
- Future storage implementation must not infer a physical container, OS-specific root, or cloud service from this setup artifact.
