use crate::https::*;

pub struct Record {
    pub key: Option<String>,
    pub value: serde_json::Value,
    pub expires_in: Option<i64>,  
    pub expires_at: Option<i64>,
}

impl Record {
    pub fn json(&self) -> serde_json::Value{
        let mut data = self.value.as_object().unwrap().clone();
        if let Some(key) = &self.key {
            data.insert("key".to_string(), serde_json::json!(key));
        }
        if let Some(expires_in) = &self.expires_in {
            let timestamp = chrono::Utc::now().timestamp() + expires_in;
            data.insert("__expires".to_string(), serde_json::json!(timestamp));
        } else if let Some(expires_at) = &self.expires_at {
            data.insert("__expires".to_string(), serde_json::json!(expires_at));
        }
        return serde_json::json!(data);
    }
}




pub struct Base { 
    pub name: String,
    pub project_id: String,
    pub project_key: String,
}

const BASE_URL: &str = "https://database.deta.sh/v1";

impl Base {

    pub fn get(&self, key: &str) -> serde_json::Value {
        let url = format!("{}/{}/{}/items/{}", BASE_URL, self.project_id, self.name, key);
        let res = get(&url, &self.project_key);
        res.json::<serde_json::Value>().unwrap()
    }

    pub fn put(&self, records: Vec<Record>) -> (u16, serde_json::Value) {
        let url = format!("{}/{}/{}/items", BASE_URL, self.project_id, self.name);
        let mut data = serde_json::Map::new();
        let mut items = Vec::new();
        for record in records {
            items.push(record.json());
        }
        data.insert("items".to_string(), serde_json::json!(items));
        let res = put(&url, &self.project_key, serde_json::json!(data));
        return (res.status().as_u16(), res.json::<serde_json::Value>().unwrap());
    }

    pub fn insert(&self, record: Record) -> (u16, serde_json::Value) {
        let url = format!("{}/{}/{}/items", BASE_URL, self.project_id, self.name);
        let mut data = serde_json::Map::new();
        data.insert("item".to_string(), serde_json::json!(record.json()));
        let res = post(&url, &self.project_key, serde_json::json!(data));
        return (res.status().as_u16(), res.json::<serde_json::Value>().unwrap());
    }

    pub fn delete(&self, key: &str) -> serde_json::Value {
        let url = format!("{}/{}/{}/items/{}", BASE_URL, self.project_id, self.name, key);
        let res = delete(&url, &self.project_key);
        res.json::<serde_json::Value>().unwrap()
    }

    pub fn update(&self, key: &str, value: serde_json::Value) -> serde_json::Value {
        let url = format!("{}/{}/{}/items/{}", BASE_URL, self.project_id, self.name, key);
        let res = patch(&url, &self.project_key, value);
        res.json::<serde_json::Value>().unwrap()
    }
  
}