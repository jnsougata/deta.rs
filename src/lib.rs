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

mod base;
mod drive;
pub mod utils;
pub mod errors;

#[derive(Clone)]
pub struct Deta {
    pub project_key: String,
    project_id: String,
}

impl Deta {
    pub fn base(&self, name: &str) -> base::Base {
        base::Base {
            name: name.to_string(),
            service: self.clone(),
        }
    }

    pub fn drive(&self, name: &str) -> drive::Drive {
        drive::Drive {
            name: name.to_string(),
            service: self.clone(),
        }
    }

    pub fn new(project_key: &str) -> Deta {
        let d = project_key.split('_').collect::<Vec<&str>>();
        if d.len() != 2 {
            panic!("Invalid project key.");
        }
        Deta {
            project_key: project_key.to_string(),
            project_id: d[0].to_string(),
        }
    }
}
