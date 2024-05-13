use std::collections::HashMap;

pub fn two_sum(arr: &[i32], num: i32) -> [i32; 2] {
    let mut mapa = HashMap::new();
    for index in 0..arr.len() {
        println!("{}", arr[index]);
        let complement = num - arr[index];
        if mapa.contains_key(&complement) {
            return [mapa.remove(&complement).is_some(), index as i32];
        }
        mapa.insert(arr[index], index);
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
