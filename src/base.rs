use crate::{
    errors::DetaError, 
    utils::UpdateBuilder,
    query::Query 
};
use serde_json::{Value, Map, json};
use serde::{Serialize, de::DeserializeOwned};
use ureq;

/// Base represents a struct that can be used to interact with a Deta Base.
/// ## Methods
/// #### get(key)
/// Gets the record with the specified key.
/// #### put(records)
/// Puts the specified records in the Base.
/// #### update(updaters)
/// Updates the specified records in the Base.
/// #### delete(keys)
/// Deletes the records with the specified keys.
/// #### query(query)
/// Queries the Base with the specified query.
///
/// ## Example
/// ```rs
/// use deta_rs::base::{Base, Record};
///
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
        print!("{}", url);
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

    pub fn get(&self, key: &str) -> Result<Value, DetaError> {
        self.request("GET", &format!("/items/{}", key), None)
    }

    pub fn get_as<T>(&self, key: &str) -> Result<T, DetaError> where T: DeserializeOwned {
        let val = serde_json::from_value::<T>(self.get(key)?);
        if val.is_err() {
            return Err(DetaError::JSONError(val.err().unwrap()));
        }
        Ok(val?)
    }

    pub fn put<T>(&self, records: Vec<T>) -> Result<Value, DetaError> where T: Serialize {
        let mut data = Map::new();
        let mut items = Vec::new();
        for record in records {
            items.push(serde_json::to_value(&record).unwrap());
        }
        data.insert("items".to_string(), json!(items));
        self.request("PUT", "/items", Some(json!(data)))
    }

    pub fn insert<T>(&self, record: T) -> Result<Value, DetaError> where T: Serialize{
        let mut data = Map::new();
        data.insert("item".to_string(), serde_json::to_value(&record).unwrap());
        self.request("POST", "/items", Some(json!(data)))
    }

    pub fn delete(&self, key: &str) -> Result<Value, DetaError> {
        self.request("DELETE", &format!("/items/{}", key), None)
    }

    pub fn update(&self, builder: UpdateBuilder) -> Result<Value, DetaError> {
        self.request("PATCH", &format!("/items/{}", builder.key), Some(builder.json()))
    }

    pub fn fetch(&self, builder: Query) -> Result<Value, DetaError> {
        self.request("POST", "/query", Some(serde_json::to_value(builder).unwrap()))
    }
    
}
