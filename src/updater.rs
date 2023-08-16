use serde_json::{Map, Value};
use serde::{Serialize, Serializer};

/// Represents the operation to be performed on a field.
#[derive(Debug, PartialEq)]
pub enum Operation {
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


#[derive(Debug)]
pub struct Updater {
    map: Vec<(String, Value, Operation)>
}

impl Updater {
    pub fn new() -> Updater {
        Updater {
            map: Vec::new()
        }
    }

    /// Updates the given field with the given value and operation.
    /// 
    /// For delete operations, the value is ignored so it can be anything.
    /// For all other operations, the value is used.
    /// 
    /// A single updater can contain multiple updates.
    /// 
    /// An Updater can not contain delete operation along with any other operation for the same field.
    /// ```rust
    /// use serde_json::Value;
    /// 
    /// let mut updater = Updater::new();
    /// updater.update("foo".to_string(), Value::String("bar".to_string()), Operation::Set);
    /// updater.update("foos".to_string(), Value::String("baz".to_string()), Operation::Append);
    /// updater.update("foos".to_string(), Value::String("qux".to_string()), Operation::Prepend);
    /// updater.update("foo_count".to_string(), Value::Number(1.into()), Operation::Increment);
    /// // Note: Delete operations can not be combined with other operations on foo.
    /// updater.update("foo".to_string(), Value::Null), Operation::Delete);
    /// ```
    pub fn update(&mut self, field: String, value: Value, operation: Operation) {
        self.map.push((field, value, operation));
    }

}

impl Serialize for Updater {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut map = Map::new();

        for &(ref field, ref value, ref operation) in &self.map {
            let operation_vec = map.entry(operation.as_string())
                    .or_insert(Value::Array(Vec::new())).as_array_mut().unwrap();
            if operation == &Operation::Delete {
                operation_vec.push(Value::String(field.clone()));
            } else {
                let mut inner_map = Map::new();
                inner_map.insert(field.clone(), value.clone());
                operation_vec.push(Value::Object(inner_map));
            }
        }
        Value::Object(map).serialize(serializer)
    }
}