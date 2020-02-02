window.addEventListener("load", async () => {
    let wasm = await WebAssembly
        .instantiateStreaming(fetch("lib.wasm"));
    let { memory, alloc_array, sum_array } = wasm.instance.exports;

    let len = 3;
    let ptr = alloc_array(len);

    let view = new Int32Array(memory.buffer)
      .subarray(ptr / 4, ptr / 4 + len);

    view.set([1, 2, 3]);

    let result = sum_array(ptr, len);
    console.log("The result is", result);
});

