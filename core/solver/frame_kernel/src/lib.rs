//! Code-neutral 3D frame stiffness kernel.
//!
//! This crate contains open mechanics routines only. It does not encode design
//! code compliance checks, protected standards content, or private project data.

use std::error::Error;
use std::fmt;

pub const DOF_PER_NODE: usize = 6;
pub const NODES_PER_ELEMENT: usize = 2;
pub const ELEMENT_DOF: usize = DOF_PER_NODE * NODES_PER_ELEMENT;
pub const UX: usize = 0;
pub const UY: usize = 1;
pub const UZ: usize = 2;
pub const RX: usize = 3;
pub const RY: usize = 4;
pub const RZ: usize = 5;

const AXIS_TOLERANCE: f64 = 1.0e-12;
const SINGULAR_TOLERANCE: f64 = 1.0e-12;

pub type Matrix3 = [[f64; 3]; 3];
pub type Matrix12 = [[f64; ELEMENT_DOF]; ELEMENT_DOF];
pub type DenseMatrix = Vec<Vec<f64>>;
pub type DenseVector = Vec<f64>;

#[derive(Debug, Clone, PartialEq)]
pub enum FrameKernelError {
    NonFiniteInput {
        name: &'static str,
        value: f64,
    },
    NonPositiveInput {
        name: &'static str,
        value: f64,
    },
    DegenerateAxis {
        detail: &'static str,
    },
    InvalidNodeIndex {
        node_index: usize,
        node_count: usize,
    },
    InvalidMatrixDimensions {
        rows: usize,
        cols: usize,
    },
    InvalidVectorLength {
        expected: usize,
        actual: usize,
    },
    RepeatedRestrainedDof {
        dof: usize,
    },
    RestrainedDofOutOfRange {
        dof: usize,
        total_dofs: usize,
    },
    SingularSystem {
        pivot: usize,
    },
}

impl fmt::Display for FrameKernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteInput { name, value } => {
                write!(f, "{name} must be finite, got {value}")
            }
            Self::NonPositiveInput { name, value } => {
                write!(f, "{name} must be positive, got {value}")
            }
            Self::DegenerateAxis { detail } => write!(f, "degenerate axis definition: {detail}"),
            Self::InvalidNodeIndex {
                node_index,
                node_count,
            } => write!(
                f,
                "node index {node_index} is outside node count {node_count}"
            ),
            Self::InvalidMatrixDimensions { rows, cols } => {
                write!(f, "matrix must be square, got {rows} by {cols}")
            }
            Self::InvalidVectorLength { expected, actual } => {
                write!(f, "vector length must be {expected}, got {actual}")
            }
            Self::RepeatedRestrainedDof { dof } => {
                write!(f, "restrained DOF {dof} was repeated")
            }
            Self::RestrainedDofOutOfRange { dof, total_dofs } => write!(
                f,
                "restrained DOF {dof} is outside total DOF count {total_dofs}"
            ),
            Self::SingularSystem { pivot } => write!(f, "singular system at pivot {pivot}"),
        }
    }
}

impl Error for FrameKernelError {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrameNode {
    pub index: usize,
    pub coordinates: [f64; 3],
}

impl FrameNode {
    pub fn new(index: usize, coordinates: [f64; 3]) -> Result<Self, FrameKernelError> {
        validate_finite_vector("coordinates", coordinates)?;
        Ok(Self { index, coordinates })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrameSection {
    pub elastic_modulus: f64,
    pub shear_modulus: f64,
    pub area: f64,
    pub second_moment_y: f64,
    pub second_moment_z: f64,
    pub torsion_constant: f64,
}

impl FrameSection {
    pub fn new(
        elastic_modulus: f64,
        shear_modulus: f64,
        area: f64,
        second_moment_y: f64,
        second_moment_z: f64,
        torsion_constant: f64,
    ) -> Result<Self, FrameKernelError> {
        validate_positive_finite("elastic_modulus", elastic_modulus)?;
        validate_positive_finite("shear_modulus", shear_modulus)?;
        validate_positive_finite("area", area)?;
        validate_positive_finite("second_moment_y", second_moment_y)?;
        validate_positive_finite("second_moment_z", second_moment_z)?;
        validate_positive_finite("torsion_constant", torsion_constant)?;

        Ok(Self {
            elastic_modulus,
            shear_modulus,
            area,
            second_moment_y,
            second_moment_z,
            torsion_constant,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrameProperties {
    pub section: FrameSection,
    pub length: f64,
}

impl FrameProperties {
    pub fn new(section: FrameSection, length: f64) -> Result<Self, FrameKernelError> {
        validate_positive_finite("length", length)?;
        Ok(Self { section, length })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrameOrientation {
    /// Rows are local unit axes expressed in global coordinates: x, y, z.
    pub local_axes: Matrix3,
}

impl FrameOrientation {
    pub fn from_x_axis_and_y_reference(
        local_x_axis: [f64; 3],
        y_reference: [f64; 3],
    ) -> Result<Self, FrameKernelError> {
        validate_finite_vector("local_x_axis", local_x_axis)?;
        validate_finite_vector("y_reference", y_reference)?;

        let x_axis = normalize(local_x_axis, "local x axis")?;
        let projection = dot(y_reference, x_axis);
        let y_candidate = subtract(y_reference, scale(x_axis, projection));
        let y_axis = normalize(y_candidate, "y reference parallel to local x axis")?;
        let z_axis = cross(x_axis, y_axis);

        Ok(Self {
            local_axes: [x_axis, y_axis, z_axis],
        })
    }

    pub fn transformation_matrix(&self) -> Matrix12 {
        let mut transform = [[0.0; ELEMENT_DOF]; ELEMENT_DOF];

        for node in 0..NODES_PER_ELEMENT {
            let base = node * DOF_PER_NODE;
            copy_rotation_block(&mut transform, base, base, self.local_axes);
            copy_rotation_block(&mut transform, base + RX, base + RX, self.local_axes);
        }

        transform
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrameElement {
    pub node_i: FrameNode,
    pub node_j: FrameNode,
    pub section: FrameSection,
    pub y_reference: [f64; 3],
}

impl FrameElement {
    pub fn new(
        node_i: FrameNode,
        node_j: FrameNode,
        section: FrameSection,
        y_reference: [f64; 3],
    ) -> Result<Self, FrameKernelError> {
        validate_finite_vector("y_reference", y_reference)?;
        let element = Self {
            node_i,
            node_j,
            section,
            y_reference,
        };
        element.properties()?;
        element.orientation()?;
        Ok(element)
    }

    pub fn length(&self) -> Result<f64, FrameKernelError> {
        let delta = subtract(self.node_j.coordinates, self.node_i.coordinates);
        normalize(delta, "element length")?;
        Ok(norm(delta))
    }

    pub fn local_x_axis(&self) -> Result<[f64; 3], FrameKernelError> {
        normalize(
            subtract(self.node_j.coordinates, self.node_i.coordinates),
            "element length",
        )
    }

    pub fn properties(&self) -> Result<FrameProperties, FrameKernelError> {
        FrameProperties::new(self.section, self.length()?)
    }

    pub fn orientation(&self) -> Result<FrameOrientation, FrameKernelError> {
        FrameOrientation::from_x_axis_and_y_reference(self.local_x_axis()?, self.y_reference)
    }

    pub fn local_stiffness(&self) -> Result<Matrix12, FrameKernelError> {
        local_stiffness(self.properties()?)
    }

    pub fn global_stiffness(&self) -> Result<Matrix12, FrameKernelError> {
        let local = self.local_stiffness()?;
        let orientation = self.orientation()?;
        Ok(transform_global_stiffness(&local, &orientation))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReducedSystem {
    pub stiffness: DenseMatrix,
    pub force: DenseVector,
    pub free_dofs: Vec<usize>,
}

pub fn local_stiffness(properties: FrameProperties) -> Result<Matrix12, FrameKernelError> {
    let section = properties.section;
    validate_positive_finite("elastic_modulus", section.elastic_modulus)?;
    validate_positive_finite("shear_modulus", section.shear_modulus)?;
    validate_positive_finite("area", section.area)?;
    validate_positive_finite("second_moment_y", section.second_moment_y)?;
    validate_positive_finite("second_moment_z", section.second_moment_z)?;
    validate_positive_finite("torsion_constant", section.torsion_constant)?;
    validate_positive_finite("length", properties.length)?;

    let e = section.elastic_modulus;
    let g = section.shear_modulus;
    let area = section.area;
    let iy = section.second_moment_y;
    let iz = section.second_moment_z;
    let j = section.torsion_constant;
    let length = properties.length;
    let length2 = length * length;
    let length3 = length2 * length;

    let axial = e * area / length;
    let torsion = g * j / length;
    let bend_y_12 = 12.0 * e * iy / length3;
    let bend_y_6 = 6.0 * e * iy / length2;
    let bend_y_4 = 4.0 * e * iy / length;
    let bend_y_2 = 2.0 * e * iy / length;
    let bend_z_12 = 12.0 * e * iz / length3;
    let bend_z_6 = 6.0 * e * iz / length2;
    let bend_z_4 = 4.0 * e * iz / length;
    let bend_z_2 = 2.0 * e * iz / length;

    let mut stiffness = [[0.0; ELEMENT_DOF]; ELEMENT_DOF];

    set_symmetric(&mut stiffness, UX, UX + DOF_PER_NODE, -axial);
    stiffness[UX][UX] = axial;
    stiffness[UX + DOF_PER_NODE][UX + DOF_PER_NODE] = axial;

    set_symmetric(&mut stiffness, RX, RX + DOF_PER_NODE, -torsion);
    stiffness[RX][RX] = torsion;
    stiffness[RX + DOF_PER_NODE][RX + DOF_PER_NODE] = torsion;

    add_bending_z(&mut stiffness, bend_z_12, bend_z_6, bend_z_4, bend_z_2);
    add_bending_y(&mut stiffness, bend_y_12, bend_y_6, bend_y_4, bend_y_2);

    Ok(stiffness)
}

pub fn transform_global_stiffness(
    local_stiffness: &Matrix12,
    orientation: &FrameOrientation,
) -> Matrix12 {
    let transform = orientation.transformation_matrix();
    multiply_transpose_left(local_stiffness, &transform)
}

pub fn assemble_global_stiffness(
    node_count: usize,
    elements: &[FrameElement],
) -> Result<DenseMatrix, FrameKernelError> {
    let total_dofs = node_count * DOF_PER_NODE;
    let mut global = vec![vec![0.0; total_dofs]; total_dofs];

    for element in elements {
        validate_node_index(element.node_i.index, node_count)?;
        validate_node_index(element.node_j.index, node_count)?;

        let element_stiffness = element.global_stiffness()?;
        let dof_map = element_dof_map(element.node_i.index, element.node_j.index);

        for local_row in 0..ELEMENT_DOF {
            let global_row = dof_map[local_row];
            for local_col in 0..ELEMENT_DOF {
                let global_col = dof_map[local_col];
                global[global_row][global_col] += element_stiffness[local_row][local_col];
            }
        }
    }

    Ok(global)
}

pub fn reduce_system(
    stiffness: &[Vec<f64>],
    force: &[f64],
    restrained_dofs: &[usize],
) -> Result<ReducedSystem, FrameKernelError> {
    let size = validate_square_matrix(stiffness)?;
    if force.len() != size {
        return Err(FrameKernelError::InvalidVectorLength {
            expected: size,
            actual: force.len(),
        });
    }
    validate_finite_slice(force)?;

    let mut restrained = vec![false; size];
    for &dof in restrained_dofs {
        if dof >= size {
            return Err(FrameKernelError::RestrainedDofOutOfRange {
                dof,
                total_dofs: size,
            });
        }
        if restrained[dof] {
            return Err(FrameKernelError::RepeatedRestrainedDof { dof });
        }
        restrained[dof] = true;
    }

    let free_dofs: Vec<usize> = (0..size).filter(|&dof| !restrained[dof]).collect();
    let mut reduced_stiffness = vec![vec![0.0; free_dofs.len()]; free_dofs.len()];
    let mut reduced_force = vec![0.0; free_dofs.len()];

    for (reduced_row, &global_row) in free_dofs.iter().enumerate() {
        reduced_force[reduced_row] = force[global_row];
        for (reduced_col, &global_col) in free_dofs.iter().enumerate() {
            reduced_stiffness[reduced_row][reduced_col] = stiffness[global_row][global_col];
        }
    }

    Ok(ReducedSystem {
        stiffness: reduced_stiffness,
        force: reduced_force,
        free_dofs,
    })
}

pub fn solve_dense(stiffness: &[Vec<f64>], force: &[f64]) -> Result<DenseVector, FrameKernelError> {
    let size = validate_square_matrix(stiffness)?;
    if force.len() != size {
        return Err(FrameKernelError::InvalidVectorLength {
            expected: size,
            actual: force.len(),
        });
    }
    validate_finite_slice(force)?;

    let mut matrix = stiffness.to_vec();
    let mut rhs = force.to_vec();

    for pivot in 0..size {
        let mut pivot_row = pivot;
        let mut pivot_value = matrix[pivot][pivot].abs();

        for (row, values) in matrix.iter().enumerate().skip(pivot + 1) {
            let candidate = values[pivot].abs();
            if candidate > pivot_value {
                pivot_row = row;
                pivot_value = candidate;
            }
        }

        if pivot_value <= SINGULAR_TOLERANCE {
            return Err(FrameKernelError::SingularSystem { pivot });
        }

        if pivot_row != pivot {
            matrix.swap(pivot, pivot_row);
            rhs.swap(pivot, pivot_row);
        }

        let pivot_diagonal = matrix[pivot][pivot];
        let pivot_tail = matrix[pivot][(pivot + 1)..].to_vec();
        for (row_index, row_values) in matrix.iter_mut().enumerate().skip(pivot + 1) {
            let factor = row_values[pivot] / pivot_diagonal;
            row_values[pivot] = 0.0;
            for (entry, pivot_entry) in row_values.iter_mut().skip(pivot + 1).zip(pivot_tail.iter())
            {
                *entry -= factor * pivot_entry;
            }
            rhs[row_index] -= factor * rhs[pivot];
        }
    }

    let mut solution = vec![0.0; size];
    for row in (0..size).rev() {
        let mut sum = rhs[row];
        for (col, value) in matrix[row].iter().enumerate().skip(row + 1) {
            sum -= value * solution[col];
        }
        if matrix[row][row].abs() <= SINGULAR_TOLERANCE {
            return Err(FrameKernelError::SingularSystem { pivot: row });
        }
        solution[row] = sum / matrix[row][row];
    }

    Ok(solution)
}

fn add_bending_z(stiffness: &mut Matrix12, k12: f64, k6: f64, k4: f64, k2: f64) {
    let n1_v = UY;
    let n1_rz = RZ;
    let n2_v = UY + DOF_PER_NODE;
    let n2_rz = RZ + DOF_PER_NODE;
    let indices = [n1_v, n1_rz, n2_v, n2_rz];
    let terms = [
        [k12, k6, -k12, k6],
        [k6, k4, -k6, k2],
        [-k12, -k6, k12, -k6],
        [k6, k2, -k6, k4],
    ];
    add_terms(stiffness, indices, terms);
}

fn add_bending_y(stiffness: &mut Matrix12, k12: f64, k6: f64, k4: f64, k2: f64) {
    let n1_w = UZ;
    let n1_ry = RY;
    let n2_w = UZ + DOF_PER_NODE;
    let n2_ry = RY + DOF_PER_NODE;
    let indices = [n1_w, n1_ry, n2_w, n2_ry];
    let terms = [
        [k12, -k6, -k12, -k6],
        [-k6, k4, k6, k2],
        [-k12, k6, k12, k6],
        [-k6, k2, k6, k4],
    ];
    add_terms(stiffness, indices, terms);
}

fn add_terms(stiffness: &mut Matrix12, indices: [usize; 4], terms: [[f64; 4]; 4]) {
    for row in 0..4 {
        for col in 0..4 {
            stiffness[indices[row]][indices[col]] += terms[row][col];
        }
    }
}

fn set_symmetric(stiffness: &mut Matrix12, row: usize, col: usize, value: f64) {
    stiffness[row][col] = value;
    stiffness[col][row] = value;
}

fn multiply_transpose_left(stiffness: &Matrix12, transform: &Matrix12) -> Matrix12 {
    let mut temp = [[0.0; ELEMENT_DOF]; ELEMENT_DOF];
    for row in 0..ELEMENT_DOF {
        for col in 0..ELEMENT_DOF {
            for inner in 0..ELEMENT_DOF {
                temp[row][col] += stiffness[row][inner] * transform[inner][col];
            }
        }
    }

    let mut result = [[0.0; ELEMENT_DOF]; ELEMENT_DOF];
    for row in 0..ELEMENT_DOF {
        for col in 0..ELEMENT_DOF {
            for inner in 0..ELEMENT_DOF {
                result[row][col] += transform[inner][row] * temp[inner][col];
            }
        }
    }
    result
}

fn copy_rotation_block(matrix: &mut Matrix12, row_offset: usize, col_offset: usize, axes: Matrix3) {
    for row in 0..3 {
        for col in 0..3 {
            matrix[row_offset + row][col_offset + col] = axes[row][col];
        }
    }
}

fn element_dof_map(node_i: usize, node_j: usize) -> [usize; ELEMENT_DOF] {
    let mut map = [0; ELEMENT_DOF];
    for local_dof in 0..DOF_PER_NODE {
        map[local_dof] = node_i * DOF_PER_NODE + local_dof;
        map[DOF_PER_NODE + local_dof] = node_j * DOF_PER_NODE + local_dof;
    }
    map
}

fn validate_square_matrix(matrix: &[Vec<f64>]) -> Result<usize, FrameKernelError> {
    let rows = matrix.len();
    for row in matrix {
        if row.len() != rows {
            return Err(FrameKernelError::InvalidMatrixDimensions {
                rows,
                cols: row.len(),
            });
        }
        validate_finite_slice(row)?;
    }
    Ok(rows)
}

fn validate_node_index(node_index: usize, node_count: usize) -> Result<(), FrameKernelError> {
    if node_index >= node_count {
        return Err(FrameKernelError::InvalidNodeIndex {
            node_index,
            node_count,
        });
    }
    Ok(())
}

fn validate_positive_finite(name: &'static str, value: f64) -> Result<(), FrameKernelError> {
    if !value.is_finite() {
        return Err(FrameKernelError::NonFiniteInput { name, value });
    }
    if value <= 0.0 {
        return Err(FrameKernelError::NonPositiveInput { name, value });
    }
    Ok(())
}

fn validate_finite_vector(name: &'static str, vector: [f64; 3]) -> Result<(), FrameKernelError> {
    for value in vector {
        if !value.is_finite() {
            return Err(FrameKernelError::NonFiniteInput { name, value });
        }
    }
    Ok(())
}

fn validate_finite_slice(values: &[f64]) -> Result<(), FrameKernelError> {
    for &value in values {
        if !value.is_finite() {
            return Err(FrameKernelError::NonFiniteInput {
                name: "matrix/vector entry",
                value,
            });
        }
    }
    Ok(())
}

fn normalize(vector: [f64; 3], detail: &'static str) -> Result<[f64; 3], FrameKernelError> {
    let magnitude = norm(vector);
    if magnitude <= AXIS_TOLERANCE {
        return Err(FrameKernelError::DegenerateAxis { detail });
    }
    Ok(scale(vector, 1.0 / magnitude))
}

fn norm(vector: [f64; 3]) -> f64 {
    dot(vector, vector).sqrt()
}

fn dot(left: [f64; 3], right: [f64; 3]) -> f64 {
    left[0] * right[0] + left[1] * right[1] + left[2] * right[2]
}

fn subtract(left: [f64; 3], right: [f64; 3]) -> [f64; 3] {
    [left[0] - right[0], left[1] - right[1], left[2] - right[2]]
}

fn scale(vector: [f64; 3], scalar: f64) -> [f64; 3] {
    [vector[0] * scalar, vector[1] * scalar, vector[2] * scalar]
}

fn cross(left: [f64; 3], right: [f64; 3]) -> [f64; 3] {
    [
        left[1] * right[2] - left[2] * right[1],
        left[2] * right[0] - left[0] * right[2],
        left[0] * right[1] - left[1] * right[0],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ASSERT_TOLERANCE: f64 = 1.0e-9;

    #[test]
    fn local_stiffness_contains_standard_terms() {
        let section = FrameSection::new(10.0, 20.0, 3.0, 5.0, 7.0, 11.0).unwrap();
        let properties = FrameProperties::new(section, 2.0).unwrap();
        let stiffness = local_stiffness(properties).unwrap();

        assert_close(stiffness[UX][UX], 15.0);
        assert_close(stiffness[UX][UX + DOF_PER_NODE], -15.0);
        assert_close(stiffness[RX][RX], 110.0);
        assert_close(stiffness[RX][RX + DOF_PER_NODE], -110.0);

        assert_close(stiffness[UY][UY], 105.0);
        assert_close(stiffness[UY][RZ], 105.0);
        assert_close(stiffness[RZ][RZ], 140.0);
        assert_close(stiffness[RZ][RZ + DOF_PER_NODE], 70.0);

        assert_close(stiffness[UZ][UZ], 75.0);
        assert_close(stiffness[UZ][RY], -75.0);
        assert_close(stiffness[RY][RY], 100.0);
        assert_close(stiffness[RY][RY + DOF_PER_NODE], 50.0);
        assert_symmetric_12(&stiffness);
    }

    #[test]
    fn global_x_member_transform_matches_local_stiffness() {
        let section = FrameSection::new(10.0, 20.0, 3.0, 5.0, 7.0, 11.0).unwrap();
        let properties = FrameProperties::new(section, 2.0).unwrap();
        let local = local_stiffness(properties).unwrap();
        let orientation =
            FrameOrientation::from_x_axis_and_y_reference([1.0, 0.0, 0.0], [0.0, 1.0, 0.0])
                .unwrap();

        let global = transform_global_stiffness(&local, &orientation);

        for row in 0..ELEMENT_DOF {
            for col in 0..ELEMENT_DOF {
                assert_close(global[row][col], local[row][col]);
            }
        }
    }

    #[test]
    fn rotated_member_transform_preserves_symmetry() {
        let section = FrameSection::new(10.0, 20.0, 3.0, 5.0, 7.0, 11.0).unwrap();
        let properties = FrameProperties::new(section, 2.0).unwrap();
        let local = local_stiffness(properties).unwrap();
        let orientation =
            FrameOrientation::from_x_axis_and_y_reference([0.0, 1.0, 0.0], [0.0, 0.0, 1.0])
                .unwrap();

        let global = transform_global_stiffness(&local, &orientation);

        assert_symmetric_12(&global);
    }

    #[test]
    fn reduction_removes_restrained_dofs() {
        let stiffness = vec![
            vec![10.0, 1.0, 2.0, 3.0],
            vec![1.0, 20.0, 4.0, 5.0],
            vec![2.0, 4.0, 30.0, 6.0],
            vec![3.0, 5.0, 6.0, 40.0],
        ];
        let force = vec![7.0, 8.0, 9.0, 10.0];

        let reduced = reduce_system(&stiffness, &force, &[1, 3]).unwrap();

        assert_eq!(reduced.free_dofs, vec![0, 2]);
        assert_eq!(reduced.force, vec![7.0, 9.0]);
        assert_eq!(reduced.stiffness, vec![vec![10.0, 2.0], vec![2.0, 30.0]]);
    }

    #[test]
    fn simple_axial_bar_solve_uses_reduced_system() {
        let section = FrameSection::new(1000.0, 400.0, 2.0, 1.0, 1.0, 1.0).unwrap();
        let node_i = FrameNode::new(0, [0.0, 0.0, 0.0]).unwrap();
        let node_j = FrameNode::new(1, [2.0, 0.0, 0.0]).unwrap();
        let element = FrameElement::new(node_i, node_j, section, [0.0, 1.0, 0.0]).unwrap();
        let stiffness = assemble_global_stiffness(2, &[element]).unwrap();
        let mut force = vec![0.0; ELEMENT_DOF];
        force[UX + DOF_PER_NODE] = 100.0;
        let restrained: Vec<usize> = (0..ELEMENT_DOF)
            .filter(|&dof| dof != UX + DOF_PER_NODE)
            .collect();

        let reduced = reduce_system(&stiffness, &force, &restrained).unwrap();
        let displacement = solve_dense(&reduced.stiffness, &reduced.force).unwrap();

        assert_eq!(reduced.free_dofs, vec![UX + DOF_PER_NODE]);
        assert_close(displacement[0], 0.1);
    }

    #[test]
    fn singular_system_is_detected() {
        let stiffness = vec![vec![1.0, 2.0], vec![2.0, 4.0]];
        let force = vec![3.0, 6.0];

        let error = solve_dense(&stiffness, &force).unwrap_err();

        assert_eq!(error, FrameKernelError::SingularSystem { pivot: 1 });
    }

    #[test]
    fn degenerate_orientation_is_rejected() {
        let error = FrameOrientation::from_x_axis_and_y_reference([1.0, 0.0, 0.0], [2.0, 0.0, 0.0])
            .unwrap_err();

        assert_eq!(
            error,
            FrameKernelError::DegenerateAxis {
                detail: "y reference parallel to local x axis"
            }
        );
    }

    fn assert_symmetric_12(matrix: &Matrix12) {
        for (row, row_values) in matrix.iter().enumerate() {
            for (col, value) in row_values.iter().enumerate() {
                assert_close(*value, matrix[col][row]);
            }
        }
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() <= ASSERT_TOLERANCE,
            "expected {expected}, got {actual}"
        );
    }
}
