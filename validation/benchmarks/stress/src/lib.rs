//! Original stress recovery verification benchmarks for OpenPipeStress.
//!
//! The fixtures in this crate use elementary open mechanics with invented
//! numeric values. They are verification aids only: no code-specific stress
//! equations, protected standards content, allowables, fatigue acceptance
//! criteria, or professional approval claims are encoded here.

use open_pipe_stress_stress_recovery::{
    recover_stresses, AnalysisStatus, ForceResultants, PressureBasis, StressComponents,
    StressRecoveryInput, StressRecoveryResult, StressSectionProperties,
};

#[cfg(test)]
const INTERNAL_ASSERTION_EPSILON: f64 = 1.0e-9;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StressBenchmarkFamily {
    AxialNormal,
    BendingNormal,
    TorsionalShear,
    PressureMembrane,
    StressRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedistributionStatus {
    PublicOriginal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewDisposition {
    AcceptedInventedFixture,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BenchmarkProvenance {
    pub source_name: &'static str,
    pub source_location: &'static str,
    pub source_license: &'static str,
    pub contributor: &'static str,
    pub contributor_certification: &'static str,
    pub redistribution_status: RedistributionStatus,
    pub review_disposition: ReviewDisposition,
}

impl BenchmarkProvenance {
    pub fn public_original(source_location: &'static str) -> Self {
        Self {
            source_name: "OpenPipeStress original stress recovery benchmark",
            source_location,
            source_license: "project-original-public-content",
            contributor: "OpenPipeStress agentic development workflow",
            contributor_certification:
                "Generated from elementary open mechanics; not copied from protected standards, code formulas, commercial software examples, or proprietary data.",
            redistribution_status: RedistributionStatus::PublicOriginal,
            review_disposition: ReviewDisposition::AcceptedInventedFixture,
        }
    }

    pub fn is_publicly_usable(&self) -> bool {
        self.redistribution_status == RedistributionStatus::PublicOriginal
            && self.review_disposition == ReviewDisposition::AcceptedInventedFixture
            && !self.source_name.is_empty()
            && !self.source_location.is_empty()
            && !self.source_license.is_empty()
            && self
                .contributor_certification
                .contains("not copied from protected standards")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpectedValue {
    pub name: &'static str,
    pub value: f64,
    pub dimension: &'static str,
    pub tolerance_policy: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StressBenchmark {
    pub fixture_id: &'static str,
    pub family: StressBenchmarkFamily,
    pub description: &'static str,
    pub assumptions: &'static [&'static str],
    pub provenance: BenchmarkProvenance,
    pub expected_values: Vec<ExpectedValue>,
}

impl StressBenchmark {
    pub fn tolerance_policy_is_unresolved(&self) -> bool {
        self.expected_values
            .iter()
            .all(|value| value.tolerance_policy.is_none())
    }

    pub fn has_dimensioned_expected_values(&self) -> bool {
        self.expected_values
            .iter()
            .all(|value| value.value.is_finite() && !value.dimension.is_empty())
    }
}

pub fn fixture_inventory() -> Vec<StressBenchmark> {
    vec![
        axial_normal_fixture(),
        bending_normal_fixture(),
        torsional_shear_fixture(),
        pressure_membrane_fixture(),
        stress_range_fixture(),
    ]
}

pub fn missing_required_families(fixtures: &[StressBenchmark]) -> Vec<StressBenchmarkFamily> {
    let required = [
        StressBenchmarkFamily::AxialNormal,
        StressBenchmarkFamily::BendingNormal,
        StressBenchmarkFamily::TorsionalShear,
        StressBenchmarkFamily::PressureMembrane,
        StressBenchmarkFamily::StressRange,
    ];

    required
        .into_iter()
        .filter(|family| !fixtures.iter().any(|fixture| fixture.family == *family))
        .collect()
}

pub fn axial_normal_fixture() -> StressBenchmark {
    StressBenchmark {
        fixture_id: "STRESS-AXIAL-NORMAL-ORIGINAL",
        family: StressBenchmarkFamily::AxialNormal,
        description: "Invented axial resultant divided by invented section area.",
        assumptions: &[
            "Positive axial force is tensile in this fixture.",
            "Area is supplied explicitly by a governed section-property boundary.",
            "The expected value is a mechanics stress component, not an allowable comparison.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/stress/axial_normal.md",
        ),
        expected_values: vec![ExpectedValue {
            name: "axial_normal",
            value: 120.0 / 12.0,
            dimension: "force_per_area",
            tolerance_policy: None,
        }],
    }
}

pub fn bending_normal_fixture() -> StressBenchmark {
    StressBenchmark {
        fixture_id: "STRESS-BENDING-NORMAL-ORIGINAL",
        family: StressBenchmarkFamily::BendingNormal,
        description: "Invented bending moments divided by invented section moduli.",
        assumptions: &[
            "Section moduli are supplied explicitly and remain positive.",
            "The signs of bending components follow the current stress-recovery API inputs.",
            "The fixture does not encode a code stress category or stress index.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/stress/bending_normal.md",
        ),
        expected_values: vec![
            ExpectedValue {
                name: "bending_normal_y",
                value: 50.0 / 25.0,
                dimension: "force_per_area",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "bending_normal_z",
                value: -30.0 / 15.0,
                dimension: "force_per_area",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn torsional_shear_fixture() -> StressBenchmark {
    StressBenchmark {
        fixture_id: "STRESS-TORSIONAL-SHEAR-ORIGINAL",
        family: StressBenchmarkFamily::TorsionalShear,
        description: "Invented torque times radius divided by invented torsion constant.",
        assumptions: &[
            "Torsion radius and torsion constant are supplied explicitly.",
            "The expected value is a shear stress component.",
            "The fixture does not encode a code allowable or fatigue criterion.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/stress/torsional_shear.md",
        ),
        expected_values: vec![ExpectedValue {
            name: "torsional_shear",
            value: 40.0 * 2.0 / 80.0,
            dimension: "force_per_area",
            tolerance_policy: None,
        }],
    }
}

pub fn pressure_membrane_fixture() -> StressBenchmark {
    StressBenchmark {
        fixture_id: "STRESS-PRESSURE-MEMBRANE-ORIGINAL",
        family: StressBenchmarkFamily::PressureMembrane,
        description: "Invented thin-wall pressure membrane components from explicit pressure basis inputs.",
        assumptions: &[
            "Pressure, membrane radius, and wall thickness are explicit fixture inputs.",
            "Hoop and longitudinal membrane components follow the upstream stress-recovery mechanics boundary.",
            "The fixture does not provide pressure design criteria or code equations.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/stress/pressure_membrane.md",
        ),
        expected_values: vec![
            ExpectedValue {
                name: "pressure_hoop",
                value: 100.0 * 3.0 / 0.5,
                dimension: "force_per_area",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "pressure_longitudinal",
                value: (100.0 * 3.0 / 0.5) / 2.0,
                dimension: "force_per_area",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn stress_range_fixture() -> StressBenchmark {
    StressBenchmark {
        fixture_id: "STRESS-RANGE-MECHANICS-ORIGINAL",
        family: StressBenchmarkFamily::StressRange,
        description: "Invented mechanics-only range between two recovered stress states.",
        assumptions: &[
            "Stress range is computed as absolute component difference between two mechanics states.",
            "The fixture is not a fatigue assessment, allowable comparison, or code compliance check.",
            "The same section properties are used for both invented states.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/stress/stress_range.md",
        ),
        expected_values: vec![
            ExpectedValue {
                name: "axial_normal_range",
                value: (180.0_f64 / 12.0 - 60.0 / 12.0).abs(),
                dimension: "force_per_area",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "bending_normal_y_range",
                value: (80.0_f64 / 25.0 - (-20.0 / 25.0)).abs(),
                dimension: "force_per_area",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "torsional_shear_range",
                value: (60.0_f64 * 2.0 / 80.0 - 20.0 * 2.0 / 80.0).abs(),
                dimension: "force_per_area",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn complete_stress_input() -> StressRecoveryInput {
    StressRecoveryInput {
        resultants: ForceResultants::new(Some(120.0), Some(50.0), Some(-30.0), Some(40.0)),
        section: benchmark_section(),
        pressure: Some(PressureBasis::new(Some(100.0), Some(3.0), Some(0.5))),
        statuses: vec![AnalysisStatus::MechanicsSolved],
    }
}

pub fn benchmark_section() -> StressSectionProperties {
    StressSectionProperties::new(Some(12.0), Some(25.0), Some(15.0), Some(80.0), Some(2.0))
}

pub fn recover_complete_fixture() -> StressRecoveryResult {
    recover_stresses(&complete_stress_input())
}

pub fn recover_range_start() -> StressRecoveryResult {
    let input = StressRecoveryInput {
        resultants: ForceResultants::new(Some(60.0), Some(-20.0), Some(10.0), Some(20.0)),
        section: benchmark_section(),
        pressure: None,
        statuses: vec![AnalysisStatus::MechanicsSolved],
    };
    recover_stresses(&input)
}

pub fn recover_range_end() -> StressRecoveryResult {
    let input = StressRecoveryInput {
        resultants: ForceResultants::new(Some(180.0), Some(80.0), Some(10.0), Some(60.0)),
        section: benchmark_section(),
        pressure: None,
        statuses: vec![AnalysisStatus::MechanicsSolved],
    };
    recover_stresses(&input)
}

pub fn component_range(start: &StressComponents, end: &StressComponents) -> StressComponents {
    StressComponents {
        axial_normal: range_optional(start.axial_normal, end.axial_normal),
        bending_normal_y: range_optional(start.bending_normal_y, end.bending_normal_y),
        bending_normal_z: range_optional(start.bending_normal_z, end.bending_normal_z),
        torsional_shear: range_optional(start.torsional_shear, end.torsional_shear),
        pressure_hoop: range_optional(start.pressure_hoop, end.pressure_hoop),
        pressure_longitudinal: range_optional(
            start.pressure_longitudinal,
            end.pressure_longitudinal,
        ),
    }
}

fn range_optional(start: Option<f64>, end: Option<f64>) -> Option<f64> {
    match (start, end) {
        (Some(a), Some(b)) => Some((b - a).abs()),
        _ => None,
    }
}

#[cfg(test)]
fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= INTERNAL_ASSERTION_EPSILON,
        "expected {expected}, got {actual}"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_covers_required_stress_families() {
        let fixtures = fixture_inventory();
        assert!(missing_required_families(&fixtures).is_empty());
        assert_eq!(fixtures.len(), 5);
    }

    #[test]
    fn fixture_provenance_is_public_original_and_reviewed() {
        for fixture in fixture_inventory() {
            assert!(
                fixture.provenance.is_publicly_usable(),
                "{} lacks accepted public-original provenance",
                fixture.fixture_id
            );
            assert!(fixture.tolerance_policy_is_unresolved());
            assert!(fixture.has_dimensioned_expected_values());
        }
    }

    #[test]
    fn recovers_axial_normal_fixture() {
        let fixture = axial_normal_fixture();
        let result = recover_complete_fixture();

        assert!(!result.is_blocked());
        assert_close(
            result.components.axial_normal.unwrap(),
            fixture.expected_values[0].value,
        );
    }

    #[test]
    fn recovers_bending_normal_fixture() {
        let fixture = bending_normal_fixture();
        let result = recover_complete_fixture();

        assert!(!result.is_blocked());
        assert_close(
            result.components.bending_normal_y.unwrap(),
            fixture.expected_values[0].value,
        );
        assert_close(
            result.components.bending_normal_z.unwrap(),
            fixture.expected_values[1].value,
        );
    }

    #[test]
    fn recovers_torsional_shear_fixture() {
        let fixture = torsional_shear_fixture();
        let result = recover_complete_fixture();

        assert!(!result.is_blocked());
        assert_close(
            result.components.torsional_shear.unwrap(),
            fixture.expected_values[0].value,
        );
    }

    #[test]
    fn recovers_pressure_membrane_fixture() {
        let fixture = pressure_membrane_fixture();
        let result = recover_complete_fixture();

        assert!(!result.is_blocked());
        assert_close(
            result.components.pressure_hoop.unwrap(),
            fixture.expected_values[0].value,
        );
        assert_close(
            result.components.pressure_longitudinal.unwrap(),
            fixture.expected_values[1].value,
        );
    }

    #[test]
    fn computes_mechanics_only_stress_range_fixture() {
        let fixture = stress_range_fixture();
        let start = recover_range_start();
        let end = recover_range_end();
        let range = component_range(&start.components, &end.components);

        assert!(!start.is_blocked());
        assert!(!end.is_blocked());
        assert_close(
            range.axial_normal.unwrap(),
            fixture.expected_values[0].value,
        );
        assert_close(
            range.bending_normal_y.unwrap(),
            fixture.expected_values[1].value,
        );
        assert_close(
            range.torsional_shear.unwrap(),
            fixture.expected_values[2].value,
        );
    }

    #[test]
    fn recovered_results_preserve_human_review_boundary() {
        let result = recover_complete_fixture();

        assert!(result.statuses.contains(&AnalysisStatus::MechanicsSolved));
        assert!(result
            .statuses
            .contains(&AnalysisStatus::HumanReviewRequired));
        assert!(!result
            .statuses
            .contains(&AnalysisStatus::HumanApprovedForProject));
        assert!(result.summary.is_some());
    }
}
