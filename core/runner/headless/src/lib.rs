//! Headless runner contract support.
//!
//! This crate validates bounded in-memory request/result records for the
//! schema-first DEL-10-05 headless runner contract. It does not parse arbitrary
//! project files, invoke external processes, access the network, mutate the
//! filesystem, run GUI/report/adapter/local-FEA workflows, or emit professional
//! or code-compliance claims.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalysisStatus {
    ModelIncomplete,
    MechanicsSolved,
    RuleInputsIncomplete,
    UserRuleChecked,
    UserRuleFailed,
    HumanReviewRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticClass {
    SolveBlocking,
    RuleCheckBlocking,
    ProvenanceWarning,
    AssumptionWarning,
    NonlinearWarning,
    IpBoundaryWarning,
    UnitWarning,
    RunnerBlocking,
    ExportBlocking,
    PrivacyWarning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Blocking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunnerOperation {
    Solve,
    ValidateInput,
    ExportResults,
    RunBenchmark,
    RunRegression,
    Tbd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobStateKind {
    Queued,
    Running,
    CancellationRequested,
    Completed,
    Failed,
    Tbd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivacyClass {
    PublicMetadata,
    PrivateProjectData,
    PrivateRulePackData,
    Redacted,
    ProtectedSuspected,
    Tbd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedistributionStatus {
    PublicPermissive,
    PrivateOnly,
    Unknown,
    ProtectedSuspected,
    InventedNonEngineeringExample,
    Tbd,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    pub ref_type: String,
    pub ref_id: String,
}

impl Reference {
    pub fn new(ref_type: impl Into<String>, ref_id: impl Into<String>) -> Self {
        Self {
            ref_type: ref_type.into(),
            ref_id: ref_id.into(),
        }
    }

    fn is_complete(&self) -> bool {
        !self.ref_type.trim().is_empty() && !self.ref_id.trim().is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provenance {
    pub source_name: String,
    pub source_location: String,
    pub source_license: String,
    pub contributor: String,
    pub contributor_certification: String,
    pub redistribution_status: RedistributionStatus,
    pub review_status: String,
}

impl Provenance {
    fn is_complete(&self) -> bool {
        !self.source_name.trim().is_empty()
            && !self.source_location.trim().is_empty()
            && !self.source_license.trim().is_empty()
            && !self.contributor.trim().is_empty()
            && !self.contributor_certification.trim().is_empty()
            && !self.review_status.trim().is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecksumRef {
    pub algorithm: String,
    pub canonicalization: String,
    pub payload_ref: Reference,
    pub value: String,
}

impl ChecksumRef {
    fn is_complete(&self) -> bool {
        !self.algorithm.trim().is_empty()
            && !self.canonicalization.trim().is_empty()
            && self.payload_ref.is_complete()
            && !self.value.trim().is_empty()
            && !self.value.trim().eq_ignore_ascii_case("TBD")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivacyContext {
    pub local_only: bool,
    pub telemetry_allowed: bool,
    pub private_payload_redacted: bool,
    pub classification: PrivacyClass,
}

impl PrivacyContext {
    pub fn local_first_public_metadata() -> Self {
        Self {
            local_only: true,
            telemetry_allowed: false,
            private_payload_redacted: true,
            classification: PrivacyClass::PublicMetadata,
        }
    }

    fn preserves_boundary(&self) -> bool {
        self.local_only
            && !self.telemetry_allowed
            && self.classification != PrivacyClass::ProtectedSuspected
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfessionalBoundary {
    pub human_review_required: bool,
    pub software_makes_compliance_claim: bool,
    pub software_makes_certification_claim: bool,
    pub software_makes_sealing_claim: bool,
    pub software_makes_approval_claim: bool,
    pub software_makes_authentication_claim: bool,
}

impl ProfessionalBoundary {
    pub fn project_default() -> Self {
        Self {
            human_review_required: true,
            software_makes_compliance_claim: false,
            software_makes_certification_claim: false,
            software_makes_sealing_claim: false,
            software_makes_approval_claim: false,
            software_makes_authentication_claim: false,
        }
    }

    fn preserves_boundary(&self) -> bool {
        self.human_review_required
            && !self.software_makes_compliance_claim
            && !self.software_makes_certification_claim
            && !self.software_makes_sealing_claim
            && !self.software_makes_approval_claim
            && !self.software_makes_authentication_claim
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: String,
    pub class: DiagnosticClass,
    pub severity: DiagnosticSeverity,
    pub source: Reference,
    pub affected_object: Reference,
    pub message: String,
    pub remediation: String,
    pub provenance: Provenance,
}

impl Diagnostic {
    fn runner_blocking(
        code: impl Into<String>,
        affected_object: Reference,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            class: DiagnosticClass::RunnerBlocking,
            severity: DiagnosticSeverity::Blocking,
            source: Reference::new("headless_runner", "DEL-10-05"),
            affected_object,
            message: message.into(),
            remediation: "Provide explicit schema-first runner metadata before relying on this request or result.".to_string(),
            provenance: invented_provenance(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TbdDecisions {
    pub final_cli_command_syntax: bool,
    pub package_scripts: bool,
    pub process_invocation: bool,
    pub network_access: bool,
    pub filesystem_mutation_policy: bool,
    pub ci_provider: bool,
    pub release_matrix: bool,
    pub public_transport_protocol: bool,
    pub external_adapter_formats: bool,
    pub physical_project_container: bool,
}

impl TbdDecisions {
    pub fn all_deferred() -> Self {
        Self {
            final_cli_command_syntax: true,
            package_scripts: true,
            process_invocation: true,
            network_access: true,
            filesystem_mutation_policy: true,
            ci_provider: true,
            release_matrix: true,
            public_transport_protocol: true,
            external_adapter_formats: true,
            physical_project_container: true,
        }
    }

    fn preserves_deferred_surface(&self) -> bool {
        self.final_cli_command_syntax
            && self.package_scripts
            && self.process_invocation
            && self.network_access
            && self.filesystem_mutation_policy
            && self.ci_provider
            && self.release_matrix
            && self.public_transport_protocol
            && self.external_adapter_formats
            && self.physical_project_container
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunnerRequest {
    pub request_id: String,
    pub operation: RunnerOperation,
    pub operation_ref: Reference,
    pub project_ref: Reference,
    pub model_ref: Reference,
    pub unit_system_ref: Reference,
    pub load_basis_refs: Vec<Reference>,
    pub input_manifest_ref: Reference,
    pub requested_outputs: Vec<String>,
    pub privacy: PrivacyContext,
    pub provenance: Provenance,
    pub professional_boundary: ProfessionalBoundary,
    pub tbd_decisions: TbdDecisions,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JobState {
    pub job_id: String,
    pub state: JobStateKind,
    pub current_step: u32,
    pub total_steps: u32,
    pub cancellation_supported: bool,
    pub cancellation_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultEnvelopeRef {
    pub schema_ref: String,
    pub envelope_ref: Reference,
    pub compatibility: String,
}

impl ResultEnvelopeRef {
    pub fn result_export(envelope_ref: Reference) -> Self {
        Self {
            schema_ref: "schemas/results.schema.yaml".to_string(),
            envelope_ref,
            compatibility: "schema_first_json_result_envelope".to_string(),
        }
    }

    fn is_compatible(&self) -> bool {
        self.schema_ref == "schemas/results.schema.yaml"
            && self.compatibility == "schema_first_json_result_envelope"
            && self.envelope_ref.is_complete()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunnerResult {
    pub run_id: String,
    pub job: JobState,
    pub analysis_status: Vec<AnalysisStatus>,
    pub result_envelope_ref: ResultEnvelopeRef,
    pub audit_manifest_ref: Reference,
    pub checksums: Vec<ChecksumRef>,
    pub diagnostics: Vec<Diagnostic>,
    pub privacy: PrivacyContext,
    pub provenance: Provenance,
    pub professional_boundary: ProfessionalBoundary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunnerValidation {
    pub diagnostics: Vec<Diagnostic>,
}

impl RunnerValidation {
    pub fn has_blocking_diagnostics(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Blocking)
    }
}

pub fn validate_request(request: &RunnerRequest) -> RunnerValidation {
    let mut diagnostics = Vec::new();

    if request.request_id.trim().is_empty() {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_REQUEST_ID_MISSING",
            Reference::new("runner_request", "UNKNOWN"),
            "headless runner request must have a request identifier",
        ));
    }

    if request.operation == RunnerOperation::Tbd || !request.operation_ref.is_complete() {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_OPERATION_MISSING",
            Reference::new("runner_request", &request.request_id),
            "headless runner request must identify a bounded operation",
        ));
    }

    if !request.project_ref.is_complete()
        || !request.model_ref.is_complete()
        || !request.unit_system_ref.is_complete()
        || !request.input_manifest_ref.is_complete()
    {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_REQUIRED_REFS_MISSING",
            Reference::new("runner_request", &request.request_id),
            "runner request must identify project, model, unit-system, and input-manifest references",
        ));
    }

    if request.load_basis_refs.is_empty()
        || request
            .load_basis_refs
            .iter()
            .any(|reference| !reference.is_complete())
    {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_LOAD_BASIS_MISSING",
            Reference::new("runner_request", &request.request_id),
            "runner request must identify load-case or combination basis references",
        ));
    }

    if request.requested_outputs.is_empty()
        || request
            .requested_outputs
            .iter()
            .any(|output| output.trim().is_empty())
    {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_OUTPUTS_MISSING",
            Reference::new("runner_request", &request.request_id),
            "runner request must identify requested structured outputs",
        ));
    }

    validate_shared_boundaries(
        "runner_request",
        &request.request_id,
        &request.privacy,
        &request.provenance,
        &request.professional_boundary,
        &mut diagnostics,
    );

    if !request.tbd_decisions.preserves_deferred_surface() {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_TBD_SURFACE_OVERSPECIFIED",
            Reference::new("runner_request", &request.request_id),
            "final CLI syntax, package scripts, process/network/filesystem policy, CI, release, transport, adapter, and container decisions must remain deferred in DEL-10-05",
        ));
    }

    RunnerValidation { diagnostics }
}

pub fn validate_result(result: &RunnerResult) -> RunnerValidation {
    let mut diagnostics = Vec::new();

    if result.run_id.trim().is_empty() || result.job.job_id.trim().is_empty() {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_RUN_ID_MISSING",
            Reference::new("runner_result", "UNKNOWN"),
            "headless runner result must identify the run and job",
        ));
    }

    if result.job.total_steps > 0 && result.job.current_step > result.job.total_steps {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_PROGRESS_INVALID",
            Reference::new("runner_result", &result.run_id),
            "job progress must not exceed total steps",
        ));
    }

    if !result
        .analysis_status
        .contains(&AnalysisStatus::HumanReviewRequired)
    {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_HUMAN_REVIEW_STATUS_MISSING",
            Reference::new("runner_result", &result.run_id),
            "runner results must preserve human-review-required status",
        ));
    }

    if !result.result_envelope_ref.is_compatible() {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_RESULT_EXPORT_REF_INVALID",
            Reference::new("runner_result", &result.run_id),
            "runner result must reference the schema-first JSON result export envelope",
        ));
    }

    if !result.audit_manifest_ref.is_complete()
        || result.checksums.is_empty()
        || result
            .checksums
            .iter()
            .any(|checksum| !checksum.is_complete())
    {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_REPRODUCIBILITY_MISSING",
            Reference::new("runner_result", &result.run_id),
            "runner result must include audit-manifest and checksum references",
        ));
    }

    validate_shared_boundaries(
        "runner_result",
        &result.run_id,
        &result.privacy,
        &result.provenance,
        &result.professional_boundary,
        &mut diagnostics,
    );

    RunnerValidation { diagnostics }
}

fn validate_shared_boundaries(
    ref_type: &str,
    ref_id: &str,
    privacy: &PrivacyContext,
    provenance: &Provenance,
    professional_boundary: &ProfessionalBoundary,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if !privacy.preserves_boundary() {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_PRIVACY_BOUNDARY_VIOLATION",
            Reference::new(ref_type, ref_id),
            "headless runner records must remain local-first, telemetry-off, and must not carry suspected protected payloads",
        ));
    }

    if !provenance.is_complete()
        || provenance.redistribution_status == RedistributionStatus::ProtectedSuspected
    {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_PROVENANCE_MISSING",
            Reference::new(ref_type, ref_id),
            "headless runner records must carry source, license, contributor, redistribution, and review provenance",
        ));
    }

    if !professional_boundary.preserves_boundary() {
        diagnostics.push(Diagnostic::runner_blocking(
            "HEADLESS_RUNNER_PROFESSIONAL_BOUNDARY_VIOLATION",
            Reference::new(ref_type, ref_id),
            "runner output must not make compliance, certification, sealing, approval, or authentication claims",
        ));
    }
}

fn invented_provenance() -> Provenance {
    Provenance {
        source_name: "OpenPipeStress headless runner validator".to_string(),
        source_location: "core/runner/headless".to_string(),
        source_license: "project".to_string(),
        contributor: "OpenPipeStress".to_string(),
        contributor_certification: "diagnostic metadata only".to_string(),
        redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
        review_status: "accepted".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn checksum(id: &str) -> ChecksumRef {
        ChecksumRef {
            algorithm: "sha256".to_string(),
            canonicalization: "JCS-compatible-json".to_string(),
            payload_ref: Reference::new("payload", id),
            value: format!("{id}-hash"),
        }
    }

    fn provenance() -> Provenance {
        Provenance {
            source_name: "invented headless runner fixture".to_string(),
            source_location: "validation/benchmarks/invented".to_string(),
            source_license: "project invented".to_string(),
            contributor: "OpenPipeStress".to_string(),
            contributor_certification: "invented non-engineering example".to_string(),
            redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
            review_status: "accepted".to_string(),
        }
    }

    fn request() -> RunnerRequest {
        RunnerRequest {
            request_id: "request-1".to_string(),
            operation: RunnerOperation::Solve,
            operation_ref: Reference::new("api_operation", "ops.solve.job"),
            project_ref: Reference::new("project", "invented-project"),
            model_ref: Reference::new("model", "invented-model"),
            unit_system_ref: Reference::new("unit_system", "invented-si"),
            load_basis_refs: vec![Reference::new("load_case", "LC1")],
            input_manifest_ref: Reference::new("audit_manifest", "manifest-1"),
            requested_outputs: vec![
                "result_envelope".to_string(),
                "audit_manifest".to_string(),
                "diagnostics".to_string(),
            ],
            privacy: PrivacyContext::local_first_public_metadata(),
            provenance: provenance(),
            professional_boundary: ProfessionalBoundary::project_default(),
            tbd_decisions: TbdDecisions::all_deferred(),
        }
    }

    fn result() -> RunnerResult {
        RunnerResult {
            run_id: "run-1".to_string(),
            job: JobState {
                job_id: "job-1".to_string(),
                state: JobStateKind::Completed,
                current_step: 3,
                total_steps: 3,
                cancellation_supported: true,
                cancellation_requested: false,
            },
            analysis_status: vec![
                AnalysisStatus::MechanicsSolved,
                AnalysisStatus::HumanReviewRequired,
            ],
            result_envelope_ref: ResultEnvelopeRef::result_export(Reference::new(
                "result_envelope",
                "result-envelope-1",
            )),
            audit_manifest_ref: Reference::new("audit_manifest", "manifest-1"),
            checksums: vec![checksum("input-manifest")],
            diagnostics: Vec::new(),
            privacy: PrivacyContext::local_first_public_metadata(),
            provenance: provenance(),
            professional_boundary: ProfessionalBoundary::project_default(),
        }
    }

    #[test]
    fn valid_request_has_no_blocking_diagnostics() {
        let validation = validate_request(&request());
        assert!(!validation.has_blocking_diagnostics(), "{validation:?}");
    }

    #[test]
    fn request_requires_load_basis_and_deferred_surface() {
        let mut request = request();
        request.load_basis_refs.clear();
        request.tbd_decisions.final_cli_command_syntax = false;
        let validation = validate_request(&request);
        let codes: Vec<_> = validation
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code.as_str())
            .collect();
        assert!(codes.contains(&"HEADLESS_RUNNER_LOAD_BASIS_MISSING"));
        assert!(codes.contains(&"HEADLESS_RUNNER_TBD_SURFACE_OVERSPECIFIED"));
    }

    #[test]
    fn valid_result_has_no_blocking_diagnostics() {
        let validation = validate_result(&result());
        assert!(!validation.has_blocking_diagnostics(), "{validation:?}");
    }

    #[test]
    fn result_requires_human_review_and_result_export_compatibility() {
        let mut result = result();
        result.analysis_status = vec![AnalysisStatus::MechanicsSolved];
        result.result_envelope_ref.schema_ref = "ad_hoc_output.json".to_string();
        let validation = validate_result(&result);
        let codes: Vec<_> = validation
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code.as_str())
            .collect();
        assert!(codes.contains(&"HEADLESS_RUNNER_HUMAN_REVIEW_STATUS_MISSING"));
        assert!(codes.contains(&"HEADLESS_RUNNER_RESULT_EXPORT_REF_INVALID"));
    }

    #[test]
    fn privacy_and_professional_boundary_violations_are_blocking() {
        let mut result = result();
        result.privacy.telemetry_allowed = true;
        result.professional_boundary.software_makes_compliance_claim = true;
        let validation = validate_result(&result);
        let codes: Vec<_> = validation
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code.as_str())
            .collect();
        assert!(codes.contains(&"HEADLESS_RUNNER_PRIVACY_BOUNDARY_VIOLATION"));
        assert!(codes.contains(&"HEADLESS_RUNNER_PROFESSIONAL_BOUNDARY_VIOLATION"));
    }
}
