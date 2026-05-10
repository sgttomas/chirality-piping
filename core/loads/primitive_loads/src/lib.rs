//! Code-neutral primitive load case mechanics.
//!
//! This crate prepares explicit primitive loads for solver-boundary consumers.
//! It does not encode design-code load combinations, protected standards
//! content, proprietary project data, rule-pack checks, or professional
//! approval.

use open_pipe_stress_frame_kernel::{DOF_PER_NODE, UX, UY, UZ};
use open_pipe_stress_linear_supports::{FrameDof, NodeDof};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveLoadCategory {
    Weight,
    Pressure,
    Thermal,
    ImposedDisplacement,
    Hydrotest,
    Wind,
    Seismic,
    Occasional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadTarget {
    Node(usize),
    Element(usize),
    Support { node_index: usize, dof: FrameDof },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadDimension {
    Force,
    Moment,
    ForcePerLength,
    Pressure,
    TemperatureChange,
    Acceleration,
    Displacement,
    Rotation,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoadQuantity {
    pub value: f64,
    pub dimension: LoadDimension,
}

impl LoadQuantity {
    pub fn new(value: f64, dimension: LoadDimension) -> Result<Self, PrimitiveLoadError> {
        validate_finite("load quantity", value)?;
        Ok(Self { value, dimension })
    }

    pub fn positive(value: f64, dimension: LoadDimension) -> Result<Self, PrimitiveLoadError> {
        validate_positive_finite("load quantity", value)?;
        Ok(Self { value, dimension })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadDirection {
    Dof(FrameDof),
    GlobalX,
    GlobalY,
    GlobalZ,
}

impl LoadDirection {
    pub fn dof_index(self) -> usize {
        match self {
            Self::Dof(dof) => dof.local_index(),
            Self::GlobalX => UX,
            Self::GlobalY => UY,
            Self::GlobalZ => UZ,
        }
    }

    pub fn is_rotational(self) -> bool {
        matches!(self, Self::Dof(FrameDof::Rx | FrameDof::Ry | FrameDof::Rz))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrimitiveLoad {
    pub load_id: String,
    pub category: PrimitiveLoadCategory,
    pub target: Option<LoadTarget>,
    pub direction: LoadDirection,
    pub magnitude: Option<LoadQuantity>,
}

impl PrimitiveLoad {
    pub fn nodal_force(
        load_id: impl Into<String>,
        category: PrimitiveLoadCategory,
        node_index: usize,
        direction: LoadDirection,
        magnitude: LoadQuantity,
    ) -> Self {
        Self {
            load_id: load_id.into(),
            category,
            target: Some(LoadTarget::Node(node_index)),
            direction,
            magnitude: Some(magnitude),
        }
    }

    pub fn uniform_element_load(
        load_id: impl Into<String>,
        category: PrimitiveLoadCategory,
        element_index: usize,
        direction: LoadDirection,
        magnitude: LoadQuantity,
    ) -> Self {
        Self {
            load_id: load_id.into(),
            category,
            target: Some(LoadTarget::Element(element_index)),
            direction,
            magnitude: Some(magnitude),
        }
    }

    pub fn imposed_displacement(
        load_id: impl Into<String>,
        node_index: usize,
        dof: FrameDof,
        magnitude: LoadQuantity,
    ) -> Self {
        Self {
            load_id: load_id.into(),
            category: PrimitiveLoadCategory::ImposedDisplacement,
            target: Some(LoadTarget::Support { node_index, dof }),
            direction: LoadDirection::Dof(dof),
            magnitude: Some(magnitude),
        }
    }

    pub fn missing_target(
        load_id: impl Into<String>,
        category: PrimitiveLoadCategory,
        direction: LoadDirection,
        magnitude: Option<LoadQuantity>,
    ) -> Self {
        Self {
            load_id: load_id.into(),
            category,
            target: None,
            direction,
            magnitude,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodalLoadContribution {
    pub load_id: String,
    pub node_index: usize,
    pub global_dof: usize,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElementUniformLoadContribution {
    pub load_id: String,
    pub element_index: usize,
    pub direction: LoadDirection,
    pub magnitude: LoadQuantity,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImposedDisplacementContribution {
    pub load_id: String,
    pub node_dof: NodeDof,
    pub value: LoadQuantity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingCode {
    MissingLoadTarget,
    MissingLoadMagnitude,
    NodeOutOfRange,
    ElementOutOfRange,
    InvalidLoadDimension,
    InvalidLoadDirection,
    UnsupportedTargetForCategory,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadFinding {
    pub code: FindingCode,
    pub load_id: String,
    pub message: String,
}

impl LoadFinding {
    fn new(code: FindingCode, load_id: &str, message: impl Into<String>) -> Self {
        Self {
            code,
            load_id: load_id.to_string(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoadApplication {
    pub nodal_loads: Vec<NodalLoadContribution>,
    pub element_uniform_loads: Vec<ElementUniformLoadContribution>,
    pub imposed_displacements: Vec<ImposedDisplacementContribution>,
    pub findings: Vec<LoadFinding>,
}

impl LoadApplication {
    pub fn is_blocked(&self) -> bool {
        !self.findings.is_empty()
    }

    pub fn global_load_vector(&self, node_count: usize) -> Vec<f64> {
        let mut vector = vec![0.0; node_count * DOF_PER_NODE];
        for load in &self.nodal_loads {
            if load.global_dof < vector.len() {
                vector[load.global_dof] += load.value;
            }
        }
        vector
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveLoadError {
    NonFiniteInput { name: &'static str, value: f64 },
    NonPositiveInput { name: &'static str, value: f64 },
}

impl fmt::Display for PrimitiveLoadError {
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

impl Error for PrimitiveLoadError {}

pub fn prepare_loads(
    node_count: usize,
    element_count: usize,
    loads: &[PrimitiveLoad],
) -> LoadApplication {
    let mut nodal_loads = Vec::new();
    let mut element_uniform_loads = Vec::new();
    let mut imposed_displacements = Vec::new();
    let mut findings = Vec::new();

    for load in loads {
        let Some(target) = load.target else {
            findings.push(LoadFinding::new(
                FindingCode::MissingLoadTarget,
                &load.load_id,
                "primitive load requires an explicit target",
            ));
            continue;
        };
        let Some(magnitude) = load.magnitude else {
            findings.push(LoadFinding::new(
                FindingCode::MissingLoadMagnitude,
                &load.load_id,
                "primitive load requires an explicit magnitude",
            ));
            continue;
        };

        match target {
            LoadTarget::Node(node_index) => prepare_node_load(
                load,
                node_index,
                magnitude,
                node_count,
                &mut nodal_loads,
                &mut findings,
            ),
            LoadTarget::Element(element_index) => prepare_element_load(
                load,
                element_index,
                magnitude,
                element_count,
                &mut element_uniform_loads,
                &mut findings,
            ),
            LoadTarget::Support { node_index, dof } => prepare_support_load(
                load,
                node_index,
                dof,
                magnitude,
                node_count,
                &mut imposed_displacements,
                &mut findings,
            ),
        }
    }

    nodal_loads.sort_by(|a, b| {
        a.node_index
            .cmp(&b.node_index)
            .then(a.global_dof.cmp(&b.global_dof))
            .then(a.load_id.cmp(&b.load_id))
    });
    element_uniform_loads.sort_by(|a, b| {
        a.element_index
            .cmp(&b.element_index)
            .then(a.direction.dof_index().cmp(&b.direction.dof_index()))
            .then(a.load_id.cmp(&b.load_id))
    });
    imposed_displacements.sort_by(|a, b| {
        a.node_dof
            .global_index()
            .cmp(&b.node_dof.global_index())
            .then(a.load_id.cmp(&b.load_id))
    });

    LoadApplication {
        nodal_loads,
        element_uniform_loads,
        imposed_displacements,
        findings,
    }
}

fn prepare_node_load(
    load: &PrimitiveLoad,
    node_index: usize,
    magnitude: LoadQuantity,
    node_count: usize,
    nodal_loads: &mut Vec<NodalLoadContribution>,
    findings: &mut Vec<LoadFinding>,
) {
    if node_index >= node_count {
        findings.push(LoadFinding::new(
            FindingCode::NodeOutOfRange,
            &load.load_id,
            format!("node index {node_index} is outside node count {node_count}"),
        ));
        return;
    }

    if !category_allows_node_target(load.category) {
        findings.push(LoadFinding::new(
            FindingCode::UnsupportedTargetForCategory,
            &load.load_id,
            format!("{:?} is not a nodal primitive load target", load.category),
        ));
        return;
    }

    if !valid_nodal_dimension(load.direction, magnitude.dimension) {
        findings.push(LoadFinding::new(
            FindingCode::InvalidLoadDimension,
            &load.load_id,
            "nodal load dimension must match translational force or rotational moment",
        ));
        return;
    }

    nodal_loads.push(NodalLoadContribution {
        load_id: load.load_id.clone(),
        node_index,
        global_dof: node_index * DOF_PER_NODE + load.direction.dof_index(),
        value: magnitude.value,
    });
}

fn prepare_element_load(
    load: &PrimitiveLoad,
    element_index: usize,
    magnitude: LoadQuantity,
    element_count: usize,
    element_uniform_loads: &mut Vec<ElementUniformLoadContribution>,
    findings: &mut Vec<LoadFinding>,
) {
    if element_index >= element_count {
        findings.push(LoadFinding::new(
            FindingCode::ElementOutOfRange,
            &load.load_id,
            format!("element index {element_index} is outside element count {element_count}"),
        ));
        return;
    }

    if !category_allows_element_target(load.category) {
        findings.push(LoadFinding::new(
            FindingCode::UnsupportedTargetForCategory,
            &load.load_id,
            format!(
                "{:?} is not an element primitive load target",
                load.category
            ),
        ));
        return;
    }

    if load.category != PrimitiveLoadCategory::Thermal && load.direction.is_rotational() {
        findings.push(LoadFinding::new(
            FindingCode::InvalidLoadDirection,
            &load.load_id,
            "element uniform primitive loads require a translational direction",
        ));
        return;
    }

    if !valid_element_dimension(load.category, magnitude.dimension) {
        findings.push(LoadFinding::new(
            FindingCode::InvalidLoadDimension,
            &load.load_id,
            "element primitive load dimension is not valid for the category",
        ));
        return;
    }

    element_uniform_loads.push(ElementUniformLoadContribution {
        load_id: load.load_id.clone(),
        element_index,
        direction: load.direction,
        magnitude,
    });
}

fn prepare_support_load(
    load: &PrimitiveLoad,
    node_index: usize,
    dof: FrameDof,
    magnitude: LoadQuantity,
    node_count: usize,
    imposed_displacements: &mut Vec<ImposedDisplacementContribution>,
    findings: &mut Vec<LoadFinding>,
) {
    if node_index >= node_count {
        findings.push(LoadFinding::new(
            FindingCode::NodeOutOfRange,
            &load.load_id,
            format!("node index {node_index} is outside node count {node_count}"),
        ));
        return;
    }

    if load.category != PrimitiveLoadCategory::ImposedDisplacement {
        findings.push(LoadFinding::new(
            FindingCode::UnsupportedTargetForCategory,
            &load.load_id,
            "support-target primitive loads are limited to imposed displacement",
        ));
        return;
    }

    if !valid_imposed_dimension(dof, magnitude.dimension) {
        findings.push(LoadFinding::new(
            FindingCode::InvalidLoadDimension,
            &load.load_id,
            "imposed displacement dimension must match translational or rotational DOF",
        ));
        return;
    }

    imposed_displacements.push(ImposedDisplacementContribution {
        load_id: load.load_id.clone(),
        node_dof: NodeDof::new(node_index, dof),
        value: magnitude,
    });
}

fn category_allows_node_target(category: PrimitiveLoadCategory) -> bool {
    matches!(
        category,
        PrimitiveLoadCategory::Wind
            | PrimitiveLoadCategory::Seismic
            | PrimitiveLoadCategory::Occasional
    )
}

fn category_allows_element_target(category: PrimitiveLoadCategory) -> bool {
    matches!(
        category,
        PrimitiveLoadCategory::Weight
            | PrimitiveLoadCategory::Pressure
            | PrimitiveLoadCategory::Thermal
            | PrimitiveLoadCategory::Hydrotest
            | PrimitiveLoadCategory::Wind
            | PrimitiveLoadCategory::Seismic
            | PrimitiveLoadCategory::Occasional
    )
}

fn valid_nodal_dimension(direction: LoadDirection, dimension: LoadDimension) -> bool {
    match (direction.is_rotational(), dimension) {
        (true, LoadDimension::Moment) => true,
        (false, LoadDimension::Force) => true,
        _ => false,
    }
}

fn valid_element_dimension(category: PrimitiveLoadCategory, dimension: LoadDimension) -> bool {
    match category {
        PrimitiveLoadCategory::Weight
        | PrimitiveLoadCategory::Wind
        | PrimitiveLoadCategory::Seismic
        | PrimitiveLoadCategory::Occasional => dimension == LoadDimension::ForcePerLength,
        PrimitiveLoadCategory::Pressure | PrimitiveLoadCategory::Hydrotest => {
            matches!(
                dimension,
                LoadDimension::Pressure | LoadDimension::ForcePerLength
            )
        }
        PrimitiveLoadCategory::Thermal => dimension == LoadDimension::TemperatureChange,
        PrimitiveLoadCategory::ImposedDisplacement => false,
    }
}

fn valid_imposed_dimension(dof: FrameDof, dimension: LoadDimension) -> bool {
    match (dof.is_translational(), dimension) {
        (true, LoadDimension::Displacement) => true,
        (false, LoadDimension::Rotation) => true,
        _ => false,
    }
}

fn validate_finite(name: &'static str, value: f64) -> Result<(), PrimitiveLoadError> {
    if !value.is_finite() {
        return Err(PrimitiveLoadError::NonFiniteInput { name, value });
    }
    Ok(())
}

fn validate_positive_finite(name: &'static str, value: f64) -> Result<(), PrimitiveLoadError> {
    validate_finite(name, value)?;
    if value <= 0.0 {
        return Err(PrimitiveLoadError::NonPositiveInput { name, value });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_pipe_stress_frame_kernel::RZ;

    fn q(value: f64, dimension: LoadDimension) -> LoadQuantity {
        LoadQuantity::new(value, dimension).unwrap()
    }

    #[test]
    fn weight_load_prepares_element_force_per_length() {
        let load = PrimitiveLoad::uniform_element_load(
            "weight-1",
            PrimitiveLoadCategory::Weight,
            0,
            LoadDirection::GlobalZ,
            q(-245.25, LoadDimension::ForcePerLength),
        );

        let prepared = prepare_loads(2, 1, &[load]);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.element_uniform_loads.len(), 1);
        assert_eq!(prepared.element_uniform_loads[0].magnitude.value, -245.25);
        assert_eq!(
            prepared.element_uniform_loads[0].magnitude.dimension,
            LoadDimension::ForcePerLength
        );
    }

    #[test]
    fn pressure_and_hydrotest_accept_pressure_or_equivalent_line_load() {
        let pressure = PrimitiveLoad::uniform_element_load(
            "pressure-1",
            PrimitiveLoadCategory::Pressure,
            0,
            LoadDirection::GlobalX,
            q(1000.0, LoadDimension::Pressure),
        );
        let hydrotest = PrimitiveLoad::uniform_element_load(
            "hydrotest-1",
            PrimitiveLoadCategory::Hydrotest,
            0,
            LoadDirection::GlobalZ,
            q(320.0, LoadDimension::ForcePerLength),
        );

        let prepared = prepare_loads(2, 1, &[pressure, hydrotest]);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.element_uniform_loads.len(), 2);
        assert_eq!(prepared.element_uniform_loads[0].load_id, "pressure-1");
        assert_eq!(
            prepared.element_uniform_loads[0].magnitude.dimension,
            LoadDimension::Pressure
        );
        assert_eq!(prepared.element_uniform_loads[1].load_id, "hydrotest-1");
    }

    #[test]
    fn thermal_load_keeps_explicit_temperature_change_boundary() {
        let load = PrimitiveLoad::uniform_element_load(
            "thermal-1",
            PrimitiveLoadCategory::Thermal,
            0,
            LoadDirection::Dof(FrameDof::Rz),
            q(55.0, LoadDimension::TemperatureChange),
        );

        let prepared = prepare_loads(2, 1, &[load]);

        assert!(!prepared.is_blocked());
        assert_eq!(
            prepared.element_uniform_loads[0].direction,
            LoadDirection::Dof(FrameDof::Rz)
        );
        assert_eq!(prepared.element_uniform_loads[0].magnitude.value, 55.0);
        assert_eq!(
            prepared.element_uniform_loads[0].magnitude.dimension,
            LoadDimension::TemperatureChange
        );
    }

    #[test]
    fn thermal_load_rejects_force_per_length_fallthrough() {
        let load = PrimitiveLoad::uniform_element_load(
            "thermal-line-load",
            PrimitiveLoadCategory::Thermal,
            0,
            LoadDirection::GlobalX,
            q(12.0, LoadDimension::ForcePerLength),
        );

        let prepared = prepare_loads(2, 1, &[load]);

        assert!(prepared.is_blocked());
        assert_eq!(prepared.findings[0].code, FindingCode::InvalidLoadDimension);
    }

    #[test]
    fn wind_seismic_and_occasional_can_prepare_nodal_loads() {
        let loads = vec![
            PrimitiveLoad::nodal_force(
                "wind-1",
                PrimitiveLoadCategory::Wind,
                0,
                LoadDirection::GlobalY,
                q(12.0, LoadDimension::Force),
            ),
            PrimitiveLoad::nodal_force(
                "seismic-1",
                PrimitiveLoadCategory::Seismic,
                0,
                LoadDirection::GlobalX,
                q(4.0, LoadDimension::Force),
            ),
            PrimitiveLoad::nodal_force(
                "occasional-1",
                PrimitiveLoadCategory::Occasional,
                1,
                LoadDirection::Dof(FrameDof::Rz),
                q(2.5, LoadDimension::Moment),
            ),
        ];

        let prepared = prepare_loads(2, 1, &loads);
        let vector = prepared.global_load_vector(2);

        assert!(!prepared.is_blocked());
        assert_eq!(vector[UX], 4.0);
        assert_eq!(vector[UY], 12.0);
        assert_eq!(vector[DOF_PER_NODE + RZ], 2.5);
    }

    #[test]
    fn imposed_displacement_preserves_support_dof_boundary() {
        let load = PrimitiveLoad::imposed_displacement(
            "settlement-1",
            1,
            FrameDof::Uz,
            q(-0.006, LoadDimension::Displacement),
        );

        let prepared = prepare_loads(2, 1, &[load]);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.imposed_displacements[0].node_dof.global_index(), 8);
        assert_eq!(prepared.imposed_displacements[0].value.value, -0.006);
    }

    #[test]
    fn missing_target_and_magnitude_are_findings() {
        let loads = vec![
            PrimitiveLoad::missing_target(
                "missing-target",
                PrimitiveLoadCategory::Weight,
                LoadDirection::GlobalZ,
                Some(q(-1.0, LoadDimension::ForcePerLength)),
            ),
            PrimitiveLoad {
                load_id: "missing-magnitude".to_string(),
                category: PrimitiveLoadCategory::Wind,
                target: Some(LoadTarget::Node(0)),
                direction: LoadDirection::GlobalY,
                magnitude: None,
            },
        ];

        let prepared = prepare_loads(1, 1, &loads);

        assert!(prepared.is_blocked());
        assert_eq!(prepared.findings[0].code, FindingCode::MissingLoadTarget);
        assert_eq!(prepared.findings[1].code, FindingCode::MissingLoadMagnitude);
    }

    #[test]
    fn invalid_targets_and_dimensions_are_findings() {
        let loads = vec![
            PrimitiveLoad::nodal_force(
                "weight-node",
                PrimitiveLoadCategory::Weight,
                0,
                LoadDirection::GlobalZ,
                q(-1.0, LoadDimension::Force),
            ),
            PrimitiveLoad::uniform_element_load(
                "bad-element",
                PrimitiveLoadCategory::Wind,
                2,
                LoadDirection::GlobalY,
                q(1.0, LoadDimension::ForcePerLength),
            ),
            PrimitiveLoad::imposed_displacement(
                "bad-imposed",
                0,
                FrameDof::Rx,
                q(0.1, LoadDimension::Displacement),
            ),
        ];

        let prepared = prepare_loads(1, 1, &loads);

        assert!(prepared.is_blocked());
        assert_eq!(
            prepared.findings[0].code,
            FindingCode::UnsupportedTargetForCategory
        );
        assert_eq!(prepared.findings[1].code, FindingCode::ElementOutOfRange);
        assert_eq!(prepared.findings[2].code, FindingCode::InvalidLoadDimension);
    }

    #[test]
    fn nonfinite_and_nonpositive_quantities_are_rejected_at_construction() {
        match LoadQuantity::new(f64::NAN, LoadDimension::Force).unwrap_err() {
            PrimitiveLoadError::NonFiniteInput { name, value } => {
                assert_eq!(name, "load quantity");
                assert!(value.is_nan());
            }
            other => panic!("expected non-finite input error, got {other:?}"),
        }
        assert_eq!(
            LoadQuantity::positive(0.0, LoadDimension::Force).unwrap_err(),
            PrimitiveLoadError::NonPositiveInput {
                name: "load quantity",
                value: 0.0
            }
        );
    }

    #[test]
    fn deterministic_sorting_stabilizes_application_outputs() {
        let loads = vec![
            PrimitiveLoad::nodal_force(
                "node-2",
                PrimitiveLoadCategory::Wind,
                1,
                LoadDirection::GlobalY,
                q(2.0, LoadDimension::Force),
            ),
            PrimitiveLoad::nodal_force(
                "node-1",
                PrimitiveLoadCategory::Wind,
                0,
                LoadDirection::GlobalX,
                q(1.0, LoadDimension::Force),
            ),
        ];

        let prepared = prepare_loads(2, 1, &loads);

        assert_eq!(prepared.nodal_loads[0].load_id, "node-1");
        assert_eq!(prepared.nodal_loads[1].load_id, "node-2");
    }
}
