
fn zero_matrix(matrix[[i32]; ]){

}


#[test]
fn zero_matrix_test(){
    
    let matrix = [
      [2, 1, 3, 0, 2],
      [7, 4, 1, 3, 8],
      [4, 0, 1, 2, 1],
      [9, 3, 4, 1, 9]
    ];
    let zeroedMatrix = [[0, 0, 0, 0, 0], [7, 0, 1, 0, 8], [0, 0, 0, 0, 0], [9, 0, 4, 0, 9]];
    zero_matrix(matrix);
    assertTrue(Arrays.deepEquals(zeroedMatrix, matrix));

    // int[][] matrix2 = {{2, 0, 2}, {0, 2, 1}, {9, 3, 4}};
    // int[][] zeroedMatrix2 = {{0, 0, 0}, {0, 0, 0}, {0, 0, 4}};
    // zeroMatrix.zeroMatrix(matrix2);
    // assertTrue(Arrays.deepEquals(zeroedMatrix2, matrix2));
}

/*

Complejidad no optimo:
Tendriamos que recorrer toda la matriz lo que significa que tnemos F*C
Depues tendriamos que validar los ceros y recorrer la fila y la columna Z*(F+C)
Finalmente la complejidad quedaria asi => O(F*C + Z*(F+C))


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