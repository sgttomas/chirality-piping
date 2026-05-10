# Desktop Physics Workflow Smoke

This smoke check covers the TP-MAC-02 desktop path over invented local preview
data. It is intentionally limited to workflow wiring and product-boundary
signals; it is not a validation, certification, compliance, or professional
approval check.

## Start

```bash
npm run dev --workspace apps/desktop
```

Open the Vite URL printed by the command, normally
`http://127.0.0.1:5173/`.

## Required Checks

1. Initial load shows `data-testid="desktop-preview-shell"` and the header
   `OpenPipeStress Technical Preview`.
2. `data-testid="solve-panel"` shows initial status values from the fixture,
   including `data-testid="status-mechanics"` with
   `ready for preview diagnostics`.
3. Click `data-testid="run-mechanics-preview"`.
4. Results render `data-testid="results-panel"` with grouped sections:
   `result-group-displacement`, `result-group-reaction`,
   `result-group-force`, `result-group-moment`, and `result-group-stress`.
5. Results include `data-testid="result-row-result:force:pipe-P-120:axial"`
   and `data-testid="result-row-result:force:pipe-P-120:axial:end-j"`.
6. Click `data-testid="result-row-result:force:pipe-P-120:axial"` and confirm
   `data-testid="result-detail-panel"` shows `axial_force`,
   `element_local`, `end_i`, `recovered_from_local_element_stiffness`,
   the sign convention, `pipe:P-120`, `DEL-14-02`, and
   `run:preview-linear-static-001`.
7. Confirm `data-testid="endpoint-pair-table"` shows both
   `result:force:pipe-P-120:axial` and
   `result:force:pipe-P-120:axial:end-j`.
8. Click `data-testid="result-row-result:force:pipe-P-120:axial:end-j"` and
   confirm the detail panel shows `end_j` and the j-end sign convention.
9. Results include `data-testid="result-row-result:stress:pipe-P-120"` and
   `data-testid="result-row-result:stress:pipe-P-120:end-j:torsional-shear"`.
10. Click
    `data-testid="result-row-result:stress:pipe-P-120:end-j:torsional-shear"`
    and confirm `data-testid="result-detail-panel"` shows
    `torsional_shear_stress`, `element_local`, `end_j`,
    `recovered_from_open_mechanics_stress_components`, `pipe:P-120`, and the
    paired `result:stress:pipe-P-120:end-i:torsional-shear`.
11. Confirm the model context selects `pipe:P-120` and the property inspector
   (`aria-label="Property inspector"`) shows `Rack span`.
12. Click `data-testid="result-row-result:force:pipe-P-120:midspan:axial"`
    and confirm `data-testid="result-detail-panel"` shows `midspan`,
    `interpolated_from_endpoint_resultants`, and no endpoint-pair table.
13. Results include
    `data-testid="result-row-result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial"`.
    Click it and confirm `data-testid="result-detail-panel"` shows
    `explicit_user_linear_combination`, `combination:combination:C-OPER-ALT`,
    source result refs for `result:force:pipe-P-120:axial` and
    `result:loadcase:load-L-200:force:pipe-P-120:axial`, and no compliance
    or professional approval claim.
14. Confirm `data-testid="mechanics-gap-ledger"` includes
   `data-testid="gap:endpoint-j-recovery"` with status `implemented`; this is
   not a compliance failure.
15. Confirm `data-testid="mechanics-gap-ledger"` includes
    `data-testid="gap:endpoint-stress-components"` with status `implemented`;
    this is not a compliance failure.
16. Confirm `data-testid="mechanics-gap-ledger"` includes
    `data-testid="gap:station-recovery"` with status `implemented`, with
    arbitrary station sweeps and exact internal diagrams deferred.
17. Confirm `data-testid="mechanics-gap-ledger"` includes
    `data-testid="gap:thermal-behavior"` with status `implemented`, with
    broader thermal combinations and temperature-dependent properties deferred.
18. Confirm `data-testid="mechanics-gap-ledger"` includes
    `data-testid="gap:load-combinations"` with status `implemented`, with
    code/rule combinations still deferred.
19. Diagnostics render `data-testid="diagnostics-panel"` and include
   `diagnostic-HIGH_DISPLACEMENT_REVIEW` and
   `diagnostic-RULE_CHECK_INPUTS_MISSING`.
20. Click `data-testid="diagnostic-HIGH_DISPLACEMENT_REVIEW"` and confirm
   `data-testid="diagnostic-detail-panel"` shows
   `diagnostic:physics:high-displacement-review`,
   `result:disp:node-N-140`, `node:N-140`, linked result value context, and the
   review-only professional boundary.
21. Confirm the model context selects `node:N-140` and the property inspector
   (`aria-label="Property inspector"`) shows `Terminal tie-in`.
22. Knowledge renders `data-testid="knowledge-panel"` and includes computed
   context for `result:disp:node-N-140` and
   `result:force:pipe-P-120:axial`.
23. Report packet renders `data-testid="report-panel"` and
   `data-testid="report-packet-body"` with selected result refs including
   `result:disp:node-N-140`, `result:force:pipe-P-120:axial`, and
   `result:force:pipe-P-120:axial:end-j`,
   `result:force:pipe-P-120:midspan:axial`, and
   `result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial`, and
   `result:stress:pipe-P-120:end-j:torsional-shear`.
24. Report packet includes `data-testid="report-analysis-run"` with `DEL-14-02`
   and `run:preview-linear-static-001`; `data-testid="report-packet-body"`
   includes the result-value hash and `result_envelope` audit context.
25. Report packet includes `data-testid="report-load-basis-refs"` with
    `load:L-100`, `load:L-200`, and `combination:C-OPER-ALT`.
26. Click `data-testid="generate-review-proposal"`.
27. Proposal renders `data-testid="proposal-body"` and references the selected
   review target `result:stress:pipe-P-120:end-j:torsional-shear` if the
   endpoint stress row remains selected.
28. After selecting `diagnostic-HIGH_DISPLACEMENT_REVIEW`, generating a proposal
   references `diagnostic:physics:high-displacement-review` and uses diagnostic
   review wording.
29. `data-testid="accept-proposal-disabled"` is disabled; the proposal must
    remain review-only and must not mutate accepted model state.
30. Footer retains the technical-preview boundary text and does not claim
    production readiness, release readiness, certification, sealing, code
    compliance, or licensed engineering reliance.

## Current Status

The selectors above are present in the React UI and covered by Vitest where
practical. A live browser run passed on 2026-05-10 using the in-app browser at
`http://127.0.0.1:5173/`.

TP-MAC-03 result-interpretation smoke also passed on 2026-05-10. The run
confirmed axial force result selection, detail metadata display, `pipe:P-120`
model/viewport context selection, selected-result review proposal text,
disabled accept control, and endpoint-j recovery listed as a deferred mechanics
gap.

TP-MAC-03 diagnostic-interpretation smoke also passed on 2026-05-10. The run
confirmed `HIGH_DISPLACEMENT_REVIEW` selection, affected result/model refs,
linked result value context, `node:N-140` model/viewport selection, selected
diagnostic review proposal text, and disabled accept control.

TP-MAC-04 endpoint-result smoke passed on 2026-05-10. The run confirmed
end-j axial result selection, end-j metadata and j-end sign convention, endpoint
pair display, endpoint-j recovery marked implemented in the mechanics gap
ledger, report-packet inclusion of the end-j axial result ref, selected-result
review proposal text, and disabled accept control.

TP-MAC-05 endpoint-stress smoke passed on 2026-05-10. The run confirmed
end-j torsional shear stress result selection, stress metadata and endpoint
pair display, endpoint stress components marked implemented in the mechanics
gap ledger, report-packet inclusion of the end-j torsional shear stress ref,
selected-result review proposal text, and disabled accept control.

TP-RUN-01 preview-runtime smoke passed on 2026-05-10. The run confirmed the
React-loaded preview model is passed into the desktop runtime solve path while
the fixture-backed browser path remains intact, endpoint force and endpoint
stress result interpretation still works, report-packet audit context remains
visible, selected diagnostic proposals remain review-only, and the disabled
accept control does not mutate accepted model state.

TP-MAC-08 load-combination smoke passed on 2026-05-10. The run confirmed the
explicit mechanics-basis combination row renders, the detail panel shows
`explicit_user_linear_combination`, `combination:C-OPER-ALT`, and source result
refs for both load cases, the mechanics gap ledger marks explicit load
combinations implemented, and the report packet load-basis refs include
`load:L-100`, `load:L-200`, and `combination:C-OPER-ALT`.
