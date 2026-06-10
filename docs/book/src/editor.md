# Editor integration

## JSON Schema

yatr ships a JSON Schema for `yatr.toml`, giving autocomplete, hover docs, and
validation in any schema-aware editor.

```bash
yatr schema > yatr.schema.json   # regenerate any time
```

With taplo / the **Even Better TOML** VS Code extension, add a directive to the
top of your `yatr.toml`:

```toml
#:schema ./yatr.schema.json
```

## Language server

`yatr lsp` runs a language server over stdio, giving any LSP-capable editor live
**diagnostics** (parse errors, validation errors, missing dependencies, cycles —
as you type) and a **task outline**.

Point your editor's LSP client at `yatr lsp` for `yatr.toml`. For example, in
Neovim:

```lua
vim.lsp.start({ name = "yatr", cmd = { "yatr", "lsp" }, root_dir = vim.fn.getcwd() })
```

## Machine-readable output

```bash
yatr run --json test               # structured per-task results + summary
yatr run --json --dry-run ci       # the execution plan as JSON
yatr run --profile trace.json ci   # a Chrome trace (chrome://tracing / Perfetto)
```
