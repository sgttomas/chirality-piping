//! Code-neutral solver diagnostics.
//!
//! This crate turns mechanics-solver findings into deterministic diagnostic
//! records. It does not encode design-code checks, professional approval,
//! protected standards content, or private project data.

use open_pipe_stress_frame_kernel::FrameKernelError;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Blocking,
    Failure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSource {
    ModelValidation,
    MechanicsSolver,
    SolverConfiguration,
    SolverIteration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolverStatus {
    ReadyToSolve,
    MechanicsSolved,
    SolvedWithWarnings,
    ModelIncomplete,
    SolveFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolverDiagnosticCode {
    SingularSystem,
    IllConditionedSystem,
    ConditioningFailure,
    InvalidRestraint,
    InvalidModelTopology,
    InvalidNumericInput,
    NonConvergence,
    SparseSolverTbd,
    TolerancePolicyTbd,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SolverDiagnostic {
    pub code: SolverDiagnosticCode,
    pub severity: DiagnosticSeverity,
    pub source: DiagnosticSource,
    pub message: String,
    pub affected_ref: Option<String>,
}

impl SolverDiagnostic {
    pub fn new(
        code: SolverDiagnosticCode,
        severity: DiagnosticSeverity,
        source: DiagnosticSource,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            severity,
            source,
            message: message.into(),
            affected_ref: None,
        }
    }

    pub fn with_affected_ref(mut self, affected_ref: impl Into<String>) -> Self {
        self.affected_ref = Some(affected_ref.into());
        self
    }

    pub fn is_blocking(&self) -> bool {
        matches!(
            self.severity,
            DiagnosticSeverity::Blocking | DiagnosticSeverity::Failure
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SolverDiagnosticReport {
    pub status: SolverStatus,
    pub diagnostics: Vec<SolverDiagnostic>,
}

impl SolverDiagnosticReport {
    pub fn new(status: SolverStatus, diagnostics: Vec<SolverDiagnostic>) -> Self {
        Self {
            status,
            diagnostics,
        }
    }

    pub fn is_blocked(&self) -> bool {
        self.diagnostics.iter().any(SolverDiagnostic::is_blocking)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiagnosticsError {
    NonFiniteInput {
        name: &'static str,
        value: f64,
    },
    NegativeInput {
        name: &'static str,
        value: f64,
    },
    InvalidThresholds {
        warning_threshold: f64,
        failure_threshold: f64,
    },
}

impl fmt::Display for DiagnosticsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteInput { name, value } => {
                write!(f, "{name} must be finite, got {value}")
            }
            Self::NegativeInput { name, value } => {
                write!(f, "{name} must be nonnegative, got {value}")
            }
            Self::InvalidThresholds {
                warning_threshold,
                failure_threshold,
            } => write!(
                f,
                "conditioning thresholds must satisfy 1.0 <= warning <= failure, got {warning_threshold} and {failure_threshold}"
            ),
        }
    }
}

impl Error for DiagnosticsError {}

pub fn diagnostic_from_frame_error(error: &FrameKernelError) -> SolverDiagnostic {
    match error {
        FrameKernelError::SingularSystem { pivot } => SolverDiagnostic::new(
            SolverDiagnosticCode::SingularSystem,
            DiagnosticSeverity::Failure,
            DiagnosticSource::MechanicsSolver,
            format!("reduced stiffness system is singular at pivot {pivot}"),
        ),
        FrameKernelError::RepeatedRestrainedDof { dof } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidRestraint,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            format!("restrained degree of freedom {dof} is repeated"),
        ),
        FrameKernelError::RestrainedDofOutOfRange { dof, total_dofs } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidRestraint,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            format!("restrained degree of freedom {dof} is outside total DOF count {total_dofs}"),
        ),
        FrameKernelError::InvalidNodeIndex {
            node_index,
            node_count,
        } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidModelTopology,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            format!("node index {node_index} is outside model node count {node_count}"),
        ),
        FrameKernelError::RepeatedElementNodeIndex { node_index } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidModelTopology,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            format!("element connects node index {node_index} to itself"),
        ),
        FrameKernelError::DegenerateAxis { detail } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidModelTopology,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            format!("degenerate local axis definition: {detail}"),
        ),
        FrameKernelError::NonFiniteInput { name, value } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidNumericInput,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            format!("{name} must be finite, got {value}"),
        ),
        FrameKernelError::NonPositiveInput { name, value } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidNumericInput,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::ModelValidation,
            format!("{name} must be positive, got {value}"),
        ),
        FrameKernelError::InvalidMatrixDimensions { rows, cols } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidNumericInput,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::MechanicsSolver,
            format!("stiffness matrix must be square, got {rows} by {cols}"),
        ),
        FrameKernelError::InvalidVectorLength { expected, actual } => SolverDiagnostic::new(
            SolverDiagnosticCode::InvalidNumericInput,
            DiagnosticSeverity::Blocking,
            DiagnosticSource::MechanicsSolver,
            format!("force vector length must be {expected}, got {actual}"),
        ),
    }
}

pub fn report_frame_error(error: &FrameKernelError) -> SolverDiagnosticReport {
    SolverDiagnosticReport::new(
        SolverStatus::SolveFailed,
        vec![diagnostic_from_frame_error(error)],
    )
}

pub fn classify_condition_ratio(
    ratio: f64,
    warning_threshold: f64,
    failure_threshold: f64,
) -> Result<Option<SolverDiagnostic>, DiagnosticsError> {
    validate_nonnegative_finite("condition_ratio", ratio)?;
    validate_nonnegative_finite("warning_threshold", warning_threshold)?;
    validate_nonnegative_finite("failure_threshold", failure_threshold)?;

    if warning_threshold < 1.0 || failure_threshold < 1.0 || warning_threshold > failure_threshold {
        return Err(DiagnosticsError::InvalidThresholds {
            warning_threshold,
            failure_threshold,
        });
    }

    if ratio >= failure_threshold {
        return Ok(Some(SolverDiagnostic::new(
            SolverDiagnosticCode::ConditioningFailure,
            DiagnosticSeverity::Failure,
            DiagnosticSource::MechanicsSolver,
            format!("estimated condition ratio {ratio} meets or exceeds failure threshold {failure_threshold}"),
        )));
    }

    if ratio >= warning_threshold {
        return Ok(Some(SolverDiagnostic::new(
            SolverDiagnosticCode::IllConditionedSystem,
            DiagnosticSeverity::Warning,
            DiagnosticSource::MechanicsSolver,
            format!("estimated condition ratio {ratio} meets or exceeds warning threshold {warning_threshold}"),
        )));
    }

    Ok(None)
}

pub fn convergence_diagnostic(
    iteration_count: usize,
    max_iterations: usize,
    residual_norm: f64,
    tolerance: f64,
) -> Result<Option<SolverDiagnostic>, DiagnosticsError> {
    validate_nonnegative_finite("residual_norm", residual_norm)?;
    validate_nonnegative_finite("tolerance", tolerance)?;

    if residual_norm <= tolerance {
        return Ok(None);
    }

    if iteration_count >= max_iterations {
        return Ok(Some(SolverDiagnostic::new(
            SolverDiagnosticCode::NonConvergence,
            DiagnosticSeverity::Failure,
            DiagnosticSource::SolverIteration,
            format!(
                "solver did not converge after {iteration_count} iterations; residual {residual_norm} exceeds tolerance {tolerance}"
            ),
        )));
    }

    Ok(Some(SolverDiagnostic::new(
        SolverDiagnosticCode::NonConvergence,
        DiagnosticSeverity::Warning,
        DiagnosticSource::SolverIteration,
        format!(
            "solver residual {residual_norm} exceeds tolerance {tolerance} before the iteration limit"
        ),
    )))
}

pub fn sparse_solver_tbd_diagnostic() -> SolverDiagnostic {
    SolverDiagnostic::new(
        SolverDiagnosticCode::SparseSolverTbd,
        DiagnosticSeverity::Warning,
        DiagnosticSource::SolverConfiguration,
        "sparse numerical solver selection remains TBD; dense solve path is for bounded verification only",
    )
}

pub fn tolerance_policy_tbd_diagnostic() -> SolverDiagnostic {
    SolverDiagnostic::new(
        SolverDiagnosticCode::TolerancePolicyTbd,
        DiagnosticSeverity::Warning,
        DiagnosticSource::SolverConfiguration,
        "solver tolerance policy remains TBD and must be accepted before release-quality performance claims",
    )
}

fn validate_nonnegative_finite(name: &'static str, value: f64) -> Result<(), DiagnosticsError> {
    if !value.is_finite() {
        return Err(DiagnosticsError::NonFiniteInput { name, value });
    }
    if value < 0.0 {
        return Err(DiagnosticsError::NegativeInput { name, value });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_singular_frame_error_to_failure_diagnostic() {
        let report = report_frame_error(&FrameKernelError::SingularSystem { pivot: 3 });

        assert_eq!(report.status, SolverStatus::SolveFailed);
        assert!(report.is_blocked());
        assert_eq!(
            report.diagnostics[0].code,
            SolverDiagnosticCode::SingularSystem
        );
        assert_eq!(report.diagnostics[0].severity, DiagnosticSeverity::Failure);
    }

    #[test]
    fn maps_invalid_restraint_to_blocking_model_diagnostic() {
        let diagnostic =
            diagnostic_from_frame_error(&FrameKernelError::RepeatedRestrainedDof { dof: 2 });

        assert_eq!(diagnostic.code, SolverDiagnosticCode::InvalidRestraint);
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Blocking);
        assert_eq!(diagnostic.source, DiagnosticSource::ModelValidation);
    }

    #[test]
    fn maps_repeated_element_node_to_topology_diagnostic() {
        let diagnostic = diagnostic_from_frame_error(&FrameKernelError::RepeatedElementNodeIndex {
            node_index: 4,
        });

        assert_eq!(diagnostic.code, SolverDiagnosticCode::InvalidModelTopology);
        assert!(diagnostic.message.contains("node index 4"));
    }

    #[test]
    fn condition_ratio_below_warning_returns_none() {
        let diagnostic = classify_condition_ratio(10.0, 100.0, 1000.0).unwrap();

        assert!(diagnostic.is_none());
    }

    #[test]
    fn condition_ratio_warning_is_nonblocking() {
        let diagnostic = classify_condition_ratio(150.0, 100.0, 1000.0)
            .unwrap()
            .unwrap();

        assert_eq!(diagnostic.code, SolverDiagnosticCode::IllConditionedSystem);
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Warning);
        assert!(!diagnostic.is_blocking());
    }

    #[test]
    fn condition_ratio_failure_is_blocking() {
        let diagnostic = classify_condition_ratio(1200.0, 100.0, 1000.0)
            .unwrap()
            .unwrap();

        assert_eq!(diagnostic.code, SolverDiagnosticCode::ConditioningFailure);
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Failure);
        assert!(diagnostic.is_blocking());
    }

    #[test]
    fn condition_ratio_rejects_non_finite_input() {
        let error = classify_condition_ratio(f64::INFINITY, 100.0, 1000.0).unwrap_err();

        assert_eq!(
            error,
            DiagnosticsError::NonFiniteInput {
                name: "condition_ratio",
                value: f64::INFINITY
            }
        );
    }

    #[test]
    fn nonconvergence_after_iteration_limit_is_failure() {
        let diagnostic = convergence_diagnostic(20, 20, 1.0e-3, 1.0e-6)
            .unwrap()
            .unwrap();

        assert_eq!(diagnostic.code, SolverDiagnosticCode::NonConvergence);
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Failure);
    }

    #[test]
    fn converged_residual_returns_no_diagnostic() {
        let diagnostic = convergence_diagnostic(5, 20, 1.0e-8, 1.0e-6).unwrap();

        assert!(diagnostic.is_none());
    }

    #[test]
    fn sparse_solver_tbd_is_warning_not_failure() {
        let diagnostic = sparse_solver_tbd_diagnostic();

        assert_eq!(diagnostic.code, SolverDiagnosticCode::SparseSolverTbd);
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Warning);
        assert!(!diagnostic.is_blocking());
    }
}
