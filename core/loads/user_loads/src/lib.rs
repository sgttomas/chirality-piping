//! Code-neutral concentrated and distributed user load application.
//!
//! This crate prepares explicit user loads for solver-boundary consumers. It
//! does not encode design-code load combinations, public default factors,
//! protected standards content, proprietary project data, rule-pack checks, or
//! professional approval.

use open_pipe_stress_frame_kernel::{DOF_PER_NODE, RX, RY, RZ, UX, UY, UZ};
use open_pipe_stress_primitive_loads::LoadDimension;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserLoadKind {
    ConcentratedForce,
    ConcentratedMoment,
    UniformDistributedLoad,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserLoadDirection {
    GlobalX,
    GlobalY,
    GlobalZ,
    RotationX,
    RotationY,
    RotationZ,
}

impl UserLoadDirection {
    pub fn dof_index(self) -> usize {
        match self {
            Self::GlobalX => UX,
            Self::GlobalY => UY,
            Self::GlobalZ => UZ,
            Self::RotationX => RX,
            Self::RotationY => RY,
            Self::RotationZ => RZ,
        }
    }

    pub fn is_rotational(self) -> bool {
        matches!(self, Self::RotationX | Self::RotationY | Self::RotationZ)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UserLoadQuantity {
    pub value: f64,
    pub dimension: LoadDimension,
}

impl UserLoadQuantity {
    pub fn new(value: f64, dimension: LoadDimension) -> Result<Self, UserLoadError> {
        validate_finite("user load quantity", value)?;
        Ok(Self { value, dimension })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserLoadTarget {
    Node(usize),
    Element {
        element_index: usize,
        span: ElementLoadSpan,
        element_length: Option<f64>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElementLoadSpan {
    pub start_fraction: f64,
    pub end_fraction: f64,
}

impl ElementLoadSpan {
    pub fn full() -> Self {
        Self {
            start_fraction: 0.0,
            end_fraction: 1.0,
        }
    }

    pub fn new(start_fraction: f64, end_fraction: f64) -> Result<Self, UserLoadError> {
        validate_finite("span start fraction", start_fraction)?;
        validate_finite("span end fraction", end_fraction)?;
        Ok(Self {
            start_fraction,
            end_fraction,
        })
    }

    pub fn length_fraction(&self) -> f64 {
        self.end_fraction - self.start_fraction
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserLoad {
    pub load_id: String,
    pub kind: UserLoadKind,
    pub target: Option<UserLoadTarget>,
    pub direction: UserLoadDirection,
    pub quantity: Option<UserLoadQuantity>,
    pub provenance_ref: Option<String>,
}

impl UserLoad {
    pub fn concentrated_force(
        load_id: impl Into<String>,
        node_index: usize,
        direction: UserLoadDirection,
        quantity: UserLoadQuantity,
    ) -> Self {
        Self {
            load_id: load_id.into(),
            kind: UserLoadKind::ConcentratedForce,
            target: Some(UserLoadTarget::Node(node_index)),
            direction,
            quantity: Some(quantity),
            provenance_ref: None,
        }
    }

    pub fn concentrated_moment(
        load_id: impl Into<String>,
        node_index: usize,
        direction: UserLoadDirection,
        quantity: UserLoadQuantity,
    ) -> Self {
        Self {
            load_id: load_id.into(),
            kind: UserLoadKind::ConcentratedMoment,
            target: Some(UserLoadTarget::Node(node_index)),
            direction,
            quantity: Some(quantity),
            provenance_ref: None,
        }
    }

    pub fn uniform_distributed(
        load_id: impl Into<String>,
        element_index: usize,
        direction: UserLoadDirection,
        quantity: UserLoadQuantity,
        span: ElementLoadSpan,
        element_length: Option<f64>,
    ) -> Self {
        Self {
            load_id: load_id.into(),
            kind: UserLoadKind::UniformDistributedLoad,
            target: Some(UserLoadTarget::Element {
                element_index,
                span,
                element_length,
            }),
            direction,
            quantity: Some(quantity),
            provenance_ref: None,
        }
    }

    pub fn with_provenance_ref(mut self, provenance_ref: impl Into<String>) -> Self {
        self.provenance_ref = Some(provenance_ref.into());
        self
    }

    pub fn missing_target(
        load_id: impl Into<String>,
        kind: UserLoadKind,
        direction: UserLoadDirection,
        quantity: Option<UserLoadQuantity>,
    ) -> Self {
        Self {
            load_id: load_id.into(),
            kind,
            target: None,
            direction,
            quantity,
            provenance_ref: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContributionTrace {
    pub load_id: String,
    pub kind: UserLoadKind,
    pub provenance_ref: Option<String>,
}

impl ContributionTrace {
    fn from_load(load: &UserLoad) -> Self {
        Self {
            load_id: load.load_id.clone(),
            kind: load.kind,
            provenance_ref: load.provenance_ref.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodalLoadContribution {
    pub node_index: usize,
    pub global_dof: usize,
    pub value: f64,
    pub dimension: LoadDimension,
    pub trace: ContributionTrace,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElementDistributedLoadContribution {
    pub element_index: usize,
    pub direction: UserLoadDirection,
    pub magnitude: UserLoadQuantity,
    pub span: ElementLoadSpan,
    pub equivalent_total: Option<f64>,
    pub trace: ContributionTrace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryHookKind {
    NodalForce,
    NodalMoment,
    ElementDistributedLoad,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResultRecoveryHook {
    pub load_id: String,
    pub hook_kind: RecoveryHookKind,
    pub target_ref: String,
    pub dimension: LoadDimension,
    pub provenance_ref: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingCode {
    MissingLoadTarget,
    MissingLoadQuantity,
    NodeOutOfRange,
    ElementOutOfRange,
    InvalidLoadDimension,
    InvalidLoadDirection,
    UnsupportedTargetForLoadKind,
    InvalidDistributionSpan,
    NonPositiveElementLength,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserLoadFinding {
    pub code: FindingCode,
    pub load_id: String,
    pub message: String,
}

impl UserLoadFinding {
    fn new(code: FindingCode, load_id: &str, message: impl Into<String>) -> Self {
        Self {
            code,
            load_id: load_id.to_string(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserLoadApplication {
    pub nodal_loads: Vec<NodalLoadContribution>,
    pub element_distributed_loads: Vec<ElementDistributedLoadContribution>,
    pub recovery_hooks: Vec<ResultRecoveryHook>,
    pub findings: Vec<UserLoadFinding>,
}

impl UserLoadApplication {
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
pub enum UserLoadError {
    NonFiniteInput { name: &'static str, value: f64 },
}

impl fmt::Display for UserLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteInput { name, value } => {
                write!(f, "{name} must be finite, got {value}")
            }
        }
    }
}

impl Error for UserLoadError {}

pub fn apply_user_loads(
    node_count: usize,
    element_count: usize,
    loads: &[UserLoad],
) -> UserLoadApplication {
    let mut nodal_loads = Vec::new();
    let mut element_distributed_loads = Vec::new();
    let mut recovery_hooks = Vec::new();
    let mut findings = Vec::new();

    for load in loads {
        let Some(target) = &load.target else {
            findings.push(UserLoadFinding::new(
                FindingCode::MissingLoadTarget,
                &load.load_id,
                "user load requires an explicit target",
            ));
            continue;
        };
        let Some(quantity) = load.quantity else {
            findings.push(UserLoadFinding::new(
                FindingCode::MissingLoadQuantity,
                &load.load_id,
                "user load requires an explicit quantity",
            ));
            continue;
        };

        match target {
            UserLoadTarget::Node(node_index) => prepare_nodal_user_load(
                load,
                *node_index,
                quantity,
                node_count,
                &mut nodal_loads,
                &mut recovery_hooks,
                &mut findings,
            ),
            UserLoadTarget::Element {
                element_index,
                span,
                element_length,
            } => prepare_element_user_load(
                load,
                *element_index,
                quantity,
                *span,
                *element_length,
                element_count,
                &mut element_distributed_loads,
                &mut recovery_hooks,
                &mut findings,
            ),
        }
    }

    nodal_loads.sort_by(|a, b| {
        a.node_index
            .cmp(&b.node_index)
            .then(a.global_dof.cmp(&b.global_dof))
            .then(a.trace.load_id.cmp(&b.trace.load_id))
    });
    element_distributed_loads.sort_by(|a, b| {
        a.element_index
            .cmp(&b.element_index)
            .then(a.direction.dof_index().cmp(&b.direction.dof_index()))
            .then(a.trace.load_id.cmp(&b.trace.load_id))
    });
    recovery_hooks.sort_by(|a, b| {
        a.target_ref
            .cmp(&b.target_ref)
            .then(a.load_id.cmp(&b.load_id))
    });

    UserLoadApplication {
        nodal_loads,
        element_distributed_loads,
        recovery_hooks,
        findings,
    }
}

fn prepare_nodal_user_load(
    load: &UserLoad,
    node_index: usize,
    quantity: UserLoadQuantity,
    node_count: usize,
    nodal_loads: &mut Vec<NodalLoadContribution>,
    recovery_hooks: &mut Vec<ResultRecoveryHook>,
    findings: &mut Vec<UserLoadFinding>,
) {
    if node_index >= node_count {
        findings.push(UserLoadFinding::new(
            FindingCode::NodeOutOfRange,
            &load.load_id,
            format!("node index {node_index} is outside node count {node_count}"),
        ));
        return;
    }

    let Some(hook_kind) = validate_nodal_kind_and_dimension(load, quantity, findings) else {
        return;
    };

    let global_dof = node_index * DOF_PER_NODE + load.direction.dof_index();
    nodal_loads.push(NodalLoadContribution {
        node_index,
        global_dof,
        value: quantity.value,
        dimension: quantity.dimension,
        trace: ContributionTrace::from_load(load),
    });
    recovery_hooks.push(ResultRecoveryHook {
        load_id: load.load_id.clone(),
        hook_kind,
        target_ref: format!("node:{node_index}:dof:{global_dof}"),
        dimension: quantity.dimension,
        provenance_ref: load.provenance_ref.clone(),
    });
}

fn prepare_element_user_load(
    load: &UserLoad,
    element_index: usize,
    quantity: UserLoadQuantity,
    span: ElementLoadSpan,
    element_length: Option<f64>,
    element_count: usize,
    element_distributed_loads: &mut Vec<ElementDistributedLoadContribution>,
    recovery_hooks: &mut Vec<ResultRecoveryHook>,
    findings: &mut Vec<UserLoadFinding>,
) {
    if element_index >= element_count {
        findings.push(UserLoadFinding::new(
            FindingCode::ElementOutOfRange,
            &load.load_id,
            format!("element index {element_index} is outside element count {element_count}"),
        ));
        return;
    }

    if load.kind != UserLoadKind::UniformDistributedLoad {
        findings.push(UserLoadFinding::new(
            FindingCode::UnsupportedTargetForLoadKind,
            &load.load_id,
            "element targets are limited to distributed user loads",
        ));
        return;
    }
    if load.direction.is_rotational() {
        findings.push(UserLoadFinding::new(
            FindingCode::InvalidLoadDirection,
            &load.load_id,
            "distributed user loads require a translational direction",
        ));
        return;
    }
    if quantity.dimension != LoadDimension::ForcePerLength {
        findings.push(UserLoadFinding::new(
            FindingCode::InvalidLoadDimension,
            &load.load_id,
            "distributed user load dimension must be force per length",
        ));
        return;
    }
    if !(0.0..=1.0).contains(&span.start_fraction)
        || !(0.0..=1.0).contains(&span.end_fraction)
        || span.start_fraction >= span.end_fraction
    {
        findings.push(UserLoadFinding::new(
            FindingCode::InvalidDistributionSpan,
            &load.load_id,
            "distributed user load span must satisfy 0 <= start < end <= 1",
        ));
        return;
    }

    let equivalent_total = match element_length {
        Some(length) if length.is_finite() && length > 0.0 => {
            Some(quantity.value * length * span.length_fraction())
        }
        Some(_) => {
            findings.push(UserLoadFinding::new(
                FindingCode::NonPositiveElementLength,
                &load.load_id,
                "element length for equivalent load must be positive and finite",
            ));
            return;
        }
        None => None,
    };

    element_distributed_loads.push(ElementDistributedLoadContribution {
        element_index,
        direction: load.direction,
        magnitude: quantity,
        span,
        equivalent_total,
        trace: ContributionTrace::from_load(load),
    });
    recovery_hooks.push(ResultRecoveryHook {
        load_id: load.load_id.clone(),
        hook_kind: RecoveryHookKind::ElementDistributedLoad,
        target_ref: format!(
            "element:{element_index}:span:{}-{}",
            span.start_fraction, span.end_fraction
        ),
        dimension: quantity.dimension,
        provenance_ref: load.provenance_ref.clone(),
    });
}

fn validate_nodal_kind_and_dimension(
    load: &UserLoad,
    quantity: UserLoadQuantity,
    findings: &mut Vec<UserLoadFinding>,
) -> Option<RecoveryHookKind> {
    match load.kind {
        UserLoadKind::ConcentratedForce => {
            if load.direction.is_rotational() {
                findings.push(UserLoadFinding::new(
                    FindingCode::InvalidLoadDirection,
                    &load.load_id,
                    "concentrated forces require a translational direction",
                ));
                return None;
            }
            if quantity.dimension != LoadDimension::Force {
                findings.push(UserLoadFinding::new(
                    FindingCode::InvalidLoadDimension,
                    &load.load_id,
                    "concentrated force dimension must be force",
                ));
                return None;
            }
            Some(RecoveryHookKind::NodalForce)
        }
        UserLoadKind::ConcentratedMoment => {
            if !load.direction.is_rotational() {
                findings.push(UserLoadFinding::new(
                    FindingCode::InvalidLoadDirection,
                    &load.load_id,
                    "concentrated moments require a rotational direction",
                ));
                return None;
            }
            if quantity.dimension != LoadDimension::Moment {
                findings.push(UserLoadFinding::new(
                    FindingCode::InvalidLoadDimension,
                    &load.load_id,
                    "concentrated moment dimension must be moment",
                ));
                return None;
            }
            Some(RecoveryHookKind::NodalMoment)
        }
        UserLoadKind::UniformDistributedLoad => {
            findings.push(UserLoadFinding::new(
                FindingCode::UnsupportedTargetForLoadKind,
                &load.load_id,
                "distributed user loads require an element target",
            ));
            None
        }
    }
}

fn validate_finite(name: &'static str, value: f64) -> Result<(), UserLoadError> {
    if !value.is_finite() {
        return Err(UserLoadError::NonFiniteInput { name, value });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q(value: f64, dimension: LoadDimension) -> UserLoadQuantity {
        UserLoadQuantity::new(value, dimension).unwrap()
    }

    #[test]
    fn concentrated_force_maps_to_translational_nodal_dof() {
        let load = UserLoad::concentrated_force(
            "user-fx",
            1,
            UserLoadDirection::GlobalX,
            q(1250.0, LoadDimension::Force),
        )
        .with_provenance_ref("manual-input");

        let prepared = apply_user_loads(3, 0, &[load]);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.nodal_loads.len(), 1);
        assert_eq!(prepared.nodal_loads[0].global_dof, DOF_PER_NODE);
        assert_eq!(prepared.nodal_loads[0].value, 1250.0);
        assert_eq!(prepared.global_load_vector(3)[DOF_PER_NODE], 1250.0);
        assert_eq!(
            prepared.recovery_hooks[0].hook_kind,
            RecoveryHookKind::NodalForce
        );
        assert_eq!(
            prepared.recovery_hooks[0].provenance_ref.as_deref(),
            Some("manual-input")
        );
    }

    #[test]
    fn concentrated_moment_maps_to_rotational_nodal_dof() {
        let load = UserLoad::concentrated_moment(
            "user-mz",
            0,
            UserLoadDirection::RotationZ,
            q(-80.0, LoadDimension::Moment),
        );

        let prepared = apply_user_loads(1, 0, &[load]);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.nodal_loads[0].global_dof, RZ);
        assert_eq!(prepared.nodal_loads[0].dimension, LoadDimension::Moment);
        assert_eq!(
            prepared.recovery_hooks[0].hook_kind,
            RecoveryHookKind::NodalMoment
        );
    }

    #[test]
    fn uniform_distributed_load_records_span_and_equivalent_total() {
        let load = UserLoad::uniform_distributed(
            "user-wy",
            2,
            UserLoadDirection::GlobalY,
            q(-12.0, LoadDimension::ForcePerLength),
            ElementLoadSpan::new(0.25, 0.75).unwrap(),
            Some(6.0),
        );

        let prepared = apply_user_loads(0, 3, &[load]);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.element_distributed_loads.len(), 1);
        assert_eq!(prepared.element_distributed_loads[0].element_index, 2);
        assert_eq!(
            prepared.element_distributed_loads[0].equivalent_total,
            Some(-36.0)
        );
        assert_eq!(
            prepared.recovery_hooks[0].hook_kind,
            RecoveryHookKind::ElementDistributedLoad
        );
    }

    #[test]
    fn distributed_load_can_defer_equivalent_total_until_element_length_known() {
        let load = UserLoad::uniform_distributed(
            "user-wz",
            0,
            UserLoadDirection::GlobalZ,
            q(4.0, LoadDimension::ForcePerLength),
            ElementLoadSpan::full(),
            None,
        );

        let prepared = apply_user_loads(0, 1, &[load]);

        assert!(!prepared.is_blocked());
        assert_eq!(prepared.element_distributed_loads[0].equivalent_total, None);
    }

    #[test]
    fn invalid_dimensions_and_directions_are_findings() {
        let bad_force = UserLoad::concentrated_force(
            "bad-force",
            0,
            UserLoadDirection::RotationX,
            q(1.0, LoadDimension::Force),
        );
        let bad_moment = UserLoad::concentrated_moment(
            "bad-moment",
            0,
            UserLoadDirection::GlobalX,
            q(1.0, LoadDimension::Moment),
        );
        let bad_distributed = UserLoad::uniform_distributed(
            "bad-dist",
            0,
            UserLoadDirection::GlobalZ,
            q(1.0, LoadDimension::Force),
            ElementLoadSpan::full(),
            Some(1.0),
        );

        let prepared = apply_user_loads(1, 1, &[bad_force, bad_moment, bad_distributed]);

        assert!(prepared.is_blocked());
        assert_eq!(prepared.findings.len(), 3);
        assert!(prepared.findings.iter().any(|finding| {
            finding.code == FindingCode::InvalidLoadDirection && finding.load_id == "bad-force"
        }));
        assert!(prepared.findings.iter().any(|finding| {
            finding.code == FindingCode::InvalidLoadDirection && finding.load_id == "bad-moment"
        }));
        assert!(prepared.findings.iter().any(|finding| {
            finding.code == FindingCode::InvalidLoadDimension && finding.load_id == "bad-dist"
        }));
    }

    #[test]
    fn missing_and_out_of_range_targets_are_findings() {
        let missing = UserLoad::missing_target(
            "missing",
            UserLoadKind::ConcentratedForce,
            UserLoadDirection::GlobalX,
            Some(q(1.0, LoadDimension::Force)),
        );
        let out_of_range = UserLoad::concentrated_force(
            "out-of-range",
            4,
            UserLoadDirection::GlobalX,
            q(1.0, LoadDimension::Force),
        );

        let prepared = apply_user_loads(1, 0, &[missing, out_of_range]);

        assert!(prepared.is_blocked());
        assert_eq!(prepared.findings[0].code, FindingCode::MissingLoadTarget);
        assert_eq!(prepared.findings[1].code, FindingCode::NodeOutOfRange);
        assert!(prepared.nodal_loads.is_empty());
    }

    #[test]
    fn invalid_span_and_length_are_findings() {
        let invalid_span = UserLoad::uniform_distributed(
            "bad-span",
            0,
            UserLoadDirection::GlobalZ,
            q(1.0, LoadDimension::ForcePerLength),
            ElementLoadSpan::new(0.8, 0.2).unwrap(),
            Some(2.0),
        );
        let invalid_length = UserLoad::uniform_distributed(
            "bad-length",
            0,
            UserLoadDirection::GlobalZ,
            q(1.0, LoadDimension::ForcePerLength),
            ElementLoadSpan::full(),
            Some(0.0),
        );

        let prepared = apply_user_loads(0, 1, &[invalid_span, invalid_length]);

        assert!(prepared.is_blocked());
        assert_eq!(prepared.findings.len(), 2);
        assert!(prepared
            .findings
            .iter()
            .any(|finding| { finding.code == FindingCode::InvalidDistributionSpan }));
        assert!(prepared
            .findings
            .iter()
            .any(|finding| { finding.code == FindingCode::NonPositiveElementLength }));
    }

    #[test]
    fn contributions_are_sorted_deterministically() {
        let loads = vec![
            UserLoad::concentrated_force(
                "z-late",
                1,
                UserLoadDirection::GlobalZ,
                q(1.0, LoadDimension::Force),
            ),
            UserLoad::concentrated_force(
                "x-early",
                0,
                UserLoadDirection::GlobalX,
                q(1.0, LoadDimension::Force),
            ),
        ];

        let prepared = apply_user_loads(2, 0, &loads);

        assert_eq!(prepared.nodal_loads[0].trace.load_id, "x-early");
        assert_eq!(prepared.nodal_loads[1].trace.load_id, "z-late");
    }

    #[test]
    fn non_finite_quantity_is_rejected_at_construction() {
        let err = UserLoadQuantity::new(f64::NAN, LoadDimension::Force).unwrap_err();
        assert!(matches!(err, UserLoadError::NonFiniteInput { .. }));
    }
}
