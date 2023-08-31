use serde_json::{ Value, Map };
use serde::{ Deserialize, Serialize };
use crate::{ base::Base, errors::DetaError };


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
    pub fn run(&self) -> Result<Value, DetaError> {
        self.base.request("POST", "/query", Some(serde_json::to_value(self).unwrap()))
    }

    /// Executes the query until there are no more results.
    pub fn run_until_end(&self) -> Result<Value, DetaError> {
        if self.limit.is_some() {
            return Err(DetaError::PayloadError { msg: "limit must be None for run_until_end".to_string() });
        }
        let mut resp = self.run()?;
        let mut result = serde_json::from_value::<QueryResult>(resp.clone()).unwrap();
        loop {
            let mut tmp = resp["items"].as_array().unwrap().clone();
            result.items.append(&mut tmp);
            if result.paging.last.is_empty() {
                break;
            }
            resp = self.clone().last(&result.paging.last).run()?;
        }
        result.paging.size = result.items.len() as u16;
        Ok(serde_json::to_value(result).unwrap())
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
        if self.last.is_some() {
            map.insert("last".to_string(), Value::from(self.last.clone()));
        }
        if self.sort.is_some() && self.sort.unwrap() {
            map.insert("sort".to_string(), serde_json::json!("desc"));
        }
        let mut tmp = self.container.clone();
        tmp.push(Value::Object(self.map.clone()));
        map.insert("query".to_string(), Value::Array(tmp));
        Value::Object(map).serialize(serializer)
    }
}