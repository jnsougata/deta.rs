# deta.rs
Rust bindings for the Deta **Base** and **Drive** [HTTP API](https://deta.space/docs/en/build/reference/http-api#content)

## Usage

`Cargo.toml`

```toml
[dependencies]
deta = { git = "https://github.com/jnsougata/deta.rs" }
```

## Quickstart

```rust
use serde;
use deta::Deta;

#[derive(serde::Serialize, serde::Deserialize)]
struct User {
    key: String,
    name: String,
    age: u8,
}

fn main() {
    let deta = Deta::new("project_key");
    let base = deta.base("base_name");
    let drive = deta.drive("drive_name");
    
    let user = User {
        key: "user_1".to_string(),
        name: "John".to_string(),
        age: 20,
    };
    
    // BASE OPERATIONS
    
    // Insert a single item
    _ = base.insert(&user).unwrap();
    
    // Get a single item
    let user = base.get("user_1").unwrap();
    
    // Get in deserialize format
    let user_d = base.get_as::<User>("user_1").unwrap();
    println!("{:?}", user_d);
    
    // ... 
    
    // DRIVE OPERATIONS
    
    // Put a single file
    _ = drive.put("hello.txt", "Hello World".as_bytes(), Some("text/plain")).unwrap();
    _ = drive.delete(vec!["hello.txt"]).unwrap();
    
    // ... 
    
}


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
- [X] `list` (with pagination
