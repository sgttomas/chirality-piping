---
doc_id: DEL-12-02-MEMORY
doc_kind: deliverable.memory
status: draft
created: 2026-05-03
deliverable_id: DEL-12-02
package_id: PKG-12
---

# MEMORY - DEL-12-02

Implemented revision 0.5 Tranche A slice for private data redaction and export
controls.

Changed artifacts:

- `schemas/redaction_export_controls.schema.yaml`
- `core/security/redaction/__init__.py`
- `core/security/redaction/controls.py`
- `tests/security/test_redaction_export_controls.py`
- `docs/security/redaction_export_controls.md`

Implementation notes:

- Classification is based on explicit metadata only.
- Public/shared contexts redact private project, material, component,
  rule-pack, owner-standard, company design-basis, path, and secret-like data.
- Missing provenance, unknown redistribution status, suspected protected
  content, and professional-boundary metadata produce explicit findings.
- Local/private exports retain private values only when explicit local/private
  intent is supplied, and still emit warnings.
- The redaction engine operates on copied export/report representations and
  does not mutate source project data.
- No cloud service behavior, secret storage, destructive quarantine movement,
  non-invented private payloads, protected standards content, or professional
  approval behavior was introduced.
- Tests use invented fixtures only.

Verification notes are intentionally kept in final worker response rather than
coordination registers or lifecycle state.
