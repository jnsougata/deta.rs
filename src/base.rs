use crate::{ errors::DetaError, query::Query, updater::Updater };

use serde::{ Serialize, de::DeserializeOwned };
use serde_json::{ Value, Map, json };

/// Represents a Deta Base.
#[derive(Clone)]
pub struct Base {
    pub name: String,
    pub(crate) service: crate::Deta,
}


impl Base {

    pub (crate) fn request(
        &self,
        method: &str,
        path: &str,
        body: Option<Value>
    ) -> Result<Value, DetaError> {
        let req = ureq::request(method, &format!(
            "https://database.deta.sh/v1/{}/{}{}", self.service.project_id, self.name, path))
            .set("X-API-Key", &self.service.project_key);
        let resp = match body {
            Some(body) => req.send_json(body),
            None => req.call()
        };
        
        resp.map_err(DetaError::from)
            .and_then(
                |res| serde_json::from_reader(res.into_reader()).map_err(DetaError::from)
            )
    }

    /// fetch a record by key from the base. 
    pub fn get(&self, key: &str) -> Result<Value, DetaError> {
        self.request("GET", &format!("/items/{}", key), None)
    }

    /// Fetch a record by key from the base and deserialize it to a struct.
    pub fn get_as<T: DeserializeOwned>(&self, key: &str) -> Result<T, DetaError> {
        self.get(key).and_then(|v| serde_json::from_value::<T>(v).map_err(DetaError::from))
    }

    /// Put a multiple serializable records into the base.
    /// 
    /// Maximum 25 records can be put at a time.
    /// 
    /// Overwrites existing records with the same key.
    pub fn put<T: Serialize>(&self, records: Vec<T>) -> Result<Value, DetaError> {
        if records.len() > 25 {
            return Err(
                DetaError::PayloadError {
                    msg: "maximum 25 records can be put at a time".to_string()
                }
            );
        }
        let mut payload = Map::new();
        payload.insert(String::from("items"), json!(&records));
        self.request("PUT", "/items", Some(json!(payload)))
    }

    /// Insert a serializable record into the base.
    pub fn insert<T: Serialize>(&self, record: T) -> Result<Value, DetaError> {
        let mut payload = Map::new();
        payload.insert(String::from("item"), json!(&record));
        self.request("POST", "/items", Some(json!(payload)))
    }

    /// Delete a record by key from the base.
    pub fn delete(&self, key: &str) -> Result<Value, DetaError> {
        self.request("DELETE", &format!("/items/{}", key), None)
    }

    /// Update a record by key in the base.
    pub fn update(&self, key: &str) -> Updater {
        Updater::new(self.clone(), key)
    }

    /// Create a new query for this base.
    pub fn query(&self) -> Query {
        Query::new(self.clone())
    }
    
}
