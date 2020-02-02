
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen(inline_js = "
function delay(time) {
    return new Promise(resolve => setTimeout(resolve, time));
}
export async function delayedAnswer(time) {
    await delay(time);
    return 42;
}
")]
extern "C" {
    #[wasm_bindgen(js_name = delayedAnswer)]
    fn delayed_answer(time: i32) -> Promise;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(text: &str);
}

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    log("Example started");
    let promise = delayed_answer(1000);
    let answer = JsFuture::from(promise)
        .await?
        .as_f64()
        .expect("Should have an answer");
    log(&format!("The answer is {}", answer));
    Ok(())
}


