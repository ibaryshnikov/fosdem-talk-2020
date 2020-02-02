#!/bin/bash
rustc lib.rs --crate-type cdylib --target wasm32-unknown-unknown
