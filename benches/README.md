# Benchmarks

A reproducible macro-benchmark comparing yatr's real-world overhead and caching
to `make`, `just`, and `task`. Run it yourself:

```bash
benches/bench.sh        # times whichever of make/just/task are installed
```

It builds a release `yatr`, generates an identical single-task workload for each
tool (a "build" that does ~100ms of work and writes an output from a source), and
times two **stable, repeatable** scenarios (min / mean of repeated runs):

- **startup (noop)** — start up + parse config + decide there's nothing to do.
- **warm rebuild** — re-invoke `build` with nothing changed (the cache's job).

## Sample results

Measured on an Apple-silicon laptop, June 2026 (`make` 3.81, `just` 1.x). Absolute
numbers vary by machine — reproduce locally with `benches/bench.sh`.

| Tool | startup (min/mean ms) | warm rebuild (min/mean ms) |
|------|----------------------:|---------------------------:|
| **yatr** | **8.4 / 8.9** | **9.5 / 10.0** |
| make | 10.7 / 11.5 | 11.2 / 15.7 |
| just *(no caching)* | 7.8 / 8.2 | 131.4 / 145.6 |

## What this shows (honestly)

- **Overhead is competitive.** yatr's single-binary startup (~8.4 ms) beats `make`
  and matches `just` — there's no "task runner tax" for choosing yatr.
- **The cache earns its keep.** A warm rebuild is a content-addressed cache hit
  (~9.5 ms) — about **14× faster than a runner with no caching** (`just`
  re-does the 100 ms of work every time), and on par with `make`'s timestamp skip.

## What a micro-benchmark *can't* show

The numbers above understate yatr's real advantages, because a single local no-op
doesn't exercise them:

- **Correctness.** yatr's cache is keyed on file *contents* (BLAKE3), so it's right
  across `git checkout`, clock skew, and tools that touch mtimes — cases where
  `make`'s timestamp cache silently does the wrong thing.
- **Cross-machine.** yatr's remote cache restores a build produced on CI or a
  teammate's machine; `make`/`just`/`task` can't.
- **Scale.** `affected` detection skips whole tasks git says can't have changed —
  the win grows with repo size, not a single task.
- **Outputs.** yatr captures and restores declared `outputs`; a hit reproduces the
  artifacts, not just a "nothing to do".

Benchmarks prove yatr is *fast*; those four are why it's *better*.
