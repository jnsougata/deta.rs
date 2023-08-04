use crate::errors::DetaError;
use crate::utils::*;
use serde_json::{json, Map, Value};
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

const BASE_URL: &str = "https://database.deta.sh/v1";

impl Base {
    pub fn get(&self, key: &str) -> Result<Value, DetaError> {
        let url = format!(
            "{}/{}/{}/items/{}",
            BASE_URL, self.service.project_id, self.name, key
        );
        let res = ureq::get(&url).set("X-API-Key", &self.service.project_key).call()?;
        res.into_json::<Value>().map_err(DetaError::IOError)
    }

    pub fn put(&self, records: Vec<Record>) -> Result<Value, DetaError> {
        let url = format!("{}/{}/{}/items", BASE_URL, self.service.project_id, self.name);
        let mut data = Map::new();
        let mut items = Vec::new();
        for record in records {
            items.push(record.json());
        }
        data.insert("items".to_string(), json!(items));
        let res = ureq::put(&url)
            .set("X-API-Key", &self.service.project_key)
            .send_json(json!(data))?;
        res.into_json::<Value>().map_err(DetaError::IOError)
    }

    pub fn insert(&self, record: Record) -> Result<Value, DetaError> {
        let url = format!("{}/{}/{}/items", BASE_URL, self.service.project_id, self.name);
        let mut data = Map::new();
        data.insert("item".to_string(), json!(record.json()));
        let res = ureq::post(&url)
            .set("X-API-Key", &self.service.project_key)
            .send_json(json!(data))?;
        res.into_json::<Value>().map_err(DetaError::IOError)
    }

    pub fn delete(&self, key: &str) -> Result<Value, DetaError> {
        let url = format!(
            "{}/{}/{}/items/{}",
            BASE_URL, self.service.project_id, self.name, key
        );
        let res = ureq::delete(&url)
            .set("X-API-Key", &self.service.project_key)
            .call()?;
        res.into_json::<Value>().map_err(DetaError::IOError)
    }

    pub fn update(&self, updater: UpdateBuilder) -> Result<Value, DetaError> {
        let url = format!(
            "{}/{}/{}/items/{}",
            BASE_URL, self.service.project_id, self.name, updater.key
        );
        let res = ureq::patch(&url)
            .set("X-API-Key", &self.service.project_key)
            .send_json(updater.json())?;
        res.into_json::<Value>().map_err(DetaError::IOError)
    }

    pub fn query(&self, query: QueryBuilder) -> Result<Value, DetaError> {
        let url = format!("{}/{}/{}/query", BASE_URL, self.service.project_id, self.name);
        let res = ureq::post(&url)
            .set("X-API-Key", &self.service.project_key)
            .send_json(query.json())?;
        res.into_json::<Value>().map_err(DetaError::IOError)
    }
}
