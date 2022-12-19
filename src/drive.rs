pub struct Drive {
    pub name: String,
    pub project_id: String,
    pub project_key: String,
}

const DRIVE_URL: &str = "https://drive.deta.sh/v1";

impl Drive {

    pub fn list(
        &self,
        prefix: Option<&str>,
        limit: Option<i32>,
        last: Option<String>,
    ) -> (u16, serde_json::Value) {
        let mut url = format!("{}/{}/{}/files?", DRIVE_URL, self.project_id, self.name);
        if let Some(limit) = limit {
            url.push_str(&format!("limit={}", limit));
        } else {
            url.push_str("limit=1000");
        }
        if let Some(prefix) = prefix {
            url.push_str(&format!("&prefix={}", prefix));
        }
        if let Some(last) = last {
            url.push_str(&format!("&last={}", last));
        }
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(&url)
            .header("X-Api-Key", &self.project_key)
            .header("Content-Type", "application/json")
            .send()
            .unwrap();
        return (resp.status().as_u16(), resp.json::<serde_json::Value>().unwrap());
    }
    
    pub fn get(&self, filename: &str) -> (u16, Vec<u8>) {
        let url = format!("{}/{}/{}/files/download?name={}", DRIVE_URL, self.project_id, self.name, filename);
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(&url)
            .header("X-Api-Key", &self.project_key)
            .header("Content-Type", "application/json")
            .send().unwrap();
        return (resp.status().as_u16(), resp.bytes().unwrap().to_vec());
    }

    pub fn put<'a >(&self, save_as: &str, content: &'a [u8]) -> (u16, serde_json::Value) {
        if content.len() <= 10 * 1024 * 1024 {
            let url = format!("{}/{}/{}/files?name={}", DRIVE_URL, self.project_id, self.name, save_as);
            let client = reqwest::blocking::Client::new();
            let resp = client
                .post(&url)
                .header("X-Api-Key", &self.project_key)
                .header("Content-Type", "application/octet-stream")
                .header("name", save_as)
                .body(content.to_owned())
                .send().unwrap();
            return (resp.status().as_u16(), resp.json::<serde_json::Value>().unwrap());
        } else {
            const CHUNK_SIZE: usize = 10 * 1024 * 1024;
            let chunks = content.chunks(CHUNK_SIZE);
            let init_url = format!(
                "{}/{}/{}/uploads?name={}", 
                DRIVE_URL, self.project_id, self.name, save_as
            );
            let client = reqwest::blocking::Client::new();
            let init_resp = client
                .post(&init_url)
                .header("X-Api-Key", &self.project_key)
                .header("Content-Type", "application/octet-stream")
                .header("name", save_as)
                .send().unwrap();
            let init_data = init_resp.json::<serde_json::Value>().unwrap();
            let upload_id = init_data["upload_id"].to_string().replace("\"", "");
            let file_name = init_data["name"].to_string().replace("\"", "");
            let mut done = vec![];
            for (index, chunk) in chunks.enumerate() {
                let part_url = format!(
                    "{}/{}/{}/uploads/{}/parts?name={}&part={}", 
                    DRIVE_URL, self.project_id, self.name, upload_id, file_name, index + 1
                );
                let client = reqwest::blocking::Client::new();
                let part_resp = client
                    .post(&part_url)
                    .header("X-Api-Key", &self.project_key)
                    .header("Content-Type", "application/octet-stream")
                    .body(chunk.to_owned())
                    .send().unwrap();
                done.push(part_resp.status().as_u16());
            }
            let success = done.iter().all(|&x| x == 200);
            if success {
                let complete_url = format!(
                    "{}/{}/{}/uploads/{}?name={}", 
                    DRIVE_URL, self.project_id, self.name, upload_id, file_name
                );
                let client = reqwest::blocking::Client::new();
                let complete_resp = client
                    .patch(&complete_url)
                    .header("X-Api-Key", &self.project_key)
                    .header("Content-Type", "application/json")
                    .send().unwrap();
                return (complete_resp.status().as_u16(), complete_resp.json::<serde_json::Value>().unwrap());
            } else {
                let abort_url = format!(
                    "{}/{}/{}/uploads/{}?name={}", 
                    DRIVE_URL, self.project_id, self.name, upload_id, file_name
                );
                let client = reqwest::blocking::Client::new();
                let abort_resp = client
                    .delete(&abort_url)
                    .header("X-Api-Key", &self.project_key)
                    .header("Content-Type", "application/json")
                    .send().unwrap();
                return (abort_resp.status().as_u16(), abort_resp.json::<serde_json::Value>().unwrap());
            }
        }
    }

    pub fn delete(&self, names: Vec<&str>) -> (u16, serde_json::Value) {
        let url = format!("{}/{}/{}/files", DRIVE_URL, self.project_id, self.name);
        let payload = serde_json::json!({
            "names": names
        });
        let client = reqwest::blocking::Client::new();
        let resp = client
            .delete(&url)
            .header("X-Api-Key", &self.project_key)
            .header("Content-Type", "application/json")
            .body(payload.to_string())
            .send().unwrap();
        return (resp.status().as_u16(), resp.json::<serde_json::Value>().unwrap());
    }
    
}