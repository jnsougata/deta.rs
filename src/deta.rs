use crate::base::Base;

pub struct Deta {
    project_key: String,
    project_id: String,
}

impl Deta {
    pub fn base(&self, name: &str) -> Base {
        Base {
            name: name.to_string(),
            project_id: self.project_id.clone(),
            project_key: self.project_key.clone(),
        }
    }
}


pub fn new(key: &str) -> Deta {
    let project_key = key;
    let project_id = project_key.split("_").collect::<Vec<&str>>()[0].to_string();
    Deta { project_key: project_key.to_string(), project_id }
}