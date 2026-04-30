# Dependencies: DEL-12-03 Telemetry off-by-default design

## Coordination (human-owned)

- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for this run.

## Upstream (I need these before I can proceed) - human-owned declarations

- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations

- Dependencies coordinated externally by humans.

## Extracted Dependency Register

- **Status:** ACTIVE
- **Dependencies.csv:** present
- **Register schema:** v3.1
- **Summary:** 2 anchor rows, 7 execution rows, 9 active rows, 0 retired rows

| DependencyClass | Active rows | Notes |
|---|---:|---|
| ANCHOR | 2 | SOW-037 implementation anchor and OBJ-010 objective trace. |
| EXECUTION | 7 | Explicit upstream constraints from contract, data-boundary, directive, architecture-basis, and open-issue sources. |

## Run Notes

- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer context:** NONE
- **Decomposition path:** `docs/_Decomposition/SOFTWARE_DECOMP.md`
- **Source documents scanned:** `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_CONTEXT.md`, `_REFERENCES.md`, `docs/CONTRACT.md`, `docs/DIRECTIVE.md`, `docs/IP_AND_DATA_BOUNDARY.md`, `docs/_Decomposition/SOFTWARE_DECOMP.md`
- **Anchor validation:** SOW-037 and OBJ-010 found in the accepted decomposition/register context.
- **Warnings:** None. One active parent anchor is present.
- **Scope note:** Rows capture information-flow and explicit constraints only; no repo-level product artifacts were edited.

## Run History

- 2026-04-30 - setup refresh completed with dependency-extract; schema v3.1 validated; 9 ACTIVE rows.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 9 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| PENDING | 9 |

## Downstream Handoff Notes

- Telemetry remains a no-op/default-off candidate in MVP.
- Any future telemetry implementation needs explicit human/security approval, an approved event allowlist, config-default tests, no-network-before-opt-in tests, and payload privacy tests.
- Current setup output is `SEMANTIC_READY`, not `ISSUED`.
