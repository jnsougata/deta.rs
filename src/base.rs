use crate::{
    errors::DetaError, 
    query::Query,
    updater::Updater, 
};
use serde_json::{Value, Map, json};
use serde::{Serialize, de::DeserializeOwned};
use ureq;

/// Represents a Deta Base.
pub struct Base {
    pub name: String,
    pub(crate) service: crate::Deta,
}


impl Base {

    fn request(
        &self, 
        method: &str, 
        path: &str, 
        body: Option<Value>
    ) -> Result<Value, DetaError> {
        let url = format!("https://database.deta.sh/v1/{}/{}{}", self.service.project_id, self.name, path);
        let mut req = ureq::request(method, &url);
        req = req.set("X-API-Key", &self.service.project_key);
        let resp = match body {
            Some(body) => req.send_json(body),
            None => req.call()
        };
        if resp.is_err() {
            return Err(DetaError::from(resp.err().unwrap()));
        } else {
            Ok(resp.unwrap().into_json().unwrap())
        }
    }

    /// fetch a record by key from the base. 
    pub fn get(&self, key: &str) -> Result<Value, DetaError> {
        self.request("GET", &format!("/items/{}", key), None)
    }

    /// Fetch a record by key from the base and deserialize it to a struct.
    pub fn get_as<T>(&self, key: &str) -> Result<T, DetaError> where T: DeserializeOwned {
        let val = serde_json::from_value::<T>(self.get(key)?);
        if val.is_err() {
            return Err(DetaError::JSONError(val.err().unwrap()));
        }
        Ok(val?)
    }

    /// Put a multiple serializable records into the base.
    /// 
    /// Maximum 25 records can be put at a time.
    /// 
    /// Overwrites existing records with the same key.
    pub fn put<T>(&self, records: Vec<T>) -> Result<Value, DetaError> where T: Serialize {
        let mut data = Map::new();
        let mut items = Vec::new();
        for record in records {
            items.push(serde_json::to_value(&record).unwrap());
        }
        data.insert("items".to_string(), json!(items));
        self.request("PUT", "/items", Some(json!(data)))
    }

    /// Insert a serializable record into the base.
    pub fn insert<T>(&self, record: T) -> Result<Value, DetaError> where T: Serialize{
        let mut data = Map::new();
        data.insert("item".to_string(), serde_json::to_value(&record).unwrap());
        self.request("POST", "/items", Some(json!(data)))
    }

    /// Delete a record by key from the base.
    pub fn delete(&self, key: &str) -> Result<Value, DetaError> {
        self.request("DELETE", &format!("/items/{}", key), None)
    }

    /// Update a record by key in the base.
    pub fn update(&self, key: &str, builder: Updater) -> Result<Value, DetaError> {
        self.request("PATCH", &format!("/items/{}", key), Some(serde_json::to_value(builder).unwrap()))
    }

    /// Fetch records from the base using a query.
    pub fn fetch(&self, builder: Query) -> Result<Value, DetaError> {
        self.request("POST", "/query", Some(serde_json::to_value(builder).unwrap()))
    }
    
}
