import init, { sumArray, sumSquaredArray } from "./pkg/wasm_bindgen_example.js";

window.addEventListener("load", async () => {
    await init();
    let result1 = sumArray([1, 2, 3]);
    console.log("The result is", result1);

    let result2 = sumSquaredArray([1, 2, 3]);
    console.log("The result is", result2);
});
