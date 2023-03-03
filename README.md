# Deta Rust
Rust bindings for the Deta [Base](https://docs.deta.sh/docs/base/http) and [Drive](https://docs.deta.sh/docs/drive/http) HTTP API

Forked from <https://github.com/jnsougata/deta-rust>.

## Differences from upstream

This fork currently implements three changes compared to upstream as of
[`344396b`](https://github.com/jnsougata/deta-rust/commit/344396b0ce5c98b2bdcebcc2261ca24e82fbc966):

1. `reqwest` is replaced by `ureq` (using `rustls` for a 100% Rust
   implementation with no sys-dependencies)
1. `Deta` now implements `Clone`
1. `thiserror` has been introduced and `unwrap()`s have been removed

The former was done in an attempt to make the project build on Deta. At the time
of writing, building Rust binaries on Deta does not work, but the change has
been left in to make it easier to build the package without needing
`openssl-sys` or `pkg-config`.

`Clone` is implemented as it makes it easier to use the `Deta` struct in an app
state for use in eg. an `axum` app.

Error handling was improved to avoid applications crashing on eg. 404 responses.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
deta_rs = { git = "https://github.com/wonderfulspam/deta-rust"}
```

## Quickstart

```rust
use deta_rs::{Deta, utils};
use serde_json::json;

fn main() {

    let d = Deta::new("your_project_key");

    // lazily create a base and drive
    let base = d.base("your_base_name"); 
    let drive = d.drive("your_drive_name");

    // build a record to put in the base
    let record = utils::Record {
        key: Some("your_key".to_string()), // or None
        value: Some(json!({"name": "John Doe", "age": 25})), // or None
        expires_at: None, // or Some(unix_timestamp i64)
        expires_in: Some(3600), // in seconds, or None
        // or use ..Default::default()
    };
    let resp = base.put(record).unwrap();
    println!("{:#?}", resp);

    // get a record from the base
    let resp = base.get("your_key").unwrap();
    println!("{:#?}", resp);

    // upload a file to drive
    let content = fs::read("sample.png").unwrap();
    let res = drive.put("sample.png", content.as_slice()).unwrap();
    println!("{:#?}", res);

    // get a file from drive
    use std::{fs, io::Write};

    let content = drive.get("sample.png").unwrap();
    let mut file = fs::File::create("sample.png").unwrap();
    file.write_all(content.as_slice()).unwrap();
}
```
## Base
Methods
- [x] `put` (batch max 25)
- [X] `get` (single)
- [X] `insert` (single)
- [X] `delete` (single)
- [X] `update` (with upsert)
- [X] `query` (with pagination)
  
  
## Drive
Methods
- [X] `put` (single)
- [X] `get` (single)
- [X] `delete` (single)
- [X] `list` (with pagination)
  
## Query
`struct QueryBuilder`
- Methods
    - `equals` 
    - `not_equals`
    - `contains`
    - `not_contains`
    - `range`
    - `grater_than`
    - `less_than`
    - `grater_than_or_equal`
    - `less_than_or_equal`
    - `do_or`  *useful for `OR`ing conditions*

Multiple conditions applied on `QueryBuilder` will `AND` the conditions together except for `do_or` which will `OR` the conditions together.

```rust
use deta_rs::{Deta, utils};
use serde_json::json;

fn main() {

    let d = Deta::new("your_project_key");

    let base = d.base("your_base_name"); 

    // build a query
    let query = utils::QueryBuilder::new();
    query.equals("name", json!("John Doe"));
    query.range("age", 20, 30)  // ANDed with the previous condition

    // add another condition as an OR
    let mut another = QueryBuilder::new();
    another.equals("name", serde_json::json!("Jenny Doe"));
    query.do_or(another);

    // query the base
    let resp = base.query(query).unwrap();
    println!("{:#?}", resp);
}
```
## Updates
`struct UpdateBuilder`
- Methods
    - `set`  
    - `delete`
    - `increment`
    - `append`
    - `prepend`
    - `delete`
  
```rust
use deta_rs::{Deta, utils};
use serde_json::json;

fn main() {

    let d = Deta::new("your_project_key");

    let base = d.base("your_base_name"); 

    // build an update
    let mut updates = UpdateBuilder::new("primary_key");
    updates.set("name", serde_json::json!("John"));
    updates.delete(vec!["age"]);
    updates.append("hobbies", serde_json::json!("reading"));
    updates.prepend("hobbies", serde_json::json!("coding"));
    updates.increment("age", 1);

    // update the base
    let resp = base.update(updates).unwrap();
    println!("{:#?}", resp);
}
```
