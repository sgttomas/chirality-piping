//! Code-neutral mechanics stress recovery.
//!
//! This crate recovers open mechanics stress components from explicit
//! resultants and section properties. It does not encode design-code stress
//! equations, allowables, stress indices, SIF/flexibility tables, protected
//! standards content, proprietary data, rule-pack checks, or professional
//! approval.

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalysisStatus {
    ModelIncomplete,
    MechanicsSolved,
    RuleInputsIncomplete,
    UserRuleChecked,
    UserRuleFailed,
    HumanReviewRequired,
    HumanApprovedForProject,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingCode {
    MissingResultant,
    MissingSectionProperty,
    MissingPressureInput,
    NonFiniteInput,
    NonPositiveInput,
    IncompleteMechanicsStatus,
    StatusBoundaryViolation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StressFinding {
    pub code: FindingCode,
    pub subject_id: String,
    pub message: String,
}

impl StressFinding {
    fn new(code: FindingCode, subject_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code,
            subject_id: subject_id.into(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForceResultants {
    pub axial_force: Option<f64>,
    pub bending_moment_y: Option<f64>,
    pub bending_moment_z: Option<f64>,
    pub torsional_moment: Option<f64>,
}

impl ForceResultants {
    pub fn new(
        axial_force: Option<f64>,
        bending_moment_y: Option<f64>,
        bending_moment_z: Option<f64>,
        torsional_moment: Option<f64>,
    ) -> Self {
        Self {
            axial_force,
            bending_moment_y,
            bending_moment_z,
            torsional_moment,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StressSectionProperties {
    pub area: Option<f64>,
    pub section_modulus_y: Option<f64>,
    pub section_modulus_z: Option<f64>,
    pub torsion_constant: Option<f64>,
    pub torsion_radius: Option<f64>,
}

impl StressSectionProperties {
    pub fn new(
        area: Option<f64>,
        section_modulus_y: Option<f64>,
        section_modulus_z: Option<f64>,
        torsion_constant: Option<f64>,
        torsion_radius: Option<f64>,
    ) -> Self {
        Self {
            area,
            section_modulus_y,
            section_modulus_z,
            torsion_constant,
            torsion_radius,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PressureBasis {
    pub pressure: Option<f64>,
    pub membrane_radius: Option<f64>,
    pub wall_thickness: Option<f64>,
}

impl PressureBasis {
    pub fn new(
        pressure: Option<f64>,
        membrane_radius: Option<f64>,
        wall_thickness: Option<f64>,
    ) -> Self {
        Self {
            pressure,
            membrane_radius,
            wall_thickness,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StressComponents {
    pub axial_normal: Option<f64>,
    pub bending_normal_y: Option<f64>,
    pub bending_normal_z: Option<f64>,
    pub torsional_shear: Option<f64>,
    pub pressure_hoop: Option<f64>,
    pub pressure_longitudinal: Option<f64>,
}

impl StressComponents {
    fn empty() -> Self {
        Self {
            axial_normal: None,
            bending_normal_y: None,
            bending_normal_z: None,
            torsional_shear: None,
            pressure_hoop: None,
            pressure_longitudinal: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StressSummary {
    pub max_normal: f64,
    pub min_normal: f64,
    pub max_shear_magnitude: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StressRecoveryInput {
    pub resultants: ForceResultants,
    pub section: StressSectionProperties,
    pub pressure: Option<PressureBasis>,
    pub statuses: Vec<AnalysisStatus>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StressRecoveryResult {
    pub components: StressComponents,
    pub summary: Option<StressSummary>,
    pub statuses: Vec<AnalysisStatus>,
    pub findings: Vec<StressFinding>,
}

impl StressRecoveryResult {
    pub fn is_blocked(&self) -> bool {
        !self.findings.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StressRecoveryError {
    NonFiniteInput { name: &'static str, value: f64 },
    NonPositiveInput { name: &'static str, value: f64 },
}

impl fmt::Display for StressRecoveryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteInput { name, value } => {
                write!(f, "{name} must be finite, got {value}")
            }
            Self::NonPositiveInput { name, value } => {
                write!(f, "{name} must be positive, got {value}")
            }
        }
    }
}

impl Error for StressRecoveryError {}

pub fn recover_stresses(input: &StressRecoveryInput) -> StressRecoveryResult {
    let mut findings = Vec::new();
    let statuses = collect_statuses(&input.statuses, &mut findings);

    let mut components = StressComponents::empty();
    components.axial_normal = divide_optional(
        input.resultants.axial_force,
        input.section.area,
        "axial_force",
        "area",
        &mut findings,
    );
    components.bending_normal_y = divide_optional(
        input.resultants.bending_moment_y,
        input.section.section_modulus_y,
        "bending_moment_y",
        "section_modulus_y",
        &mut findings,
    );
    components.bending_normal_z = divide_optional(
        input.resultants.bending_moment_z,
        input.section.section_modulus_z,
        "bending_moment_z",
        "section_modulus_z",
        &mut findings,
    );
    components.torsional_shear = torsional_shear(&input.resultants, &input.section, &mut findings);

    if let Some(pressure) = &input.pressure {
        let (hoop, longitudinal) = pressure_membrane(pressure, &mut findings);
        components.pressure_hoop = hoop;
        components.pressure_longitudinal = longitudinal;
    }

    let summary = if findings.is_empty() {
        Some(summarize_components(&components))
    } else {
        None
    };

    StressRecoveryResult {
        components: if findings.is_empty() {
            components
        } else {
            StressComponents::empty()
        },
        summary,
        statuses,
        findings,
    }
}

pub fn checked_positive(name: &'static str, value: f64) -> Result<f64, StressRecoveryError> {
    if !value.is_finite() {
        return Err(StressRecoveryError::NonFiniteInput { name, value });
    }
    if value <= 0.0 {
        return Err(StressRecoveryError::NonPositiveInput { name, value });
    }
    Ok(value)
}

fn divide_optional(
    numerator: Option<f64>,
    denominator: Option<f64>,
    numerator_name: &'static str,
    denominator_name: &'static str,
    findings: &mut Vec<StressFinding>,
) -> Option<f64> {
    let numerator = require_finite(
        numerator,
        numerator_name,
        FindingCode::MissingResultant,
        findings,
    )?;
    let denominator = require_positive(
        denominator,
        denominator_name,
        FindingCode::MissingSectionProperty,
        findings,
    )?;
    Some(numerator / denominator)
}

fn torsional_shear(
    resultants: &ForceResultants,
    section: &StressSectionProperties,
    findings: &mut Vec<StressFinding>,
) -> Option<f64> {
    let torque = require_finite(
        resultants.torsional_moment,
        "torsional_moment",
        FindingCode::MissingResultant,
        findings,
    )?;
    let radius = require_positive(
        section.torsion_radius,
        "torsion_radius",
        FindingCode::MissingSectionProperty,
        findings,
    )?;
    let torsion_constant = require_positive(
        section.torsion_constant,
        "torsion_constant",
        FindingCode::MissingSectionProperty,
        findings,
    )?;
    Some(torque * radius / torsion_constant)
}

fn pressure_membrane(
    pressure: &PressureBasis,
    findings: &mut Vec<StressFinding>,
) -> (Option<f64>, Option<f64>) {
    let pressure_value = require_finite(
        pressure.pressure,
        "pressure",
        FindingCode::MissingPressureInput,
        findings,
    );
    let radius = require_positive(
        pressure.membrane_radius,
        "membrane_radius",
        FindingCode::MissingPressureInput,
        findings,
    );
    let wall = require_positive(
        pressure.wall_thickness,
        "wall_thickness",
        FindingCode::MissingPressureInput,
        findings,
    );

    match (pressure_value, radius, wall) {
        (Some(p), Some(r), Some(t)) => {
            let hoop = p * r / t;
            let longitudinal = hoop / 2.0;
            (Some(hoop), Some(longitudinal))
        }
        _ => (None, None),
    }
}

fn require_finite(
    value: Option<f64>,
    subject: &'static str,
    missing_code: FindingCode,
    findings: &mut Vec<StressFinding>,
) -> Option<f64> {
    let Some(value) = value else {
        findings.push(StressFinding::new(
            missing_code,
            subject,
            format!("{subject} is required for this recovery component"),
        ));
        return None;
    };
    if !value.is_finite() {
        findings.push(StressFinding::new(
            FindingCode::NonFiniteInput,
            subject,
            format!("{subject} must be finite"),
        ));
        return None;
    }
    Some(value)
}

fn require_positive(
    value: Option<f64>,
    subject: &'static str,
    missing_code: FindingCode,
    findings: &mut Vec<StressFinding>,
) -> Option<f64> {
    let value = require_finite(value, subject, missing_code, findings)?;
    if value <= 0.0 {
        findings.push(StressFinding::new(
            FindingCode::NonPositiveInput,
            subject,
            format!("{subject} must be positive"),
        ));
        return None;
    }
    Some(value)
}

fn summarize_components(components: &StressComponents) -> StressSummary {
    let axial = components.axial_normal.unwrap_or(0.0);
    let pressure_longitudinal = components.pressure_longitudinal.unwrap_or(0.0);
    let bending_y = components.bending_normal_y.unwrap_or(0.0).abs();
    let bending_z = components.bending_normal_z.unwrap_or(0.0).abs();
    let base_normal = axial + pressure_longitudinal;
    let bending_total = bending_y + bending_z;

    StressSummary {
        max_normal: base_normal + bending_total,
        min_normal: base_normal - bending_total,
        max_shear_magnitude: components.torsional_shear.unwrap_or(0.0).abs(),
    }
}

fn collect_statuses(
    source_statuses: &[AnalysisStatus],
    findings: &mut Vec<StressFinding>,
) -> Vec<AnalysisStatus> {
    let mut statuses = Vec::new();
    if source_statuses.is_empty() {
        findings.push(StressFinding::new(
            FindingCode::IncompleteMechanicsStatus,
            "analysis_status",
            "stress recovery requires explicit mechanics status",
        ));
    }

    for status in source_statuses {
        if !automatic_status_allowed(*status) {
            findings.push(StressFinding::new(
                FindingCode::StatusBoundaryViolation,
                "analysis_status",
                "stress recovery cannot emit human approval or compliance status",
            ));
            continue;
        }
        if !statuses.contains(status) {
            statuses.push(*status);
        }
    }

    if !statuses.contains(&AnalysisStatus::MechanicsSolved) {
        findings.push(StressFinding::new(
            FindingCode::IncompleteMechanicsStatus,
            "analysis_status",
            "stress recovery requires mechanics-solved input",
        ));
    }
    if !statuses.contains(&AnalysisStatus::HumanReviewRequired) {
        statuses.push(AnalysisStatus::HumanReviewRequired);
    }
    statuses
}

fn automatic_status_allowed(status: AnalysisStatus) -> bool {
    matches!(
        status,
        AnalysisStatus::ModelIncomplete
            | AnalysisStatus::MechanicsSolved
            | AnalysisStatus::RuleInputsIncomplete
            | AnalysisStatus::UserRuleChecked
            | AnalysisStatus::UserRuleFailed
            | AnalysisStatus::HumanReviewRequired
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn complete_input() -> StressRecoveryInput {
        StressRecoveryInput {
            resultants: ForceResultants::new(Some(120.0), Some(50.0), Some(-30.0), Some(40.0)),
            section: StressSectionProperties::new(
                Some(12.0),
                Some(25.0),
                Some(15.0),
                Some(80.0),
                Some(2.0),
            ),
            pressure: Some(PressureBasis::new(Some(100.0), Some(3.0), Some(0.5))),
            statuses: vec![AnalysisStatus::MechanicsSolved],
        }
    }

    #[test]
    fn recovers_axial_bending_torsion_and_pressure_components() {
        let result = recover_stresses(&complete_input());

        assert!(!result.is_blocked());
        assert_eq!(result.components.axial_normal, Some(10.0));
        assert_eq!(result.components.bending_normal_y, Some(2.0));
        assert_eq!(result.components.bending_normal_z, Some(-2.0));
        assert_eq!(result.components.torsional_shear, Some(1.0));
        assert_eq!(result.components.pressure_hoop, Some(600.0));
        assert_eq!(result.components.pressure_longitudinal, Some(300.0));
    }

    #[test]
    fn summarizes_extreme_normal_and_shear_without_allowables() {
        let result = recover_stresses(&complete_input());
        let summary = result.summary.unwrap();

        assert_eq!(summary.max_normal, 314.0);
        assert_eq!(summary.min_normal, 306.0);
        assert_eq!(summary.max_shear_magnitude, 1.0);
    }

    #[test]
    fn missing_resultant_blocks_recovery() {
        let mut input = complete_input();
        input.resultants.axial_force = None;

        let result = recover_stresses(&input);

        assert!(result.is_blocked());
        assert_eq!(result.components.axial_normal, None);
        assert_eq!(result.summary, None);
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::MissingResultant));
    }

    #[test]
    fn non_positive_section_property_blocks_recovery() {
        let mut input = complete_input();
        input.section.area = Some(0.0);

        let result = recover_stresses(&input);

        assert!(result.is_blocked());
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::NonPositiveInput));
    }

    #[test]
    fn non_finite_pressure_is_reported() {
        let mut input = complete_input();
        input.pressure = Some(PressureBasis::new(Some(f64::NAN), Some(3.0), Some(0.5)));

        let result = recover_stresses(&input);

        assert!(result.is_blocked());
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::NonFiniteInput));
    }

    #[test]
    fn incomplete_status_blocks_mechanics_recovery() {
        let mut input = complete_input();
        input.statuses = vec![AnalysisStatus::ModelIncomplete];

        let result = recover_stresses(&input);

        assert!(result.is_blocked());
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::IncompleteMechanicsStatus));
        assert!(result
            .statuses
            .contains(&AnalysisStatus::HumanReviewRequired));
    }

    #[test]
    fn human_approval_status_is_not_accepted_as_software_output() {
        let mut input = complete_input();
        input.statuses = vec![
            AnalysisStatus::MechanicsSolved,
            AnalysisStatus::HumanApprovedForProject,
        ];

        let result = recover_stresses(&input);

        assert!(result.is_blocked());
        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::StatusBoundaryViolation));
        assert!(!result
            .statuses
            .contains(&AnalysisStatus::HumanApprovedForProject));
    }

    #[test]
    fn boundary_constructor_rejects_invalid_positive_values() {
        let err = checked_positive("area", f64::INFINITY).unwrap_err();
        assert!(matches!(
            err,
            StressRecoveryError::NonFiniteInput { name: "area", .. }
        ));

        let err = checked_positive("area", -1.0).unwrap_err();
        assert!(matches!(
            err,
            StressRecoveryError::NonPositiveInput { name: "area", .. }
        ));
    }
}
