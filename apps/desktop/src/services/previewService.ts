import { invoke } from "@tauri-apps/api/core";
import agentProposalFixture from "../../../../fixtures/product_preview/invented_agent_proposal.json";
import knowledgeFixture from "../../../../fixtures/product_preview/invented_design_knowledge.json";
import mechanicsFixture from "../../../../fixtures/product_preview/invented_mechanics_result.json";
import modelFixture from "../../../../fixtures/product_preview/invented_preview_model.json";
import type { AgentProposal, DesignKnowledge, MechanicsResult, PreviewModel } from "../types";

async function invokeOrFixture<T>(command: string, fixture: T): Promise<T> {
  if (typeof window === "undefined" || !("__TAURI_INTERNALS__" in window)) {
    return fixture;
  }
  try {
    return await invoke<T>(command);
  } catch {
    return fixture;
  }
}

export async function loadPreviewModel(): Promise<PreviewModel> {
  return invokeOrFixture("load_preview_model", modelFixture as PreviewModel);
}

export async function loadDesignKnowledge(): Promise<DesignKnowledge> {
  return invokeOrFixture("load_design_knowledge", knowledgeFixture as DesignKnowledge);
}

export async function runPreviewMechanics(): Promise<MechanicsResult> {
  return invokeOrFixture("run_preview_mechanics", mechanicsFixture as MechanicsResult);
}

export async function loadSampleProposal(mechanicsResult?: MechanicsResult | null): Promise<AgentProposal> {
  const response = await invokeOrFixture<{ proposal: AgentProposal }>("sample_agent_proposal", {
    proposal: buildProposalFromMechanics(mechanicsResult ?? (mechanicsFixture as MechanicsResult))
  });
  return response.proposal;
}

function buildProposalFromMechanics(result: MechanicsResult): AgentProposal {
  const primary =
    result.diagnostics.find((item) => item.severity === "warning" || item.severity === "blocking") ??
    result.diagnostics[0];
  const targetRef =
    primary?.id ??
    primary?.affected_refs?.[0] ??
    result.summary.max_displacement?.result_ref ??
    result.results[0]?.id ??
    "diagnostic:physics:context-unavailable";

  return {
    ...(agentProposalFixture as AgentProposal),
    proposal_id: "proposal:physics-diagnostic-review",
    prompt: "Review current computed mechanics diagnostics and suggest a non-mutating follow-up.",
    operation: {
      ...(agentProposalFixture as AgentProposal).operation,
      operation_id: "op:review-computed-diagnostic",
      operation_kind: "attach_design_knowledge",
      affected_entity_ids: [targetRef],
      changes: [
        {
          change_id: "change:add-review-note",
          change_kind: "attach_design_knowledge",
          target_ref: targetRef,
          before: "No computed mechanics review note attached.",
          after: "Attach review note referencing the current computed preview diagnostic/result context."
        }
      ]
    },
    rationale: `Generated from current preview mechanics context; primary reference is ${targetRef}.`,
    validation: {
      ...(agentProposalFixture as AgentProposal).validation,
      constraint_validation: "warning_computed_context_requires_human_review",
      diff_preview_status: "generated_from_computed_context"
    }
  };
}
