# Benchmarks

Reproduce the numbers yourself:

```bash
benches/bench.sh   # times whichever of make/just/task are installed
```

It generates an identical single-task workload for each tool and times two stable
scenarios: startup overhead, and a warm rebuild (the cache's job).

## Sample results

Apple-silicon laptop, June 2026 (min / mean ms — lower is better):

| Tool | startup | warm rebuild |
|------|--------:|-------------:|
| **yatr** | **8.4 / 8.9** | **9.5 / 10.0** |
| make | 10.7 / 11.5 | 11.2 / 15.7 |
| just *(no caching)* | 7.8 / 8.2 | 131.4 / 145.6 |

- **Overhead is competitive** — yatr's single-binary startup beats `make` and
  matches `just`; no "task-runner tax".
- **The cache earns its keep** — a warm rebuild is a content-addressed cache hit,
  ~14× faster than a runner with no caching, and on par with `make`'s timestamp
  skip — but content-correct where `make`'s mtime cache is fragile.

## Scheduler

The ready-queue scheduler starts each task the moment its dependencies finish,
instead of waiting for the whole dependency "level". On a DAG with a fast chain
beside a slow sibling, that measured **~1.8× faster** (791 ms → 430 ms).

Benchmarks prove yatr is *fast*; [caching correctness](./caching.md), the
[remote cache](./remote-cache.md), and [affected detection](./monorepo.md) are
why it's *better* — and don't show up in a single local no-op.
