# WASM plugins

A task can be implemented by a WebAssembly plugin — write it in any language that
compiles to `wasm32`, ship a single `.wasm`, and run it anywhere yatr runs:

```toml
[tasks.codegen]
wasm = "plugins/codegen.wasm"                         # local path
[tasks.shared]
wasm = "https://example.com/v1/plugin.wasm"            # …or an http(s) URL
[tasks.gh]
wasm = "github:owner/repo@v1.0.0/plugin.wasm"          # …or a GitHub release asset
```

Plugins are **capability-sandboxed**: they run in a pure-Rust interpreter
(`wasmi`) with *only* yatr's host ABI imported — no filesystem, network, or clock.
A plugin that tries to import anything else fails to load, so even an untrusted
remote plugin can't escape. Remote plugins are downloaded once and cached
(override with `YATR_PLUGIN_DIR`); plugin output is captured and cached like any
task.

## Host ABI

A plugin exports its `memory` and `run() -> i32` (`0` = success), and may import:

| Import | Signature | Effect |
|--------|-----------|--------|
| `emit` | `(ptr, len)` | Append a UTF-8 string to the task's output |
| `log`  | `(ptr, len)` | Log an info message |
| `input_len` | `() -> i32` | Byte length of the task input |
| `input_read` | `(ptr) -> i32` | Copy the input (JSON `{task, env}`) into memory |

## Writing plugins in Rust

The [`yatr-plugin`](https://github.com/cargopete/yatr/tree/main/crates/yatr-plugin)
crate wraps the raw ABI so you write plain Rust:

```rust
yatr_plugin::plugin!({
    let input = yatr_plugin::input_string();  // {"task":...,"env":{...}}
    yatr_plugin::emit(&format!("hello from a plugin; input = {input}"));
    Ok(())
});
```

```bash
cargo build --release --target wasm32-unknown-unknown
```

Then point a task at the resulting `.wasm`.
