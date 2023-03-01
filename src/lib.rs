//! # Deta Rust SDK
//! This is the unofficial Rust SDK for Deta Base and Drive.
//! ## Usage
//! ```rust
//! use deta_rs;
//!
//! let deta = deta_rs::new("project_key");
//! let base = deta.base("base_name");
//! let drive = deta.drive("drive_name");
//!
//! let record = base.get("key_here");
//! let file = drive.get("sample.png");
//! ```

pub mod base;
pub mod drive;
pub mod utils;

#[derive(Clone)]
pub struct Deta {
    pub project_key: String,
    pub project_id: String,
}

impl Deta {
    pub fn base(&self, name: &str) -> base::Base {
        base::Base {
            name: name.to_string(),
            project_id: self.project_id.clone(),
            project_key: self.project_key.clone(),
        }
    }

    pub fn drive(&self, name: &str) -> drive::Drive {
        drive::Drive {
            name: name.to_string(),
            project_id: self.project_id.clone(),
            project_key: self.project_key.clone(),
        }
    }

    pub fn new(project_key: &str) -> Deta {
        let project_id = project_key.split("_").collect::<Vec<&str>>()[0];
        Deta {
            project_key: project_key.to_string(),
            project_id: project_id.to_string(),
        }
    }
}
