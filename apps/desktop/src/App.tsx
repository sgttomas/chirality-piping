import { Activity, Database, FileWarning } from "lucide-react";
import type React from "react";
import { useEffect, useState } from "react";
import { AgentProposalPanel } from "./features/agent-proposals/AgentProposalPanel";
import { DiagnosticsPanel } from "./features/diagnostics/DiagnosticsPanel";
import { KnowledgePanel } from "./features/knowledge/KnowledgePanel";
import { defaultSelection } from "./features/model-workspace/modelView";
import { ModelTree } from "./features/model-tree/ModelTree";
import { PropertyInspector } from "./features/model-tree/PropertyInspector";
import { ResultsPanel } from "./features/results/ResultsPanel";
import { SolvePanel } from "./features/solve/SolvePanel";
import { PipeViewport } from "./features/viewport/PipeViewport";
import {
  loadDesignKnowledge,
  loadPreviewModel,
  loadSampleProposal,
  runPreviewMechanics
} from "./services/previewService";
import type { AgentProposal, DesignKnowledge, EntityRef, MechanicsResult, PreviewModel } from "./types";

export function App() {
  const [model, setModel] = useState<PreviewModel | null>(null);
  const [knowledge, setKnowledge] = useState<DesignKnowledge | null>(null);
  const [selection, setSelection] = useState<EntityRef | null>(null);
  const [result, setResult] = useState<MechanicsResult | null>(null);
  const [proposal, setProposal] = useState<AgentProposal | null>(null);
  const [running, setRunning] = useState(false);

  useEffect(() => {
    let active = true;
    Promise.all([loadPreviewModel(), loadDesignKnowledge()]).then(([loadedModel, loadedKnowledge]) => {
      if (!active) return;
      setModel(loadedModel);
      setKnowledge(loadedKnowledge);
      setSelection(defaultSelection(loadedModel));
    });
    return () => {
      active = false;
    };
  }, []);

  async function handleRun() {
    setRunning(true);
    const output = await runPreviewMechanics();
    setResult(output);
    setRunning(false);
  }

  async function handleProposal() {
    setProposal(await loadSampleProposal(result));
  }

  if (!model || !selection) {
    return <div className="loading-screen">Loading local OpenPipeStress preview fixture.</div>;
  }

  return (
    <main className="app-shell">
      <header className="topbar">
        <div>
          <h1>OpenPipeStress Technical Preview</h1>
          <p>{model.project.description}</p>
        </div>
        <div className="topbar-actions" aria-label="Preview status">
          <Badge icon={<Database size={16} />} label="Invented data" />
          <Badge icon={<Activity size={16} />} label="Local execution" />
          <Badge icon={<FileWarning size={16} />} label="Human review required" />
        </div>
      </header>

      <section className="workspace-grid">
        <aside className="left-rail">
          <ModelTree model={model} selection={selection} onSelect={setSelection} />
          <PropertyInspector model={model} selection={selection} />
        </aside>

        <section className="center-stage">
          <PipeViewport model={model} selection={selection} />
          <div className="bottom-panels">
            <KnowledgePanel knowledge={knowledge} result={result} />
            <DiagnosticsPanel model={model} knowledge={knowledge} result={result} />
          </div>
        </section>

        <aside className="right-rail">
          <SolvePanel model={model} result={result} running={running} onRun={handleRun} />
          <ResultsPanel result={result} />
          <AgentProposalPanel proposal={proposal} mechanicsReady={Boolean(result)} onLoad={handleProposal} />
        </aside>
      </section>

      <footer className="app-footer">
        Technical preview only: no production-readiness, release-readiness, certification, sealing, code-compliance; no licensed engineering reliance claim.
      </footer>
    </main>
  );
}

function Badge({ icon, label }: { icon: React.ReactNode; label: string }) {
  return (
    <span className="badge">
      {icon}
      {label}
    </span>
  );
}
