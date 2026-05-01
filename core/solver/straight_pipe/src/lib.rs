//! Straight pipe element mechanics.
//!
//! This crate adapts explicit straight-pipe properties into the frame-kernel
//! solver boundary. It does not provide pipe tables, material defaults,
//! code-specific checks, protected standards data, or engineering approval.

use open_pipe_stress_frame_kernel::{
    FrameElement, FrameKernelError, FrameNode, FrameOrientation, FrameSection, Matrix12,
    DOF_PER_NODE, ELEMENT_DOF,
};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StraightPipeSectionProperties {
    pub elastic_modulus: f64,
    pub shear_modulus: f64,
    pub area: f64,
    pub second_moment_y: f64,
    pub second_moment_z: f64,
    pub torsion_constant: f64,
    pub mass_per_length: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StraightPipeElement {
    pub element_id: String,
    pub node_i: FrameNode,
    pub node_j: FrameNode,
    pub section: StraightPipeSectionProperties,
    pub y_reference: [f64; 3],
}

#[derive(Debug, Clone, PartialEq)]
pub struct WeightHook {
    pub mass_per_length: f64,
    pub gravity: f64,
    pub weight_force_per_length: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalElementForces {
    pub local_displacements: [f64; ELEMENT_DOF],
    pub local_forces: [f64; ELEMENT_DOF],
}

#[derive(Debug, Clone, PartialEq)]
pub enum StraightPipeError {
    MissingInput { name: &'static str },
    NonFiniteInput { name: &'static str, value: f64 },
    NonPositiveInput { name: &'static str, value: f64 },
    InvalidDisplacementLength { expected: usize, actual: usize },
    FrameKernel(FrameKernelError),
}

impl fmt::Display for StraightPipeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingInput { name } => write!(f, "missing solve-required input {name}"),
            Self::NonFiniteInput { name, value } => {
                write!(f, "{name} must be finite, got {value}")
            }
            Self::NonPositiveInput { name, value } => {
                write!(f, "{name} must be positive, got {value}")
            }
            Self::InvalidDisplacementLength { expected, actual } => {
                write!(
                    f,
                    "displacement vector length must be {expected}, got {actual}"
                )
            }
            Self::FrameKernel(error) => write!(f, "{error}"),
        }
    }
}

impl Error for StraightPipeError {}

impl From<FrameKernelError> for StraightPipeError {
    fn from(error: FrameKernelError) -> Self {
        Self::FrameKernel(error)
    }
}

impl StraightPipeSectionProperties {
    pub fn new(
        elastic_modulus: f64,
        shear_modulus: f64,
        area: f64,
        second_moment_y: f64,
        second_moment_z: f64,
        torsion_constant: f64,
        mass_per_length: Option<f64>,
    ) -> Result<Self, StraightPipeError> {
        validate_positive_finite("elastic_modulus", elastic_modulus)?;
        validate_positive_finite("shear_modulus", shear_modulus)?;
        validate_positive_finite("area", area)?;
        validate_positive_finite("second_moment_y", second_moment_y)?;
        validate_positive_finite("second_moment_z", second_moment_z)?;
        validate_positive_finite("torsion_constant", torsion_constant)?;
        if let Some(value) = mass_per_length {
            validate_positive_finite("mass_per_length", value)?;
        }

        Ok(Self {
            elastic_modulus,
            shear_modulus,
            area,
            second_moment_y,
            second_moment_z,
            torsion_constant,
            mass_per_length,
        })
    }

    pub fn frame_section(&self) -> Result<FrameSection, StraightPipeError> {
        Ok(FrameSection::new(
            self.elastic_modulus,
            self.shear_modulus,
            self.area,
            self.second_moment_y,
            self.second_moment_z,
            self.torsion_constant,
        )?)
    }
}

impl StraightPipeElement {
    pub fn new(
        element_id: impl Into<String>,
        node_i: FrameNode,
        node_j: FrameNode,
        section: StraightPipeSectionProperties,
        y_reference: [f64; 3],
    ) -> Result<Self, StraightPipeError> {
        let element = Self {
            element_id: element_id.into(),
            node_i,
            node_j,
            section,
            y_reference,
        };
        element.frame_element()?;
        Ok(element)
    }

    pub fn length(&self) -> Result<f64, StraightPipeError> {
        Ok(self.frame_element()?.length()?)
    }

    pub fn frame_element(&self) -> Result<FrameElement, StraightPipeError> {
        Ok(FrameElement::new(
            self.node_i,
            self.node_j,
            self.section.frame_section()?,
            self.y_reference,
        )?)
    }

    pub fn local_stiffness(&self) -> Result<Matrix12, StraightPipeError> {
        Ok(self.frame_element()?.local_stiffness()?)
    }

    pub fn global_stiffness(&self) -> Result<Matrix12, StraightPipeError> {
        Ok(self.frame_element()?.global_stiffness()?)
    }

    pub fn weight_hook(&self, gravity: f64) -> Result<WeightHook, StraightPipeError> {
        validate_positive_finite("gravity", gravity)?;
        let mass_per_length =
            self.section
                .mass_per_length
                .ok_or(StraightPipeError::MissingInput {
                    name: "mass_per_length",
                })?;

        Ok(WeightHook {
            mass_per_length,
            gravity,
            weight_force_per_length: mass_per_length * gravity,
        })
    }

    pub fn recover_local_forces(
        &self,
        global_element_displacements: &[f64],
    ) -> Result<LocalElementForces, StraightPipeError> {
        if global_element_displacements.len() != ELEMENT_DOF {
            return Err(StraightPipeError::InvalidDisplacementLength {
                expected: ELEMENT_DOF,
                actual: global_element_displacements.len(),
            });
        }
        validate_finite_slice("global_element_displacements", global_element_displacements)?;

        let frame_element = self.frame_element()?;
        let orientation = frame_element.orientation()?;
        let local_stiffness = frame_element.local_stiffness()?;
        let local_displacements =
            transform_global_displacements_to_local(&orientation, global_element_displacements);
        let local_forces = multiply_matrix_vector(&local_stiffness, &local_displacements);

        Ok(LocalElementForces {
            local_displacements,
            local_forces,
        })
    }

    pub fn recover_local_forces_from_global_model(
        &self,
        global_model_displacements: &[f64],
    ) -> Result<LocalElementForces, StraightPipeError> {
        let required = (self.node_i.index.max(self.node_j.index) + 1) * DOF_PER_NODE;
        if global_model_displacements.len() < required {
            return Err(StraightPipeError::InvalidDisplacementLength {
                expected: required,
                actual: global_model_displacements.len(),
            });
        }
        validate_finite_slice("global_model_displacements", global_model_displacements)?;

        let mut element_displacements = [0.0; ELEMENT_DOF];
        copy_node_displacements(
            global_model_displacements,
            self.node_i.index,
            &mut element_displacements[0..DOF_PER_NODE],
        );
        copy_node_displacements(
            global_model_displacements,
            self.node_j.index,
            &mut element_displacements[DOF_PER_NODE..ELEMENT_DOF],
        );

        self.recover_local_forces(&element_displacements)
    }
}

fn transform_global_displacements_to_local(
    orientation: &FrameOrientation,
    global_element_displacements: &[f64],
) -> [f64; ELEMENT_DOF] {
    let transform = orientation.transformation_matrix();
    let mut local = [0.0; ELEMENT_DOF];
    for row in 0..ELEMENT_DOF {
        for col in 0..ELEMENT_DOF {
            local[row] += transform[row][col] * global_element_displacements[col];
        }
    }
    local
}

fn multiply_matrix_vector(matrix: &Matrix12, vector: &[f64; ELEMENT_DOF]) -> [f64; ELEMENT_DOF] {
    let mut result = [0.0; ELEMENT_DOF];
    for row in 0..ELEMENT_DOF {
        for col in 0..ELEMENT_DOF {
            result[row] += matrix[row][col] * vector[col];
        }
    }
    result
}

fn copy_node_displacements(source: &[f64], node_index: usize, target: &mut [f64]) {
    let offset = node_index * DOF_PER_NODE;
    target.copy_from_slice(&source[offset..offset + DOF_PER_NODE]);
}

fn validate_positive_finite(name: &'static str, value: f64) -> Result<(), StraightPipeError> {
    if !value.is_finite() {
        return Err(StraightPipeError::NonFiniteInput { name, value });
    }
    if value <= 0.0 {
        return Err(StraightPipeError::NonPositiveInput { name, value });
    }
    Ok(())
}

fn validate_finite_slice(name: &'static str, values: &[f64]) -> Result<(), StraightPipeError> {
    for &value in values {
        if !value.is_finite() {
            return Err(StraightPipeError::NonFiniteInput { name, value });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_pipe_stress_frame_kernel::{UX, UY};

    fn section(mass_per_length: Option<f64>) -> StraightPipeSectionProperties {
        StraightPipeSectionProperties::new(
            2.0e11,
            7.7e10,
            0.01,
            8.0e-6,
            9.0e-6,
            1.7e-5,
            mass_per_length,
        )
        .unwrap()
    }

    fn element(mass_per_length: Option<f64>) -> StraightPipeElement {
        StraightPipeElement::new(
            "pipe-1",
            FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap(),
            FrameNode::new(1, [2.0, 0.0, 0.0]).unwrap(),
            section(mass_per_length),
            [0.0, 1.0, 0.0],
        )
        .unwrap()
    }

    #[test]
    fn straight_pipe_stiffness_matches_frame_kernel_boundary() {
        let pipe = element(Some(25.0));
        let frame = pipe.frame_element().unwrap();

        assert_eq!(pipe.length().unwrap(), 2.0);
        assert_eq!(
            pipe.local_stiffness().unwrap(),
            frame.local_stiffness().unwrap()
        );
        assert_eq!(
            pipe.global_stiffness().unwrap(),
            frame.global_stiffness().unwrap()
        );
    }

    #[test]
    fn weight_hook_requires_explicit_mass_per_length() {
        let pipe = element(Some(25.0));
        let hook = pipe.weight_hook(9.81).unwrap();

        assert_eq!(hook.mass_per_length, 25.0);
        assert_eq!(hook.gravity, 9.81);
        assert_eq!(hook.weight_force_per_length, 245.25);

        let missing = element(None).weight_hook(9.81).unwrap_err();
        assert_eq!(
            missing,
            StraightPipeError::MissingInput {
                name: "mass_per_length"
            }
        );
    }

    #[test]
    fn section_properties_reject_nonpositive_inputs() {
        let error =
            StraightPipeSectionProperties::new(2.0e11, 7.7e10, 0.0, 8.0e-6, 9.0e-6, 1.7e-5, None)
                .unwrap_err();

        assert_eq!(
            error,
            StraightPipeError::NonPositiveInput {
                name: "area",
                value: 0.0
            }
        );
    }

    #[test]
    fn recovers_local_axial_end_forces_from_global_displacements() {
        let pipe = element(Some(25.0));
        let axial_extension = 0.001;
        let mut displacements = [0.0; ELEMENT_DOF];
        displacements[DOF_PER_NODE + UX] = axial_extension;

        let recovered = pipe.recover_local_forces(&displacements).unwrap();
        let expected_axial = pipe.section.elastic_modulus * pipe.section.area
            / pipe.length().unwrap()
            * axial_extension;

        assert!((recovered.local_forces[UX] + expected_axial).abs() < 1.0e-6);
        assert!((recovered.local_forces[DOF_PER_NODE + UX] - expected_axial).abs() < 1.0e-6);
    }

    #[test]
    fn recovers_local_bending_shear_from_global_model_displacements() {
        let pipe = element(Some(25.0));
        let transverse_displacement = 0.002;
        let mut model_displacements = vec![0.0; 2 * DOF_PER_NODE];
        model_displacements[DOF_PER_NODE + UY] = transverse_displacement;

        let recovered = pipe
            .recover_local_forces_from_global_model(&model_displacements)
            .unwrap();

        assert!(recovered.local_forces[UY].abs() > 0.0);
        assert!(recovered.local_forces[DOF_PER_NODE + UY].abs() > 0.0);
        assert!(
            (recovered.local_forces[UY] + recovered.local_forces[DOF_PER_NODE + UY]).abs() < 1.0e-6
        );
    }

    #[test]
    fn recovery_rejects_invalid_displacement_length() {
        let pipe = element(Some(25.0));

        let error = pipe.recover_local_forces(&[0.0; 3]).unwrap_err();

        assert_eq!(
            error,
            StraightPipeError::InvalidDisplacementLength {
                expected: ELEMENT_DOF,
                actual: 3
            }
        );
    }
}
