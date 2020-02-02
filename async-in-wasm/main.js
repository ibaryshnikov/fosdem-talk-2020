
import init from "./pkg/async_in_wasm.js";

window.addEventListener("load", async () => {
    await init();
});

