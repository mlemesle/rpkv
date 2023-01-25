# RPKV (Rust-Python Key Value)

A very naive (and very useless) key value store, created to test PyO3.

The workflow is :

```mermaid
  flowchart LR
      rest[HTTP request] -- calls --> py[Python API]
      py -- calls --> rust[Rust bindgen]
      rust[Rust bindgen] -- calls --> fs[filesystem]
```

Why is it so complicated ? *Because why not ?*
