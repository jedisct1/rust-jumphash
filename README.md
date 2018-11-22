Jump Consistent Hash
====================

[A fast, minimal memory, consistent hash algorithm](https://arxiv.org/pdf/1406.2294.pdf).

[API documentation](https://docs.rs/jumphash)

# Example

Cargo dependencies:
```toml
[dependencies]
jumphash = "~0"
```

Rust code:

```rust
extern crate jumphash;

let jh = jumphash::JumpHasher::new();
let slot_count = 100;
let slot_for_key = jh.slot(&"key", slot_count);
```
