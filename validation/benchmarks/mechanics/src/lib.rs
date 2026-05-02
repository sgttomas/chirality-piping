//! Original mechanics verification benchmarks for OpenPipeStress.
//!
//! The fixtures in this crate use elementary open mechanics with invented
//! numeric values. They are verification aids only: no code-specific
//! acceptance criteria, protected standards content, or professional approval
//! claims are encoded here.

use open_pipe_stress_frame_kernel::{
    assemble_global_stiffness, reduce_system, solve_dense, FrameElement, FrameKernelError,
    FrameNode, FrameSection, DOF_PER_NODE, RX, RY, RZ, UX, UY, UZ,
};
use open_pipe_stress_linear_supports::{
    prepare_boundary, FrameDof, LinearSupport, QuantityDimension, SupportQuantity,
};
use open_pipe_stress_primitive_loads::{
    prepare_loads, LoadDimension, LoadDirection, LoadQuantity, PrimitiveLoad, PrimitiveLoadCategory,
};
use open_pipe_stress_straight_pipe::{StraightPipeElement, StraightPipeSectionProperties};

const INTERNAL_ASSERTION_EPSILON: f64 = 1.0e-9;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BenchmarkFamily {
    Cantilever,
    Frame,
    ThermalGrowth,
    ImposedDisplacement,
    StiffnessTransform,
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
            source_name: "OpenPipeStress original mechanics benchmark",
            source_location,
            source_license: "project-original-public-content",
            contributor: "OpenPipeStress agentic development workflow",
            contributor_certification:
                "Generated from elementary open mechanics; not copied from protected standards, commercial software examples, or proprietary data.",
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
pub struct MechanicsBenchmark {
    pub fixture_id: &'static str,
    pub family: BenchmarkFamily,
    pub description: &'static str,
    pub assumptions: &'static [&'static str],
    pub provenance: BenchmarkProvenance,
    pub expected_values: Vec<ExpectedValue>,
}

impl MechanicsBenchmark {
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

pub fn fixture_inventory() -> Vec<MechanicsBenchmark> {
    vec![
        cantilever_tip_force_fixture(),
        portal_frame_sway_fixture(),
        fixed_fixed_thermal_fixture(),
        imposed_displacement_spring_fixture(),
        inclined_member_transform_fixture(),
    ]
}

pub fn missing_required_families(fixtures: &[MechanicsBenchmark]) -> Vec<BenchmarkFamily> {
    let required = [
        BenchmarkFamily::Cantilever,
        BenchmarkFamily::Frame,
        BenchmarkFamily::ThermalGrowth,
        BenchmarkFamily::ImposedDisplacement,
        BenchmarkFamily::StiffnessTransform,
    ];

    required
        .into_iter()
        .filter(|family| !fixtures.iter().any(|fixture| fixture.family == *family))
        .collect()
}

pub fn cantilever_tip_force_fixture() -> MechanicsBenchmark {
    let length: f64 = 10.0;
    let elastic_modulus: f64 = 1200.0;
    let second_moment_z: f64 = 4.0;
    let tip_force: f64 = 6.0;
    let tip_displacement_y = tip_force * length.powi(3) / (3.0 * elastic_modulus * second_moment_z);
    let fixed_end_moment_z = tip_force * length;

    MechanicsBenchmark {
        fixture_id: "MECH-CANTILEVER-TIP-FORCE",
        family: BenchmarkFamily::Cantilever,
        description:
            "Two-node cantilever with invented lateral tip force in the local y/global Y direction.",
        assumptions: &[
            "Euler-Bernoulli frame stiffness as implemented by the frame kernel.",
            "Node 0 restrained in all six degrees of freedom.",
            "Node 1 free in all six degrees of freedom.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/cantilever_tip_force.md",
        ),
        expected_values: vec![
            ExpectedValue {
                name: "tip_displacement_y",
                value: tip_displacement_y,
                dimension: "length",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "fixed_end_moment_z",
                value: fixed_end_moment_z,
                dimension: "force_length",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn portal_frame_sway_fixture() -> MechanicsBenchmark {
    MechanicsBenchmark {
        fixture_id: "MECH-PORTAL-SWAY-ORIGINAL",
        family: BenchmarkFamily::Frame,
        description: "Original two-column portal-frame assembly used as a deterministic frame assembly smoke benchmark.",
        assumptions: &[
            "Three frame elements are assembled into one global stiffness matrix.",
            "Base nodes are restrained in all six degrees of freedom.",
            "A lateral nodal force is applied at the upper-right joint.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/portal_frame_sway.md",
        ),
        expected_values: vec![ExpectedValue {
            name: "top_right_sway_x",
            value: solve_portal_frame_sway().expect("fixture construction must remain valid"),
            dimension: "length",
            tolerance_policy: None,
        }],
    }
}

pub fn fixed_fixed_thermal_fixture() -> MechanicsBenchmark {
    let elastic_modulus = 2000.0;
    let area = 3.0;
    let alpha = 1.2e-5;
    let delta_temperature = 75.0;
    let restrained_force = elastic_modulus * area * alpha * delta_temperature;

    MechanicsBenchmark {
        fixture_id: "MECH-FIXED-FIXED-THERMAL-AXIAL",
        family: BenchmarkFamily::ThermalGrowth,
        description: "Fixed-fixed axial member with invented thermal strain and fully restrained free expansion.",
        assumptions: &[
            "Uniform temperature change over a prismatic member.",
            "Both axial ends are restrained.",
            "Expected axial force magnitude follows open mechanics EA alpha delta_T.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/fixed_fixed_thermal_axial.md",
        ),
        expected_values: vec![
            ExpectedValue {
                name: "free_thermal_strain",
                value: alpha * delta_temperature,
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "restrained_axial_force_magnitude",
                value: restrained_force,
                dimension: "force",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn imposed_displacement_spring_fixture() -> MechanicsBenchmark {
    let stiffness = 150.0;
    let imposed_displacement = 0.04;
    let reaction = stiffness * imposed_displacement;

    MechanicsBenchmark {
        fixture_id: "MECH-IMPOSED-DISPLACEMENT-SPRING",
        family: BenchmarkFamily::ImposedDisplacement,
        description: "Single translational spring with an invented imposed displacement.",
        assumptions: &[
            "Linear spring reaction follows k times imposed displacement.",
            "The support model records the imposed displacement without solving a full frame system.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/imposed_displacement_spring.md",
        ),
        expected_values: vec![ExpectedValue {
            name: "spring_reaction_force",
            value: reaction,
            dimension: "force",
            tolerance_policy: None,
        }],
    }
}

pub fn inclined_member_transform_fixture() -> MechanicsBenchmark {
    let direction_cosine = 1.0 / 2.0_f64.sqrt();

    MechanicsBenchmark {
        fixture_id: "MECH-INCLINED-MEMBER-TRANSFORM",
        family: BenchmarkFamily::StiffnessTransform,
        description: "Inclined two-node frame member with 45-degree global XY projection.",
        assumptions: &[
            "Local x axis is the normalized node-to-node vector.",
            "Global stiffness transform preserves matrix symmetry.",
        ],
        provenance: BenchmarkProvenance::public_original(
            "validation/hand_calcs/mechanics/inclined_member_transform.md",
        ),
        expected_values: vec![
            ExpectedValue {
                name: "local_x_global_x_component",
                value: direction_cosine,
                dimension: "dimensionless",
                tolerance_policy: None,
            },
            ExpectedValue {
                name: "local_x_global_y_component",
                value: direction_cosine,
                dimension: "dimensionless",
                tolerance_policy: None,
            },
        ],
    }
}

pub fn solve_cantilever_tip_force() -> Result<f64, FrameKernelError> {
    let section = benchmark_section()?;
    let element = FrameElement::new(
        FrameNode::new(0, [0.0, 0.0, 0.0])?,
        FrameNode::new(1, [10.0, 0.0, 0.0])?,
        section,
        [0.0, 1.0, 0.0],
    )?;
    let stiffness = assemble_global_stiffness(2, &[element])?;
    let mut force = vec![0.0; 2 * DOF_PER_NODE];
    force[DOF_PER_NODE + UY] = 6.0;
    let reduced = reduce_system(&stiffness, &force, &[UX, UY, UZ, RX, RY, RZ])?;
    let displacement = solve_dense(&reduced.stiffness, &reduced.force)?;
    let reduced_index = reduced
        .free_dofs
        .iter()
        .position(|&dof| dof == DOF_PER_NODE + UY)
        .expect("tip UY is free in this fixture");
    Ok(displacement[reduced_index])
}

pub fn solve_portal_frame_sway() -> Result<f64, FrameKernelError> {
    let section = FrameSection::new(1800.0, 700.0, 2.5, 2.0, 2.0, 1.0)?;
    let nodes = [
        FrameNode::new(0, [0.0, 0.0, 0.0])?,
        FrameNode::new(1, [0.0, 4.0, 0.0])?,
        FrameNode::new(2, [6.0, 4.0, 0.0])?,
        FrameNode::new(3, [6.0, 0.0, 0.0])?,
    ];
    let elements = [
        FrameElement::new(nodes[0], nodes[1], section, [1.0, 0.0, 0.0])?,
        FrameElement::new(nodes[1], nodes[2], section, [0.0, 1.0, 0.0])?,
        FrameElement::new(nodes[3], nodes[2], section, [1.0, 0.0, 0.0])?,
    ];
    let stiffness = assemble_global_stiffness(nodes.len(), &elements)?;
    let load = PrimitiveLoad::nodal_force(
        "portal-lateral-load",
        PrimitiveLoadCategory::Occasional,
        2,
        LoadDirection::GlobalX,
        LoadQuantity::new(3.0, LoadDimension::Force).expect("fixture load is finite"),
    );
    let load_application = prepare_loads(nodes.len(), elements.len(), &[load]);
    assert!(
        !load_application.is_blocked(),
        "portal frame fixture load preparation should not have findings"
    );
    let force = load_application.global_load_vector(nodes.len());
    let base_restraints = all_node_dofs(0)
        .into_iter()
        .chain(all_node_dofs(3))
        .collect::<Vec<_>>();
    let reduced = reduce_system(&stiffness, &force, &base_restraints)?;
    let displacement = solve_dense(&reduced.stiffness, &reduced.force)?;
    let reduced_index = reduced
        .free_dofs
        .iter()
        .position(|&dof| dof == 2 * DOF_PER_NODE + UX)
        .expect("top-right UX is free in this fixture");
    Ok(displacement[reduced_index])
}

pub fn validate_imposed_displacement_fixture() -> bool {
    let stiffness = SupportQuantity::positive(150.0, QuantityDimension::TranslationalStiffness)
        .expect("fixture stiffness is positive");
    let imposed = SupportQuantity::new(0.04, QuantityDimension::Displacement)
        .expect("fixture displacement is finite");
    let supports = [
        LinearSupport::spring("fixture-spring", 0, FrameDof::Ux, Some(stiffness)),
        LinearSupport::imposed_displacement("fixture-imposed", 0, FrameDof::Ux, Some(imposed)),
    ];
    let preparation = prepare_boundary(1, &supports);
    !preparation.is_blocked()
        && preparation.springs.len() == 1
        && preparation.imposed_displacements.len() == 1
        && (stiffness.value * imposed.value
            - imposed_displacement_spring_fixture().expected_values[0].value)
            .abs()
            <= INTERNAL_ASSERTION_EPSILON
}

pub fn validate_straight_pipe_boundary() -> bool {
    let section = StraightPipeSectionProperties::new(1200.0, 500.0, 2.0, 3.0, 4.0, 1.5, None)
        .expect("fixture section is valid");
    let pipe = StraightPipeElement::new(
        "straight-pipe-cantilever",
        FrameNode::new(0, [0.0, 0.0, 0.0]).expect("fixture node is valid"),
        FrameNode::new(1, [10.0, 0.0, 0.0]).expect("fixture node is valid"),
        section,
        [0.0, 1.0, 0.0],
    )
    .expect("fixture pipe is valid");
    pipe.local_stiffness().is_ok() && pipe.global_stiffness().is_ok()
}

pub fn validate_transform_fixture() -> Result<bool, FrameKernelError> {
    let section = benchmark_section()?;
    let element = FrameElement::new(
        FrameNode::new(0, [0.0, 0.0, 0.0])?,
        FrameNode::new(1, [1.0, 1.0, 0.0])?,
        section,
        [0.0, 0.0, 1.0],
    )?;
    let orientation = element.orientation()?;
    let global = element.global_stiffness()?;
    let expected = 1.0 / 2.0_f64.sqrt();
    Ok(
        (orientation.local_axes[0][0] - expected).abs() <= INTERNAL_ASSERTION_EPSILON
            && (orientation.local_axes[0][1] - expected).abs() <= INTERNAL_ASSERTION_EPSILON
            && matrix_is_symmetric(&global),
    )
}

fn benchmark_section() -> Result<FrameSection, FrameKernelError> {
    FrameSection::new(1200.0, 500.0, 2.0, 3.0, 4.0, 1.5)
}

fn all_node_dofs(node_index: usize) -> Vec<usize> {
    let base = node_index * DOF_PER_NODE;
    vec![
        base + UX,
        base + UY,
        base + UZ,
        base + RX,
        base + RY,
        base + RZ,
    ]
}

fn matrix_is_symmetric(matrix: &[[f64; 12]; 12]) -> bool {
    for row in 0..12 {
        for col in 0..12 {
            if (matrix[row][col] - matrix[col][row]).abs() > INTERNAL_ASSERTION_EPSILON {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_covers_required_mechanics_families() {
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
    fn cantilever_tip_force_matches_open_mechanics_formula() {
        let solved_tip_displacement = solve_cantilever_tip_force().unwrap();
        let fixture = cantilever_tip_force_fixture();
        let expected = fixture.expected_values[0].value;
        assert!((solved_tip_displacement - expected).abs() <= INTERNAL_ASSERTION_EPSILON);
        assert!(validate_straight_pipe_boundary());
    }

    #[test]
    fn portal_frame_fixture_solves_repeatably() {
        let fixture = portal_frame_sway_fixture();
        let solved = solve_portal_frame_sway().unwrap();
        assert!((solved - fixture.expected_values[0].value).abs() <= INTERNAL_ASSERTION_EPSILON);
        assert!(solved.is_finite());
    }

    #[test]
    fn thermal_growth_fixture_records_open_axial_restraint_formula() {
        let fixture = fixed_fixed_thermal_fixture();
        let strain = fixture.expected_values[0].value;
        let force = fixture.expected_values[1].value;
        assert!((strain - 0.0009).abs() <= INTERNAL_ASSERTION_EPSILON);
        assert!((force - 5.4).abs() <= INTERNAL_ASSERTION_EPSILON);
    }

    #[test]
    fn imposed_displacement_fixture_prepares_support_boundary() {
        assert!(validate_imposed_displacement_fixture());
    }

    #[test]
    fn inclined_member_transform_fixture_checks_direction_cosines_and_symmetry() {
        let fixture = inclined_member_transform_fixture();
        assert!(validate_transform_fixture().unwrap());
        assert!(
            (fixture.expected_values[0].value - fixture.expected_values[1].value).abs()
                <= INTERNAL_ASSERTION_EPSILON
        );
    }
}
