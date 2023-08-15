//! # deta.rs
//! This is the unofficial Rust SDK for Deta Base and Drive.


mod base;
mod drive;
pub mod query;
pub mod utils;
pub mod errors;


fn validate(key: &str) -> String {
    let d = key.split('_').collect::<Vec<&str>>();
    assert!(d.len() == 2, "invalid project key");
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
    /// use deta::Deta;
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
    /// use deta::Deta;
    /// 
    /// let deta = Deta::new();
    /// let base = deta.base("world");
    /// ```
    pub fn new() -> Deta {
        let var = std::env::var("DETA_PROJECT_KEY").expect("DETA_PROJECT_KEY not found");

        Deta {
            project_id: validate(&var),
            project_key:var,
        }
    }

    /// Create a new Deta Base instance
    /// ```rust
    /// use deta::Deta;
    /// 
    /// let deta = Deta::new();
    /// let base = deta.base("hello");
    /// ```
    pub fn base(&self, name: &str) -> base::Base {
        base::Base {
            name: name.to_string(),
            service: self.clone(),
        }
    }

    /// Create a new Deta Drive instance
    /// ```rust
    /// use deta::Deta;
    /// 
    /// let deta = Deta::new();
    /// let drive = deta.drive("world");
    /// ```
    pub fn drive(&self, name: &str) -> drive::Drive {
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

        let mut q = query::Query::new();
        q.limit = Some(1);
        q.set(query::Operator::Eq, "name", serde_json::Value::String("John".to_string()));


        let deta = Deta::new();
        let base = deta.base("hello");
        let user = User {
            key: "1234".to_string(),
            name: "John".to_string(),
            age: 20,
            address: "123 Main St".to_string(),
        };
        _ = base.insert(&user);
        let deserialized = base.get_as::<User>(user.key.as_str()).unwrap();

        assert_eq!(deserialized.key, user.key);

        let qr = base.fetch(q).unwrap();

        assert_eq!(qr["items"].as_array().unwrap().len(), 1);

    }

}