---
doc_id: TP-MAC-02
doc_kind: implementation.plan
status: implemented
created: 2026-05-09
---

# TP-MAC-02 Physics-First Application Plan

This tranche shifts the product preview from fixture-only mechanics output to a
bounded linear-static physics loop over invented public example data.

## Scope

- Add `core/product_physics` as the product-preview mechanics adapter.
- Keep `core/product_preview` as the schema-adjacent fixture and validation
  layer.
- Use explicit invented mechanics inputs only: material moduli, pipe dimensions,
  support restraints, and primitive load cases.
- Return displacement, reaction, element-force-derived stress summaries, and
  diagnostics in a stable preview result envelope.
- Preserve the professional boundary: no compliance, certification, sealing,
  protected-table, allowable, or autonomous mutation claims.

## Verification

- `cargo test --manifest-path core/product_physics/Cargo.toml`
- Existing solver crate tests.
- Desktop build and tests.
- Product-preview Python tests.
- `git diff --check`
