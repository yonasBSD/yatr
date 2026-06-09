# yatr-plugin

Write [yatr](https://github.com/cargopete/yatr) WASM task plugins in ergonomic Rust.

yatr can run a task as a sandboxed WebAssembly plugin. This crate wraps the raw
host ABI (linear-memory pointers + `i32` lengths) so you write plain Rust.

```toml
# Cargo.toml of your plugin
[lib]
crate-type = ["cdylib"]

[dependencies]
yatr-plugin = "0.1"
```

```rust
yatr_plugin::plugin!({
    let input = yatr_plugin::input_string(); // {"task":...,"env":{...}}
    yatr_plugin::emit(&format!("hello from a plugin; input = {input}"));
    Ok(())
});
```

Build it and point a task at the artifact:

```bash
cargo build --release --target wasm32-unknown-unknown
```

```toml
[tasks.greet]
wasm = "target/wasm32-unknown-unknown/release/my_plugin.wasm"
```

## API

- `emit(&str)` — append to the task's output
- `log(&str)` — log an info message
- `input_string()` / `input_bytes()` — the task name + env as JSON
- `plugin!({ … Ok(()) })` — declare the `run()` entry point

Plugins run **capability-sandboxed**: only these host functions are available —
no filesystem, network, or clock.
