//! # deta.rs
//! This is the unofficial Rust SDK for Deta Base and Drive.


use base::Base;
use drive::Drive;

mod base;
mod drive;
pub mod query;
pub mod errors;
pub mod updater;

fn validate(key: &str) -> Option<&str> {
    let splits = key.split('_').collect::<Vec<&str>>();
    if splits.len() != 2 {
        None
    } else {
        Some(splits[0])
    }
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
        let v = validate(project_key);
        if v.is_none() {
            panic!("Invalid project key, must be in the format `projectId_secret`.");
        }
        Deta{
            project_id: v.unwrap().to_string(),
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
        let env_var = std::env::var("DETA_PROJECT_KEY")
            .expect("Environment variable `DETA_PROJECT_KEY` is not set.");
        let v = validate(&env_var);
        if v.is_none() {
            panic!("Invalid project key, must be in the format `projectId_secret`.");
        }
        Deta {
            project_id: v.unwrap().to_string(),
            project_key: env_var,
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
        Base {
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
        Drive {
            name: name.to_string(),
            service: self.clone(),
        }
    }
}


#[cfg(test)]
mod run_tests {
    use serde_json::json;

    use super::*;

    #[derive(serde::Deserialize, Debug, serde::Serialize)]
    pub struct User {
        pub key: String,
        pub name: String,
        pub age: u8,
        pub address: String,
    }

    #[test]
    fn base() {
        let db = Deta::new().base("hello");
        let user: &User = &User {
            key: String::from("db8213bc"),
            name: String::from("John Doe"),
            age: 20,
            address: String::from("123 Broadway")
        };
        assert!(db.put(vec![user]).unwrap().to_string().contains("db8213bc"));
        assert_eq!(db.get_as::<User>("db8213bc").unwrap().name, user.name);
        assert!(db.insert(user).is_err_and(|e| e.to_string().contains("409")));
        assert!(!db.query()
            .sort(true)
            .contains("name", json!("John"))
            .greater_than("age", json!(18))
            .walk().unwrap().is_empty()
        );
        assert!(db.update("db8213bc")
            .set("name", json!("John"))
            .increment("age", json!(24))
            .commit().unwrap().to_string().contains("db8213bc")
        );
        assert!(db.delete("db8213bc").is_ok());
    }

    #[test]
    fn drive() {
        let db = Deta::new().drive("world");
        assert!(db.put("test.txt", b"Hello, World!", None).is_ok());
        assert!(!db.list(None, None, None).unwrap().names.is_empty());
        assert!(!db.walk(None).is_empty());
        assert!(db.get("test.txt").is_ok());
        assert!(db.delete(vec!["test.txt"]).is_ok());
    }
}