//! Product-preview physics adapter.
//!
//! This crate maps invented public preview-model data into the code-neutral
//! mechanics crates. It emits mechanics quantities and diagnostics only; it
//! does not encode standards criteria, allowables, SIF tables, private data, or
//! professional acceptance.

use open_pipe_stress_frame_kernel::{
    assemble_global_stiffness, reduce_system, solve_dense, FrameElement, FrameKernelError,
    FrameNode, DOF_PER_NODE, RX, RY, RZ, UX, UY, UZ,
};
use open_pipe_stress_linear_supports::{
    prepare_boundary, FrameDof, LinearSupport, QuantityDimension, SupportFamily, SupportQuantity,
};
use open_pipe_stress_primitive_loads::{
    prepare_loads, LoadDimension, LoadDirection, LoadQuantity, PrimitiveLoad, PrimitiveLoadCategory,
};
use open_pipe_stress_straight_pipe::{StraightPipeElement, StraightPipeSectionProperties};
use open_pipe_stress_stress_recovery::{
    recover_stresses, AnalysisStatus, ForceResultants, PressureBasis, StressRecoveryInput,
    StressSectionProperties,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::f64::consts::PI;

#[derive(Debug, Clone, Deserialize)]
pub struct PreviewModel {
    pub schema_version: String,
    pub document_kind: String,
    pub project: Project,
    pub analysis_status: StatusEnvelope,
    pub nodes: Vec<PreviewNode>,
    pub pipe_segments: Vec<PreviewPipe>,
    pub supports: Vec<PreviewSupport>,
    #[serde(default)]
    pub materials: Vec<MaterialInput>,
    #[serde(default)]
    pub load_cases: Vec<PreviewLoadCase>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Project {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct StatusEnvelope {
    pub mechanics: String,
    pub rule_check: String,
    pub professional_acceptance: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PreviewNode {
    pub id: String,
    pub position: Vec3,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PreviewPipe {
    pub id: String,
    pub from: String,
    pub to: String,
    pub section: PipeSectionInput,
    pub material: String,
    #[serde(default)]
    pub y_reference: Option<Vec3>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PipeSectionInput {
    pub outside_diameter: Quantity,
    pub wall_thickness: Quantity,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Quantity {
    pub value: f64,
    #[allow(dead_code)]
    pub unit: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PreviewSupport {
    pub id: String,
    pub node: String,
    pub restraints: Vec<String>,
    #[serde(default)]
    pub family: Option<String>,
    #[serde(default)]
    pub stiffness: Option<SupportStiffnessInput>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SupportStiffnessInput {
    pub dof: String,
    pub value: Quantity,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PreviewLoadCase {
    pub id: String,
    #[serde(default)]
    pub primitive_loads: Vec<PreviewPrimitiveLoad>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PreviewPrimitiveLoad {
    pub id: String,
    pub category: String,
    pub target: LoadTargetInput,
    pub direction: String,
    pub magnitude: Quantity,
    pub dimension: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LoadTargetInput {
    Node { node: String },
    Element { pipe: String },
}

#[derive(Debug, Clone, Deserialize)]
pub struct MaterialInput {
    pub id: String,
    pub elastic_modulus: Quantity,
    pub shear_modulus: Quantity,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LinearStaticPreviewRequest {
    pub model: PreviewModel,
    #[serde(default)]
    pub materials: Vec<MaterialInput>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MechanicsEnvelope {
    pub schema_version: String,
    pub document_kind: String,
    pub run_id: String,
    pub model_ref: String,
    pub status: StatusEnvelope,
    pub summary: Summary,
    pub results: Vec<ResultItem>,
    pub diagnostics: Vec<Diagnostic>,
    pub professional_boundary: ProfessionalBoundary,
    pub accepted_model_state_mutated: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct Summary {
    pub node_count: usize,
    pub segment_count: usize,
    pub support_count: usize,
    pub load_case_count: usize,
    pub max_displacement: Option<LocatedQuantity>,
    pub max_open_formula_stress: Option<LocatedQuantity>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LocatedQuantity {
    pub value: f64,
    pub unit: String,
    pub location_ref: String,
    pub result_ref: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResultItem {
    pub id: String,
    pub kind: String,
    pub value: f64,
    pub unit: String,
    pub entity_ref: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Diagnostic {
    pub id: String,
    pub code: String,
    pub severity: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub affected_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProfessionalBoundary {
    pub human_review_required: bool,
    pub software_makes_compliance_claim: bool,
    pub software_makes_certification_claim: bool,
    pub software_makes_sealing_claim: bool,
    pub software_makes_approval_claim: bool,
}

#[derive(Debug)]
struct BuiltModel {
    nodes: Vec<FrameNode>,
    pipes: Vec<StraightPipeElement>,
    frame_elements: Vec<FrameElement>,
    supports: Vec<LinearSupport>,
    loads: Vec<PrimitiveLoad>,
    sections: HashMap<String, DerivedSection>,
}

#[derive(Debug, Clone, Copy)]
struct DerivedSection {
    area: f64,
    second_moment: f64,
    torsion_constant: f64,
    section_modulus: f64,
    torsion_radius: f64,
    membrane_radius: f64,
    wall_thickness: f64,
}

pub fn run_linear_static_preview(request: LinearStaticPreviewRequest) -> MechanicsEnvelope {
    let model = request.model;
    let materials = if request.materials.is_empty() {
        model.materials.clone()
    } else {
        request.materials
    };
    let mut diagnostics = Vec::new();

    if model.document_kind != "openpipestress.product_preview.model" {
        diagnostics.push(diag(
            "diagnostic:physics:document-kind",
            "PREVIEW_DOCUMENT_KIND_INVALID",
            "blocking",
            "physics adapter requires an openpipestress.product_preview.model document",
            vec!["model".to_string()],
        ));
    }
    if model.schema_version.is_empty() {
        diagnostics.push(diag(
            "diagnostic:physics:schema-version",
            "PREVIEW_SCHEMA_VERSION_MISSING",
            "blocking",
            "preview model requires an explicit schema version",
            vec!["model".to_string()],
        ));
    }

    let built = build_model(&model, &materials, &mut diagnostics);
    if has_blocking(&diagnostics) {
        return blocked_envelope(model, diagnostics);
    }
    let built = built.expect("build_model returns Some when no blocking diagnostics were added");

    let boundary = prepare_boundary(built.nodes.len(), &built.supports);
    if boundary.restrained_dofs.is_empty() && boundary.springs.is_empty() {
        diagnostics.push(diag(
            "diagnostic:physics:no-restraints",
            "SUPPORT_INPUT_MISSING",
            "blocking",
            "linear-static preview requires explicit support restraints or springs; no automatic boundary conditions are applied",
            vec!["supports".to_string()],
        ));
    } else if boundary.restrained_dofs.len() < DOF_PER_NODE {
        diagnostics.push(diag(
            "diagnostic:physics:under-restrained",
            "SOLVER_SYSTEM_BLOCKED",
            "blocking",
            "linear-static preview has fewer than six rigid restraints; treat as under-restrained before dense solve",
            vec!["supports".to_string()],
        ));
    }
    for finding in boundary.findings {
        diagnostics.push(diag(
            &format!("diagnostic:support:{}", finding.support_id),
            "SUPPORT_INPUT_INVALID",
            "blocking",
            finding.message,
            vec![finding.support_id],
        ));
    }

    let load_application = prepare_loads(built.nodes.len(), built.pipes.len(), &built.loads);
    for finding in &load_application.findings {
        diagnostics.push(diag(
            &format!("diagnostic:load:{}", finding.load_id),
            "LOAD_INPUT_INVALID",
            "blocking",
            finding.message.clone(),
            vec![finding.load_id.clone()],
        ));
    }
    if load_application
        .element_uniform_loads
        .iter()
        .any(|item| item.magnitude.dimension == LoadDimension::Pressure)
    {
        diagnostics.push(diag(
            "diagnostic:physics:pressure-not-applied",
            "PRESSURE_LOAD_NOT_APPLIED_TO_FRAME_VECTOR",
            "warning",
            "pressure primitive loads are retained for stress recovery context but are not converted to frame nodal loads in this preview slice",
            vec![],
        ));
    }
    if has_blocking(&diagnostics) {
        return blocked_envelope(model, diagnostics);
    }

    let mut stiffness = match assemble_global_stiffness(built.nodes.len(), &built.frame_elements) {
        Ok(stiffness) => stiffness,
        Err(error) => return solver_blocked(model, diagnostics, error),
    };
    for spring in boundary.springs {
        stiffness[spring.node_dof.global_index()][spring.node_dof.global_index()] +=
            spring.stiffness.value;
    }

    let mut force = load_application.global_load_vector(built.nodes.len());
    add_uniform_element_loads(
        &mut force,
        &model,
        &load_application.element_uniform_loads,
        &built.pipes,
    );

    let reduced = match reduce_system(&stiffness, &force, &boundary.restrained_dofs) {
        Ok(reduced) => reduced,
        Err(error) => return solver_blocked(model, diagnostics, error),
    };
    let reduced_displacements = match solve_dense(&reduced.stiffness, &reduced.force) {
        Ok(solution) => solution,
        Err(error) => return solver_blocked(model, diagnostics, error),
    };

    let mut displacements = vec![0.0; built.nodes.len() * DOF_PER_NODE];
    for (index, dof) in reduced.free_dofs.iter().enumerate() {
        displacements[*dof] = reduced_displacements[index];
    }

    let mut results = Vec::new();
    let mut max_displacement = None;
    for node in &model.nodes {
        let node_index = node_index(&model, &node.id).unwrap();
        let magnitude = displacement_magnitude(&displacements, node_index);
        let result_id = format!("result:disp:{}", stable_suffix(&node.id));
        if max_displacement
            .as_ref()
            .map(|q: &LocatedQuantity| magnitude * 1000.0 > q.value)
            .unwrap_or(true)
        {
            max_displacement = Some(LocatedQuantity {
                value: round6(magnitude * 1000.0),
                unit: "mm".to_string(),
                location_ref: node.id.clone(),
                result_ref: result_id.clone(),
            });
        }
        results.push(ResultItem {
            id: result_id,
            kind: "displacement_magnitude".to_string(),
            value: round6(magnitude * 1000.0),
            unit: "mm".to_string(),
            entity_ref: node.id.clone(),
        });
    }

    let reactions = multiply_matrix_vector(&stiffness, &displacements)
        .into_iter()
        .zip(force.iter())
        .map(|(internal, applied)| internal - applied)
        .collect::<Vec<_>>();
    for support in &model.supports {
        if let Some(index) = node_index(&model, &support.node) {
            let magnitude = support
                .restraints
                .iter()
                .filter_map(|dof| parse_dof(dof).ok())
                .map(|dof| {
                    let global = index * DOF_PER_NODE + dof_index(dof);
                    reactions[global] * reactions[global]
                })
                .sum::<f64>()
                .sqrt();
            results.push(ResultItem {
                id: format!("result:reaction:{}", stable_suffix(&support.id)),
                kind: "reaction_resultant".to_string(),
                value: round6(magnitude),
                unit: "N".to_string(),
                entity_ref: support.id.clone(),
            });
        }
    }

    let mut max_stress = None;
    for (pipe_index, pipe) in built.pipes.iter().enumerate() {
        let local = match pipe.recover_local_forces_from_global_model(&displacements) {
            Ok(local) => local,
            Err(error) => {
                diagnostics.push(diag(
                    &format!("diagnostic:stress:{}", stable_suffix(&pipe.element_id)),
                    "ELEMENT_FORCE_RECOVERY_FAILED",
                    "warning",
                    error.to_string(),
                    vec![pipe.element_id.clone()],
                ));
                continue;
            }
        };
        let section = built
            .sections
            .get(&pipe.element_id)
            .expect("section exists for pipe");
        let pressure = pressure_for_pipe(&model, pipe_index, &pipe.element_id);
        let stress = recover_stresses(&StressRecoveryInput {
            resultants: ForceResultants::new(
                Some(local.local_forces[UX]),
                Some(local.local_forces[RY]),
                Some(local.local_forces[RZ]),
                Some(local.local_forces[RX]),
            ),
            section: StressSectionProperties::new(
                Some(section.area),
                Some(section.section_modulus),
                Some(section.section_modulus),
                Some(section.torsion_constant),
                Some(section.torsion_radius),
            ),
            pressure: pressure.map(|p| {
                PressureBasis::new(
                    Some(p),
                    Some(section.membrane_radius),
                    Some(section.wall_thickness),
                )
            }),
            statuses: vec![AnalysisStatus::MechanicsSolved],
        });
        for finding in stress.findings {
            diagnostics.push(diag(
                &format!(
                    "diagnostic:stress:{}:{:?}",
                    stable_suffix(&pipe.element_id),
                    finding.code
                ),
                "STRESS_RECOVERY_LIMITED",
                "warning",
                finding.message,
                vec![pipe.element_id.clone()],
            ));
        }
        if let Some(summary) = stress.summary {
            let value = summary.max_normal.abs().max(summary.min_normal.abs()) / 1_000_000.0;
            let result_id = format!("result:stress:{}", stable_suffix(&pipe.element_id));
            if max_stress
                .as_ref()
                .map(|q: &LocatedQuantity| value > q.value)
                .unwrap_or(true)
            {
                max_stress = Some(LocatedQuantity {
                    value: round6(value),
                    unit: "MPa".to_string(),
                    location_ref: pipe.element_id.clone(),
                    result_ref: result_id.clone(),
                });
            }
            results.push(ResultItem {
                id: result_id,
                kind: "open_formula_stress_summary".to_string(),
                value: round6(value),
                unit: "MPa".to_string(),
                entity_ref: pipe.element_id.clone(),
            });
        }
    }

    if let Some(maximum) = &max_displacement {
        if maximum.value > 5.0 {
            diagnostics.push(diag(
                "diagnostic:physics:high-displacement-review",
                "HIGH_DISPLACEMENT_REVIEW",
                "warning",
                "computed preview displacement exceeds the invented review threshold; inspect support/load inputs before relying on trends",
                vec![maximum.result_ref.clone(), maximum.location_ref.clone()],
            ));
        }
    }
    diagnostics.push(diag(
        "diagnostic:physics:rule-inputs-missing",
        "RULE_CHECK_INPUTS_MISSING",
        "warning",
        "rule-check inputs and protected criteria are absent; no compliance result is produced",
        vec![],
    ));

    MechanicsEnvelope {
        schema_version: "0.1.0".to_string(),
        document_kind: "openpipestress.product_preview.mechanics_result".to_string(),
        run_id: "run:preview-linear-static-001".to_string(),
        model_ref: model.project.id,
        status: StatusEnvelope {
            mechanics: "MECHANICS_SOLVED".to_string(),
            rule_check: "RULE_INPUTS_INCOMPLETE".to_string(),
            professional_acceptance: "NOT_PROVIDED".to_string(),
        },
        summary: Summary {
            node_count: model.nodes.len(),
            segment_count: model.pipe_segments.len(),
            support_count: model.supports.len(),
            load_case_count: model.load_cases.len(),
            max_displacement,
            max_open_formula_stress: max_stress,
        },
        results,
        diagnostics,
        professional_boundary: professional_boundary(),
        accepted_model_state_mutated: false,
    }
}

fn build_model(
    model: &PreviewModel,
    materials: &[MaterialInput],
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<BuiltModel> {
    let material_map = materials
        .iter()
        .map(|m| (m.id.as_str(), m))
        .collect::<HashMap<_, _>>();
    let mut node_map = HashMap::new();
    let mut nodes = Vec::new();
    for (index, node) in model.nodes.iter().enumerate() {
        node_map.insert(node.id.as_str(), index);
        match FrameNode::new(index, [node.position.x, node.position.y, node.position.z]) {
            Ok(frame_node) => nodes.push(frame_node),
            Err(error) => diagnostics.push(diag(
                &format!("diagnostic:node:{}", stable_suffix(&node.id)),
                "NODE_INPUT_INVALID",
                "blocking",
                error.to_string(),
                vec![node.id.clone()],
            )),
        }
    }

    let mut pipes = Vec::new();
    let mut frame_elements = Vec::new();
    let mut sections = HashMap::new();
    for pipe in &model.pipe_segments {
        let Some(&from) = node_map.get(pipe.from.as_str()) else {
            diagnostics.push(diag(
                &format!("diagnostic:pipe:{}:from", stable_suffix(&pipe.id)),
                "PIPE_ENDPOINT_UNKNOWN",
                "blocking",
                "pipe from-node is not present in preview model",
                vec![pipe.id.clone(), pipe.from.clone()],
            ));
            continue;
        };
        let Some(&to) = node_map.get(pipe.to.as_str()) else {
            diagnostics.push(diag(
                &format!("diagnostic:pipe:{}:to", stable_suffix(&pipe.id)),
                "PIPE_ENDPOINT_UNKNOWN",
                "blocking",
                "pipe to-node is not present in preview model",
                vec![pipe.id.clone(), pipe.to.clone()],
            ));
            continue;
        };
        let Some(material) = material_map.get(pipe.material.as_str()) else {
            diagnostics.push(diag(&format!("diagnostic:material:{}", stable_suffix(&pipe.material)), "MATERIAL_INPUT_MISSING", "blocking", "pipe material requires explicit elastic and shear modulus inputs; no defaults are applied", vec![pipe.id.clone(), pipe.material.clone()]));
            continue;
        };
        let Some(derived) = derive_pipe_section(&pipe.section, &pipe.id, diagnostics) else {
            continue;
        };
        let section = match StraightPipeSectionProperties::new(
            material.elastic_modulus.value,
            material.shear_modulus.value,
            derived.area,
            derived.second_moment,
            derived.second_moment,
            derived.torsion_constant,
            None,
        ) {
            Ok(section) => section,
            Err(error) => {
                diagnostics.push(diag(
                    &format!("diagnostic:pipe-section:{}", stable_suffix(&pipe.id)),
                    "PIPE_SECTION_INPUT_INVALID",
                    "blocking",
                    error.to_string(),
                    vec![pipe.id.clone()],
                ));
                continue;
            }
        };
        let y_ref = pipe
            .y_reference
            .map(|v| [v.x, v.y, v.z])
            .unwrap_or([0.0, 0.0, 1.0]);
        let element =
            match StraightPipeElement::new(&pipe.id, nodes[from], nodes[to], section, y_ref) {
                Ok(element) => element,
                Err(error) => {
                    diagnostics.push(diag(
                        &format!("diagnostic:pipe-element:{}", stable_suffix(&pipe.id)),
                        "PIPE_ELEMENT_INPUT_INVALID",
                        "blocking",
                        error.to_string(),
                        vec![pipe.id.clone()],
                    ));
                    continue;
                }
            };
        frame_elements.push(
            element
                .frame_element()
                .expect("validated straight pipe frame element"),
        );
        sections.insert(pipe.id.clone(), derived);
        pipes.push(element);
    }

    let supports = model
        .supports
        .iter()
        .filter_map(|support| {
            let Some(&node) = node_map.get(support.node.as_str()) else {
                diagnostics.push(diag(
                    &format!("diagnostic:support:{}:node", stable_suffix(&support.id)),
                    "SUPPORT_NODE_UNKNOWN",
                    "blocking",
                    "support node is not present in preview model",
                    vec![support.id.clone(), support.node.clone()],
                ));
                return None;
            };
            let dofs = support
                .restraints
                .iter()
                .filter_map(|dof| match parse_dof(dof) {
                    Ok(dof) => Some(dof),
                    Err(message) => {
                        diagnostics.push(diag(
                            &format!("diagnostic:support:{}:dof", stable_suffix(&support.id)),
                            "SUPPORT_DOF_INVALID",
                            "blocking",
                            message,
                            vec![support.id.clone()],
                        ));
                        None
                    }
                })
                .collect::<Vec<_>>();
            if support.family.as_deref() == Some("spring") {
                let stiffness = support.stiffness.as_ref().and_then(|input| {
                    let dimension = if parse_dof(&input.dof).ok()?.is_translational() {
                        QuantityDimension::TranslationalStiffness
                    } else {
                        QuantityDimension::RotationalStiffness
                    };
                    SupportQuantity::positive(input.value.value, dimension).ok()
                });
                Some(LinearSupport::spring(
                    &support.id,
                    node,
                    dofs.first().copied().unwrap_or(FrameDof::Uz),
                    stiffness,
                ))
            } else if dofs.len() == 6 {
                Some(LinearSupport::anchor(&support.id, node))
            } else {
                Some(LinearSupport {
                    support_id: support.id.clone(),
                    family: SupportFamily::Guide,
                    node_index: node,
                    restrained_dofs: dofs,
                    stiffness: None,
                    imposed_displacement: None,
                })
            }
        })
        .collect::<Vec<_>>();

    let pipe_map = model
        .pipe_segments
        .iter()
        .enumerate()
        .map(|(i, p)| (p.id.as_str(), i))
        .collect::<HashMap<_, _>>();
    let loads = model
        .load_cases
        .iter()
        .flat_map(|case| case.primitive_loads.iter())
        .filter_map(|load| {
            let category = parse_category(&load.category);
            let direction = parse_direction(&load.direction);
            let dimension = parse_load_dimension(&load.dimension);
            match (category, direction, dimension) {
                (Ok(category), Ok(direction), Ok(dimension)) => {
                    let Ok(quantity) = LoadQuantity::new(load.magnitude.value, dimension) else {
                        diagnostics.push(diag(
                            &format!("diagnostic:load:{}:quantity", stable_suffix(&load.id)),
                            "LOAD_MAGNITUDE_INVALID",
                            "blocking",
                            "load magnitude must be finite",
                            vec![load.id.clone()],
                        ));
                        return None;
                    };
                    match &load.target {
                        LoadTargetInput::Node { node } => {
                            let Some(&node_index) = node_map.get(node.as_str()) else {
                                diagnostics.push(diag(
                                    &format!("diagnostic:load:{}:node", stable_suffix(&load.id)),
                                    "LOAD_NODE_UNKNOWN",
                                    "blocking",
                                    "load target node is not present in preview model",
                                    vec![load.id.clone(), node.clone()],
                                ));
                                return None;
                            };
                            Some(PrimitiveLoad::nodal_force(
                                &load.id, category, node_index, direction, quantity,
                            ))
                        }
                        LoadTargetInput::Element { pipe } => {
                            let Some(&pipe_index) = pipe_map.get(pipe.as_str()) else {
                                diagnostics.push(diag(
                                    &format!("diagnostic:load:{}:pipe", stable_suffix(&load.id)),
                                    "LOAD_PIPE_UNKNOWN",
                                    "blocking",
                                    "load target pipe is not present in preview model",
                                    vec![load.id.clone(), pipe.clone()],
                                ));
                                return None;
                            };
                            Some(PrimitiveLoad::uniform_element_load(
                                &load.id, category, pipe_index, direction, quantity,
                            ))
                        }
                    }
                }
                _ => {
                    diagnostics.push(diag(
                        &format!("diagnostic:load:{}:kind", stable_suffix(&load.id)),
                        "LOAD_INPUT_INVALID",
                        "blocking",
                        "load category, direction, and dimension must use supported preview values",
                        vec![load.id.clone()],
                    ));
                    None
                }
            }
        })
        .collect::<Vec<_>>();

    Some(BuiltModel {
        nodes,
        pipes,
        frame_elements,
        supports,
        loads,
        sections,
    })
}

fn derive_pipe_section(
    input: &PipeSectionInput,
    pipe_id: &str,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<DerivedSection> {
    let od = input.outside_diameter.value;
    let thickness = input.wall_thickness.value;
    if !od.is_finite()
        || !thickness.is_finite()
        || od <= 0.0
        || thickness <= 0.0
        || 2.0 * thickness >= od
    {
        diagnostics.push(diag(&format!("diagnostic:section:{}", stable_suffix(pipe_id)), "PIPE_DIMENSION_INVALID", "blocking", "outside diameter and wall thickness must be explicit positive values with wall thickness less than radius", vec![pipe_id.to_string()]));
        return None;
    }
    let id = od - 2.0 * thickness;
    let area = PI * (od.powi(2) - id.powi(2)) / 4.0;
    let second_moment = PI * (od.powi(4) - id.powi(4)) / 64.0;
    let torsion_constant = 2.0 * second_moment;
    Some(DerivedSection {
        area,
        second_moment,
        torsion_constant,
        section_modulus: second_moment / (od / 2.0),
        torsion_radius: od / 2.0,
        membrane_radius: (od - thickness) / 2.0,
        wall_thickness: thickness,
    })
}

fn add_uniform_element_loads(
    force: &mut [f64],
    model: &PreviewModel,
    loads: &[open_pipe_stress_primitive_loads::ElementUniformLoadContribution],
    pipes: &[StraightPipeElement],
) {
    for load in loads {
        if load.magnitude.dimension == LoadDimension::Pressure {
            continue;
        }
        let Ok(length) = pipes[load.element_index].length() else {
            continue;
        };
        let pipe = &model.pipe_segments[load.element_index];
        let Some(i) = node_index(model, &pipe.from) else {
            continue;
        };
        let Some(j) = node_index(model, &pipe.to) else {
            continue;
        };
        let dof = load.direction.dof_index();
        let share = load.magnitude.value * length / 2.0;
        force[i * DOF_PER_NODE + dof] += share;
        force[j * DOF_PER_NODE + dof] += share;
    }
}

fn blocked_envelope(model: PreviewModel, diagnostics: Vec<Diagnostic>) -> MechanicsEnvelope {
    MechanicsEnvelope {
        schema_version: "0.1.0".to_string(),
        document_kind: "openpipestress.product_preview.mechanics_result".to_string(),
        run_id: "run:preview-linear-static-blocked".to_string(),
        model_ref: model.project.id,
        status: StatusEnvelope {
            mechanics: "MODEL_INCOMPLETE".to_string(),
            rule_check: "RULE_INPUTS_INCOMPLETE".to_string(),
            professional_acceptance: "NOT_PROVIDED".to_string(),
        },
        summary: Summary {
            node_count: model.nodes.len(),
            segment_count: model.pipe_segments.len(),
            support_count: model.supports.len(),
            load_case_count: model.load_cases.len(),
            max_displacement: None,
            max_open_formula_stress: None,
        },
        results: vec![],
        diagnostics,
        professional_boundary: professional_boundary(),
        accepted_model_state_mutated: false,
    }
}

fn solver_blocked(
    model: PreviewModel,
    mut diagnostics: Vec<Diagnostic>,
    error: FrameKernelError,
) -> MechanicsEnvelope {
    diagnostics.push(diag(
        "diagnostic:physics:solver",
        "SOLVER_SYSTEM_BLOCKED",
        "blocking",
        error.to_string(),
        vec!["model".to_string()],
    ));
    blocked_envelope(model, diagnostics)
}

fn pressure_for_pipe(model: &PreviewModel, pipe_index: usize, pipe_id: &str) -> Option<f64> {
    model
        .load_cases
        .iter()
        .flat_map(|case| &case.primitive_loads)
        .find_map(|load| match &load.target {
            LoadTargetInput::Element { pipe }
                if pipe == pipe_id && load.dimension == "pressure" =>
            {
                Some(load.magnitude.value)
            }
            LoadTargetInput::Element { pipe }
                if model.pipe_segments.get(pipe_index).map(|p| &p.id) == Some(pipe)
                    && load.dimension == "pressure" =>
            {
                Some(load.magnitude.value)
            }
            _ => None,
        })
}

fn displacement_magnitude(displacements: &[f64], node_index: usize) -> f64 {
    let base = node_index * DOF_PER_NODE;
    (displacements[base + UX].powi(2)
        + displacements[base + UY].powi(2)
        + displacements[base + UZ].powi(2))
    .sqrt()
}

fn multiply_matrix_vector(matrix: &[Vec<f64>], vector: &[f64]) -> Vec<f64> {
    matrix
        .iter()
        .map(|row| row.iter().zip(vector).map(|(a, b)| a * b).sum())
        .collect()
}

fn node_index(model: &PreviewModel, id: &str) -> Option<usize> {
    model.nodes.iter().position(|node| node.id == id)
}

fn parse_dof(value: &str) -> Result<FrameDof, String> {
    match value {
        "UX" | "Ux" | "ux" => Ok(FrameDof::Ux),
        "UY" | "Uy" | "uy" => Ok(FrameDof::Uy),
        "UZ" | "Uz" | "uz" => Ok(FrameDof::Uz),
        "RX" | "Rx" | "rx" => Ok(FrameDof::Rx),
        "RY" | "Ry" | "ry" => Ok(FrameDof::Ry),
        "RZ" | "Rz" | "rz" => Ok(FrameDof::Rz),
        _ => Err(format!("unsupported frame DOF {value}")),
    }
}

fn parse_direction(value: &str) -> Result<LoadDirection, String> {
    match value {
        "global_x" | "GLOBAL_X" => Ok(LoadDirection::GlobalX),
        "global_y" | "GLOBAL_Y" => Ok(LoadDirection::GlobalY),
        "global_z" | "GLOBAL_Z" => Ok(LoadDirection::GlobalZ),
        _ => parse_dof(value).map(LoadDirection::Dof),
    }
}

fn parse_category(value: &str) -> Result<PrimitiveLoadCategory, String> {
    match value {
        "weight" => Ok(PrimitiveLoadCategory::Weight),
        "pressure" => Ok(PrimitiveLoadCategory::Pressure),
        "thermal" => Ok(PrimitiveLoadCategory::Thermal),
        "hydrotest" => Ok(PrimitiveLoadCategory::Hydrotest),
        "wind" => Ok(PrimitiveLoadCategory::Wind),
        "seismic" => Ok(PrimitiveLoadCategory::Seismic),
        "occasional" => Ok(PrimitiveLoadCategory::Occasional),
        _ => Err(format!("unsupported load category {value}")),
    }
}

fn parse_load_dimension(value: &str) -> Result<LoadDimension, String> {
    match value {
        "force" => Ok(LoadDimension::Force),
        "moment" => Ok(LoadDimension::Moment),
        "force_per_length" => Ok(LoadDimension::ForcePerLength),
        "pressure" => Ok(LoadDimension::Pressure),
        "temperature_change" => Ok(LoadDimension::TemperatureChange),
        "acceleration" => Ok(LoadDimension::Acceleration),
        "displacement" => Ok(LoadDimension::Displacement),
        "rotation" => Ok(LoadDimension::Rotation),
        _ => Err(format!("unsupported load dimension {value}")),
    }
}

fn dof_index(dof: FrameDof) -> usize {
    match dof {
        FrameDof::Ux => UX,
        FrameDof::Uy => UY,
        FrameDof::Uz => UZ,
        FrameDof::Rx => RX,
        FrameDof::Ry => RY,
        FrameDof::Rz => RZ,
    }
}

fn has_blocking(diagnostics: &[Diagnostic]) -> bool {
    diagnostics.iter().any(|d| d.severity == "blocking")
}

fn professional_boundary() -> ProfessionalBoundary {
    ProfessionalBoundary {
        human_review_required: true,
        software_makes_compliance_claim: false,
        software_makes_certification_claim: false,
        software_makes_sealing_claim: false,
        software_makes_approval_claim: false,
    }
}

fn diag(
    id: &str,
    code: &str,
    severity: &str,
    message: impl Into<String>,
    affected_refs: Vec<String>,
) -> Diagnostic {
    Diagnostic {
        id: id.to_string(),
        code: code.to_string(),
        severity: severity.to_string(),
        message: message.into(),
        source: Some("core/product_physics".to_string()),
        affected_refs,
    }
}

fn stable_suffix(id: &str) -> String {
    id.replace(':', "-")
}

fn round6(value: f64) -> f64 {
    (value * 1_000_000.0).round() / 1_000_000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> LinearStaticPreviewRequest {
        serde_json::from_str(include_str!(
            "../../../fixtures/product_preview/invented_preview_model.json"
        ))
        .map(|model| LinearStaticPreviewRequest {
            model,
            materials: invented_materials(),
        })
        .unwrap()
    }

    fn invented_materials() -> Vec<MaterialInput> {
        vec![MaterialInput {
            id: "material:invented-carbon-steel".to_string(),
            elastic_modulus: Quantity {
                value: 200_000_000_000.0,
                unit: "Pa".to_string(),
            },
            shear_modulus: Quantity {
                value: 77_000_000_000.0,
                unit: "Pa".to_string(),
            },
        }]
    }

    #[test]
    fn valid_invented_model_solves_deterministically() {
        let result = run_linear_static_preview(request());

        assert_eq!(result.status.mechanics, "MECHANICS_SOLVED");
        assert!(!result.results.is_empty());
        assert!(result.summary.max_displacement.as_ref().unwrap().value > 0.0);
        assert!(result
            .results
            .iter()
            .any(|item| item.id == "result:disp:node-N-140"));
    }

    #[test]
    fn missing_material_blocks_with_diagnostic() {
        let mut request = request();
        request.materials.clear();
        request.model.materials.clear();

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "MATERIAL_INPUT_MISSING"));
        assert!(result.results.is_empty());
    }

    #[test]
    fn missing_load_input_blocks_with_diagnostic() {
        let mut request = request();
        request.model.load_cases[0].primitive_loads[0]
            .magnitude
            .value = f64::NAN;

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "LOAD_MAGNITUDE_INVALID"));
    }

    #[test]
    fn under_restrained_model_reports_solver_diagnostic() {
        let mut request = request();
        request.model.supports.truncate(1);
        request.model.supports[0].restraints = vec!["UZ".to_string()];

        let result = run_linear_static_preview(request);

        assert_eq!(result.status.mechanics, "MODEL_INCOMPLETE");
        assert!(result
            .diagnostics
            .iter()
            .any(|item| item.code == "SOLVER_SYSTEM_BLOCKED"));
    }

    #[test]
    fn envelope_keeps_status_boundaries_separate() {
        let result = run_linear_static_preview(request());

        assert_eq!(result.status.rule_check, "RULE_INPUTS_INCOMPLETE");
        assert_eq!(result.status.professional_acceptance, "NOT_PROVIDED");
        assert!(result.professional_boundary.human_review_required);
        assert!(!result.professional_boundary.software_makes_compliance_claim);
        assert!(!result.accepted_model_state_mutated);
    }
}
