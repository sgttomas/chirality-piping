# Dependencies: DEL-01-01 Project governance baseline

## Coordination (human-owned)
- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for PREPARATION.

## Upstream (I need these before I can proceed) - human-owned declarations
- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations
- Dependencies coordinated externally by humans.

## Extracted Dependency Register (populated by TASK+dependency-extract)
- **Status:** UPDATED
- **Dependencies.csv:** `Dependencies.csv`
- **Summary:** 10 ACTIVE rows: 5 ANCHOR, 5 EXECUTION.

## Run Notes and History (populated by TASK+dependency-extract)
- **Run Date:** 2026-04-30
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Scope:** DEL-01-01
- **Run Root:** `/Users/ryan/ai-env/projects/chirality-piping/execution`
- **Decomposition Path:** `/Users/ryan/ai-env/projects/chirality-piping/docs/_Decomposition/SOFTWARE_DECOMP.md`
- **Source Docs:** AUTO; scanned `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, and `_REFERENCES.md`.
- **Anchor Doc:** AUTO -> `_CONTEXT.md` plus register/decomposition evidence.
- **Execution Docs:** AUTO -> `Specification.md`, `Guidance.md`, `Procedure.md`, `Datasheet.md`.
- **Warnings:** parent anchor count is 1. `validate_id_format.sh` rejected `PKG-01` and `DEL-01-01` because the helper expects legacy three-digit IDs; stable software IDs were preserved per `docs/TYPES.md` and the registers.

## Run History
- 2026-04-30 - TASK+dependency-extract MODE=UPDATE STRICTNESS=CONSERVATIVE; wrote `Dependencies.csv` v3.1; ACTIVE counts ANCHOR=5 EXECUTION=5; warnings=legacy ID validator mismatch for two-digit software IDs.

## Lifecycle Summary (populated by TASK+dependency-extract)
- **ACTIVE:** 10
- **RETIRED:** 0
- **Closure Status:** NOT_APPLICABLE=5; PENDING=5

## Consumer Handoff Notes
- `CONSUMER_CONTEXT=NONE`; no downstream-specific estimating, aggregation, or reconciliation hints were requested.
