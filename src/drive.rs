use serde_json::{json, Value};

/// Drive is the struct that represents a Deta Drive.
/// ## Methods
/// #### `list(prefix, limit, last)`
/// Gets a list of files in the drive.
/// #### `get(filename)`
/// Gets a file from the drive. Returns a tuple of the status code and the file contents.
/// #### `put(save_as, content)`
/// Puts a file in the drive. Returns a tuple of the status code and the response.
/// #### `delete(names)`
/// Deletes file(s) from the drive. Returns a tuple of the status code and the response.

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
    ) -> Result<Value, std::io::Error> {
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
        let resp = ureq::get(&url)
            .set("X-Api-Key", &self.project_key)
            .set("Content-Type", "application/json")
            .call()
            .unwrap();
        resp.into_json::<Value>()
    }

    pub fn get(&self, filename: &str) -> Result<String, std::io::Error> {
        let url = format!(
            "{}/{}/{}/files/download?name={}",
            DRIVE_URL, self.project_id, self.name, filename
        );
        let resp = ureq::get(&url)
            .set("X-Api-Key", &self.project_key)
            .set("Content-Type", "application/json")
            .call()
            .unwrap();
        resp.into_string()
    }

    pub fn put(&self, save_as: &str, content: &[u8]) -> Result<Value, std::io::Error> {
        if content.len() <= 10 * 1024 * 1024 {
            let url = format!(
                "{}/{}/{}/files?name={}",
                DRIVE_URL, self.project_id, self.name, save_as
            );
            let resp = ureq::post(&url)
                .set("X-Api-Key", &self.project_key)
                .set("Content-Type", "application/octet-stream")
                .set("name", save_as)
                .send_bytes(content)
                .unwrap();
            resp.into_json::<Value>()
        } else {
            const CHUNK_SIZE: usize = 10 * 1024 * 1024;
            let chunks = content.chunks(CHUNK_SIZE);
            let init_url = format!(
                "{}/{}/{}/uploads?name={}",
                DRIVE_URL, self.project_id, self.name, save_as
            );
            let init_resp = ureq::post(&init_url)
                .set("X-Api-Key", &self.project_key)
                .set("Content-Type", "application/octet-stream")
                .set("name", save_as)
                .call()
                .unwrap();
            let init_data = init_resp.into_json::<Value>().unwrap();
            let upload_id = init_data["upload_id"].to_string().replace('\"', "");
            let file_name = init_data["name"].to_string().replace('\"', "");
            let mut done = vec![];
            for (index, chunk) in chunks.enumerate() {
                let part_url = format!(
                    "{}/{}/{}/uploads/{}/parts?name={}&part={}",
                    DRIVE_URL,
                    self.project_id,
                    self.name,
                    upload_id,
                    file_name,
                    index + 1
                );
                let part_resp = ureq::post(&part_url)
                    .set("X-Api-Key", &self.project_key)
                    .set("Content-Type", "application/octet-stream")
                    .send_bytes(chunk)
                    .unwrap();
                done.push(part_resp.status());
            }
            let success = done.iter().all(|&x| x == 200);
            if success {
                let complete_url = format!(
                    "{}/{}/{}/uploads/{}?name={}",
                    DRIVE_URL, self.project_id, self.name, upload_id, file_name
                );
                let complete_resp = ureq::patch(&complete_url)
                    .set("X-Api-Key", &self.project_key)
                    .set("Content-Type", "application/json")
                    .call()
                    .unwrap();
                complete_resp.into_json::<Value>()
            } else {
                let abort_url = format!(
                    "{}/{}/{}/uploads/{}?name={}",
                    DRIVE_URL, self.project_id, self.name, upload_id, file_name
                );
                let abort_resp = ureq::delete(&abort_url)
                    .set("X-Api-Key", &self.project_key)
                    .set("Content-Type", "application/json")
                    .call()
                    .unwrap();
                abort_resp.into_json::<Value>()
            }
        }
    }

    pub fn delete(&self, names: Vec<&str>) -> Result<Value, std::io::Error> {
        let url = format!("{}/{}/{}/files", DRIVE_URL, self.project_id, self.name);
        let payload = json!({ "names": names });
        let resp = ureq::delete(&url)
            .set("X-Api-Key", &self.project_key)
            .set("Content-Type", "application/json")
            .send_json(payload.to_string())
            .unwrap();
        resp.into_json::<Value>()
    }
}
