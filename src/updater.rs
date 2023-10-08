use serde_json::{ Map, Value };
use serde::{ Serialize, Serializer };

use crate::{ base::Base, errors::DetaError };

/// Represents the operation to be performed on a field.
#[derive(Debug, PartialEq)]
pub (crate) enum Operation {
    /// Set the field to the given value.
    Set,
    /// Delete the field.
    Delete,
    /// Append the given value to the field.
    Append,
    /// Prepend the given value to the field.
    Prepend,
    /// Increment the field by the given numeric value. Use negative values to decrement.
    Increment,
}

impl Operation {
    pub fn as_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

/// Represents an updater to update a field in a record.
/// 
/// For delete operations, the value is ignored so it can be anything.
/// For all other operations, the value is used.
/// 
/// A single updater can contain multiple updates.
/// 
/// An Updater can not contain delete operation along with any other operation for the same field.
pub struct Updater {
    key: String,
    base: Base,
    data: Vec<(String, Value, Operation)>
}

impl Updater {

    pub (crate) fn new(base: Base, key: &str) -> Updater {
        Updater {
            base,
            key: key.to_string(),
            data: Vec::new()
        }
    }

    /// Set a field to the given value.
    /// 
    /// This will overwrite the existing value.
    pub fn set(mut self, field: &str, value: Value) -> Self {
        self.data.push((field.to_string(), value, Operation::Set));
        self
    }

    /// Delete a field.
    /// 
    /// This will delete the field from the record.
    pub fn delete(mut self, field: &str) -> Self {
        self.data.push((field.to_string(), Value::Null, Operation::Delete));
        self
    }

     
    /// Appends the given value to the field if the field is an array.
    pub fn append(mut self, field: &str, value: Value) -> Self {
        self.data.push((field.to_string(), value, Operation::Append));
        self
    }

    /// Prepends the given value to the field if the field is an array.
    pub fn prepend(mut self, field: &str, value: Value) -> Self {
        self.data.push((field.to_string(), value, Operation::Prepend));
        self
    }

    /// Increments the field by the given numeric value. Use negative values to decrement.
    pub fn increment(mut self, field: &str, value: Value) -> Self {
        self.data.push((field.to_string(), value, Operation::Increment));
        self
    }

    /// Commits the updates to the record.
    pub fn commit(&self) -> Result<Value, DetaError> {
        self.base.request(
            "PATCH", &format!("/items/{}", self.key),
            Some(serde_json::to_value(self).unwrap())
        )
    }

}

impl Serialize for Updater {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut map = Map::new();
        for (field, value, operation) in self.data.iter() {
            let op_vec = map.entry(operation.as_string())
                    .or_insert(Value::Array(Vec::new())).as_array_mut().unwrap();
            if operation == &Operation::Delete {
                op_vec.push(Value::String(field.clone()));
            } else {
                let mut inner_map = Map::new();
                inner_map.insert(field.clone(), value.clone());
                op_vec.push(Value::Object(inner_map));
            }
        }
        Value::Object(map).serialize(serializer)
    }
}