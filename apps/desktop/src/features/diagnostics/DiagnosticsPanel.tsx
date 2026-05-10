import { AlertTriangle } from "lucide-react";
import type { DesignKnowledge, Diagnostic, MechanicsResult, PreviewModel } from "../../types";

export function DiagnosticsPanel({
  model,
  knowledge,
  result
}: {
  model: PreviewModel;
  knowledge: DesignKnowledge | null;
  result: MechanicsResult | null;
}) {
  const diagnostics: Diagnostic[] = [
    ...model.diagnostics,
    ...(knowledge?.diagnostics ?? []),
    ...(result?.diagnostics ?? [])
  ];
  return (
    <section className="panel diagnostics-panel" aria-label="Diagnostics">
      <div className="panel-title">
        <AlertTriangle size={16} />
        Diagnostics
      </div>
      <div className="diagnostic-list">
        {diagnostics.map((item, index) => (
          <div className={`diagnostic ${item.severity}`} key={`${item.code}-${index}`}>
            <strong>{item.code}</strong>
            <p>{item.message}</p>
          </div>
        ))}
      </div>
    </section>
  );
}
