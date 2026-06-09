//! Write [yatr](https://github.com/cargopete/yatr) WASM task plugins in
//! ergonomic Rust.
//!
//! yatr can run a task as a sandboxed WebAssembly plugin. This crate wraps the
//! raw host ABI (linear-memory pointers and `i32` lengths) so you can just call
//! [`emit`], [`log`], and [`input_string`], and declare your entry point with
//! the [`plugin!`] macro.
//!
//! # Example
//!
//! ```ignore
//! // Cargo.toml: [lib] crate-type = ["cdylib"]; build for wasm32-unknown-unknown.
//! yatr_plugin::plugin!({
//!     let input = yatr_plugin::input_string(); // {"task":...,"env":{...}}
//!     yatr_plugin::emit(&format!("hello from a plugin; input = {input}"));
//!     Ok(())
//! });
//! ```
//!
//! The module is then a yatr task:
//!
//! ```toml
//! [tasks.greet]
//! wasm = "target/wasm32-unknown-unknown/release/my_plugin.wasm"
//! ```

#![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]

/// Raw host imports (under wasm module `"yatr"`), with host-side stubs so this
/// crate also compiles and tests on a normal target.
mod sys {
    #[cfg(target_arch = "wasm32")]
    #[link(wasm_import_module = "yatr")]
    extern "C" {
        pub fn emit(ptr: i32, len: i32);
        pub fn log(ptr: i32, len: i32);
        pub fn input_len() -> i32;
        pub fn input_read(ptr: i32) -> i32;
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub unsafe fn emit(_ptr: i32, _len: i32) {}
    #[cfg(not(target_arch = "wasm32"))]
    pub unsafe fn log(_ptr: i32, _len: i32) {}
    #[cfg(not(target_arch = "wasm32"))]
    pub unsafe fn input_len() -> i32 {
        0
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub unsafe fn input_read(_ptr: i32) -> i32 {
        0
    }
}

/// Append a string to the task's output.
pub fn emit(s: &str) {
    unsafe { sys::emit(s.as_ptr() as usize as i32, s.len() as i32) }
}

/// Log a string as an info message (shown by yatr in verbose mode).
pub fn log(s: &str) {
    unsafe { sys::log(s.as_ptr() as usize as i32, s.len() as i32) }
}

/// The raw input bytes yatr passed to this task (JSON of `{ task, env }`).
#[must_use]
pub fn input_bytes() -> Vec<u8> {
    let len = unsafe { sys::input_len() };
    if len <= 0 {
        return Vec::new();
    }
    let len = len as usize;
    let mut buf = vec![0u8; len];
    unsafe {
        sys::input_read(buf.as_mut_ptr() as usize as i32);
    }
    buf
}

/// The task input as a UTF-8 string (JSON of `{ task, env }`).
#[must_use]
pub fn input_string() -> String {
    String::from_utf8_lossy(&input_bytes()).into_owned()
}

/// Run a plugin body, mapping its `Result` to the yatr exit code. On error the
/// message is logged and a non-zero status is returned. Used by [`plugin!`].
pub fn run_main<F: FnOnce() -> Result<(), String>>(f: F) -> i32 {
    match f() {
        Ok(()) => 0,
        Err(e) => {
            log(&e);
            1
        }
    }
}

/// Declare a plugin entry point. The body must evaluate to
/// `Result<(), String>`; the macro exports the `run() -> i32` function yatr calls.
#[macro_export]
macro_rules! plugin {
    ($body:block) => {
        #[no_mangle]
        pub extern "C" fn run() -> i32 {
            $crate::run_main(|| $body)
        }
    };
    ($f:path) => {
        #[no_mangle]
        pub extern "C" fn run() -> i32 {
            $crate::run_main($f)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_main_maps_result_to_status() {
        assert_eq!(run_main(|| Ok(())), 0);
        assert_eq!(run_main(|| Err("boom".to_string())), 1);
    }

    #[test]
    fn input_is_empty_on_host() {
        // Host stubs report no input; the real values arrive only under wasm32.
        assert!(input_bytes().is_empty());
        assert!(input_string().is_empty());
    }
}
