---
doc_id: DEL-15-04-MEMORY
doc_kind: implementation.memory
status: draft
created: 2026-05-07
---

# DEL-15-04 Memory - External Prover Boundary Metadata

Implemented a narrow external-prover boundary metadata module in
`core/handoff/external_prover/`. The module builds deterministic
non-authoritative metadata records for invented external workflow names, tags,
notes, external references, attachments, handoff package references, target
mapping references, export workflow references, immutable model state
references, assumptions, warnings, and unsupported-target flags.

The module emits diagnostics for missing context links, embedded attachment
payloads, unsupported attachment kinds, proposed authority claims, prohibited
authority/lifecycle wording, software authority flags, and external execution
or commercial-result ingestion flags. Proposed authority claims are preserved
only as rejected boundary diagnostics, not as accepted statuses.

Added focused tests in `tests/test_external_prover_boundary_metadata.py` using
invented public metadata only. The implementation does not invoke external
solvers/provers, parse commercial formats, ingest commercial results, or
create professional reliance state.

Unresolved TBDs:

- Full Draft 2020-12 JSON Schema materialization remains outside this slice;
  repository tests currently use stdlib structural checks.
- Concrete external prover tools, target-specific parsers, lifecycle
  promotion, human acceptance records, certification/compliance decisions, and
  commercial result ingestion remain out of scope.
