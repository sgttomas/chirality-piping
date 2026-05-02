//! Sandboxed declarative rule-pack expression evaluator.
//!
//! This crate evaluates explicit expression trees. It does not parse text,
//! execute host-language code, access files, access the network, spawn
//! processes, load plugins, embed protected standards content, or emit
//! professional/code-compliance claims.

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dimension {
    Dimensionless,
    Length,
    Mass,
    Time,
    Temperature,
    Angle,
    Force,
    Moment,
    ForcePerLength,
    Pressure,
    Stress,
    Area,
    Volume,
    Density,
    Stiffness,
    Displacement,
    Rotation,
    Velocity,
    Acceleration,
}

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
pub enum BindingSource {
    RulePackRequiredInput,
    UserSuppliedValue,
    SolverResultField,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Quantity {
    pub value: f64,
    pub dimension: Dimension,
}

impl Quantity {
    pub fn new(value: f64, dimension: Dimension) -> Result<Self, EvaluationError> {
        validate_finite("quantity", value)?;
        Ok(Self { value, dimension })
    }

    pub fn dimensionless(value: f64) -> Result<Self, EvaluationError> {
        Self::new(value, Dimension::Dimensionless)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableBinding {
    pub variable_id: String,
    pub source: BindingSource,
    pub quantity: Option<Quantity>,
}

impl VariableBinding {
    pub fn new(variable_id: impl Into<String>, source: BindingSource, quantity: Quantity) -> Self {
        Self {
            variable_id: variable_id.into(),
            source,
            quantity: Some(quantity),
        }
    }

    pub fn missing(variable_id: impl Into<String>, source: BindingSource) -> Self {
        Self {
            variable_id: variable_id.into(),
            source,
            quantity: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Negate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonOperator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Quantity),
    VariableRef(String),
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    Binary {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Compare {
        operator: ComparisonOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    UnsupportedForm {
        form_id: String,
    },
    UnsafeHostAccess {
        request: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvaluationValue {
    Quantity(Quantity),
    Boolean(bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingCode {
    UnsafeConstruct,
    UnsupportedExpressionForm,
    MissingVariable,
    DuplicateBinding,
    InvalidReference,
    MissingRequiredValue,
    NonFiniteInput,
    DivisionByZero,
    DimensionMismatch,
    TypeMismatch,
    StatusBoundaryViolation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EvaluationFinding {
    pub code: FindingCode,
    pub subject_id: String,
    pub message: String,
}

impl EvaluationFinding {
    fn new(code: FindingCode, subject_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code,
            subject_id: subject_id.into(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EvaluationInput {
    pub expression: Expression,
    pub bindings: Vec<VariableBinding>,
    pub required_variable_ids: Vec<String>,
    pub statuses: Vec<AnalysisStatus>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EvaluationResult {
    pub value: Option<EvaluationValue>,
    pub statuses: Vec<AnalysisStatus>,
    pub source_variable_ids: Vec<String>,
    pub findings: Vec<EvaluationFinding>,
}

impl EvaluationResult {
    pub fn is_blocked(&self) -> bool {
        !self.findings.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvaluationError {
    NonFiniteInput { name: &'static str, value: f64 },
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteInput { name, value } => {
                write!(f, "{name} must be finite, got {value}")
            }
        }
    }
}

impl Error for EvaluationError {}

pub fn evaluate(input: &EvaluationInput) -> EvaluationResult {
    let mut findings = Vec::new();
    let statuses = collect_statuses(&input.statuses, &mut findings);
    let binding_map = build_binding_map(&input.bindings, &mut findings);
    check_required_variables(&input.required_variable_ids, &binding_map, &mut findings);

    let mut source_variable_ids = Vec::new();
    let value = eval_expression(
        &input.expression,
        &binding_map,
        &mut source_variable_ids,
        &mut findings,
    );

    source_variable_ids.sort();
    source_variable_ids.dedup();

    EvaluationResult {
        value: if findings.is_empty() { value } else { None },
        statuses,
        source_variable_ids,
        findings,
    }
}

fn build_binding_map<'a>(
    bindings: &'a [VariableBinding],
    findings: &mut Vec<EvaluationFinding>,
) -> HashMap<&'a str, &'a VariableBinding> {
    let mut map = HashMap::new();
    let mut seen = HashSet::new();
    for binding in bindings {
        if binding.variable_id.trim().is_empty() {
            findings.push(EvaluationFinding::new(
                FindingCode::InvalidReference,
                "binding",
                "variable binding id must not be empty",
            ));
            continue;
        }
        if !seen.insert(binding.variable_id.as_str()) {
            findings.push(EvaluationFinding::new(
                FindingCode::DuplicateBinding,
                &binding.variable_id,
                "duplicate variable binding",
            ));
            continue;
        }
        if let Some(quantity) = &binding.quantity {
            if !quantity.value.is_finite() {
                findings.push(EvaluationFinding::new(
                    FindingCode::NonFiniteInput,
                    &binding.variable_id,
                    "variable binding quantity must be finite",
                ));
                continue;
            }
        }
        map.insert(binding.variable_id.as_str(), binding);
    }
    map
}

fn check_required_variables(
    required_variable_ids: &[String],
    bindings: &HashMap<&str, &VariableBinding>,
    findings: &mut Vec<EvaluationFinding>,
) {
    let mut seen = HashSet::new();
    for variable_id in required_variable_ids {
        if variable_id.trim().is_empty() {
            findings.push(EvaluationFinding::new(
                FindingCode::InvalidReference,
                "required_variable",
                "required variable id must not be empty",
            ));
            continue;
        }
        if !seen.insert(variable_id.as_str()) {
            findings.push(EvaluationFinding::new(
                FindingCode::DuplicateBinding,
                variable_id,
                "duplicate required variable id",
            ));
            continue;
        }
        match bindings.get(variable_id.as_str()) {
            Some(binding) if binding.quantity.is_some() => {}
            Some(_) => findings.push(EvaluationFinding::new(
                FindingCode::MissingRequiredValue,
                variable_id,
                "required variable has no supplied value",
            )),
            None => findings.push(EvaluationFinding::new(
                FindingCode::MissingRequiredValue,
                variable_id,
                "required variable is not bound",
            )),
        }
    }
}

fn eval_expression(
    expression: &Expression,
    bindings: &HashMap<&str, &VariableBinding>,
    source_variable_ids: &mut Vec<String>,
    findings: &mut Vec<EvaluationFinding>,
) -> Option<EvaluationValue> {
    match expression {
        Expression::Literal(quantity) => {
            if !quantity.value.is_finite() {
                findings.push(EvaluationFinding::new(
                    FindingCode::NonFiniteInput,
                    "literal",
                    "literal quantity must be finite",
                ));
                None
            } else {
                Some(EvaluationValue::Quantity(quantity.clone()))
            }
        }
        Expression::VariableRef(variable_id) => {
            eval_variable_ref(variable_id, bindings, source_variable_ids, findings)
        }
        Expression::Unary { operator, operand } => {
            let value = eval_expression(operand, bindings, source_variable_ids, findings)?;
            eval_unary(*operator, value, findings)
        }
        Expression::Binary {
            operator,
            left,
            right,
        } => {
            let left = eval_expression(left, bindings, source_variable_ids, findings)?;
            let right = eval_expression(right, bindings, source_variable_ids, findings)?;
            eval_binary(*operator, left, right, findings)
        }
        Expression::Compare {
            operator,
            left,
            right,
        } => {
            let left = eval_expression(left, bindings, source_variable_ids, findings)?;
            let right = eval_expression(right, bindings, source_variable_ids, findings)?;
            eval_compare(*operator, left, right, findings)
        }
        Expression::UnsupportedForm { form_id } => {
            findings.push(EvaluationFinding::new(
                FindingCode::UnsupportedExpressionForm,
                form_id,
                "expression form is not supported by this evaluator",
            ));
            None
        }
        Expression::UnsafeHostAccess { request } => {
            findings.push(EvaluationFinding::new(
                FindingCode::UnsafeConstruct,
                request,
                "host-language, filesystem, network, process, plugin, or reflection access is not permitted",
            ));
            None
        }
    }
}

fn eval_variable_ref(
    variable_id: &str,
    bindings: &HashMap<&str, &VariableBinding>,
    source_variable_ids: &mut Vec<String>,
    findings: &mut Vec<EvaluationFinding>,
) -> Option<EvaluationValue> {
    if variable_id.trim().is_empty() {
        findings.push(EvaluationFinding::new(
            FindingCode::InvalidReference,
            "variable_ref",
            "variable reference must not be empty",
        ));
        return None;
    }

    let Some(binding) = bindings.get(variable_id) else {
        findings.push(EvaluationFinding::new(
            FindingCode::MissingVariable,
            variable_id,
            "variable reference is not bound",
        ));
        return None;
    };
    let Some(quantity) = &binding.quantity else {
        findings.push(EvaluationFinding::new(
            FindingCode::MissingRequiredValue,
            variable_id,
            "variable reference has no supplied value",
        ));
        return None;
    };
    source_variable_ids.push(variable_id.to_string());
    Some(EvaluationValue::Quantity(quantity.clone()))
}

fn eval_unary(
    operator: UnaryOperator,
    value: EvaluationValue,
    findings: &mut Vec<EvaluationFinding>,
) -> Option<EvaluationValue> {
    match (operator, value) {
        (UnaryOperator::Negate, EvaluationValue::Quantity(quantity)) => {
            Some(EvaluationValue::Quantity(Quantity {
                value: -quantity.value,
                dimension: quantity.dimension,
            }))
        }
        (UnaryOperator::Negate, EvaluationValue::Boolean(_)) => {
            findings.push(EvaluationFinding::new(
                FindingCode::TypeMismatch,
                "unary_negate",
                "cannot negate a boolean expression",
            ));
            None
        }
    }
}

fn eval_binary(
    operator: BinaryOperator,
    left: EvaluationValue,
    right: EvaluationValue,
    findings: &mut Vec<EvaluationFinding>,
) -> Option<EvaluationValue> {
    let (EvaluationValue::Quantity(left), EvaluationValue::Quantity(right)) = (left, right) else {
        findings.push(EvaluationFinding::new(
            FindingCode::TypeMismatch,
            "binary_expression",
            "binary arithmetic requires numeric quantities",
        ));
        return None;
    };

    match operator {
        BinaryOperator::Add => add_or_subtract(left, right, 1.0, findings),
        BinaryOperator::Subtract => add_or_subtract(left, right, -1.0, findings),
        BinaryOperator::Multiply => multiply(left, right, findings),
        BinaryOperator::Divide => divide(left, right, findings),
    }
}

fn add_or_subtract(
    left: Quantity,
    right: Quantity,
    sign: f64,
    findings: &mut Vec<EvaluationFinding>,
) -> Option<EvaluationValue> {
    if left.dimension != right.dimension {
        findings.push(dimension_mismatch(
            "add_subtract",
            left.dimension,
            right.dimension,
        ));
        return None;
    }
    Some(EvaluationValue::Quantity(Quantity {
        value: left.value + sign * right.value,
        dimension: left.dimension,
    }))
}

fn multiply(
    left: Quantity,
    right: Quantity,
    findings: &mut Vec<EvaluationFinding>,
) -> Option<EvaluationValue> {
    match (left.dimension, right.dimension) {
        (Dimension::Dimensionless, dim) => Some(EvaluationValue::Quantity(Quantity {
            value: left.value * right.value,
            dimension: dim,
        })),
        (dim, Dimension::Dimensionless) => Some(EvaluationValue::Quantity(Quantity {
            value: left.value * right.value,
            dimension: dim,
        })),
        _ => {
            findings.push(EvaluationFinding::new(
                FindingCode::UnsupportedExpressionForm,
                "multiply",
                "multiplication of two dimensional quantities remains TBD",
            ));
            None
        }
    }
}

fn divide(
    left: Quantity,
    right: Quantity,
    findings: &mut Vec<EvaluationFinding>,
) -> Option<EvaluationValue> {
    if right.value == 0.0 {
        findings.push(EvaluationFinding::new(
            FindingCode::DivisionByZero,
            "divide",
            "division by zero is not permitted",
        ));
        return None;
    }

    match (left.dimension, right.dimension) {
        (dim, Dimension::Dimensionless) => Some(EvaluationValue::Quantity(Quantity {
            value: left.value / right.value,
            dimension: dim,
        })),
        (left_dim, right_dim) if left_dim == right_dim => {
            Some(EvaluationValue::Quantity(Quantity {
                value: left.value / right.value,
                dimension: Dimension::Dimensionless,
            }))
        }
        _ => {
            findings.push(EvaluationFinding::new(
                FindingCode::UnsupportedExpressionForm,
                "divide",
                "division producing derived dimensions remains TBD",
            ));
            None
        }
    }
}

fn eval_compare(
    operator: ComparisonOperator,
    left: EvaluationValue,
    right: EvaluationValue,
    findings: &mut Vec<EvaluationFinding>,
) -> Option<EvaluationValue> {
    let (EvaluationValue::Quantity(left), EvaluationValue::Quantity(right)) = (left, right) else {
        findings.push(EvaluationFinding::new(
            FindingCode::TypeMismatch,
            "comparison",
            "comparison requires numeric quantities",
        ));
        return None;
    };
    if left.dimension != right.dimension {
        findings.push(dimension_mismatch(
            "comparison",
            left.dimension,
            right.dimension,
        ));
        return None;
    }
    let result = match operator {
        ComparisonOperator::LessThan => left.value < right.value,
        ComparisonOperator::LessThanOrEqual => left.value <= right.value,
        ComparisonOperator::GreaterThan => left.value > right.value,
        ComparisonOperator::GreaterThanOrEqual => left.value >= right.value,
        ComparisonOperator::Equal => left.value == right.value,
        ComparisonOperator::NotEqual => left.value != right.value,
    };
    Some(EvaluationValue::Boolean(result))
}

fn collect_statuses(
    statuses: &[AnalysisStatus],
    findings: &mut Vec<EvaluationFinding>,
) -> Vec<AnalysisStatus> {
    let mut collected = Vec::new();
    if statuses.is_empty() {
        collected.push(AnalysisStatus::RuleInputsIncomplete);
        return collected;
    }
    for status in statuses {
        if *status == AnalysisStatus::HumanApprovedForProject {
            findings.push(EvaluationFinding::new(
                FindingCode::StatusBoundaryViolation,
                "analysis_status",
                "human approval is an external project record, not an evaluator output",
            ));
            collected.push(AnalysisStatus::HumanReviewRequired);
            continue;
        }
        if !collected.contains(status) {
            collected.push(*status);
        }
    }
    collected
}

fn dimension_mismatch(
    subject_id: impl Into<String>,
    left: Dimension,
    right: Dimension,
) -> EvaluationFinding {
    EvaluationFinding::new(
        FindingCode::DimensionMismatch,
        subject_id,
        format!("dimension mismatch: left={left:?}, right={right:?}"),
    )
}

fn validate_finite(name: &'static str, value: f64) -> Result<(), EvaluationError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(EvaluationError::NonFiniteInput { name, value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn binding(id: &str, value: f64, dimension: Dimension) -> VariableBinding {
        VariableBinding::new(
            id,
            BindingSource::RulePackRequiredInput,
            Quantity::new(value, dimension).unwrap(),
        )
    }

    fn input(expression: Expression, bindings: Vec<VariableBinding>) -> EvaluationInput {
        EvaluationInput {
            expression,
            bindings,
            required_variable_ids: vec![],
            statuses: vec![AnalysisStatus::MechanicsSolved],
        }
    }

    #[test]
    fn evaluates_invented_dimensionally_compatible_expression() {
        let expression = Expression::Compare {
            operator: ComparisonOperator::LessThanOrEqual,
            left: Box::new(Expression::Binary {
                operator: BinaryOperator::Add,
                left: Box::new(Expression::VariableRef("actual_stress".to_string())),
                right: Box::new(Expression::Literal(
                    Quantity::new(5.0, Dimension::Stress).unwrap(),
                )),
            }),
            right: Box::new(Expression::VariableRef("invented_limit".to_string())),
        };
        let result = evaluate(&input(
            expression,
            vec![
                binding("actual_stress", 95.0, Dimension::Stress),
                binding("invented_limit", 120.0, Dimension::Stress),
            ],
        ));

        assert_eq!(result.value, Some(EvaluationValue::Boolean(true)));
        assert!(result.findings.is_empty());
        assert_eq!(
            result.source_variable_ids,
            vec!["actual_stress".to_string(), "invented_limit".to_string()]
        );
    }

    #[test]
    fn rejects_unsafe_host_access() {
        let result = evaluate(&input(
            Expression::UnsafeHostAccess {
                request: "filesystem".to_string(),
            },
            vec![],
        ));

        assert!(result.is_blocked());
        assert_eq!(result.findings[0].code, FindingCode::UnsafeConstruct);
        assert_eq!(result.value, None);
    }

    #[test]
    fn rejects_unsupported_expression_form() {
        let result = evaluate(&input(
            Expression::UnsupportedForm {
                form_id: "loop".to_string(),
            },
            vec![],
        ));

        assert_eq!(
            result.findings[0].code,
            FindingCode::UnsupportedExpressionForm
        );
        assert_eq!(result.value, None);
    }

    #[test]
    fn reports_missing_variable_binding() {
        let result = evaluate(&input(
            Expression::VariableRef("missing".to_string()),
            vec![],
        ));

        assert_eq!(result.findings[0].code, FindingCode::MissingVariable);
        assert_eq!(result.value, None);
    }

    #[test]
    fn reports_duplicate_binding() {
        let result = evaluate(&input(
            Expression::VariableRef("x".to_string()),
            vec![
                binding("x", 1.0, Dimension::Force),
                binding("x", 2.0, Dimension::Force),
            ],
        ));

        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::DuplicateBinding));
        assert_eq!(result.value, None);
    }

    #[test]
    fn reports_invalid_empty_reference() {
        let result = evaluate(&input(Expression::VariableRef(" ".to_string()), vec![]));

        assert_eq!(result.findings[0].code, FindingCode::InvalidReference);
        assert_eq!(result.value, None);
    }

    #[test]
    fn reports_missing_required_value() {
        let mut evaluation = input(
            Expression::VariableRef("allowable".to_string()),
            vec![VariableBinding::missing(
                "allowable",
                BindingSource::UserSuppliedValue,
            )],
        );
        evaluation.required_variable_ids = vec!["allowable".to_string()];

        let result = evaluate(&evaluation);

        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::MissingRequiredValue));
        assert_eq!(result.value, None);
    }

    #[test]
    fn reports_non_finite_literal() {
        let result = evaluate(&input(
            Expression::Literal(Quantity {
                value: f64::NAN,
                dimension: Dimension::Force,
            }),
            vec![],
        ));

        assert_eq!(result.findings[0].code, FindingCode::NonFiniteInput);
        assert_eq!(result.value, None);
    }

    #[test]
    fn rejects_division_by_zero() {
        let result = evaluate(&input(
            Expression::Binary {
                operator: BinaryOperator::Divide,
                left: Box::new(Expression::VariableRef("force".to_string())),
                right: Box::new(Expression::Literal(Quantity::dimensionless(0.0).unwrap())),
            },
            vec![binding("force", 10.0, Dimension::Force)],
        ));

        assert_eq!(result.findings[0].code, FindingCode::DivisionByZero);
        assert_eq!(result.value, None);
    }

    #[test]
    fn rejects_dimension_mismatch() {
        let result = evaluate(&input(
            Expression::Compare {
                operator: ComparisonOperator::LessThan,
                left: Box::new(Expression::VariableRef("force".to_string())),
                right: Box::new(Expression::VariableRef("stress".to_string())),
            },
            vec![
                binding("force", 10.0, Dimension::Force),
                binding("stress", 10.0, Dimension::Stress),
            ],
        ));

        assert_eq!(result.findings[0].code, FindingCode::DimensionMismatch);
        assert_eq!(result.value, None);
    }

    #[test]
    fn permits_dimensionless_scaling_without_derived_dimension_policy() {
        let result = evaluate(&input(
            Expression::Binary {
                operator: BinaryOperator::Multiply,
                left: Box::new(Expression::Literal(Quantity::dimensionless(2.0).unwrap())),
                right: Box::new(Expression::VariableRef("force".to_string())),
            },
            vec![binding("force", 10.0, Dimension::Force)],
        ));

        assert_eq!(
            result.value,
            Some(EvaluationValue::Quantity(Quantity {
                value: 20.0,
                dimension: Dimension::Force,
            }))
        );
        assert!(result.findings.is_empty());
    }

    #[test]
    fn rejects_dimensional_multiplication_until_policy_is_accepted() {
        let result = evaluate(&input(
            Expression::Binary {
                operator: BinaryOperator::Multiply,
                left: Box::new(Expression::VariableRef("force".to_string())),
                right: Box::new(Expression::VariableRef("length".to_string())),
            },
            vec![
                binding("force", 10.0, Dimension::Force),
                binding("length", 2.0, Dimension::Length),
            ],
        ));

        assert_eq!(
            result.findings[0].code,
            FindingCode::UnsupportedExpressionForm
        );
        assert_eq!(result.value, None);
    }

    #[test]
    fn converts_same_dimension_division_to_dimensionless_ratio() {
        let result = evaluate(&input(
            Expression::Binary {
                operator: BinaryOperator::Divide,
                left: Box::new(Expression::VariableRef("actual".to_string())),
                right: Box::new(Expression::VariableRef("limit".to_string())),
            },
            vec![
                binding("actual", 25.0, Dimension::Stress),
                binding("limit", 100.0, Dimension::Stress),
            ],
        ));

        assert_eq!(
            result.value,
            Some(EvaluationValue::Quantity(Quantity {
                value: 0.25,
                dimension: Dimension::Dimensionless,
            }))
        );
        assert!(result.findings.is_empty());
    }

    #[test]
    fn maps_human_approval_status_to_boundary_finding() {
        let mut evaluation = input(
            Expression::Literal(Quantity::dimensionless(1.0).unwrap()),
            vec![],
        );
        evaluation.statuses = vec![AnalysisStatus::HumanApprovedForProject];

        let result = evaluate(&evaluation);

        assert!(result
            .findings
            .iter()
            .any(|finding| finding.code == FindingCode::StatusBoundaryViolation));
        assert!(result
            .statuses
            .contains(&AnalysisStatus::HumanReviewRequired));
        assert_eq!(result.value, None);
    }
}
