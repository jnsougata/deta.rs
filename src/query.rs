use serde_json::{Value, Map};
use serde::Serialize;


#[derive(Debug, PartialEq)]
pub enum Operator {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
    Range,
    Contains
}

impl Operator {
    pub fn as_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}


#[derive(Debug, Clone)]
pub struct Query {
    pub limit: Option<u32>,
    pub last: Option<String>,
    pub sort: Option<bool>,
    container: Vec<Value>,
    map: Map<String, Value>
}

impl Query {
    
    pub fn new() -> Query {
        Query {
            limit: Some(1000),
            last: None,
            sort: Some(false),
            container: Vec::new(),
            map: Map::new()
        }
    }

    pub fn union(&mut self, other: Query) {
        for item in other.container {
            self.container.push(item);
        }
        self.container.push(Value::Object(other.map));
    }

    pub fn set(&mut self, op: Operator, field: &str, value: Value) {
        let f = match op {
            Operator::Eq => field.to_string(),
            _ => format!("{}?{}", field, op.as_string())
        };
        self.map.insert(f, value);
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