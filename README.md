# deta.rs
Rust bindings for the Deta **Base** and **Drive** [HTTP API](https://deta.space/docs/en/build/reference/http-api#content)

## Installation
```shell
cargo add detalib
```

## Usage

Cargo.toml

```toml
[dependencies]
detalib = "0.1.1"
serde_json = "1.0"
serde = { version = "1.0", features = ["derive"] }
```

## Quickstart

```rust
use detalib::{ Deta, Operation, Operator };
use serde::{ Deserialize, Serialize };
use serde_json::{ Number, Value };


#[derive(Serialize, Deserialize)]
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

    ///////////////////////////////////
    //          BASE Methods        //
    //////////////////////////////////
    
    // Insert a record
    _ = base.insert(&user).unwrap();

    // Put multiple records
    let users = vec![
        User {
            key: "user_1".to_string(),
            name: "John".to_string(),
            age: 20,
        },
        User {
            key: "user_2".to_string(),
            name: "Jane".to_string(),
            age: 21,
        },
    ];
    _ = base.put(&users).unwrap();
    
    // Get a single record
    let user = base.get("user_1").unwrap();
    println!("{:?}", user);
    
    // Get in deserialized format
    let user_d = base.get_as::<User>("user_1").unwrap();
    println!("{:?}", user_d);

    // Delete a record
    _ = base.delete("user_1").unwrap();

    // Update a record
    _ = base.update("user_1")
        .operation(Operation::Set, "name", Value::String("Johnny".to_string()))
        .operation(Operation::Increment, "age", Value::Number(Number::from(1)))
        .run()
        .unwrap();

    // Query records
    let resp = base.query()
        .limit(1)
        .sort(true)
        .set(Operator::Eq, "name", Value::String("Johnny".to_string()))
        .set(Operator::Gt, "age", Value::Number(Number::from(20)))
        .set(Operator::Lt, "age", Value::Number(Number::from(23)))
        .run()
        .unwrap();
    println!("{:?}", resp);

    // Query all records
    let resp = base.query().run_until_end().unwrap();
    println!("{:?}", resp);
    

    /////////////////////////////////
    //        Drive Methods       //
    ////////////////////////////////
    
    // Put a file
    _ = drive.put("hello.txt", "Hello World".as_bytes(), Some("text/plain")).unwrap();

    // Delete a file
    _ = drive.delete(vec!["hello.txt"]).unwrap();

    // Get a file
    let resp = drive.get("hello.txt").unwrap();
    // read the response body as bytes and do something with it

    // List files up to 10, default is 1000
    let value = drive.list(None, 10, None).unwrap();
    println!("{:?}", value);

    // List all files
    let value = drive.list_all(None).unwrap();
    println!("{:?}", value);
    
}


```

## Base
Methods
- [x] `put` (batch max 25)
- [X] `get` (single)
- [X] `insert` (single)
- [X] `delete` (single)
- [X] `update` (with upsert)
- [X] `query` (paginated)
  
  
## Drive
Methods
- [X] `put` (single)
- [X] `get` (single)
- [X] `delete` (single)
- [X] `list` (paginated)
- [X] `list_all`
