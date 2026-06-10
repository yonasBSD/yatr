# Remote cache

Point yatr at a shared HTTP cache and a task built on one machine (or in CI) is
restored on the next, instead of rebuilt.

```toml
[settings.remote_cache]
url = "https://cache.example.com/yatr"
token_env = "YATR_CACHE_TOKEN"   # optional bearer token, read from this env var
sign_key_env = "YATR_CACHE_KEY"  # optional signing secret (see below)
read = true                      # pull on a local miss (default: true)
write = true                     # push after a successful run (default: true)
```

It speaks a small REST protocol — `GET`/`PUT`/`HEAD` on `<url>/ac/<key>` (action
results) and `<url>/cas/<blob>` (content blobs) — the same path layout as Bazel's
HTTP cache, so it works against an off-the-shelf blob store or a tiny server.

Keys are **content-addressed and machine-portable**: identical inputs produce
identical keys regardless of checkout path, so a build on CI restores on your
laptop. A flaky or unreachable remote is **non-fatal** — yatr warns and runs the
task locally.

## Integrity & signing

Downloaded CAS blobs are verified against their content digests, so a tampered
blob is rejected automatically. To defend *action results* against a compromised
cache (the "CREEP" cache-poisoning class), set `sign_key_env` to a shared secret:
yatr signs each action result with a keyed BLAKE3 MAC and rejects any entry whose
signature doesn't verify under your key.

Keep secrets out of the committed config by using the `*_env` options rather than
inline values.
