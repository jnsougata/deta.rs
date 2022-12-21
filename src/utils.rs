
use chrono::DateTime;
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
/// ## Exampldetae
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
    pub value: Option<Value>,
    pub expires_in: Option<i64>,  
    pub expires_at: Option<DateTime<chrono::Utc>>,
}

impl Record {

    pub fn json(&self) -> Value {
        let mut data = Map::new();
        if let Some(value) = &self.value {
            data = value.as_object().unwrap().clone();
        }
        if let Some(key) = &self.key {
            data.insert("key".to_string(), json!(key));
        }
        if let Some(expires_in) = &self.expires_in {
            let timestamp = chrono::Utc::now().timestamp() + expires_in;
            data.insert("__expires".to_string(), json!(timestamp));
        } else if let Some(expires_at) = &self.expires_at {
            data.insert("__expires".to_string(), json!(expires_at.timestamp()));
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

/// UpdateBuilder represents a struct that can be used to update a record in a Deta Base.
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
pub struct UpdateBuilder {
    pub key: String,
    updates: Value,
}

impl UpdateBuilder {

    pub fn new(key: &str) -> UpdateBuilder {
        return UpdateBuilder {
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

    pub fn json(&self) -> Value {
        let mut updates = self.updates.as_object().unwrap().clone();
        for (key, value) in updates.clone() {
            if value.as_object().unwrap().is_empty() {
                updates.remove(&key);
            }
        }
        return json!(updates);
    }
    
}

/// QueryBuilder represents a struct that can be used to form a query for Deta Base.
/// ## Fields
/// #### `payload` (Optional)
/// The query payload.
/// #### `limit` (Optional)
/// The maximum number of records to return (default: 1000, max: 1000).
/// #### `last` (Optional)
/// The key of the last record returned in the previous query in case of pagination.
/// ## Methods
/// #### `new()`
/// Creates a new QueryBuilder with default default limit of 1000.
/// #### `equals(field, value)`
/// Adds a condition to the query that the field must be equal to the specified value.
/// #### `not_equals(field, value)`
/// Adds a condition to the query that the field must not be equal to the specified value.
/// #### `contains(field, value)`
/// Adds a condition to the query that the field must contain the specified value.
/// #### `not_contains(field, value)`
/// Adds a condition to the query that the field must not contain the specified value.
/// #### `greater_than(field, value)`
/// Adds a condition to the query that the field must be greater than the specified value.
/// #### `greater_than_or_equal(field, value)`
/// Adds a condition to the query that the field must be greater than or equal to the specified value.
/// #### `less_than(field, value)`
/// Adds a condition to the query that the field must be less than the specified value.
/// #### `less_than_or_equal(field, value)`
/// Adds a condition to the query that the field must be less than or equal to the specified value.
/// #### `prefix(field, value)`
/// Adds a condition to the query that the field must start with the specified value.
/// #### `range(field, start, end)`
/// Adds a condition to the query that the field must be between the specified start and end values.
/// #### `do_or(another)`
/// Do an OR query with the specified query.
/// ## Example
/// ```rs
/// use deta_rs::utils::Query;
/// 
/// let query = Query::new()
/// query.equals("name", serde_json::json!("John"))
/// query.not_equals("age", serde_json::json!(20))
/// query.contains("hobbies", serde_json::json!("reading"))
/// 
/// let resp = base.query(query);
/// ```

#[derive(Default, Clone, Debug)]
pub struct QueryBuilder {
    pub map: Option<Map<String, Value>>,
    pub limit: Option<i64>,
    pub last: Option<String>,
    ors : Option<Vec<Map<String, Value>>>,
}

impl QueryBuilder {

    pub fn new() -> QueryBuilder {
        return QueryBuilder {
            map: Some(Map::new()),
            limit: Some(1000),
            last: None,
            ors: Some(Vec::new()),
        };
    }

    pub fn do_or(&mut self, another: QueryBuilder) {
        self.ors.as_mut().unwrap().push(another.map.unwrap());
    }

    pub fn equals(&mut self, field: &str, value: Value) {
        self.map.as_mut().unwrap().insert(field.to_string(), value);
    }

    pub fn not_equals(&mut self, field: &str, value: Value) {
        self.map.as_mut().unwrap().insert(format!("{}?ne", field), value);
    }

    pub fn greater_than(&mut self, field: &str, value: Value) {
        self.map.as_mut().unwrap().insert(format!("{}?gt", field), value);
    }

    pub fn greater_than_or_equal(&mut self, field: &str, value: Value) {
        self.map.as_mut().unwrap().insert(format!("{}?gte", field), value);
    }

    pub fn less_than(&mut self, field: &str, value: Value) {
        self.map.as_mut().unwrap().insert(format!("{}?lt", field), value);
    }

    pub fn less_than_or_equal(&mut self, field: &str, value: Value) {
        self.map.as_mut().unwrap().insert(format!("{}?lte", field), value);
    }

    pub fn prefix(&mut self, field: &str, value: &str) {
        self.map.as_mut().unwrap().insert(format!("{}?pfx", field), json!(value));
    }

    pub fn contains(&mut self, field: &str, value: &str) {
        self.map.as_mut().unwrap().insert(format!("{}?contains", field), json!(value));
    }

    pub fn range(&mut self, field: &str, start: f64, end: f64) {
        self.map.as_mut().unwrap().insert(format!("{}?range", field), json!([start, end]));
    }

    pub fn not_contains(&mut self, field: &str, value: &str) {
        self.map.as_mut().unwrap().insert(format!("{}?not_contains", field), json!(value));
    }

    pub fn json(&self) -> Value {
        let mut data = Map::new();
        if let Some(limit) = &self.limit {
            data.insert("limit".to_string(), json!(limit));
        }
        if let Some(last) = &self.last {
            data.insert("last".to_string(), json!(last));
        }
        let mut temp = vec![];
        if let Some(ors) = &self.ors {
            for or in ors {
                temp.push(json!(or));
            }
            if let Some(map) = &self.map {
                temp.push(json!(map));
            }
        } else {
            temp.push(json!(self.map.as_ref().unwrap()));
        }
        data.insert("query".to_string(), json!(temp));
        json!(data)
    }
}
