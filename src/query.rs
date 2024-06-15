use serde_json::{ Value, Map };
use serde::{ Deserialize, Serialize };
use crate::{ base::Base, errors::DetaError };


#[derive(Deserialize, Serialize)]
pub (crate) struct Paging {
    pub(crate) size: u16,
    #[serde(default)]
    pub(crate) last: String
}

#[derive(Deserialize, Serialize)]
struct QueryResult {
    paging: Paging,
    items: Vec<Value>
}

/// Represents a query.
#[derive(Clone)]
pub struct Query {
    base: Base,
    limit: Option<u16>,
    last: Option<String>,
    sort: Option<bool>,
    container: Vec<Value>,
    map: Map<String, Value>
}

impl Query {
    
    pub (crate) fn new(base: Base) -> Query {
        Query {
            base,
            limit: Some(1000),
            last: None,
            sort: Some(false),
            container: Vec::new(),
            map: Map::new()
        }
    }

    /// Executes the query on the base.
    pub fn run(&self) -> Result<Value, DetaError> {
        self.base.request("POST", "/query", Some(serde_json::to_value(self).unwrap()))
    }

    /// Executes the query until there are no more results.
    pub fn walk(&self) -> Result<Vec<Value>, DetaError> {
        let mut items: Vec<Value> = Vec::new();
        let mut resp = self.run();
        if resp.is_err() {
            return Err(resp.err().unwrap());
        }
        let result = serde_json::from_value::<QueryResult>
            (resp.unwrap()).map_err(DetaError::from).unwrap();
        items.extend(result.items);
        let mut last = result.paging.last;
        while !last.is_empty() {
            let mut query = self.clone();
            query = query.last(&last);
            resp = query.run();
            if resp.is_err() {
                break;
            }
            let result = serde_json::from_value::<QueryResult>
                (resp.unwrap()).map_err(DetaError::from).unwrap();
            last = result.paging.last;
            items.extend(result.items);
        }
        Ok(items)
    }

    /// Sets the limit of the query.
    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the last key of the query if the query is paginated.
    pub fn last(mut self, last: &str) -> Self {
        self.last = Some(last.to_string());
        self
    }

    /// Sets whether to sort the results in descending order.
    pub fn sort(mut self, desc: bool) -> Self {
        self.sort = Some(desc);
        self
    }

    /// Adds a manually constructed query to the query.
    pub fn append(mut self, value: Value) -> Self {
        self.container.push(value);
        self
    }

    /// Merges the given query into this query.
    pub fn union(mut self, other: Query) -> Self {
        for item in other.container {
            self.container.push(item);
        }
        self.container.push(Value::Object(other.map));
        self
    }

    /// Checks equality of the given field with the given value.
    pub fn equals(mut self, field: &str, value: Value) -> Self {
        self.map.insert(field.to_string(), value);
        self
    }

    /// Checks inequality of the given field with the given value.
    pub fn not_equals(mut self, field: &str, value: Value) -> Self {
        self.map.insert(format!("{}?ne", field), value);
        self
    }

    /// Checks if the given field is greater than the given value.
    pub fn greater_than(mut self, field: &str, value: Value) -> Self {
        self.map.insert(format!("{}?gt", field), value);
        self
    }

    /// Checks if the given field is greater than or equal to the given value.
    pub fn greater_than_or_equals(mut self, field: &str, value: Value) -> Self {
        self.map.insert(format!("{}?gte", field), value);
        self
    }

    /// Checks if the given field is less than the given value.
    pub fn less_than(mut self, field: &str, value: Value) -> Self {
        self.map.insert(format!("{}?lt", field), value);
        self
    }

    /// Checks if the given field is less than or equal to the given value.
    pub fn less_than_or_equals(mut self, field: &str, value: Value) -> Self {
        self.map.insert(format!("{}?lte", field), value);
        self
    }

    /// Checks if the given field is in the given range.
    pub fn in_range(mut self, field: &str, value: Value) -> Self {
        self.map.insert(format!("{}?range", field), value);
        self
    }

    /// Checks if the given field contains the given value.
    pub fn contains(mut self, field: &str, value: Value) -> Self {
        self.map.insert(format!("{}?contains", field), value);
        self
    }

}

impl Serialize for Query {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut map = Map::new();
        map.insert("limit".to_string(), Value::from(self.limit.unwrap()));
        if self.last.is_some() {
            map.insert("last".to_string(), Value::from(self.last.clone()));
        }
        if self.sort.is_some() && self.sort.unwrap() {
            map.insert("sort".to_string(), serde_json::json!("desc"));
        }
        let mut outer = self.container.clone();
        outer.push(Value::Object(self.map.clone()));
        map.insert(String::from("query"), Value::Array(outer));
        Value::Object(map).serialize(serializer)
    }
}