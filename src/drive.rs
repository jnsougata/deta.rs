use crate::{errors::DetaError, query::Paging };

use ureq::Response;
use serde::{ Serialize, Deserialize };
use serde::de::DeserializeOwned;
use serde_json::{ json, Value };


const MAX_CHUNK_SIZE: usize = 10 * 1024 * 1024;

#[derive(Deserialize, Serialize)]
pub struct FileList {
    pub(crate) paging: Option<Paging>,
    pub(crate) names: Vec<String>
}

#[derive(Deserialize, Serialize)]
struct Metadata {
    name: String,
    upload_id: String,
    project_id: String,
    drive_name: String
}


fn de<T: DeserializeOwned>(r: Result<Response, DetaError>) -> Result<T, DetaError> {
    r.map_err(DetaError::from).and_then(|r| {
        r.into_json::<T>().map_err(DetaError::from)
    })
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
        body: Option<&[u8]>,
        content_type: Option<&str>
    ) -> Result<Response, DetaError> {
        let mut req = ureq::request(method, &format!(
            "https://drive.deta.sh/v1/{}/{}{}", self.service.project_id, self.name, path))
            .set("X-API-Key", &self.service.project_key);
        match (json, body) {
            (Some(_), Some(_)) => Err(
                DetaError::PayloadError { msg: String::from("body and json are mutually exclusive.") }
            ),
            (Some(o), None) => {
                req = req.set("Content-Type", "application/json");
                req.send_json(o).map_err(DetaError::from)
            },
            (None, Some(b)) => {
                if content_type.is_some() {
                    req = req.set("Content-Type", content_type.unwrap());
                }
                req.send_bytes(b).map_err(DetaError::from)
            },
            (None, None) => req.call().map_err(DetaError::from),
        }
    }

    /// List files in drive.
    pub fn list(
        &self,
        prefix: Option<&str>,
        limit: Option<i32>,
        last: Option<&str>,
    ) -> Result<FileList, DetaError> {
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
        de::<FileList>(self.request("GET", &path, None, None, None))
    }

    /// Walk through all files in drive and returns a list of file names.
    pub fn walk(&self, prefix: Option<&str>) -> Vec<String> {
        let mut files: Vec<String> = vec![];
        let mut res = self.list(prefix, None, None);
        if res.is_err() {
            return files;
        }
        let mut list = res.unwrap();
        files.append(&mut list.names);
        if list.paging.is_none() {
            return files;
        }
        let mut last = list.paging.unwrap().last;
        while !last.is_empty() {
            res = self.list(prefix, Some(1000), Some(&last));
            if res.is_err() {
                return files;
            }
            list = res.unwrap();
            files.append(&mut list.names);
            last = list.paging.unwrap().last
        }
        files
    }

    /// Get a file from drive.
    pub fn get(&self, name: &str) -> Result<Response, DetaError> {
        let path = format!("/files/download?name={}", name);
        let url = format!(
            "https://drive.deta.sh/v1/{}/{}{}", self.service.project_id, self.name, path);
        ureq::get(&url)
            .set("X-API-Key", &self.service.project_key)
            .call()
            .map_err(DetaError::from)
    }

    /// Put a new file to drive.
    pub fn put(
        &self, save_as: &str, content: &[u8], content_type: Option<&str>
    ) -> Result<Response, DetaError> {
        let encoded = &urlencoding::encode(save_as).into_owned();
        if content.len() <= MAX_CHUNK_SIZE {
            return self.request(
                "POST",
                &format!("/files?name={}", encoded),
                None,
                Some(content),
                content_type
            );
        }
        let res = de::<Metadata>(
            self.request(
                "POST", &format!("/uploads?name={}", encoded), None, None, None));
        if res.is_err() {
            return Err(res.err().unwrap());
        }
        let meta = res.unwrap();
        for (i, chunk) in content.chunks(MAX_CHUNK_SIZE).enumerate() {
            let path = &format!("/uploads/{}/parts?name={}&part={}", meta.upload_id, encoded, i+1);
            let resp = self.request(
                "POST", path, None, Some(chunk), content_type);
            if resp.is_err() {
                _ = self.request("DELETE", path, None, None, None);
                return Err(resp.err().unwrap());
            }
        }
        self.request("PATCH", &format!("/uploads?name={}", encoded), None, None, None)
    }

    /// Delete multiple files from drive.
    pub fn delete(&self, names: Vec<&str>) -> Result<Response, DetaError> {
        self.request("DELETE", "/files", Some(json!({ "names": names })), None, None)
    }
}
