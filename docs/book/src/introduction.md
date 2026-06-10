# yatr 🦬

Yet another task runner. But this one's actually good.

```bash
yatr test          # Run tests
yatr build         # Build with dependencies
yatr --watch test  # Re-run on file changes
```

yatr is a fast, single-binary, polyglot task runner with a cache that actually
tells the truth: **content-addressed**, **output-restoring**, and **shareable
across machines and CI**.

## Why yatr?

| Feature | Make | Just | cargo-make | **yatr** |
|---------|------|------|------------|------------|
| Content-addressed caching | ❌ | ❌ | ⚠️ | ✅ |
| Captures & restores outputs | ❌ | ❌ | ❌ | ✅ |
| Remote / shared cache | ❌ | ❌ | ❌ | ✅ |
| Signed cache (anti-poisoning) | ❌ | ❌ | ❌ | ✅ |
| Affected (monorepo) detection | ❌ | ❌ | ⚠️ | ✅ |
| Toolchain management | ❌ | ❌ | ❌ | ✅ |
| Sandboxed WASM plugins | ❌ | ❌ | ❌ | ✅ |
| Config schema + LSP | ❌ | ⚠️ | ❌ | ✅ |
| Single binary, zero runtime deps | N/A | ✅ | ❌ | ✅ |

It sits in the sweet spot: **simpler than cargo-make**, **far more capable than
just** — a *correct, fast, hermetic polyglot command runner*.

## The four things it gets right

- **Trusted** — a content-addressed cache, cryptographically signed for sharing,
  with IO-tracing to catch under-declared outputs.
- **Fast** — ~8 ms startup, a ready-queue scheduler, and affected-detection that
  skips what git says can't have changed. ([Benchmarks](./benchmarks.md).)
- **Polyglot-complete** — pin language [toolchains](./toolchains.md) so a fresh
  checkout runs green; write tasks in shell, Rhai, or [WASM](./plugins.md).
- **Delightful** — a JSON Schema and a [language server](./editor.md) for
  `yatr.toml`, structured output, and profiling.
