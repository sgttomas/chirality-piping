import { FileText } from "lucide-react";
import type { AgentProposal, MechanicsResult, PreviewModel } from "../../types";

export function ReportPanel({
  model,
  result,
  proposal
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  proposal: AgentProposal | null;
}) {
  const resultRefs = result ? selectedResultRefs(result) : [];
  const diagnostics = result?.diagnostics ?? [];
  return (
    <section className="panel report-panel" aria-label="Report packet" data-testid="report-panel">
      <div className="panel-title">
        <FileText size={16} />
        Report Packet
      </div>
      {result ? (
        <div className="report-list" data-testid="report-packet-body">
          <ReportLine label="Model" value={result.model_ref} />
          <ReportLine label="Mechanics" value={result.status.mechanics.replaceAll("_", " ")} />
          <ReportLine label="Rule check" value={result.status.rule_check.replaceAll("_", " ")} />
          <ReportLine label="Professional acceptance" value={result.status.professional_acceptance.replaceAll("_", " ")} />
          <ReportLine label="Selected result refs" value={resultRefs.join(", ")} testId="report-selected-result-refs" />
          <ReportLine label="Diagnostics" value={`${diagnostics.length} computed finding${diagnostics.length === 1 ? "" : "s"}`} />
          <ReportLine label="Proposal" value={proposal?.proposal_id ?? "not generated"} />
          <ReportLine label="Boundary" value="technical preview only; human review required; no compliance or professional approval claim" />
        </div>
      ) : (
        <p className="muted">
          Run the bounded preview mechanics path to assemble a report packet from computed result and diagnostic IDs.
        </p>
      )}
      <small className="report-note">
        Uses invented or cleared preview data for {model.project.id}; private rule criteria and professional acceptance are not bundled.
      </small>
    </section>
  );
}

function ReportLine({ label, value, testId }: { label: string; value: string; testId?: string }) {
  return (
    <div className="report-line" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function selectedResultRefs(result: MechanicsResult): string[] {
  return [
    result.summary.max_displacement?.result_ref,
    result.summary.max_open_formula_stress?.result_ref,
    result.results.find((item) => item.id === "result:force:pipe-P-120:axial")?.id
  ].filter((value): value is string => Boolean(value));
}
