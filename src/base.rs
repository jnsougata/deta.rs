use reqwest;

#[derive(Default, Clone, Debug)]
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
    pub fn copy(&self) -> Record {
        return Record {
            key: self.key.clone(),
            value: self.value.clone(),
            expires_in: self.expires_in.clone(),
            expires_at: self.expires_at.clone(),
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct Updater {
    pub key: String,
    updates: serde_json::Value,
}

impl Updater {

    pub fn new(key: &str) -> Updater {
        return Updater {
            key: key.to_string(),
            updates: serde_json::json!({
                "set": {},
                "delete": [],
                "append": {},
                "prepend": {},
                "increment": {},
            }),
        }
    }
    
    pub fn set(&mut self, field: &str, value: serde_json::Value) {
        self.updates["set"][field] = value;
    }

    pub fn delete(&mut self, fields: Vec<&str>) {
        for field in fields {
            self.updates["delete"].as_array_mut().unwrap().push(serde_json::json!(field));
        }
    }

    pub fn append(&mut self, field: &str, value: serde_json::Value) {
        self.updates["append"][field] = value;
    }

    pub fn prepend(&mut self, field: &str, value: serde_json::Value) {
        self.updates["prepend"][field] = value;
    }

    pub fn increment(&mut self, field: &str, value: i64) {
        self.updates["increment"][field] = serde_json::json!(value);
    }

    fn json(&self) -> serde_json::Value {
        return self.updates.clone();
    }
    
}

#[derive(Default, Clone, Debug)]
pub struct Query {
    pub payload: Option<Vec<serde_json::Value>>,
    pub limit: Option<i64>,
    pub last: Option<String>,
}

impl Query {
    pub fn json(&self) -> serde_json::Value {
        let mut data = serde_json::Map::new();
        if let Some(payload) = &self.payload {
            data.insert("query".to_string(), serde_json::json!(payload));
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

    pub fn get(&self, key: &str) -> (u16, serde_json::Value) {
        let url = format!("{}/{}/{}/items/{}", BASE_URL, self.project_id, self.name, key);
        let res = reqwest::blocking::Client::new()
            .get(&url)
            .header("X-API-Key", &self.project_key)
            .send()
            .unwrap();
        return (res.status().as_u16(), res.json::<serde_json::Value>().unwrap());
    }

    pub fn put(&self, records: Vec<Record>) -> (u16, serde_json::Value) {
        let url = format!("{}/{}/{}/items", BASE_URL, self.project_id, self.name);
        let mut data = serde_json::Map::new();
        let mut items = Vec::new();
        for record in records {
            items.push(record.json());
        }
        data.insert("items".to_string(), serde_json::json!(items));
        let res = reqwest::blocking::Client::new()
            .put(&url)
            .header("X-API-Key", &self.project_key)
            .json(&serde_json::json!(data))
            .send()
            .unwrap();
        return (res.status().as_u16(), res.json::<serde_json::Value>().unwrap());
    }

    pub fn insert(&self, record: Record) -> (u16, serde_json::Value) {
        let url = format!("{}/{}/{}/items", BASE_URL, self.project_id, self.name);
        let mut data = serde_json::Map::new();
        data.insert("item".to_string(), serde_json::json!(record.json()));
        let res = reqwest::blocking::Client::new()
            .post(&url)
            .header("X-API-Key", &self.project_key)
            .json(&serde_json::json!(data))
            .send()
            .unwrap();
        return (res.status().as_u16(), res.json::<serde_json::Value>().unwrap());
    }

    pub fn delete(&self, key: &str) -> (u16, serde_json::Value) {
        let url = format!("{}/{}/{}/items/{}", BASE_URL, self.project_id, self.name, key);
        //let res = delete(&url, &self.project_key);
        let res = reqwest::blocking::Client::new()
            .delete(&url)
            .header("X-API-Key", &self.project_key)
            .send()
            .unwrap();
        return (res.status().as_u16(), res.json::<serde_json::Value>().unwrap());
    }

    pub fn update(&self, updater: Updater) -> (u16, serde_json::Value) {
        let url = format!("{}/{}/{}/items/{}", BASE_URL, self.project_id, self.name, updater.key);
        let res = reqwest::blocking::Client::new()
            .patch(&url)
            .header("X-API-Key", &self.project_key)
            .json(&updater.json())
            .send()
            .unwrap();
        return (res.status().as_u16(), res.json::<serde_json::Value>().unwrap());
    }

    pub fn query(&self, query: Query) -> (u16, serde_json::Value) {
        let url = format!("{}/{}/{}/query", BASE_URL, self.project_id, self.name);
        //let res = post(&url, &self.project_key, query.json());
        let res = reqwest::blocking::Client::new()
            .post(&url)
            .header("X-API-Key", &self.project_key)
            .json(&query.json())
            .send()
            .unwrap();
        return (res.status().as_u16(), res.json::<serde_json::Value>().unwrap());
    }

}