use js_sys::Uint32Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn insertion_sort(array: Uint32Array) -> Uint32Array {
    let mut vec = array.to_vec();

    for i in 1..vec.len() {
        let element = vec[i];
        let mut j = i;
        while (j > 0) && (vec[j - 1] > element) {
            vec[j] = vec[j - 1];
            j = j - 1;
        }
        vec[j] = element;
    }
    let result = Uint32Array::new_with_length(vec.len().try_into().unwrap());
    result.copy_from(&vec);

    return result;
}

// #[wasm_bindgen]
// pub fn insertion_sort(array: &mut [i32]){
//     for i in 1..array.len() {
//         let element = array[i];
//         let mut j = i;
//         while (j > 0) && (array[j - 1] > element) {
//             array[j] = array[j - 1];
//             j = j - 1;
//         }
//         array[j] = element;
//     }
// }

// #[cfg(test)]
// pub mod tests {
//     use super::*;
//     #[test]
//     fn insertion_sort_test() {
//         let mut input = [
//             5040, 9139, 9301, 8933, 5997, 5721, 841, 1737, 8151, 7125, 8699, 7161, 9031, 1548,
//             8656, 8213, 2260, 817, 2241, 3324, 7027, 1181, 2300
//         ];
//         let expected_result = [
//             817, 841, 1181, 1548, 1737, 2241, 2260, 2300, 3324, 5040, 5721, 5997, 7027, 7125, 7161,
//             8151, 8213, 8656, 8699, 8933, 9031, 9139, 9301
//         ];
//         insertion_sort(&mut input);
//         assert_eq!(input, expected_result);
//     }
// }
