# Dependencies: DEL-12-02 Private data redaction and export controls

## Coordination (human-owned)
- **Coordination Mode:** NOT_TRACKED
- **Notes:** No human-declared dependency list was provided for this setup run.

## Upstream (I need these before I can proceed) - human-owned declarations
- Dependencies coordinated externally by humans.

## Downstream (These need me) - human-owned declarations
- Dependencies coordinated externally by humans.

## Extracted Dependency Register
- **Status:** ACTIVE
- **Dependencies.csv:** present
- **Summary:** 8 active rows

| DependencyID | Class | Direction | TargetType | TargetName | Status |
|---|---|---|---|---|---|
| DEL-12-02-ANCHOR-001 | ANCHOR | UPSTREAM | WBS_NODE | PKG-12 Security Privacy and Private Data Handling | ACTIVE |
| DEL-12-02-ANCHOR-002 | ANCHOR | UPSTREAM | REQUIREMENT | SOW-040 | ACTIVE |
| DEL-12-02-ANCHOR-003 | ANCHOR | UPSTREAM | REQUIREMENT | OBJ-010 | ACTIVE |
| DEL-12-02-EXEC-001 | EXECUTION | UPSTREAM | DOCUMENT | _CONTEXT.md | ACTIVE |
| DEL-12-02-EXEC-002 | EXECUTION | UPSTREAM | DELIVERABLE | Diagnostics warning and result-envelope contract | ACTIVE |
| DEL-12-02-EXEC-003 | EXECUTION | UPSTREAM | DELIVERABLE | API boundary and adapter contract map | ACTIVE |
| DEL-12-02-EXEC-004 | EXECUTION | DOWNSTREAM | DELIVERABLE | Result export format | ACTIVE |
| DEL-12-02-EXEC-005 | EXECUTION | DOWNSTREAM | DELIVERABLE | Report protected-content linter | ACTIVE |

## Run Notes
- `SOURCE_DOCS`: AUTO
- `DOC_ROLE_MAP`: DEFAULT
- `ANCHOR_DOC`: AUTO
- `EXECUTION_DOC_ORDER`: AUTO
- `MODE`: UPDATE
- `STRICTNESS`: CONSERVATIVE
- `CONSUMER_CONTEXT`: NONE
- `DECOMPOSITION_PATH`: docs/_Decomposition/SOFTWARE_DECOMP.md
- `Warnings`: none
- `Scope discipline`: security-sensitive setup only; no product source, schema, real config, executable tests, protected data, private data, secrets, cloud behavior, or `ISSUED` artifacts created.

## Run History
- 2026-04-30T14:35:00-06:00 - mode=UPDATE strictness=CONSERVATIVE decomposition=docs/_Decomposition/SOFTWARE_DECOMP.md status=found warnings=none active=8

## Lifecycle Summary
- ACTIVE: 8
- RETIRED: 0
- Closure State: open

## Consumer Handoff Notes
- Downstream report/export deliverables should treat this register as setup evidence only and verify their own sealed context before consuming any proposed handoff.
