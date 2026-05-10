import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { App } from "./App";

describe("OpenPipeStress desktop preview", () => {
  it("renders the engineering workspace from invented local fixtures", async () => {
    render(<App />);

    expect(await screen.findByText("OpenPipeStress Technical Preview")).toBeInTheDocument();
    expect(await screen.findByTestId("desktop-preview-shell")).toBeInTheDocument();
    expect(await screen.findByTestId("solve-panel")).toBeInTheDocument();
    expect(await screen.findByLabelText("Model tree")).toBeInTheDocument();
    expect(await screen.findByLabelText("Three.js pipe centerline viewport")).toBeInTheDocument();
    expect(screen.getByText("Human review required")).toBeInTheDocument();
  });

  it("does not claim professional or release acceptance", async () => {
    render(<App />);

    const footer = await screen.findByText(/Technical preview only/i);
    expect(footer.textContent).toContain("no production-readiness");
    expect(footer.textContent).toContain("no licensed engineering reliance claim");
  });

  it("shows computed mechanics diagnostics in results, knowledge, and review-only proposal context", async () => {
    render(<App />);

    const runButton = await screen.findByRole("button", { name: /Run mechanics preview/i });
    fireEvent.click(runButton);

    expect(await screen.findByTestId("result-group-displacement")).toBeInTheDocument();
    const results = await screen.findByLabelText("Results");
    expect(within(results).getByTestId("result-group-displacement")).toBeInTheDocument();
    expect(within(results).getByTestId("result-group-force")).toBeInTheDocument();
    expect(within(results).getByTestId("result-group-moment")).toBeInTheDocument();
    expect(within(results).getByTestId("result-row-result:force:pipe-P-120:axial")).toBeInTheDocument();
    expect(within(results).getByText("result:disp:node-N-140")).toBeInTheDocument();
    expect(within(results).getByText("result:force:pipe-P-120:axial")).toBeInTheDocument();
    expect(within(results).getByText(/26.120937 mm/i)).toBeInTheDocument();

    fireEvent.click(within(results).getByTestId("result-row-result:force:pipe-P-120:axial"));
    const detail = within(results).getByTestId("result-detail-panel");
    expect(within(detail).getByTestId("selected-result-id").textContent).toContain("result:force:pipe-P-120:axial");
    expect(within(detail).getByTestId("selected-result-component").textContent).toContain("axial_force");
    expect(within(detail).getByTestId("selected-result-coordinate-system").textContent).toContain("element_local");
    expect(within(detail).getByTestId("selected-result-location").textContent).toContain("end_i");
    expect(within(detail).getByTestId("selected-result-entity-ref").textContent).toContain("pipe:P-120");
    expect(within(detail).getByTestId("selected-result-recovery-basis").textContent).toContain("recovered_from_local_element_stiffness");
    expect(within(detail).getByTestId("selected-result-sign-convention").textContent).toContain("positive value follows");
    expect(await screen.findByRole("heading", { name: "Rack span" })).toBeInTheDocument();

    const gapLedger = within(results).getByTestId("mechanics-gap-ledger");
    expect(within(gapLedger).getByTestId("gap:endpoint-j-recovery").textContent).toContain("deferred");
    expect(within(gapLedger).getByTestId("gap:endpoint-j-recovery").textContent).not.toContain("compliance failure");

    const knowledge = await screen.findByLabelText("Design knowledge");
    expect(within(knowledge).getByText(/HIGH DISPLACEMENT REVIEW/i)).toBeInTheDocument();
    expect(within(knowledge).getByText(/result:disp:node-N-140 is 26.120937 mm/i)).toBeInTheDocument();
    expect(within(knowledge).getByText(/result:force:pipe-P-120:axial is/i)).toBeInTheDocument();

    const report = await screen.findByLabelText("Report packet");
    expect(within(report).getByTestId("report-packet-body")).toBeInTheDocument();
    expect(within(report).getByTestId("report-selected-result-refs").textContent).toContain("result:disp:node-N-140");
    expect(within(report).getByTestId("report-selected-result-refs").textContent).toContain("result:force:pipe-P-120:axial");
    expect(within(report).getByTestId("report-analysis-run").textContent).toContain("DEL-14-02");
    expect(within(report).getByTestId("report-analysis-run").textContent).toContain("run:preview-linear-static-001");
    expect(within(report).getByText(/result value hashes/i)).toBeInTheDocument();
    expect(within(report).getByText(/result_envelope/i)).toBeInTheDocument();
    expect(within(report).getByText(/no compliance or professional approval claim/i)).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: /Generate review proposal/i }));
    const proposal = await screen.findByLabelText("Agentic proposal");
    expect(within(proposal).getByText("proposal:physics-diagnostic-review")).toBeInTheDocument();
    expect(within(proposal).getByTestId("selected-review-target").textContent).toContain("result: result:force:pipe-P-120:axial");
    expect(within(proposal).getAllByText(/result:force:pipe-P-120:axial/i).length).toBeGreaterThan(0);
    expect(within(proposal).getByText(/review-only and does not mutate accepted model state/i)).toBeInTheDocument();
    expect(within(proposal).getByRole("button", { name: /Accept disabled/i })).toBeDisabled();

    expect(within(report).getByText("proposal:physics-diagnostic-review")).toBeInTheDocument();
  });

  it("links selected diagnostics to affected result and model context", async () => {
    render(<App />);

    fireEvent.click(await screen.findByRole("button", { name: /Run mechanics preview/i }));
    expect(await screen.findByTestId("diagnostic-HIGH_DISPLACEMENT_REVIEW")).toBeInTheDocument();

    const diagnostics = await screen.findByLabelText("Diagnostics");
    fireEvent.click(within(diagnostics).getByTestId("diagnostic-HIGH_DISPLACEMENT_REVIEW"));

    const diagnosticDetail = within(diagnostics).getByTestId("diagnostic-detail-panel");
    expect(within(diagnosticDetail).getByTestId("selected-diagnostic-id").textContent).toContain(
      "diagnostic:physics:high-displacement-review"
    );
    expect(within(diagnosticDetail).getByTestId("selected-diagnostic-affected-refs").textContent).toContain(
      "result:disp:node-N-140"
    );
    expect(within(diagnosticDetail).getByTestId("selected-diagnostic-affected-refs").textContent).toContain("node:N-140");
    expect(within(diagnosticDetail).getByTestId("selected-diagnostic-linked-results").textContent).toContain(
      "result:disp:node-N-140"
    );
    expect(within(diagnosticDetail).getByTestId("selected-diagnostic-explanation").textContent).toContain(
      "review threshold"
    );
    expect(await screen.findByRole("heading", { name: "Terminal tie-in" })).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: /Generate review proposal/i }));
    const proposal = await screen.findByLabelText("Agentic proposal");
    expect(within(proposal).getByTestId("selected-review-target").textContent).toContain(
      "diagnostic: diagnostic:physics:high-displacement-review"
    );
    expect(within(proposal).getAllByText(/diagnostic:physics:high-displacement-review/i).length).toBeGreaterThan(0);
    expect(within(proposal).getByRole("button", { name: /Accept disabled/i })).toBeDisabled();
  });
});
