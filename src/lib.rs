//! # deta.rs
//! This is the unofficial Rust SDK for Deta Base and Drive.


mod base;
mod drive;
pub mod query;
pub mod errors;
pub mod updater;

use base::Base;
use drive::Drive;


fn validate(key: &str) -> String {
    let d = key.split('_').collect::<Vec<&str>>();
    assert_eq!(d.len(), 2, "invalid project key");
    d[0].to_string()
}

#[derive(Clone)]
pub struct Deta {
    project_id: String,
    project_key: String,
}

impl Deta {

    /// Create a new Deta instance from a project key
    /// ```rust
    /// use detalib::Deta;
    /// 
    /// let deta = Deta::from("project_key");
    /// let base = deta.base("hello");
    /// ```
    pub fn from(project_key: &str) -> Deta {
        Deta{
            project_id: validate(project_key),
            project_key: project_key.to_string(),
        }
    }

    /// Create a new Deta instance from the `DETA_PROJECT_KEY` environment variable
    /// ```rust
    /// use detalib::Deta;
    /// 
    /// let deta = Deta::new();
    /// let base = deta.base("world");
    /// ```
    pub fn new() -> Deta {
        let var = std::env::var("DETA_PROJECT_KEY").expect("DETA_PROJECT_KEY not found");

        Deta {
            project_id: validate(&var),
            project_key: var,
        }
    }

    /// Create a new Deta Base instance
    /// ```rust
    /// use detalib::Deta;
    /// 
    /// let deta = Deta::new();
    /// let base = deta.base("hello");
    /// ```
    pub fn base(&self, name: &str) -> Base {
        base::Base {
            name: name.to_string(),
            service: self.clone(),
        }
    }

    /// Create a new Deta Drive instance
    /// ```rust
    /// use detalib::Deta;
    /// 
    /// let deta = Deta::new();
    /// let drive = deta.drive("world");
    /// ```
    pub fn drive(&self, name: &str) -> Drive {
        drive::Drive {
            name: name.to_string(),
            service: self.clone(),
        }
    }
}


#[cfg(test)]
mod check {
    use serde;
    use super::*;


    #[derive(serde::Deserialize, Debug, serde::Serialize)]
    pub struct User {
        pub key: String,
        pub name: String,
        pub age: u8,
        pub address: String,
    }

    #[test]
    fn sdk_test() {

        let deta = Deta::new();
        let base = deta.base("hello");

        assert_eq!(base.name, "hello");
    }

    #[test]
    fn sdk_base_get() {
        let deta = Deta::new();
        let base = deta.base("hello");
        let user = User {
            key: "1234".to_string(),
            name: "John".to_string(),
            age: 20,
            address: "123 Main St".to_string(),
        };
        _ = base.insert(&user);
        let deserialized = base.get_as::<User>(user.key.as_str());

        assert!(deserialized.is_ok());
    }

    #[test]
    #[should_panic]
    fn sdk_base_insert() {
        let deta = Deta::new();
        let base = deta.base("hello");
        let user = User {
            key: "1234".to_string(),
            name: "John".to_string(),
            age: 20,
            address: "123 Main St".to_string(),
        };
        let resp = base.insert(&user);

        assert!(resp.is_ok());
    }

    #[test]
    fn sdk_base_query() {
        use serde_json::{Value, Number};
        use query::Operator;

        let base = Deta::new().base("hello");
        let resp = base.query()
            .limit(1)
            .sort(true)
            .set(Operator::Eq, "name", Value::String("John".to_string()))
            .set(Operator::Gt, "age", Value::Number(Number::from(18)))
            .set(Operator::Lt, "age", Value::Number(Number::from(21)))
            .execute();

        assert!(resp.is_ok());
    }

    #[test]
    fn sdk_base_update() {
        use serde_json::{Value, Number};
        use updater::Operation;

        let base = Deta::new().base("hello");
        let resp = base.update("1234")
            .operation(Operation::Set, "name", Value::String("John".to_string()))
            .operation(Operation::Increment, "age", Value::Number(Number::from(1)))
            .execute();
        
        assert!(resp.is_ok());
    }

}