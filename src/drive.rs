use crate::{errors::DetaError, query::Paging};
use serde_json::{json, Value};
use ureq::Response;
use serde::{Serialize, Deserialize};


const MAX_CHUNK_SIZE: usize = 10 * 1024 * 1024;

#[derive(Deserialize, Serialize)]
struct FileList {
    paging: Paging,
    names: Vec<String>
}

/// Represents a Deta Drive.
pub struct Drive {
    pub name: String,
    pub(crate) service: crate::Deta,
}

impl Drive {

    fn request(
        &self, 
        method: &str, 
        path: &str, 
        json: Option<Value>,
        content: Option<&[u8]>,
        content_type: Option<&str>
    ) -> Result<Value, DetaError> {
        let url = format!(
            "https://drive.deta.sh/v1/{}/{}{}", self.service.project_id, self.name, path);
        let mut req = ureq::request(method, &url);
        req = req.set("X-API-Key", &self.service.project_key);
        match content_type {
            Some(content_type) => req = req.set("Content-Type", content_type),
            None => req = req.set("Content-Type", "application/json")
        }
        let resp = match (json, content) {
            (Some(_), Some(_)) => return Err(DetaError::TransportError),
            (Some(body), None) => req.send_json(body),
            (None, Some(body)) => req.send_bytes(body),
            (None, None) => req.call(),
        };
        if resp.is_err() {
            return Err(DetaError::from(resp.err().unwrap()));
        } else {
            Ok(resp.unwrap().into_json().unwrap())
        }
    }

    /// List files in drive.
    pub fn list(
        &self,
        prefix: Option<&str>,
        limit: Option<i32>,
        last: Option<&str>,
    ) -> Result<Value, DetaError> {
        let mut path = String::from("/files?");
        if let Some(limit) = limit {
            path.push_str(&format!("limit={}", limit));
        } else {
            path.push_str("limit=1000");
        }
        if let Some(prefix) = prefix {
            path.push_str(&format!("&prefix={}", prefix));
        }
        if let Some(last) = last {
            path.push_str(&format!("&last={}", last));
        }
        self.request("GET", &path, None, None, None)
    }

    pub fn list_all(&self, prefix: Option<&str>) -> Result<Value, DetaError> {
        let mut resp = self.list(prefix, None, None)?;
        let mut result = serde_json::from_value::<FileList>(resp.clone()).unwrap();
        while result.paging.last != "" {
            resp = self.list(prefix, Some(1000), Some(result.paging.last.as_str()))?;
            let tmp = serde_json::from_value::<FileList>(resp.clone()).unwrap();
            result.paging = tmp.paging;
            result.names.append(&mut tmp.names.clone());
        }
        result.paging.size = result.names.len() as u16;
        Ok(serde_json::to_value(result).unwrap())
    }

    /// Get a file from drive.
    pub fn get(&self, name: &str) -> Result<Response, DetaError> {
        let path = format!("/files/download?name={}", name);
        let url = format!(
            "https://drive.deta.sh/v1/{}/{}{}", self.service.project_id, self.name, path);
        let mut req = ureq::get(&url);
        req = req.set("X-API-Key", &self.service.project_key);
        let resp = req.call();

        if resp.is_err() {
            return Err(DetaError::from(resp.err().unwrap()));
        } else {
            Ok(resp.unwrap())
        }
    }

    /// Put a new file to drive.
    pub fn put(
        &self, save_as: &str, content: &[u8], content_type: Option<&str>
    ) -> Result<Value, DetaError> {
        if content.len() <= MAX_CHUNK_SIZE {
            self.request(
                "POST",
                &format!("/files?name={}", save_as),
                None,
                Some(content),
                content_type
            )
        } else {
            let chunks = content.chunks(MAX_CHUNK_SIZE);
            let init = self.request(
                "POST",
                &format!("/uploads?name={}", save_as),
                None,
                None,
                None
            )?;
            let upload_id = init["upload_id"].to_string().replace('\"', "");
            let mut done: Vec<Result<Value, DetaError> > = vec![];
            for (i, chunk) in chunks.enumerate() {
                let path = format!("/uploads/{}/parts?name={}&part={}", upload_id, save_as, i+1);
                done.push(self.request("POST", &path, None, Some(chunk), content_type)
                )
            }
            let success = done.iter().all(|x| x.is_err());
            let path = format!("/uploads/{}?name={}", upload_id, save_as);
            if success {
                self.request("PATCH", &path, None, None, None)
            } else {
                self.request("DELETE", &path, None, None, None)
            }
        }
    }

    /// Delete multiple files from drive.
    pub fn delete(&self, names: Vec<&str>) -> Result<Value, DetaError> {
        self.request("DELETE", "/files", Some(json!({ "names": names })), None, None)
    }

}
