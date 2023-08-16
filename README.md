# Deta Rust
Rust bindings for the Deta [Base](https://docs.deta.sh/docs/base/http) and [Drive](https://docs.deta.sh/docs/drive/http) HTTP API

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
deta = { git = "https://github.com/jnsougata/deta.rs" }
```

## Base
Methods
- [x] `put` (batch max 25)
- [X] `get` (single)
- [X] `insert` (single)
- [X] `delete` (single)
- [X] `update` (with upsert)
- [X] `fetch` (with pagination)
  
  
## Drive
Methods
- [X] `put` (single)
- [X] `get` (single)
- [X] `delete` (single)
- [X] `list` (with pagination)
  
