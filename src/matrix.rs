use quaternion::Quaternion;

pub fn mul_matrices(mat1: [[f32; 4]; 4], mat2: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    [
        [
            vec4_dot(mat1[0], col_of_matrix(mat2, 0)),
            vec4_dot(mat1[0], col_of_matrix(mat2, 1)),
            vec4_dot(mat1[0], col_of_matrix(mat2, 2)),
            vec4_dot(mat1[0], col_of_matrix(mat2, 3)),
        ],
        [
            vec4_dot(mat1[1], col_of_matrix(mat2, 0)),
            vec4_dot(mat1[1], col_of_matrix(mat2, 1)),
            vec4_dot(mat1[1], col_of_matrix(mat2, 2)),
            vec4_dot(mat1[1], col_of_matrix(mat2, 3)),
        ],
        [
            vec4_dot(mat1[2], col_of_matrix(mat2, 0)),
            vec4_dot(mat1[2], col_of_matrix(mat2, 1)),
            vec4_dot(mat1[2], col_of_matrix(mat2, 2)),
            vec4_dot(mat1[2], col_of_matrix(mat2, 3)),
        ],
        [
            vec4_dot(mat1[3], col_of_matrix(mat2, 0)),
            vec4_dot(mat1[3], col_of_matrix(mat2, 1)),
            vec4_dot(mat1[3], col_of_matrix(mat2, 2)),
            vec4_dot(mat1[3], col_of_matrix(mat2, 3)),
        ],
    ]
}

/**
 * Creates a rotation-matrix representing a rotation of the quaternion around
 * the specified center
 *
 * http://www.euclideanspace.com/maths/geometry/rotations/conversions/quaternionToMatrix/
 */
pub fn rot_matrix_by(q: &Quaternion<f32>, center: [f32; 3]) -> [[f32; 4]; 4] {
    let sqw: f32 = q.0 * q.0;
    let sqx: f32 = q.1[0] * q.1[0];
    let sqy: f32 = q.1[1] * q.1[1];
    let sqz: f32 = q.1[2] * q.1[2];
    let m00 = sqx - sqy - sqz + sqw; // since sqw + sqx + sqy + sqz =1
    let m11 = -sqx + sqy - sqz + sqw;
    let m22 = -sqx - sqy + sqz + sqw;

    let tmp1 = q.1[0] * q.1[1];
    let tmp2 = q.1[2] * q.0;
    let m01 = 2.0 * (tmp1 + tmp2);
    let m10 = 2.0 * (tmp1 - tmp2);

    let tmp1 = q.1[0] * q.1[2];
    let tmp2 = q.1[1] * q.0;
    let m02 = 2.0 * (tmp1 - tmp2);
    let m20 = 2.0 * (tmp1 + tmp2);

    let tmp1 = q.1[1] * q.1[2];
    let tmp2 = q.1[0] * q.0;
    let m12 = 2.0 * (tmp1 + tmp2);
    let m21 = 2.0 * (tmp1 - tmp2);

    let a1 = center[0];
    let a2 = center[1];
    let a3 = center[2];
    let m03 = a1 - a1 * m00 - a2 * m01 - a3 * m02;
    let m13 = a2 - a1 * m10 - a2 * m11 - a3 * m12;
    let m23 = a3 - a1 * m20 - a2 * m21 - a3 * m22;
    let m30 = 0.0; let m31 = 0.0; let m32 = 0.0;
    let m33 = 1.0;
    [
        [m00, m10, m20, m30],
        [m01, m11, m21, m31],
        [m02, m12, m22, m32],
        [m03, m13, m23, m33],
    ]
}

fn col_of_matrix(mat: [[f32; 4]; 4], col_number: usize) -> [f32; 4] {
    [mat[0][col_number], mat[1][col_number], mat[2][col_number], mat[3][col_number]]
}

fn vec4_dot(a: [f32; 4], b: [f32; 4]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]
}
