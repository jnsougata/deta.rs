use reqwest;
use crate::utils::*;
use serde_json::{json, Value, Map};


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
    pub project_id: String,
    pub project_key: String,
}

const BASE_URL: &str = "https://database.deta.sh/v1";

impl Base {

    pub fn get(&self, key: &str) -> Result<Value, reqwest::Error> {
        let url = format!("{}/{}/{}/items/{}", BASE_URL, self.project_id, self.name, key);
        let res = reqwest::blocking::Client::new()
            .get(&url)
            .header("X-API-Key", &self.project_key)
            .send()
            .unwrap();
        res.json::<Value>()
    }

    pub fn put(&self, records: Vec<Record>) -> Result<Value, reqwest::Error> {
        let url = format!("{}/{}/{}/items", BASE_URL, self.project_id, self.name);
        let mut data = Map::new();
        let mut items = Vec::new();
        for record in records {
            items.push(record.json());
        }
        data.insert("items".to_string(), json!(items));
        let res = reqwest::blocking::Client::new()
            .put(&url)
            .header("X-API-Key", &self.project_key)
            .json(&json!(data))
            .send()
            .unwrap();
        res.json::<Value>()
    }

    pub fn insert(&self, record: Record) -> Result<Value, reqwest::Error> {
        let url = format!("{}/{}/{}/items", BASE_URL, self.project_id, self.name);
        let mut data = Map::new();
        data.insert("item".to_string(), json!(record.json()));
        let res = reqwest::blocking::Client::new()
            .post(&url)
            .header("X-API-Key", &self.project_key)
            .json(&json!(data))
            .send()
            .unwrap();
        res.json::<Value>()
    }

    pub fn delete(&self, key: &str) -> Result<Value, reqwest::Error> {
        let url = format!("{}/{}/{}/items/{}", BASE_URL, self.project_id, self.name, key);
        //let res = delete(&url, &self.project_key);
        let res = reqwest::blocking::Client::new()
            .delete(&url)
            .header("X-API-Key", &self.project_key)
            .send()
            .unwrap();
        res.json::<Value>()
    }

    pub fn update(&self, updater: UpdateBuilder) -> Result<Value, reqwest::Error> {
        let url = format!("{}/{}/{}/items/{}", BASE_URL, self.project_id, self.name, updater.key);
        let res = reqwest::blocking::Client::new()
            .patch(&url)
            .header("X-API-Key", &self.project_key)
            .json(&updater.json())
            .send()
            .unwrap();
        res.json::<Value>()
    }

    pub fn query(&self, query: QueryBuilder) ->  Result<Value, reqwest::Error> {
        let url = format!("{}/{}/{}/query", BASE_URL, self.project_id, self.name);
        let res = reqwest::blocking::Client::new()
            .post(&url)
            .header("X-API-Key", &self.project_key)
            .json(&query.json())
            .send()
            .unwrap();
        res.json::<Value>()
    }

}