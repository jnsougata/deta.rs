use reqwest;
use serde_json::{json, Value, Map};


/// Record represents a struct that can be inserted into a Deta Base.
/// ## Fields
/// #### `key` (Optional) 
/// The key of the record. If not provided, a random key will be generated.
/// #### `value` (Required)
/// The value of the record. Must be a valid JSON object.
/// #### `expires_in` (Optional)
/// The number of seconds after which the record will expire.
/// If not provided, the record will not expire.
/// #### `expires_at` (Optional)
/// The timestamp at which the record will expire. 
/// If not provided, the record will not expire.
/// ## Example
/// ```rs
/// use deta_rs::base::Record;
/// 
/// let record = Record {
///     key: Some("key".to_string()),
///     value: serde_json::json!({"name": "John", "age": 30}),
///     expires_in: None,
///     expires_at: None,
/// };
/// base.put(vec![record])
/// ```
/// Record implements the Default trait, 
/// so you can use the default() method to create a new Record.
/// also, you can use the copy() method to create a copy of an existing Record.
/// ```rs
/// use deta_rs::base::Record;
/// let record = Record::default();
/// let record_copy = record.copy();
/// ```

#[derive(Default, Clone, Debug)]
pub struct Record {
    pub key: Option<String>,
    pub value: Value,
    pub expires_in: Option<i64>,  
    pub expires_at: Option<i64>,
}

impl Record {

    pub fn json(&self) -> Value {
        let mut data = self.value.as_object().unwrap().clone();
        if let Some(key) = &self.key {
            data.insert("key".to_string(), json!(key));
        }
        if let Some(expires_in) = &self.expires_in {
            let timestamp = chrono::Utc::now().timestamp() + expires_in;
            data.insert("__expires".to_string(), json!(timestamp));
        } else if let Some(expires_at) = &self.expires_at {
            data.insert("__expires".to_string(), json!(expires_at));
        }
        return json!(data);
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

/// Updater represents a struct that can be used to update a record in a Deta Base.
/// ## Fields
/// #### `key` (Required)
/// The key of the record to update.
/// #### `updates` (Required)
/// The updates to apply to the record.
/// ## Methods
/// #### `set(field, value)`
/// Sets the value of a field in the record.
/// #### `delete(fields)`
/// Deletes the specified fields from the record.
/// #### `append(field, value)`
/// Appends the value to the end of the field in the record.
/// #### `prepend(field, value)`
/// Prepends the value to the beginning of the field in the record.
/// #### `increment(field, value)`
/// Increments the value of the field in the record by the specified amount.
/// Example
/// ```rs
/// use deta_rs::base::Updater;
/// let mut updater = Updater::new("key");
/// updater.set("name", serde_json::json!("John"));
/// updater.delete(vec!["age"]);
/// updater.append("hobbies", serde_json::json!("reading"));
/// updater.prepend("hobbies", serde_json::json!("coding"));
/// updater.increment("age", 1);
/// base.update(vec![updater]);
/// ```

#[derive(Default, Clone, Debug)]
pub struct Updater {
    pub key: String,
    updates: Value,
}

impl Updater {

    pub fn new(key: &str) -> Updater {
        return Updater {
            key: key.to_string(),
            updates: json!({
                "set": {},
                "delete": [],
                "append": {},
                "prepend": {},
                "increment": {},
            }),
        }
    }
    
    pub fn set(&mut self, field: &str, value: Value) {
        self.updates["set"][field] = value;
    }

    pub fn delete(&mut self, fields: Vec<&str>) {
        for field in fields {
            self.updates["delete"].as_array_mut().unwrap().push(json!(field));
        }
    }

    pub fn append(&mut self, field: &str, value: Value) {
        self.updates["append"][field] = value;
    }

    pub fn prepend(&mut self, field: &str, value: Value) {
        self.updates["prepend"][field] = value;
    }

    pub fn increment(&mut self, field: &str, value: i64) {
        self.updates["increment"][field] = json!(value);
    }

    fn json(&self) -> Value {
        return self.updates.clone();
    }
    
}

/// Query represents a struct that can be used to query a Deta Base.
/// ## Fields
/// #### `payload` (Optional)
/// The query payload.
/// #### `limit` (Optional)
/// The maximum number of records to return (default: 1000, max: 1000).
/// #### `last` (Optional)
/// The key of the last record returned in the previous query in case of pagination.
/// ## Example
/// ```rs
/// use deta_rs::base::Query;
/// let query = Query {
///     payload: Some(vec![serde_json::json!({"name": "John"})]),
///     limit: Some(1),
///     last: None,
/// };
/// let resp = base.query(query);
/// ```

#[derive(Default, Clone, Debug)]
pub struct Query {
    pub payload: Option<Vec<Value>>,
    pub limit: Option<i64>,
    pub last: Option<String>,
}

impl Query {
    pub fn json(&self) -> Value {
        let mut data = Map::new();
        if let Some(payload) = &self.payload {
            data.insert("query".to_string(), json!(payload));
        }
        if let Some(limit) = &self.limit {
            data.insert("limit".to_string(), json!(limit));
        }
        if let Some(last) = &self.last {
            data.insert("last".to_string(), json!(last));
        }
        return json!(data);
    }
}

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

    pub fn get(&self, key: &str) -> (u16, Value) {
        let url = format!("{}/{}/{}/items/{}", BASE_URL, self.project_id, self.name, key);
        let res = reqwest::blocking::Client::new()
            .get(&url)
            .header("X-API-Key", &self.project_key)
            .send()
            .unwrap();
        return (res.status().as_u16(), res.json::<Value>().unwrap());
    }

    pub fn put(&self, records: Vec<Record>) -> (u16, Value) {
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
        return (res.status().as_u16(), res.json::<Value>().unwrap());
    }

    pub fn insert(&self, record: Record) -> (u16, Value) {
        let url = format!("{}/{}/{}/items", BASE_URL, self.project_id, self.name);
        let mut data = Map::new();
        data.insert("item".to_string(), json!(record.json()));
        let res = reqwest::blocking::Client::new()
            .post(&url)
            .header("X-API-Key", &self.project_key)
            .json(&json!(data))
            .send()
            .unwrap();
        return (res.status().as_u16(), res.json::<Value>().unwrap());
    }

    pub fn delete(&self, key: &str) -> (u16, Value) {
        let url = format!("{}/{}/{}/items/{}", BASE_URL, self.project_id, self.name, key);
        //let res = delete(&url, &self.project_key);
        let res = reqwest::blocking::Client::new()
            .delete(&url)
            .header("X-API-Key", &self.project_key)
            .send()
            .unwrap();
        return (res.status().as_u16(), res.json::<Value>().unwrap());
    }

    pub fn update(&self, updater: Updater) -> (u16, Value) {
        let url = format!("{}/{}/{}/items/{}", BASE_URL, self.project_id, self.name, updater.key);
        let res = reqwest::blocking::Client::new()
            .patch(&url)
            .header("X-API-Key", &self.project_key)
            .json(&updater.json())
            .send()
            .unwrap();
        return (res.status().as_u16(), res.json::<Value>().unwrap());
    }

    pub fn query(&self, query: Query) -> (u16, Value) {
        let url = format!("{}/{}/{}/query", BASE_URL, self.project_id, self.name);
        let res = reqwest::blocking::Client::new()
            .post(&url)
            .header("X-API-Key", &self.project_key)
            .json(&query.json())
            .send()
            .unwrap();
        return (res.status().as_u16(), res.json::<Value>().unwrap());
    }

}