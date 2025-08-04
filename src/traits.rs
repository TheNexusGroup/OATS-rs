use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Trait identifier
pub type TraitId = Uuid;

/// A trait represents immutable domain state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trait {
    /// Unique identifier for this trait
    pub id: TraitId,
    /// Name of the trait
    pub name: String,
    /// Version of the trait
    pub version: u32,
    /// The actual trait data
    pub data: TraitData,
    /// Metadata about the trait
    pub metadata: HashMap<String, String>,
}

/// The actual data contained in a trait
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TraitData {
    /// Simple string value
    String(String),
    /// Numeric value
    Number(f64),
    /// Boolean value
    Boolean(bool),
    /// Complex structured data
    Object(HashMap<String, serde_json::Value>),
    /// Array of values
    Array(Vec<serde_json::Value>),
    /// Binary data
    Binary(Vec<u8>),
}

impl Trait {
    /// Create a new trait with the given name and data
    pub fn new(name: impl Into<String>, data: TraitData) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            version: 1,
            data,
            metadata: HashMap::new(),
        }
    }

    /// Create a new trait with metadata
    pub fn with_metadata(
        name: impl Into<String>,
        data: TraitData,
        metadata: HashMap<String, String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            version: 1,
            data,
            metadata,
        }
    }

    /// Get the trait name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the trait data
    pub fn data(&self) -> &TraitData {
        &self.data
    }

    /// Get mutable trait data
    pub fn data_mut(&mut self) -> &mut TraitData {
        &mut self.data
    }

    /// Get a metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Set a metadata value
    pub fn set_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    /// Create a new version of this trait
    pub fn new_version(&self, data: TraitData) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: self.name.clone(),
            version: self.version + 1,
            data,
            metadata: self.metadata.clone(),
        }
    }
}

impl TraitData {
    /// Check if this trait data is a string
    pub fn is_string(&self) -> bool {
        matches!(self, TraitData::String(_))
    }

    /// Check if this trait data is a number
    pub fn is_number(&self) -> bool {
        matches!(self, TraitData::Number(_))
    }

    /// Check if this trait data is a boolean
    pub fn is_boolean(&self) -> bool {
        matches!(self, TraitData::Boolean(_))
    }

    /// Check if this trait data is an object
    pub fn is_object(&self) -> bool {
        matches!(self, TraitData::Object(_))
    }

    /// Check if this trait data is an array
    pub fn is_array(&self) -> bool {
        matches!(self, TraitData::Array(_))
    }

    /// Check if this trait data is binary
    pub fn is_binary(&self) -> bool {
        matches!(self, TraitData::Binary(_))
    }

    /// Try to get the string value
    pub fn as_string(&self) -> Option<&String> {
        match self {
            TraitData::String(s) => Some(s),
            _ => None,
        }
    }

    /// Try to get the number value
    pub fn as_number(&self) -> Option<f64> {
        match self {
            TraitData::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Try to get the boolean value
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            TraitData::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Try to get the object value
    pub fn as_object(&self) -> Option<&HashMap<String, serde_json::Value>> {
        match self {
            TraitData::Object(o) => Some(o),
            _ => None,
        }
    }

    /// Try to get the array value
    pub fn as_array(&self) -> Option<&Vec<serde_json::Value>> {
        match self {
            TraitData::Array(a) => Some(a),
            _ => None,
        }
    }

    /// Try to get the binary value
    pub fn as_binary(&self) -> Option<&Vec<u8>> {
        match self {
            TraitData::Binary(b) => Some(b),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_creation() {
        let trait_data = TraitData::String("test".to_string());
        let trait_obj = Trait::new("test_trait", trait_data);
        
        assert_eq!(trait_obj.name(), "test_trait");
        assert_eq!(trait_obj.version, 1);
        assert!(trait_obj.data.is_string());
    }

    #[test]
    fn test_trait_data_methods() {
        let string_data = TraitData::String("hello".to_string());
        let number_data = TraitData::Number(42.0);
        let bool_data = TraitData::Boolean(true);
        
        assert!(string_data.is_string());
        assert!(number_data.is_number());
        assert!(bool_data.is_boolean());
        
        assert_eq!(string_data.as_string(), Some(&"hello".to_string()));
        assert_eq!(number_data.as_number(), Some(42.0));
        assert_eq!(bool_data.as_boolean(), Some(true));
    }

    #[test]
    fn test_trait_metadata() {
        let mut trait_obj = Trait::new("test", TraitData::String("value".to_string()));
        trait_obj.set_metadata("key", "value");
        
        assert_eq!(trait_obj.get_metadata("key"), Some(&"value".to_string()));
        assert_eq!(trait_obj.get_metadata("nonexistent"), None);
    }
} 