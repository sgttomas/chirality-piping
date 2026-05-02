//! Result export envelope support.
//!
//! This crate validates bounded in-memory result export records and prepares
//! deterministic ordering for schema-first JSON result envelopes. It does not
//! parse project files, call solver internals, render reports, run GUI or CLI
//! workflows, implement adapters, access host resources, or emit professional
//! or code-compliance claims.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    ExportBlocking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Blocking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DimensionId {
    Dimensionless,
    Length,
    Angle,
    Force,
    Moment,
    Stress,
    Ratio,
    Time,
    Temperature,
    Pressure,
    Tbd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResultFamily {
    Displacement,
    Rotation,
    Force,
    Moment,
    Reaction,
    Stress,
    Ratio,
    RuleCheck,
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

#[derive(Debug, Clone, PartialEq)]
pub struct QuantityResult {
    pub result_id: String,
    pub family: ResultFamily,
    pub object_ref: Reference,
    pub basis_ref: Reference,
    pub station_ref: Option<Reference>,
    pub magnitude: f64,
    pub unit: String,
    pub dimension: DimensionId,
    pub provenance: Provenance,
}

impl QuantityResult {
    fn ordering_key(&self) -> (&str, ResultFamily, &str, &str) {
        (
            self.basis_ref.ref_id.as_str(),
            self.family,
            self.object_ref.ref_id.as_str(),
            self.result_id.as_str(),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResultSet {
    pub set_id: String,
    pub set_type: String,
    pub basis_ref: Reference,
    pub values: Vec<QuantityResult>,
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
    fn export_blocking(
        code: impl Into<String>,
        affected_object: Reference,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            class: DiagnosticClass::ExportBlocking,
            severity: DiagnosticSeverity::Blocking,
            source: Reference::new("result_export", "DEL-08-04"),
            affected_object,
            message: message.into(),
            remediation: "Provide explicit result export metadata before relying on this envelope."
                .to_string(),
            provenance: invented_provenance(),
        }
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
pub struct RulePackExportRef {
    pub rule_pack_id: String,
    pub version: String,
    pub checksum: ChecksumRef,
    pub source_notice: String,
    pub redistribution_status: RedistributionStatus,
    pub completeness_status: String,
    pub private_payload_redacted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReproducibilityRefs {
    pub model_hash: ChecksumRef,
    pub run_hashes: Vec<ChecksumRef>,
    pub audit_manifest_ref: Reference,
    pub deterministic_ordering: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResultEnvelope {
    pub envelope_id: String,
    pub schema_version: String,
    pub model_ref: Reference,
    pub run_ref: Reference,
    pub solver_name: String,
    pub solver_version: String,
    pub solver_build_ref: String,
    pub unit_system_ref: Reference,
    pub load_basis_refs: Vec<Reference>,
    pub result_sets: Vec<ResultSet>,
    pub diagnostics: Vec<Diagnostic>,
    pub provenance: Provenance,
    pub reproducibility: ReproducibilityRefs,
    pub analysis_status: Vec<AnalysisStatus>,
    pub rule_pack_refs: Vec<RulePackExportRef>,
    pub professional_boundary: ProfessionalBoundary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportValidation {
    pub diagnostics: Vec<Diagnostic>,
}

impl ExportValidation {
    pub fn has_blocking_diagnostics(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Blocking)
    }
}

pub fn validate_result_envelope(envelope: &ResultEnvelope) -> ExportValidation {
    let mut diagnostics = Vec::new();

    if envelope.envelope_id.trim().is_empty() {
        diagnostics.push(Diagnostic::export_blocking(
            "RESULT_EXPORT_ID_MISSING",
            Reference::new("result_envelope", "UNKNOWN"),
            "result envelope must have an envelope identifier",
        ));
    }

    if !envelope.model_ref.is_complete() || !envelope.run_ref.is_complete() {
        diagnostics.push(Diagnostic::export_blocking(
            "RESULT_EXPORT_REFERENCE_MISSING",
            Reference::new("result_envelope", &envelope.envelope_id),
            "result envelope must reference a model and run",
        ));
    }

    if envelope.solver_name.trim().is_empty() || envelope.solver_version.trim().is_empty() {
        diagnostics.push(Diagnostic::export_blocking(
            "RESULT_EXPORT_SOLVER_VERSION_MISSING",
            Reference::new("result_envelope", &envelope.envelope_id),
            "result export must include solver name and version",
        ));
    }

    if !envelope.unit_system_ref.is_complete() {
        diagnostics.push(Diagnostic::export_blocking(
            "RESULT_EXPORT_UNIT_SYSTEM_MISSING",
            Reference::new("result_envelope", &envelope.envelope_id),
            "result export must identify the unit-system basis",
        ));
    }

    if envelope.load_basis_refs.is_empty()
        || envelope
            .load_basis_refs
            .iter()
            .any(|reference| !reference.is_complete())
    {
        diagnostics.push(Diagnostic::export_blocking(
            "RESULT_EXPORT_LOAD_BASIS_MISSING",
            Reference::new("result_envelope", &envelope.envelope_id),
            "result export must identify load-case or combination basis references",
        ));
    }

    if !envelope
        .analysis_status
        .contains(&AnalysisStatus::HumanReviewRequired)
    {
        diagnostics.push(Diagnostic::export_blocking(
            "RESULT_EXPORT_HUMAN_REVIEW_STATUS_MISSING",
            Reference::new("result_envelope", &envelope.envelope_id),
            "result exports must preserve human-review-required status",
        ));
    }

    if !envelope.professional_boundary.preserves_boundary() {
        diagnostics.push(Diagnostic::export_blocking(
            "RESULT_EXPORT_PROFESSIONAL_BOUNDARY_VIOLATION",
            Reference::new("result_envelope", &envelope.envelope_id),
            "result export must not make compliance, certification, sealing, approval, or authentication claims",
        ));
    }

    if !envelope.provenance.is_complete() {
        diagnostics.push(Diagnostic::export_blocking(
            "RESULT_EXPORT_PROVENANCE_MISSING",
            Reference::new("result_envelope", &envelope.envelope_id),
            "result export must include source and redistribution provenance",
        ));
    }

    if !envelope.reproducibility.model_hash.is_complete()
        || !envelope.reproducibility.audit_manifest_ref.is_complete()
        || !envelope.reproducibility.deterministic_ordering
    {
        diagnostics.push(Diagnostic::export_blocking(
            "RESULT_EXPORT_REPRODUCIBILITY_MISSING",
            Reference::new("result_envelope", &envelope.envelope_id),
            "result export must include model hash, audit manifest reference, and deterministic ordering",
        ));
    }

    check_rule_pack_refs(envelope, &mut diagnostics);
    check_result_sets(envelope, &mut diagnostics);

    ExportValidation { diagnostics }
}

pub fn sorted_result_values(envelope: &ResultEnvelope) -> Vec<&QuantityResult> {
    let mut values: Vec<_> = envelope
        .result_sets
        .iter()
        .flat_map(|set| set.values.iter())
        .collect();
    values.sort_by_key(|value| value.ordering_key());
    values
}

fn check_rule_pack_refs(envelope: &ResultEnvelope, diagnostics: &mut Vec<Diagnostic>) {
    for rule_pack in &envelope.rule_pack_refs {
        if rule_pack.rule_pack_id.trim().is_empty()
            || rule_pack.version.trim().is_empty()
            || !rule_pack.checksum.is_complete()
            || rule_pack.source_notice.trim().is_empty()
            || rule_pack.completeness_status.trim().is_empty()
            || !rule_pack.private_payload_redacted
        {
            diagnostics.push(Diagnostic::export_blocking(
                "RESULT_EXPORT_RULE_PACK_REF_INCOMPLETE",
                Reference::new("rule_pack", &rule_pack.rule_pack_id),
                "rule-pack exports must include identity, version, checksum, source notice, completeness, and redaction metadata",
            ));
        }

        if rule_pack.redistribution_status == RedistributionStatus::ProtectedSuspected {
            diagnostics.push(Diagnostic::export_blocking(
                "RESULT_EXPORT_PROTECTED_RULE_PACK_SUSPECTED",
                Reference::new("rule_pack", &rule_pack.rule_pack_id),
                "suspected protected rule-pack content must not be exported as a public payload",
            ));
        }
    }
}

fn check_result_sets(envelope: &ResultEnvelope, diagnostics: &mut Vec<Diagnostic>) {
    if envelope.result_sets.is_empty() {
        diagnostics.push(Diagnostic::export_blocking(
            "RESULT_EXPORT_RESULT_SET_MISSING",
            Reference::new("result_envelope", &envelope.envelope_id),
            "result export must include at least one result set",
        ));
        return;
    }

    for set in &envelope.result_sets {
        if set.set_id.trim().is_empty()
            || set.set_type.trim().is_empty()
            || !set.basis_ref.is_complete()
        {
            diagnostics.push(Diagnostic::export_blocking(
                "RESULT_EXPORT_RESULT_SET_INCOMPLETE",
                Reference::new("result_set", &set.set_id),
                "result set must include identity, type, and basis reference",
            ));
        }

        for value in &set.values {
            if value.result_id.trim().is_empty()
                || !value.object_ref.is_complete()
                || !value.basis_ref.is_complete()
                || !value.magnitude.is_finite()
                || value.unit.trim().is_empty()
                || value.dimension == DimensionId::Tbd
                || !value.provenance.is_complete()
            {
                diagnostics.push(Diagnostic::export_blocking(
                    "RESULT_EXPORT_VALUE_METADATA_INCOMPLETE",
                    Reference::new("result_value", &value.result_id),
                    "result values must carry object, basis, finite magnitude, unit, dimension, and provenance metadata",
                ));
            }
        }
    }
}

fn invented_provenance() -> Provenance {
    Provenance {
        source_name: "OpenPipeStress result export validator".to_string(),
        source_location: "core/reporting/result_export".to_string(),
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
            canonicalization: "JCS".to_string(),
            payload_ref: Reference::new("payload", id),
            value: format!("{id}-hash"),
        }
    }

    fn provenance() -> Provenance {
        Provenance {
            source_name: "invented mechanics fixture".to_string(),
            source_location: "validation/benchmarks/invented".to_string(),
            source_license: "project invented".to_string(),
            contributor: "OpenPipeStress".to_string(),
            contributor_certification: "invented non-engineering example".to_string(),
            redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
            review_status: "accepted".to_string(),
        }
    }

    fn quantity(
        id: &str,
        family: ResultFamily,
        unit: &str,
        dimension: DimensionId,
    ) -> QuantityResult {
        QuantityResult {
            result_id: id.to_string(),
            family,
            object_ref: Reference::new("node", "N1"),
            basis_ref: Reference::new("load_case", "LC1"),
            station_ref: None,
            magnitude: 1.25,
            unit: unit.to_string(),
            dimension,
            provenance: provenance(),
        }
    }

    fn envelope() -> ResultEnvelope {
        ResultEnvelope {
            envelope_id: "result-envelope-1".to_string(),
            schema_version: "0.1.0".to_string(),
            model_ref: Reference::new("model", "invented-model"),
            run_ref: Reference::new("analysis_run", "run-1"),
            solver_name: "open_pipe_stress_core".to_string(),
            solver_version: "0.1.0".to_string(),
            solver_build_ref: "test-build".to_string(),
            unit_system_ref: Reference::new("unit_system", "invented-si"),
            load_basis_refs: vec![Reference::new("load_case", "LC1")],
            result_sets: vec![ResultSet {
                set_id: "set-1".to_string(),
                set_type: "mechanics".to_string(),
                basis_ref: Reference::new("load_case", "LC1"),
                values: vec![
                    quantity("stress-1", ResultFamily::Stress, "Pa", DimensionId::Stress),
                    quantity(
                        "disp-1",
                        ResultFamily::Displacement,
                        "m",
                        DimensionId::Length,
                    ),
                    quantity(
                        "reaction-1",
                        ResultFamily::Reaction,
                        "N",
                        DimensionId::Force,
                    ),
                ],
            }],
            diagnostics: Vec::new(),
            provenance: provenance(),
            reproducibility: ReproducibilityRefs {
                model_hash: checksum("model"),
                run_hashes: vec![checksum("run")],
                audit_manifest_ref: Reference::new("audit_manifest", "manifest-1"),
                deterministic_ordering: true,
            },
            analysis_status: vec![
                AnalysisStatus::MechanicsSolved,
                AnalysisStatus::HumanReviewRequired,
            ],
            rule_pack_refs: vec![RulePackExportRef {
                rule_pack_id: "invented-rule-pack".to_string(),
                version: "0.1.0".to_string(),
                checksum: checksum("rule-pack"),
                source_notice: "invented non-engineering example".to_string(),
                redistribution_status: RedistributionStatus::InventedNonEngineeringExample,
                completeness_status: "complete_for_user_rule_check".to_string(),
                private_payload_redacted: true,
            }],
            professional_boundary: ProfessionalBoundary::project_default(),
        }
    }

    #[test]
    fn valid_envelope_has_no_blocking_diagnostics() {
        let validation = validate_result_envelope(&envelope());
        assert!(validation.diagnostics.is_empty());
    }

    #[test]
    fn missing_units_and_dimensions_are_blocking() {
        let mut envelope = envelope();
        envelope.result_sets[0].values[0].unit.clear();
        envelope.result_sets[0].values[0].dimension = DimensionId::Tbd;

        let validation = validate_result_envelope(&envelope);

        assert!(validation.has_blocking_diagnostics());
        assert!(validation
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "RESULT_EXPORT_VALUE_METADATA_INCOMPLETE"));
    }

    #[test]
    fn human_review_required_status_is_mandatory() {
        let mut envelope = envelope();
        envelope.analysis_status = vec![AnalysisStatus::MechanicsSolved];

        let validation = validate_result_envelope(&envelope);

        assert!(validation.has_blocking_diagnostics());
        assert!(validation
            .diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.code == "RESULT_EXPORT_HUMAN_REVIEW_STATUS_MISSING" }));
    }

    #[test]
    fn professional_boundary_violation_is_blocking() {
        let mut envelope = envelope();
        envelope
            .professional_boundary
            .software_makes_compliance_claim = true;

        let validation = validate_result_envelope(&envelope);

        assert!(validation.has_blocking_diagnostics());
        assert!(validation.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "RESULT_EXPORT_PROFESSIONAL_BOUNDARY_VIOLATION"
        }));
    }

    #[test]
    fn rule_pack_payload_must_remain_redacted() {
        let mut envelope = envelope();
        envelope.rule_pack_refs[0].private_payload_redacted = false;

        let validation = validate_result_envelope(&envelope);

        assert!(validation.has_blocking_diagnostics());
        assert!(validation
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "RESULT_EXPORT_RULE_PACK_REF_INCOMPLETE"));
    }

    #[test]
    fn protected_rule_pack_reference_blocks_export() {
        let mut envelope = envelope();
        envelope.rule_pack_refs[0].redistribution_status = RedistributionStatus::ProtectedSuspected;

        let validation = validate_result_envelope(&envelope);

        assert!(validation.has_blocking_diagnostics());
        assert!(validation.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "RESULT_EXPORT_PROTECTED_RULE_PACK_SUSPECTED"
        }));
    }

    #[test]
    fn result_values_sort_deterministically_for_regression_comparison() {
        let envelope = envelope();
        let ids: Vec<_> = sorted_result_values(&envelope)
            .into_iter()
            .map(|value| value.result_id.as_str())
            .collect();

        assert_eq!(ids, vec!["disp-1", "reaction-1", "stress-1"]);
    }
}
