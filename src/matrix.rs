

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

fn col_of_matrix(mat: [[f32; 4]; 4], col_number: usize) -> [f32; 4] {
    [mat[0][col_number], mat[1][col_number], mat[2][col_number], mat[3][col_number]]
}

fn vec4_dot(a: [f32; 4], b: [f32; 4]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]
}
