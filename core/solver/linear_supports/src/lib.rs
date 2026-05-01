//! Linear support and restraint mechanics-boundary models.
//!
//! This crate prepares explicit linear support data for the frame-kernel DOF
//! boundary. It does not implement nonlinear support behavior, code checks,
//! protected standards content, or professional approval.

use open_pipe_stress_frame_kernel::{DOF_PER_NODE, RX, RY, RZ, UX, UY, UZ};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrameDof {
    Ux,
    Uy,
    Uz,
    Rx,
    Ry,
    Rz,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportFamily {
    Anchor,
    Guide,
    LineStop,
    VerticalSupport,
    Spring,
    ImposedDisplacement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantityDimension {
    TranslationalStiffness,
    RotationalStiffness,
    Displacement,
    Rotation,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SupportQuantity {
    pub value: f64,
    pub dimension: QuantityDimension,
}

impl SupportQuantity {
    pub fn new(value: f64, dimension: QuantityDimension) -> Result<Self, LinearSupportError> {
        validate_finite("support quantity", value)?;
        Ok(Self { value, dimension })
    }

    pub fn positive(value: f64, dimension: QuantityDimension) -> Result<Self, LinearSupportError> {
        validate_positive_finite("support quantity", value)?;
        Ok(Self { value, dimension })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeDof {
    pub node_index: usize,
    pub dof: FrameDof,
}

impl NodeDof {
    pub fn new(node_index: usize, dof: FrameDof) -> Self {
        Self { node_index, dof }
    }

    pub fn global_index(&self) -> usize {
        self.node_index * DOF_PER_NODE + self.dof.local_index()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LinearSupport {
    pub support_id: String,
    pub family: SupportFamily,
    pub node_index: usize,
    pub restrained_dofs: Vec<FrameDof>,
    pub stiffness: Option<SupportQuantity>,
    pub imposed_displacement: Option<SupportQuantity>,
}

impl LinearSupport {
    pub fn anchor(support_id: impl Into<String>, node_index: usize) -> Self {
        Self {
            support_id: support_id.into(),
            family: SupportFamily::Anchor,
            node_index,
            restrained_dofs: vec![
                FrameDof::Ux,
                FrameDof::Uy,
                FrameDof::Uz,
                FrameDof::Rx,
                FrameDof::Ry,
                FrameDof::Rz,
            ],
            stiffness: None,
            imposed_displacement: None,
        }
    }

    pub fn guide(
        support_id: impl Into<String>,
        node_index: usize,
        restrained_translations: Vec<FrameDof>,
    ) -> Self {
        Self {
            support_id: support_id.into(),
            family: SupportFamily::Guide,
            node_index,
            restrained_dofs: restrained_translations,
            stiffness: None,
            imposed_displacement: None,
        }
    }

    pub fn line_stop(support_id: impl Into<String>, node_index: usize, dof: FrameDof) -> Self {
        Self {
            support_id: support_id.into(),
            family: SupportFamily::LineStop,
            node_index,
            restrained_dofs: vec![dof],
            stiffness: None,
            imposed_displacement: None,
        }
    }

    pub fn vertical_support(support_id: impl Into<String>, node_index: usize) -> Self {
        Self {
            support_id: support_id.into(),
            family: SupportFamily::VerticalSupport,
            node_index,
            restrained_dofs: vec![FrameDof::Uz],
            stiffness: None,
            imposed_displacement: None,
        }
    }

    pub fn spring(
        support_id: impl Into<String>,
        node_index: usize,
        dof: FrameDof,
        stiffness: Option<SupportQuantity>,
    ) -> Self {
        Self {
            support_id: support_id.into(),
            family: SupportFamily::Spring,
            node_index,
            restrained_dofs: vec![dof],
            stiffness,
            imposed_displacement: None,
        }
    }

    pub fn imposed_displacement(
        support_id: impl Into<String>,
        node_index: usize,
        dof: FrameDof,
        displacement: Option<SupportQuantity>,
    ) -> Self {
        Self {
            support_id: support_id.into(),
            family: SupportFamily::ImposedDisplacement,
            node_index,
            restrained_dofs: vec![dof],
            stiffness: None,
            imposed_displacement: displacement,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpringEntry {
    pub support_id: String,
    pub node_dof: NodeDof,
    pub stiffness: SupportQuantity,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImposedDisplacementEntry {
    pub support_id: String,
    pub node_dof: NodeDof,
    pub displacement: SupportQuantity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingCode {
    MissingSupportDof,
    MissingSupportStiffness,
    MissingImposedDisplacement,
    InvalidSupportDof,
    InvalidSupportDimension,
    RepeatedRestrainedDof,
    NodeOutOfRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SupportFinding {
    pub code: FindingCode,
    pub support_id: String,
    pub message: String,
}

impl SupportFinding {
    fn new(code: FindingCode, support_id: &str, message: impl Into<String>) -> Self {
        Self {
            code,
            support_id: support_id.to_string(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoundaryPreparation {
    pub restrained_dofs: Vec<usize>,
    pub springs: Vec<SpringEntry>,
    pub imposed_displacements: Vec<ImposedDisplacementEntry>,
    pub findings: Vec<SupportFinding>,
}

impl BoundaryPreparation {
    pub fn is_blocked(&self) -> bool {
        !self.findings.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LinearSupportError {
    NonFiniteInput { name: &'static str, value: f64 },
    NonPositiveInput { name: &'static str, value: f64 },
}

impl fmt::Display for LinearSupportError {
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

impl Error for LinearSupportError {}

pub fn prepare_boundary(node_count: usize, supports: &[LinearSupport]) -> BoundaryPreparation {
    let mut restrained_dofs = Vec::new();
    let mut springs = Vec::new();
    let mut imposed_displacements = Vec::new();
    let mut findings = Vec::new();

    for support in supports {
        if support.node_index >= node_count {
            findings.push(SupportFinding::new(
                FindingCode::NodeOutOfRange,
                &support.support_id,
                format!(
                    "node index {} is outside node count {}",
                    support.node_index, node_count
                ),
            ));
            continue;
        }

        match support.family {
            SupportFamily::Anchor
            | SupportFamily::Guide
            | SupportFamily::LineStop
            | SupportFamily::VerticalSupport => {
                prepare_rigid_support(support, &mut restrained_dofs, &mut findings)
            }
            SupportFamily::Spring => {
                prepare_spring(support, &mut springs, &mut findings);
            }
            SupportFamily::ImposedDisplacement => prepare_imposed_displacement(
                support,
                &mut restrained_dofs,
                &mut imposed_displacements,
                &mut findings,
            ),
        }
    }

    restrained_dofs.sort_unstable();
    BoundaryPreparation {
        restrained_dofs,
        springs,
        imposed_displacements,
        findings,
    }
}

fn prepare_rigid_support(
    support: &LinearSupport,
    restrained_dofs: &mut Vec<usize>,
    findings: &mut Vec<SupportFinding>,
) {
    if support.restrained_dofs.is_empty() {
        findings.push(SupportFinding::new(
            FindingCode::MissingSupportDof,
            &support.support_id,
            "linear restraint support requires at least one affected DOF",
        ));
        return;
    }

    for &dof in &support.restrained_dofs {
        if !dof_allowed_for_family(support.family, dof) {
            findings.push(SupportFinding::new(
                FindingCode::InvalidSupportDof,
                &support.support_id,
                format!("{:?} is not valid for {:?}", dof, support.family),
            ));
            continue;
        }
        add_restrained_dof(support, dof, restrained_dofs, findings);
    }
}

fn prepare_spring(
    support: &LinearSupport,
    springs: &mut Vec<SpringEntry>,
    findings: &mut Vec<SupportFinding>,
) {
    let Some(&dof) = support.restrained_dofs.first() else {
        findings.push(SupportFinding::new(
            FindingCode::MissingSupportDof,
            &support.support_id,
            "spring support requires one affected DOF",
        ));
        return;
    };

    let Some(stiffness) = support.stiffness else {
        findings.push(SupportFinding::new(
            FindingCode::MissingSupportStiffness,
            &support.support_id,
            "spring support requires explicit stiffness",
        ));
        return;
    };

    if !dimension_matches_dof(stiffness.dimension, dof, true) {
        findings.push(SupportFinding::new(
            FindingCode::InvalidSupportDimension,
            &support.support_id,
            "spring stiffness dimension must match translational or rotational DOF",
        ));
        return;
    }

    springs.push(SpringEntry {
        support_id: support.support_id.clone(),
        node_dof: NodeDof::new(support.node_index, dof),
        stiffness,
    });
}

fn prepare_imposed_displacement(
    support: &LinearSupport,
    restrained_dofs: &mut Vec<usize>,
    imposed_displacements: &mut Vec<ImposedDisplacementEntry>,
    findings: &mut Vec<SupportFinding>,
) {
    let Some(&dof) = support.restrained_dofs.first() else {
        findings.push(SupportFinding::new(
            FindingCode::MissingSupportDof,
            &support.support_id,
            "imposed displacement requires one affected DOF",
        ));
        return;
    };

    let Some(displacement) = support.imposed_displacement else {
        findings.push(SupportFinding::new(
            FindingCode::MissingImposedDisplacement,
            &support.support_id,
            "imposed displacement requires explicit displacement or rotation",
        ));
        return;
    };

    if !dimension_matches_dof(displacement.dimension, dof, false) {
        findings.push(SupportFinding::new(
            FindingCode::InvalidSupportDimension,
            &support.support_id,
            "imposed value dimension must match translational or rotational DOF",
        ));
        return;
    }

    add_restrained_dof(support, dof, restrained_dofs, findings);
    imposed_displacements.push(ImposedDisplacementEntry {
        support_id: support.support_id.clone(),
        node_dof: NodeDof::new(support.node_index, dof),
        displacement,
    });
}

fn add_restrained_dof(
    support: &LinearSupport,
    dof: FrameDof,
    restrained_dofs: &mut Vec<usize>,
    findings: &mut Vec<SupportFinding>,
) {
    let global_dof = NodeDof::new(support.node_index, dof).global_index();
    if restrained_dofs.contains(&global_dof) {
        findings.push(SupportFinding::new(
            FindingCode::RepeatedRestrainedDof,
            &support.support_id,
            format!("restrained DOF {global_dof} is repeated"),
        ));
        return;
    }
    restrained_dofs.push(global_dof);
}

fn dof_allowed_for_family(family: SupportFamily, dof: FrameDof) -> bool {
    match family {
        SupportFamily::Anchor => true,
        SupportFamily::Guide => dof.is_translational(),
        SupportFamily::LineStop => dof.is_translational(),
        SupportFamily::VerticalSupport => dof == FrameDof::Uz,
        SupportFamily::Spring | SupportFamily::ImposedDisplacement => true,
    }
}

fn dimension_matches_dof(dimension: QuantityDimension, dof: FrameDof, stiffness: bool) -> bool {
    match (stiffness, dof.is_translational(), dimension) {
        (true, true, QuantityDimension::TranslationalStiffness) => true,
        (true, false, QuantityDimension::RotationalStiffness) => true,
        (false, true, QuantityDimension::Displacement) => true,
        (false, false, QuantityDimension::Rotation) => true,
        _ => false,
    }
}

fn validate_finite(name: &'static str, value: f64) -> Result<(), LinearSupportError> {
    if !value.is_finite() {
        return Err(LinearSupportError::NonFiniteInput { name, value });
    }
    Ok(())
}

fn validate_positive_finite(name: &'static str, value: f64) -> Result<(), LinearSupportError> {
    validate_finite(name, value)?;
    if value <= 0.0 {
        return Err(LinearSupportError::NonPositiveInput { name, value });
    }
    Ok(())
}

impl FrameDof {
    pub fn local_index(self) -> usize {
        match self {
            Self::Ux => UX,
            Self::Uy => UY,
            Self::Uz => UZ,
            Self::Rx => RX,
            Self::Ry => RY,
            Self::Rz => RZ,
        }
    }

    pub fn is_translational(self) -> bool {
        matches!(self, Self::Ux | Self::Uy | Self::Uz)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchor_maps_all_six_node_dofs() {
        let support = LinearSupport::anchor("anchor-1", 1);
        let prepared = prepare_boundary(2, &[support]);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.restrained_dofs, vec![6, 7, 8, 9, 10, 11]);
        assert!(prepared.springs.is_empty());
    }

    #[test]
    fn support_families_map_expected_linear_dofs() {
        let supports = vec![
            LinearSupport::guide("guide-1", 0, vec![FrameDof::Uy, FrameDof::Uz]),
            LinearSupport::line_stop("stop-1", 1, FrameDof::Ux),
            LinearSupport::vertical_support("vertical-1", 2),
        ];
        let prepared = prepare_boundary(3, &supports);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.restrained_dofs, vec![1, 2, 6, 14]);
    }

    #[test]
    fn invalid_family_dof_produces_finding_not_default() {
        let support = LinearSupport::guide("guide-1", 0, vec![FrameDof::Ry]);
        let prepared = prepare_boundary(1, &[support]);

        assert!(prepared.is_blocked());
        assert_eq!(prepared.findings[0].code, FindingCode::InvalidSupportDof);
        assert!(prepared.restrained_dofs.is_empty());
    }

    #[test]
    fn spring_requires_explicit_matching_stiffness() {
        let missing = LinearSupport::spring("spring-1", 0, FrameDof::Ux, None);
        let prepared = prepare_boundary(1, &[missing]);

        assert_eq!(
            prepared.findings[0].code,
            FindingCode::MissingSupportStiffness
        );

        let stiffness =
            SupportQuantity::positive(1200.0, QuantityDimension::TranslationalStiffness).unwrap();
        let spring = LinearSupport::spring("spring-2", 0, FrameDof::Ux, Some(stiffness));
        let prepared = prepare_boundary(1, &[spring]);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.springs[0].node_dof.global_index(), 0);
        assert_eq!(prepared.springs[0].stiffness, stiffness);
    }

    #[test]
    fn rotational_spring_requires_rotational_stiffness_dimension() {
        let stiffness =
            SupportQuantity::positive(1200.0, QuantityDimension::TranslationalStiffness).unwrap();
        let support = LinearSupport::spring("spring-1", 0, FrameDof::Rz, Some(stiffness));
        let prepared = prepare_boundary(1, &[support]);

        assert_eq!(
            prepared.findings[0].code,
            FindingCode::InvalidSupportDimension
        );
    }

    #[test]
    fn imposed_displacement_restrains_dof_and_records_value() {
        let value = SupportQuantity::new(0.003, QuantityDimension::Displacement).unwrap();
        let support =
            LinearSupport::imposed_displacement("settlement-1", 0, FrameDof::Uz, Some(value));
        let prepared = prepare_boundary(1, &[support]);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.restrained_dofs, vec![2]);
        assert_eq!(prepared.imposed_displacements[0].displacement, value);
    }

    #[test]
    fn repeated_restrained_dof_is_reported() {
        let supports = vec![
            LinearSupport::vertical_support("vertical-1", 0),
            LinearSupport::line_stop("stop-1", 0, FrameDof::Uz),
        ];
        let prepared = prepare_boundary(1, &supports);

        assert!(prepared.is_blocked());
        assert_eq!(
            prepared.findings[0].code,
            FindingCode::RepeatedRestrainedDof
        );
        assert_eq!(prepared.restrained_dofs, vec![2]);
    }

    #[test]
    fn node_out_of_range_is_reported() {
        let support = LinearSupport::anchor("anchor-1", 2);
        let prepared = prepare_boundary(2, &[support]);

        assert!(prepared.is_blocked());
        assert_eq!(prepared.findings[0].code, FindingCode::NodeOutOfRange);
        assert!(prepared.restrained_dofs.is_empty());
    }
}
