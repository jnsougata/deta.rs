use crate::https::*;

pub struct Record {
    pub key: Option<String>,
    pub value: serde_json::Value,
    pub expires_in: Option<i64>,  
    pub expires_at: Option<i64>,
}

impl Record {
    pub fn json(&self) -> serde_json::Value {
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

pub struct Updater {
    pub key: String,
    pub set: Option<serde_json::Value>,
    pub delete: Option<Vec<String>>,
    pub append: Option<serde_json::Value>,
    pub prepend: Option<serde_json::Value>,
    pub increment: Option<serde_json::Value>,
}

impl Updater {
    fn json(&self) -> serde_json::Value {
        let mut data = serde_json::Map::new();
        if let Some(set) = &self.set {
            data.insert("set".to_string(), set.clone());
        }
        if let Some(delete) = &self.delete {
            data.insert("delete".to_string(), serde_json::json!(delete));
        }
        if let Some(append) = &self.append {
            data.insert("append".to_string(), append.clone());
        }
        if let Some(prepend) = &self.prepend {
            data.insert("prepend".to_string(), prepend.clone());
        }
        if let Some(increment) = &self.increment {
            data.insert("increment".to_string(), increment.clone());
        }
        return serde_json::json!(data);
    }
}

pub struct Query {
    pub payload: Option<serde_json::Value>,
    pub limit: Option<i64>,
    pub last: Option<String>,
}

impl Query {
    pub fn json(&self) -> serde_json::Value {
        let mut data = serde_json::Map::new();
        if let Some(payload) = &self.payload {
            data.insert("query".to_string(), payload.clone());
        }
        if let Some(limit) = &self.limit {
            data.insert("limit".to_string(), serde_json::json!(limit));
        }
        if let Some(last) = &self.last {
            data.insert("last".to_string(), serde_json::json!(last));
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

    pub fn delete(&self, key: &str) -> (u16, serde_json::Value) {
        let url = format!("{}/{}/{}/items/{}", BASE_URL, self.project_id, self.name, key);
        let res = delete(&url, &self.project_key);
        return (res.status().as_u16(), res.json::<serde_json::Value>().unwrap());
    }

    pub fn update(&self, updater: Updater) -> (u16, serde_json::Value) {
        let url = format!("{}/{}/{}/items/{}", BASE_URL, self.project_id, self.name, updater.key);
        let res = patch(&url, &self.project_key, updater.json());
        return (res.status().as_u16(), res.json::<serde_json::Value>().unwrap());
    }

    pub fn query(&self, query: Query) -> (u16, serde_json::Value) {
        let url = format!("{}/{}/{}/query", BASE_URL, self.project_id, self.name);
        let res = post(&url, &self.project_key, query.json());
        return (res.status().as_u16(), res.json::<serde_json::Value>().unwrap());
    }
    
}