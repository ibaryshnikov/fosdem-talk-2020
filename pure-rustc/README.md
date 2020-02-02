## Example of compiling Rust to Wasm with rustc

### Building

```bash
rustc lib.rs --crate-type cdylib --target wasm32-unknown-unknown
```

or just use [./build.sh](./build.sh)

### Running

Serve the statics and open the page in
the browser. Check the browser console for
the output.
