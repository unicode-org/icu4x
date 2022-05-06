// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use wasmer::{Instance, Module, Store};
use wasmer_wasi::{Pipe, WasiState};
use std::path::PathBuf;
use wasmer_cache::{Cache, FileSystemCache, Hash};

fn get_module() -> Module {
    let cache_path = PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("wasmcache");
    let mut fs_cache = FileSystemCache::new(cache_path).unwrap();

    let wasm_path = PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("list_to_ucptrie").with_extension("wasm");
    let wasm_bytes = std::fs::read(wasm_path).unwrap();

    // Compute a key for a given WebAssembly binary
    let key = Hash::generate(&wasm_bytes);

    let store = Store::default();

    let maybe_module = unsafe { fs_cache.load(&store, key) };
    if let Ok(module) = maybe_module {
        println!("Loaded module from cache");
        return module;
    }

    println!("Compiling module...");
    let module = Module::new(&store, &wasm_bytes).unwrap();
    fs_cache.store(key, &module).unwrap();
    return module;
}

fn main() {
    let module = get_module();

    println!("Creating `WasiEnv`...");
    // First, we create the `WasiEnv` with the stdio pipes
    let input = Pipe::new();
    let output = Pipe::new();
    let mut wasi_env = WasiState::new("hello")
        .stdin(Box::new(input))
        .stdout(Box::new(output))
        .args(&["0", "0", "fast"])
        .finalize().unwrap();

    println!("Instantiating module with WASI imports...");
    // Then, we get the import object related to our WASI
    // and attach it to the Wasm instance.
    let import_object = wasi_env.import_object(&module).unwrap();
    let instance = Instance::new(&module, &import_object).unwrap();

    let msg = "2\n3\n4";
    println!("Writing \"{}\" to the WASI stdin...", msg);
    // To write to the stdin, we need a mutable reference to the pipe
    //
    // We access WasiState in a nested scope to ensure we're not holding
    // the mutex after we need it.
    {
        let mut state = wasi_env.state();
        let wasi_stdin = state.fs.stdin_mut().unwrap().as_mut().unwrap();
        // Then we can write to it!
        writeln!(wasi_stdin, "{}", msg).unwrap();
    }

    println!("Call WASI `_start` function...");
    // And we just call the `_start` function!
    let start = instance.exports.get_function("_start").unwrap();
    start.call(&[]).unwrap();

    println!("Reading from the WASI stdout...");
    // To read from the stdout, we again need a mutable reference to the pipe
    let mut state = wasi_env.state();
    let wasi_stdout = state.fs.stdout_mut().unwrap().as_mut().unwrap();
    // Then we can read from it!
    let mut buf = String::new();
    wasi_stdout.read_to_string(&mut buf).unwrap();
    println!("Read \"{}\" from the WASI stdout!", buf.trim());
}
