# TP-MAC-08 Next Tranche Assessment

**Date:** 2026-05-10
**Posture:** Superseded by implemented TP-MAC-08 closeout
**Disposition:** Historical selection record. TP-MAC-08 was implemented from
the explicit sealed plan in
`plans/TP-MAC-08_CODE_NEUTRAL_LOAD_COMBINATION_PREVIEW_PLAN.md`.

## Context

TP-MAC-07 closed the first midspan station preview slice. This assessment then
selected TP-MAC-08 as the next governed target. A subsequent explicit sealed
plan authorized and completed the implementation.

The active product-preview baseline now includes endpoint force/moment
recovery, endpoint stress component recovery, uniform axial thermal preview
behavior, midspan force/moment/stress rows, explicit mechanics-basis user load
combinations, runtime execution, and schema-shaped persistence.

## Candidate Screen

| Candidate | Current status | Assessment |
|---|---|---|
| Pressure-to-frame load conversion | `not_implemented` gap-ledger entry | Defer. Pressure thrust and frame-load representation need a separate mechanics specification to avoid hidden assumptions. |
| Support stiffness and nonlinear support richness | `deferred` gap-ledger entry | Defer. This crosses support data modeling, solver behavior, diagnostics, and GUI inputs. |
| Load combinations | `implemented` for explicit mechanics-basis user combinations | Completed by TP-MAC-08. Code/rule combinations remain deferred. |
| Protected rule/code checks | `requires_private_inputs` gap-ledger entry | Defer. Public implementation must not bundle allowables, code factors, criteria, SIF/flexibility data, or private rule packs. |
| Arbitrary station sweeps and exact internal diagrams | Deferred by TP-MAC-07 | Defer. TP-MAC-07 intentionally scoped only deterministic midspan interpolation. |
| Temperature-dependent materials and broader thermal behavior | Deferred by TP-MAC-06 | Defer. Requires material library semantics and explicit data provenance beyond the current preview slice. |

## Selected Next Governed Tranche

**TP-MAC-08 Code-Neutral Load Combination Preview**

Implemented purpose:

Add the first product-preview load-combination slice by evaluating explicit
user-defined linear combinations over supported solved preview load cases, using
the existing `core/loads/load_case_algebra` mechanics boundary and no
code-specific public defaults.

Package/deliverable mapping:

- Primary: `PKG-05 / DEL-05-02` load-case algebra engine.
- Supporting: `PKG-07 / DEL-07-05` results viewer, `PKG-08 / DEL-08-04`
  result export metadata, `PKG-14 / DEL-14-02` analysis-run records, and
  `PKG-02 / DEL-02-05` persistence run-history refs if fixture hashes change.

Implemented boundaries:

- Use invented or explicitly cleared data only.
- Only explicit user-authored combination factors are allowed.
- Do not introduce code-specific combinations, public default factors,
  allowables, SIF/flexibility data, protected criteria, rule-pack checks, or
  compliance/professional claims.
- Do not implement a general expression language, rule evaluator, load-case
  algebra UI editor, final CLI syntax, durable save/open UX, or external
  execution.
- Do not change TP-MAC-07 midspan interpolation semantics, TP-MAC-06 thermal
  behavior, endpoint result IDs, runtime request shape, or persistence physical
  container status unless the future TP-MAC-08 plan scopes a narrow compatible
  schema extension.

Implemented acceptance themes:

- Preview input can carry explicit invented load-case combination records.
- Unsupported or invalid combination records produce structured diagnostics.
- Solved combination results are clearly labeled as explicit
  user-combination results, not protected code combinations.
- Result metadata and analysis-run hashes include combination basis refs.
- Desktop result display distinguishes primitive load-case preview results from
  explicit combination preview results.
- The mechanics gap ledger marks only the bounded explicit-combination preview
  slice implemented and keeps code/rule combinations deferred.

## Gate

This assessment is retained for audit traceability only. TP-MAC-08 is now
closed. No next governed tranche is selected here.
