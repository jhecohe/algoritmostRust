pub fn zero_matrix_optimized() {}

pub fn zero_matrix(mut matrix: [[u16; 5]; 4]) -> [[u16; 5]; 4] {
    // Complejidad no optimo:
    // Temporal: O(F*C + Z*(F+C))
    // Espacial: O(F*C)
    // Tendriamos que recorrer toda la matriz lo que significa que tnemos F*C
    // Depues tendriamos que validar los ceros y recorrer la fila y la columna Z*(F+C)
    // Finalmente la complejidad quedaria asi => O(F*C + Z*(F+C))

    let mut aux_matrix = [[0u16; 5]; 4];
    for (_i, row) in matrix.iter().enumerate() {
        for (_y, col) in row.iter().enumerate() {
            if *col == 0 as u16 {
                aux_matrix[_i][_y] = 1
            }
        }
    }
    println!("{:?}", aux_matrix);

    for (_i, row) in aux_matrix.iter_mut().enumerate() {
        for (_y, col) in row.iter_mut().enumerate() {
            if *col == 1 {
                for j in 0..matrix[_i].len() {
                    matrix[_i][j] = 0;
                }
                for l in 0..matrix.len() {
                    matrix[l][_y] = 0;
                }
            }
        }
    }

    return matrix;
}

#[test]
fn zero_matrix_test() {
    let matrix = [
        [2, 1, 3, 0, 2],
        [7, 4, 1, 3, 8],
        [4, 0, 1, 2, 1],
        [9, 3, 4, 1, 9],
    ];
    let result_matrix = [
        [0, 0, 0, 0, 0],
        [7, 0, 1, 0, 8],
        [0, 0, 0, 0, 0],
        [9, 0, 4, 0, 9],
    ];
    let mut matriz = zero_matrix(matrix);

    for (_i, row) in matriz.iter_mut().enumerate() {
        for (_y, col) in row.iter_mut().enumerate() {
            print!("{} ", col);
        }
        println!()
    }

    assert_eq!(result_matrix, zero_matrix(matrix));

    // int[][] matrix2 = {{2, 0, 2}, {0, 2, 1}, {9, 3, 4}};
    // int[][] zeroedMatrix2 = {{0, 0, 0}, {0, 0, 0}, {0, 0, 4}};
    // zeroMatrix.zeroMatrix(matrix2);
    // assertTrue(Arrays.deepEquals(zeroedMatrix2, matrix2));
}

/*
 * Dada una matriz, escribe un algoritmo para establecer ceros en la fila F y columna C si existe un
 * 0 en la celda F:C
 *
 * Ejemplo:
 *  Input: 2 1 3 0 2
 *         7 4 1 3 8
 *         4 0 1 2 1
 *         9 3 4 1 9
 *
 *  Output: 0 0 0 0 0
 *          7 0 1 0 8
 *          0 0 0 0 0
 *          9 0 4 0 9
 */
