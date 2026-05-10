export type Vec3 = { x: number; y: number; z: number };

export type PreviewModel = {
  schema_version: string;
  document_kind: string;
  data_boundary: Record<string, string>;
  project: {
    id: string;
    name: string;
    description: string;
    units: Record<string, string>;
  };
  analysis_status: {
    mechanics: string;
    rule_check: string;
    professional_acceptance: string;
  };
  materials?: Array<{
    id: string;
    label: string;
    elastic_modulus: { value: number; unit: string };
    shear_modulus: { value: number; unit: string };
    provenance: string;
  }>;
  nodes: Array<{ id: string; label: string; position: Vec3; provenance: string }>;
  pipe_segments: Array<{
    id: string;
    label: string;
    from: string;
    to: string;
    section: Record<string, { value: number; unit: string }>;
    material: string;
    provenance: string;
  }>;
  supports: Array<{ id: string; label: string; node: string; restraints: string[]; provenance: string }>;
  components: Array<{ id: string; label: string; kind: string; node: string; provenance: string }>;
  load_cases: Array<{
    id: string;
    label: string;
    kind: string;
    status: string;
    provenance: string;
    primitive_loads?: Array<Record<string, unknown>>;
  }>;
  diagnostics: Diagnostic[];
};

export type Diagnostic = {
  id?: string;
  code: string;
  severity: "info" | "warning" | "error" | "blocking";
  message: string;
  source?: string;
  affected_refs?: string[];
};

export type KnowledgeRecord = {
  id: string;
  kind: string;
  title: string;
  summary: string;
  affected_refs: string[];
  provenance: string;
  status: string;
};

export type DesignKnowledge = {
  schema_version: string;
  document_kind: string;
  knowledge_set_id: string;
  model_ref: string;
  records: KnowledgeRecord[];
  diagnostics: Diagnostic[];
};

export type MechanicsResult = {
  schema_version: string;
  document_kind: string;
  run_id: string;
  model_ref: string;
  status: {
    mechanics: string;
    rule_check: string;
    professional_acceptance: string;
  };
  summary: Record<string, unknown> & {
    max_displacement?: { value: number; unit: string; location_ref: string; result_ref: string } | null;
    max_open_formula_stress?: { value: number; unit: string; location_ref: string; result_ref: string } | null;
  };
  results: Array<{ id: string; kind: string; value: number; unit: string; entity_ref: string }>;
  diagnostics: Diagnostic[];
};

export type AgentProposal = {
  schema_version: string;
  document_kind: string;
  proposal_id: string;
  prompt: string;
  operation: {
    operation_id: string;
    operation_kind: string;
    operation_status: string;
    affected_entity_ids: string[];
    changes: Array<{ change_id: string; change_kind: string; target_ref: string; before: string; after: string }>;
  };
  rationale: string;
  assumptions: string[];
  validation: Record<string, string>;
  audit_boundary: {
    requires_user_acceptance: boolean;
    mutates_accepted_model_state: boolean;
    acceptance_recorded_as_review_only: boolean;
  };
  professional_boundary: Record<string, boolean>;
};

export type EntityRef = {
  id: string;
  type: "project" | "node" | "pipe" | "support" | "component" | "load" | "diagnostic";
};
