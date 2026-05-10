import { CheckSquare, Wand2 } from "lucide-react";
import type { AgentProposal } from "../../types";

export function AgentProposalPanel({
  proposal,
  mechanicsReady,
  onLoad
}: {
  proposal: AgentProposal | null;
  mechanicsReady: boolean;
  onLoad: () => void;
}) {
  return (
    <section className="panel agent-panel" aria-label="Agentic proposal">
      <div className="panel-title">Proposal</div>
      <button className="secondary-action" onClick={onLoad} disabled={!mechanicsReady} type="button">
        <Wand2 size={16} />
        Generate review proposal
      </button>
      {proposal ? (
        <div className="proposal-body">
          <h2>{proposal.proposal_id}</h2>
          <p>{proposal.rationale}</p>
          <div className="status-pill">
            <span>Validation</span>
            <strong>{proposal.validation.diff_preview_status}</strong>
          </div>
          <div className="diff-list">
            {proposal.operation.changes.map((change) => (
              <article key={change.change_id}>
                <strong>{change.target_ref}</strong>
                <small>{change.change_kind}</small>
                <p>{change.before}</p>
                <p>{change.after}</p>
              </article>
            ))}
          </div>
          <button className="review-action" disabled type="button" title="Review-only until accepted mutation is implemented">
            <CheckSquare size={16} />
            Accept disabled
          </button>
        </div>
      ) : (
        <p className="muted">Local proposal examples are deterministic and review-only.</p>
      )}
    </section>
  );
}
