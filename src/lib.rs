mod https;
pub mod base;


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

    pub fn drive() {
        // not implemented
    }
}

pub fn new(project_key: &str) -> Deta {
    let project_key = project_key;
    let project_id = project_key.split("_").collect::<Vec<&str>>()[0].to_string();
    Deta { project_key: project_key.to_string(), project_id }
}
