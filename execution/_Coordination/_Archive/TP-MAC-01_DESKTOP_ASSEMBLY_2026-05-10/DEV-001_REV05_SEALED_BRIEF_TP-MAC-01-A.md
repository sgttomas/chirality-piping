---
doc_id: DEV-001-REV05-SEALED-BRIEF-TP-MAC-01-A
doc_kind: coordination.sealed_product_assembly_brief
status: sealed_brief_prepared_dispatch_not_authorized
created: 2026-05-10
prepared_by: ORCHESTRATOR
tranche: TP-MAC-01
unit_id: TP-MAC-01-A
unit_name: Desktop app scaffold
primary_deliverables: DEL-10-04, DEL-07-06
worker_launch: not_authorized
implementation_dispatch: not_authorized
source_plan: execution/_Coordination/DEV-001_REV05_MACOS_DESKTOP_PRODUCT_ASSEMBLY_TRANCHE_PLAN.md
---

# Sealed Brief - TP-MAC-01-A Desktop App Scaffold

## Dispatch Boundary

This sealed implementation brief is prepared for later bounded worker launch
only. It does not authorize worker launch, Type 2 dispatch, lifecycle/evidence
promotion, blocker refresh, dependency refresh, `DAG-002` mutation, candidate
promotion, commit, push, release creation, signing, publishing, protected data,
private project data, or professional/code-compliance claims.

## Product Slice

Create the macOS-capable desktop application scaffold for the OpenPipeStress
technical preview. The scaffold must be a usable engineering workspace shell,
not a landing page, and must be ready for the other `TP-MAC-01` units to wire
3D rendering, model panels, diagnostics, solve status, and proposal UI into it.

## PKG/DEL Basis

| Package | Deliverable | Use in this brief |
|---|---|---|
| `PKG-10` | `DEL-10-04` Build, packaging, and CI/CD pipeline | Provider-neutral desktop build route and local dev/build discipline. |
| `PKG-07` | `DEL-07-06` Accessibility and usability baseline | Quiet engineering workspace layout, keyboard/focus/readability baseline, and no marketing screen. |

Applicable architecture basis: `AB-00-01`, `AB-00-02`, `AB-00-03`,
`AB-00-05`, `AB-00-06`, `AB-00-07`, `AB-00-08`.

## Allowed Write Scope

- `apps/desktop/`
- `apps/desktop/src-tauri/`
- root `package.json`, workspace/package-manager config, or Rust workspace
  config only if required to run the app scaffold
- root `.gitignore` only if needed for generated desktop build artifacts
- focused desktop scaffold tests/config under `apps/desktop/`

Do not edit `core/`, `schemas/`, `execution/_DAG/`, dependency mirrors,
implementation evidence, lifecycle state files, or release/signing/publishing
configuration outside local scaffold needs.

## Required Output

- A Tauri 2 + React/Vite desktop app scaffold under `apps/desktop`.
- Local development and build scripts sufficient for later review.
- A first-pass OpenPipeStress workspace shell with:
  - top navigation or command surface;
  - 3D viewport region placeholder;
  - model/navigation side panel region;
  - diagnostics/results/proposal panel regions;
  - explicit non-claim footer/status text that does not overstate engineering
    authority.
- No remote services, telemetry, secrets, private data paths, signing keys, or
  publishing configuration.

## Acceptance Criteria

- `apps/desktop` can install dependencies and start as a local dev target.
- The scaffold renders a nonblank workspace shell.
- UI layout is restrained, dense enough for engineering review, and does not
  present a marketing/landing page.
- Accessibility baseline includes keyboard-visible focus, readable contrast,
  stable layout dimensions, and no text overlap in normal desktop viewport.
- No artifact claims production readiness, professional approval, code
  compliance, certification, sealing, or release readiness.

## Required Verification

Expected commands after implementation, adjusted only if scaffold tooling
requires equivalent names:

```text
npm install
npm run dev --workspace apps/desktop
npm run build --workspace apps/desktop
npm test --workspace apps/desktop
git diff --check
```

If network dependency installation is required, request sandbox escalation
through normal command approval. Final review should include a screenshot or
Playwright/browser verification that the app shell is nonblank.

## Stop Conditions

Stop before adding product runtime behavior beyond the scaffold, protected or
private data, release/signing/publishing behavior, professional reliance
language, or edits outside the allowed write scope unless separately
authorized.
