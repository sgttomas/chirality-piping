# Dependencies: DEL-09-03 Nonlinear support regression suite

## Coordination (human-owned)

- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for PREPARATION.

## Upstream (I need these before I can proceed) - human-owned declarations

- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations

- Dependencies coordinated externally by humans.

## Extracted Dependency Register

- **Status:** GENERATED
- **Dependencies.csv:** `Dependencies.csv`
- **Register Schema Version:** v3.1
- **ACTIVE rows:** 6
- **ANCHOR rows:** 3
- **EXECUTION rows:** 3
- **RETIRED rows:** 0

| DependencyID | Class | Direction | Type | Target | Status | Satisfaction |
|---|---|---|---|---|---|---|
| DEP-09-03-001 | ANCHOR | UPSTREAM | OTHER | DEL-09-03 Nonlinear support regression suite | ACTIVE | NOT_APPLICABLE |
| DEP-09-03-002 | ANCHOR | UPSTREAM | OTHER | SOW-026 Verification benchmarks and regression quality checks | ACTIVE | NOT_APPLICABLE |
| DEP-09-03-003 | ANCHOR | UPSTREAM | OTHER | OBJ-008 Rigorous verification validation regression testing and release gates | ACTIVE | NOT_APPLICABLE |
| DEP-09-03-004 | EXECUTION | UPSTREAM | CONSTRAINT | AB-00-06 Diagnostics warning and result-envelope contract | ACTIVE | PENDING |
| DEP-09-03-005 | EXECUTION | UPSTREAM | CONSTRAINT | AB-00-08 Layered software test and acceptance strategy | ACTIVE | PENDING |
| DEP-09-03-006 | EXECUTION | UPSTREAM | PREREQUISITE | DEL-04-04 Nonlinear support active-set solver | ACTIVE | PENDING |

## Run Notes

- Mode: UPDATE.
- Strictness: CONSERVATIVE.
- Consumer context: NONE.
- Source documents scanned: `_CONTEXT.md`, `Datasheet.md`, `Specification.md`, `Guidance.md`, `Procedure.md`, `_REFERENCES.md`, and decomposition/register references named in `_CONTEXT.md`.
- Decomposition path: `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4.
- Parent anchor check: PASS; one ACTIVE `IMPLEMENTS_NODE` anchor is present.
- Evidence check: PASS; every ACTIVE row includes `EvidenceFile` and `SourceRef`.
- Schema check: PASS with `tools/validation/validate_dependencies_schema.py`.
- Enum check: PASS for emitted enum values.
- [WARNING] ID_FORMAT_TOOL_LEGACY: `tools/validation/validate_id_format.sh` expects legacy `PKG-000` and `DEL-000-00` formats, while current `docs/TYPES.md` defines `PKG-XX` and `DEL-XX-YY`. No repo-level tool was edited in this deliverable.

## Run History

- 2026-04-30 10:51 MDT - TASK+dependency-extract generated v3.1 dependency register in UPDATE/CONSERVATIVE mode; ACTIVE rows: 6; warnings: ID_FORMAT_TOOL_LEGACY.

## Lifecycle Summary

| Status | Count |
|---|---:|
| ACTIVE | 6 |
| RETIRED | 0 |

| SatisfactionStatus | Count |
|---|---:|
| NOT_APPLICABLE | 3 |
| PENDING | 3 |

## Downstream Handoff Notes

- Future implementation work should treat `DEP-09-03-006` as a substantive prerequisite for creating meaningful nonlinear support regression cases.
- `DEP-09-03-004` and `DEP-09-03-005` are architecture-basis constraints for diagnostics/result envelopes and layered acceptance gates.
- This setup pass does not compute global dependency closure and does not move anything to `ISSUED`.
