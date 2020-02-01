
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = sumArray)]
pub fn sum_array(array: Vec<i32>) -> i32 {
    array.iter().sum::<i32>()
}

#[wasm_bindgen(inline_js = "
export function makeSquares(array) {
    return array.map(value => value ** 2);
}
")]
extern "C" {
    #[wasm_bindgen(js_name = makeSquares)]
    fn make_squares(array: Vec<i32>) -> Vec<i32>;
}

#[wasm_bindgen(js_name = sumSquaredArray)]
pub fn sum_squared_array(array: Vec<i32>) -> i32 {
    let squared = make_squares(array);
    squared.iter().sum::<i32>()
}



