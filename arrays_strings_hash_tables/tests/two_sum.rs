use std::collections::HashMap;

pub fn two_sum(arr: &[i32], num: i32) -> [i32; 2] {
    let mut mapa = HashMap::new();
    for index in 0..arr.len() {
        println!("{}", arr[index]);
        let complement = num - arr[index];
        match mapa.get(&complement) {
            Some(num) => {
                return [*num, index as i32];
            }
            None => {
                mapa.insert(arr[index], index as i32);
            }
        }
    }
    [0, 0]
}

#[test]
fn two_sum_test() {
    let arr = vec![9, 2, 5, 6];
    let result = two_sum(&arr, 7);
    assert!((result[0] == 1 && result[1] == 2) || (result[0] == 2 && result[1] == 1));
    assert_eq!([0, 0], two_sum(&arr, 50));
}

/*
 * Dado un array de números enteros y un target, retorna los índices de dos
 * números para los que la suma de ambos sea igual al target.
 *
 * Puedes asumir que hay solamente una solución.
 *
 * Ejemplo 1:
 *  Input: nums = [9,2,5,6], target = 7
 *  Output: [1,2]
 *  Explicación: nums[1] + nums[2] == 7, devolvemos [1, 2].
 *
 * Ejemplo 2:
 *  Input: nums = [9,2,5,6], target = 100
 *  Output: null
 */
