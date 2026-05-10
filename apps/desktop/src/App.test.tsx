import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { App } from "./App";

describe("OpenPipeStress desktop preview", () => {
  it("renders the engineering workspace from invented local fixtures", async () => {
    render(<App />);

    expect(await screen.findByText("OpenPipeStress Technical Preview")).toBeInTheDocument();
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

    const results = await screen.findByLabelText("Results");
    expect(within(results).getByText("result:disp:node-N-140")).toBeInTheDocument();
    expect(within(results).getByText(/26.120937 mm/i)).toBeInTheDocument();

    const knowledge = await screen.findByLabelText("Design knowledge");
    expect(within(knowledge).getByText(/HIGH DISPLACEMENT REVIEW/i)).toBeInTheDocument();
    expect(within(knowledge).getByText(/result:disp:node-N-140 is 26.120937 mm/i)).toBeInTheDocument();

    fireEvent.click(screen.getByRole("button", { name: /Generate review proposal/i }));
    const proposal = await screen.findByLabelText("Agentic proposal");
    expect(within(proposal).getByText("proposal:physics-diagnostic-review")).toBeInTheDocument();
    expect(within(proposal).getAllByText(/diagnostic:physics:pressure-not-applied/i).length).toBeGreaterThan(0);
    expect(within(proposal).getByRole("button", { name: /Accept disabled/i })).toBeDisabled();
  });
});
