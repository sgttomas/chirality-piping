import type {
  AnalysisRunEnvelope,
  DesignKnowledge,
  Diagnostic,
  DiagnosticInterpretation,
  EntityRef,
  KnowledgeRecord,
  MechanicsGap,
  MechanicsResult,
  PreviewModel,
  ResultInterpretation
} from "../../types";
import { physicsRecords } from "../knowledge/KnowledgePanel";

export function buildResultInterpretation({
  result,
  resultId,
  knowledge,
  analysisRun
}: {
  result: MechanicsResult;
  resultId: string | null;
  knowledge: DesignKnowledge | null;
  analysisRun: AnalysisRunEnvelope | null;
}): ResultInterpretation | null {
  const item = result.results.find((candidate) => candidate.id === resultId);
  if (!item) return null;

  const linkedDiagnostics = result.diagnostics.filter((diagnostic) =>
    [item.id, item.entity_ref].some((ref) => diagnostic.affected_refs?.includes(ref))
  );
  const linkedKnowledge = allKnowledgeRecords(knowledge, result).filter((record) =>
    [item.id, item.entity_ref].some((ref) => record.affected_refs.includes(ref))
  );
  const resultHashes =
    analysisRun?.analysis_run.result_refs.find((ref) => ref.result_ref.ref === item.id)?.hash_refs.length ?? 0;
  const envelopeHashAvailable = Boolean(
    analysisRun?.analysis_run.hashes.some((hash) => hash.payload_scope === "result_envelope")
  );

  return {
    result_id: item.id,
    family: resultFamily(item),
    entity_ref: item.entity_ref,
    value_label: `${item.value} ${item.unit}`,
    component: item.metadata?.component ?? item.kind,
    coordinate_system: item.metadata?.coordinate_system ?? "result_envelope",
    location: item.metadata?.location ?? "summary",
    recovery_basis: item.metadata?.basis ?? "reported_result_value",
    sign_convention: item.metadata?.sign_convention ?? "not specified in result metadata",
    linked_diagnostics: linkedDiagnostics,
    linked_knowledge: linkedKnowledge,
    source_run: {
      run_id: result.run_id,
      model_ref: result.model_ref,
      audit_ref: analysisRun?.deliverable_id ?? "DEL-14-02 audit context not generated",
      result_hash_count: resultHashes,
      envelope_hash_available: envelopeHashAvailable
    },
    professional_boundary: "human review required; review-only interpretation; no compliance or professional approval claim"
  };
}

export function buildDiagnosticInterpretation({
  model,
  knowledge,
  result,
  diagnosticId
}: {
  model: PreviewModel;
  knowledge: DesignKnowledge | null;
  result: MechanicsResult | null;
  diagnosticId: string | null;
}): DiagnosticInterpretation | null {
  const diagnostic = allDiagnostics(model, knowledge, result).find(
    (candidate) => (candidate.id ?? candidate.code) === diagnosticId
  );
  if (!diagnostic) return null;

  const affectedRefs = diagnostic.affected_refs ?? [];
  const linkedResults =
    result?.results
      .filter((item) => affectedRefs.includes(item.id) || affectedRefs.includes(item.entity_ref))
      .map((item) => ({
        id: item.id,
        kind: item.kind,
        entity_ref: item.entity_ref,
        value_label: `${item.value} ${item.unit}`
      })) ?? [];
  const linkedKnowledge = allKnowledgeRecords(knowledge, result).filter((record) =>
    [diagnostic.id, diagnostic.code, ...affectedRefs]
      .filter((value): value is string => Boolean(value))
      .some((ref) => record.affected_refs.includes(ref))
  );

  return {
    diagnostic_id: diagnostic.id ?? diagnostic.code,
    code: diagnostic.code,
    severity: diagnostic.severity,
    source: diagnostic.source ?? "local_preview_context",
    message: diagnostic.message,
    affected_refs: affectedRefs,
    linked_results: linkedResults,
    linked_knowledge: linkedKnowledge,
    review_explanation: diagnosticExplanation(diagnostic, affectedRefs, linkedResults.length),
    professional_boundary: "review-only diagnostic explanation; no compliance or professional approval claim"
  };
}

export function resolveEntitySelection(model: PreviewModel, entityRef: string): EntityRef | null {
  if (model.nodes.some((item) => item.id === entityRef)) return { type: "node", id: entityRef };
  if (model.pipe_segments.some((item) => item.id === entityRef)) return { type: "pipe", id: entityRef };
  if (model.supports.some((item) => item.id === entityRef)) return { type: "support", id: entityRef };
  if (model.components.some((item) => item.id === entityRef)) return { type: "component", id: entityRef };
  if (model.load_cases.some((item) => item.id === entityRef)) return { type: "load", id: entityRef };
  return null;
}

export function resolveDiagnosticEntitySelection({
  model,
  result,
  diagnosticId,
  knowledge
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  diagnosticId: string;
  knowledge: DesignKnowledge | null;
}): EntityRef | null {
  const diagnostic = allDiagnostics(model, knowledge, result).find(
    (candidate) => (candidate.id ?? candidate.code) === diagnosticId
  );
  const affectedRefs = diagnostic?.affected_refs ?? [];
  for (const ref of affectedRefs) {
    const directSelection = resolveEntitySelection(model, ref);
    if (directSelection) return directSelection;

    const resultEntityRef = result?.results.find((item) => item.id === ref)?.entity_ref;
    if (resultEntityRef) {
      const resultSelection = resolveEntitySelection(model, resultEntityRef);
      if (resultSelection) return resultSelection;
    }
  }
  return null;
}

export function mechanicsGaps(): MechanicsGap[] {
  return [
    {
      id: "gap:endpoint-j-recovery",
      capability: "Endpoint-j local force/moment recovery",
      status: "deferred",
      review_note: "Deferred until workflows require both element ends; current results retain end_i metadata."
    },
    {
      id: "gap:station-recovery",
      capability: "Intermediate station result recovery",
      status: "not_implemented",
      review_note: "No station sweep is computed in this preview interpretation layer."
    },
    {
      id: "gap:pressure-frame-load",
      capability: "Pressure-to-frame load conversion",
      status: "not_implemented",
      review_note: "Pressure loads are retained as stress context and reported through diagnostics when unsupported."
    },
    {
      id: "gap:thermal-behavior",
      capability: "Thermal behavior",
      status: "not_implemented",
      review_note: "Thermal strain, expansion cases, and temperature-dependent properties are outside this slice."
    },
    {
      id: "gap:support-stiffness",
      capability: "Support stiffness completeness",
      status: "deferred",
      review_note: "Linear support restraints are preview inputs; richer stiffness and nonlinear behavior require explicit data."
    },
    {
      id: "gap:load-combinations",
      capability: "Load combinations",
      status: "not_implemented",
      review_note: "Only the active invented primitive-load preview case is interpreted."
    },
    {
      id: "gap:protected-rule-checks",
      capability: "Protected rule/code checks",
      status: "requires_private_inputs",
      review_note: "Private criteria, allowables, SIF/flexibility tables, and code checks are not bundled publicly."
    }
  ];
}

function allKnowledgeRecords(knowledge: DesignKnowledge | null, result: MechanicsResult | null): KnowledgeRecord[] {
  return [...(knowledge?.records ?? []), ...physicsRecords(result)];
}

function allDiagnostics(
  model: PreviewModel,
  knowledge: DesignKnowledge | null,
  result: MechanicsResult | null
): Diagnostic[] {
  return [...model.diagnostics, ...(knowledge?.diagnostics ?? []), ...(result?.diagnostics ?? [])];
}

function diagnosticExplanation(
  diagnostic: Diagnostic,
  affectedRefs: string[],
  linkedResultCount: number
): string {
  const affectedSummary =
    affectedRefs.length > 0
      ? `It is linked to ${affectedRefs.join(", ")}.`
      : "It is not linked to a specific model entity or computed result.";
  const resultSummary =
    linkedResultCount > 0
      ? `${linkedResultCount} computed result reference${linkedResultCount === 1 ? "" : "s"} should be inspected with this diagnostic.`
      : "No computed result row is directly linked to this diagnostic.";
  return `${diagnostic.message} ${affectedSummary} ${resultSummary}`;
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
