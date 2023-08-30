use serde_json::{Value, Map};
use serde::Serialize;
use crate::{base::Base, errors::DetaError};


/// Represents a query operator.
#[derive(Debug, PartialEq)]
pub enum Operator {
    /// Equal to
    Eq,
    /// Not equal to
    Ne,
    /// Greater than
    Gt,
    /// Greater than or equal to
    Gte,
    /// Less than
    Lt,
    /// Less than or equal to
    Lte,
    /// In range
    Range,
    /// Contains
    Contains
}

impl Operator {
    fn as_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

/// Represents a query.
pub struct Query {
    base: Base,
    /// The maximum number of items to return. Default maximum is 1000.
    limit: Option<u16>,
    /// The last key returned in the previous query. Used for pagination.
    last: Option<String>,
    /// Whether to sort the results in descending order. Default is false.
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
    pub fn execute(&self) -> Result<Value, DetaError> {
        self.base.request("POST", "/query", Some(serde_json::to_value(self).unwrap()))
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
    pub fn sort(mut self, sort: bool) -> Self {
        self.sort = Some(sort);
        self
    }

    /// Adds a raw query operation to this query.
    pub fn raw(mut self, value: Value) -> Self {
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

    /// Adds a query operation to this query.
    pub fn set(mut self, op: Operator, field: &str, value: Value) -> Self {
        let f = match op {
            Operator::Eq => field.to_string(),
            _ => format!("{}?{}", field, op.as_string())
        };
        self.map.insert(f, value);
        self
    }
}

impl Serialize for Query {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        let mut map = Map::new();
        map.insert("limit".to_string(), Value::from(self.limit.unwrap()));
        if !self.last.is_none() {
            map.insert("last".to_string(), Value::from(self.last.clone()));
        }
        if !self.sort.is_none() && self.sort.unwrap() {
            map.insert("sort".to_string(), serde_json::json!("desc"));
        }
        let mut tmp = self.container.clone();
        tmp.push(Value::Object(self.map.clone()));
        map.insert("query".to_string(), Value::Array(tmp));
        Value::Object(map).serialize(serializer)
    }
}