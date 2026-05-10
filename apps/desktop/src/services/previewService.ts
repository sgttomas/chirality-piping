import { invoke } from "@tauri-apps/api/core";
import agentProposalFixture from "../../../../fixtures/product_preview/invented_agent_proposal.json";
import knowledgeFixture from "../../../../fixtures/product_preview/invented_design_knowledge.json";
import mechanicsFixture from "../../../../fixtures/product_preview/invented_mechanics_result.json";
import modelFixture from "../../../../fixtures/product_preview/invented_preview_model.json";
import type {
  AgentProposal,
  AnalysisRunEnvelope,
  DesignKnowledge,
  MechanicsResult,
  ObjectRef,
  PreviewModel,
  SelectedReviewTarget
} from "../types";

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

export async function buildAnalysisRunPreview(result: MechanicsResult): Promise<AnalysisRunEnvelope> {
  const runRef = ref("AnalysisRun", result.run_id);
  const resultEnvelopeRef = ref("ResultEnvelope", `result-envelope:${result.run_id}`);
  const resultRefs = await Promise.all(
    result.results
      .slice()
      .sort((left, right) => left.id.localeCompare(right.id))
      .map(async (item) => ({
        result_ref: ref("Result", item.id),
        result_family: resultFamily(item),
        hash_refs: [
          {
            algorithm: "sha256" as const,
            canonicalization: "JCS",
            payload_ref: ref("Result", item.id),
            payload_scope: "result_value",
            value: await sha256(canonicalJson(item))
          }
        ],
        privacy_classification: "invented_public_example"
      }))
  );
  const status = new Set([
    "HUMAN_REVIEW_REQUIRED",
    result.status.mechanics,
    result.status.rule_check
  ].filter(Boolean));
  const resultIds = result.results.map((item) => item.id).sort();
  const diagnosticIds = result.diagnostics.map((item) => item.id ?? "diagnostic:unknown").sort();

  return {
    schema_version: "0.1.0",
    deliverable_id: "DEL-14-02",
    package_id: "PKG-14",
    scope_item: "SOW-072",
    objectives: ["OBJ-016"],
    run_contract_status: {
      record_contract: "schema_first_analysis_run_records",
      model_state_binding: "schemas/model_state.schema.json",
      result_binding: "schemas/results.schema.yaml",
      physical_project_container: "TBD",
      external_validation_boundary: "reference_only_not_determined_by_software"
    },
    analysis_run: {
      run_id: result.run_id,
      run_name: `${result.run_id} preview mechanics run`,
      run_kind: "mechanics_solve",
      model_state_ref: ref("ModelState", `state:${result.model_ref}:preview`),
      result_refs: resultRefs,
      hashes: [
        {
          algorithm: "sha256",
          canonicalization: "JCS",
          payload_ref: runRef,
          payload_scope: "analysis_run_record",
          value: await sha256(
            canonicalJson({
              run_id: result.run_id,
              model_ref: result.model_ref,
              status: result.status,
              result_ids: resultIds,
              diagnostic_ids: diagnosticIds
            })
          )
        },
        {
          algorithm: "sha256",
          canonicalization: "JCS",
          payload_ref: resultEnvelopeRef,
          payload_scope: "result_envelope",
          value: await sha256(canonicalJson(result))
        }
      ],
      analysis_status: Array.from(status).sort(),
      reproducibility: {
        input_manifest_refs: [resultEnvelopeRef],
        determinism_notes: [
          "analysis run record was generated from an already computed invented preview mechanics result",
          "canonical hashes use stable JSON key ordering"
        ],
        unresolved_tbd: ["physical project container", "release-grade solver build provenance"]
      },
      immutability_policy: {
        run_record_is_read_only: true,
        mutation_policy: "changes_create_new_analysis_run",
        new_run_required_for_change: true,
        hash_invalidates_external_acceptance: true
      },
      professional_boundary: {
        human_review_required: true,
        software_makes_compliance_claim: false,
        software_makes_certification_claim: false,
        software_makes_sealing_claim: false,
        software_makes_approval_claim: false,
        software_makes_authentication_claim: false
      }
    }
  };
}

export async function loadSampleProposal(
  mechanicsResult?: MechanicsResult | null,
  selectedTarget?: SelectedReviewTarget | null
): Promise<AgentProposal> {
  const response = await invokeOrFixture<{ proposal: AgentProposal }>("sample_agent_proposal", {
    proposal: buildProposalFromMechanics(mechanicsResult ?? (mechanicsFixture as MechanicsResult), selectedTarget)
  });
  return response.proposal;
}

function ref(objectType: string, value: string): ObjectRef {
  return { object_type: objectType, ref: value };
}

function resultFamily(result: MechanicsResult["results"][number]): string {
  const kind = result.kind.toLowerCase();
  const id = result.id.toLowerCase();
  if (kind.includes("displacement") || id.includes("disp")) return "displacement";
  if (kind.includes("reaction") || id.includes("reaction")) return "reaction";
  if (kind.includes("force") || id.includes("force")) return "force";
  if (kind.includes("moment") || id.includes("moment")) return "moment";
  if (kind.includes("stress") || id.includes("stress")) return "stress";
  if (kind.includes("ratio") || id.includes("ratio")) return "ratio";
  return "TBD";
}

function canonicalJson(value: unknown): string {
  return JSON.stringify(sortJson(value));
}

function sortJson(value: unknown): unknown {
  if (Array.isArray(value)) {
    return value.map(sortJson);
  }
  if (value && typeof value === "object") {
    return Object.fromEntries(
      Object.entries(value)
        .sort(([left], [right]) => left.localeCompare(right))
        .map(([key, nested]) => [key, sortJson(nested)])
    );
  }
  return value;
}

async function sha256(payload: string): Promise<string> {
  if (globalThis.crypto?.subtle) {
    const bytes = new TextEncoder().encode(payload);
    const digest = await globalThis.crypto.subtle.digest("SHA-256", bytes);
    return Array.from(new Uint8Array(digest))
      .map((byte) => byte.toString(16).padStart(2, "0"))
      .join("");
  }
  let hash = 5381;
  for (let index = 0; index < payload.length; index += 1) {
    hash = (hash * 33) ^ payload.charCodeAt(index);
  }
  return `sha256-unavailable-${(hash >>> 0).toString(16).padStart(8, "0")}`;
}

function buildProposalFromMechanics(result: MechanicsResult, selectedTarget?: SelectedReviewTarget | null): AgentProposal {
  const forceResult =
    result.results.find((item) => item.id === "result:force:pipe-P-120:axial") ??
    result.results.find((item) => item.kind === "element_local_axial_force");
  const primaryDiagnostic =
    result.diagnostics.find((item) => item.severity === "warning" || item.severity === "blocking") ??
    result.diagnostics[0];
  const targetRef =
    selectedTarget?.id ??
    forceResult?.id ??
    primaryDiagnostic?.id ??
    primaryDiagnostic?.affected_refs?.[0] ??
    result.summary.max_displacement?.result_ref ??
    result.results[0]?.id ??
    "diagnostic:physics:context-unavailable";
  const targetKind = selectedTarget?.target_type.replaceAll("_", " ") ?? "computed mechanics";

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
          before: `No review note attached for the selected ${targetKind} context.`,
          after: `Attach review note referencing the current computed preview ${targetKind} context.`
        }
      ]
    },
    rationale: `Generated from current preview mechanics context; selected review reference is ${targetRef}. This narrative is review-only and does not mutate accepted model state.`,
    validation: {
      ...(agentProposalFixture as AgentProposal).validation,
      constraint_validation: "warning_computed_context_requires_human_review",
      diff_preview_status: "generated_from_computed_context"
    }
  };
}
