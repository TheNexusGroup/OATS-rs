use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{Result, Object, Trait};

/// Action identifier
pub type ActionId = uuid::Uuid;

/// Context passed to actions containing relevant objects and traits
#[derive(Debug, Clone)]
pub struct ActionContext {
    /// Objects relevant to this action
    pub objects: HashMap<String, Object>,
    /// Additional parameters for the action
    pub parameters: HashMap<String, serde_json::Value>,
    /// Metadata about the action execution
    pub metadata: HashMap<String, String>,
}

impl ActionContext {
    /// Create a new action context
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            parameters: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    /// Create a new action context with expected capacity
    pub fn with_capacity(expected_objects: usize, expected_parameters: usize) -> Self {
        Self {
            objects: HashMap::with_capacity(expected_objects),
            parameters: HashMap::with_capacity(expected_parameters),
            metadata: HashMap::new(),
        }
    }

    /// Add an object to the context
    pub fn add_object(&mut self, name: impl Into<String>, object: Object) {
        self.objects.insert(name.into(), object);
    }

    /// Get an object from the context
    pub fn get_object(&self, name: &str) -> Option<&Object> {
        self.objects.get(name)
    }

    /// Get multiple objects efficiently
    pub fn get_objects(&self, names: &[&str]) -> HashMap<String, &Object> {
        names.iter()
            .filter_map(|name| self.objects.get(*name).map(|obj| (name.to_string(), obj)))
            .collect()
    }

    /// Add a parameter to the context
    pub fn add_parameter(&mut self, name: impl Into<String>, value: serde_json::Value) {
        self.parameters.insert(name.into(), value);
    }

    /// Get a parameter from the context
    pub fn get_parameter(&self, name: &str) -> Option<&serde_json::Value> {
        self.parameters.get(name)
    }

    /// Add metadata to the context
    pub fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    /// Get metadata from the context
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

impl Default for ActionContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of an action execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    /// Whether the action was successful
    pub success: bool,
    /// New traits to be applied
    pub trait_updates: Vec<Trait>,
    /// Messages or logs from the action
    pub messages: Vec<String>,
    /// Additional data returned by the action
    pub data: HashMap<String, serde_json::Value>,
}

impl ActionResult {
    /// Create a successful action result
    pub fn success() -> Self {
        Self {
            success: true,
            trait_updates: Vec::new(),
            messages: Vec::new(),
            data: HashMap::new(),
        }
    }

    /// Create a failed action result
    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            success: false,
            trait_updates: Vec::new(),
            messages: vec![message.into()],
            data: HashMap::new(),
        }
    }

    /// Add a trait update to the result
    pub fn add_trait_update(&mut self, trait_obj: Trait) {
        self.trait_updates.push(trait_obj);
    }

    /// Add multiple trait updates efficiently
    pub fn add_trait_updates(&mut self, trait_updates: impl IntoIterator<Item = Trait>) {
        self.trait_updates.extend(trait_updates);
    }

    /// Add a message to the result
    pub fn add_message(&mut self, message: impl Into<String>) {
        self.messages.push(message.into());
    }

    /// Add multiple messages efficiently
    pub fn add_messages(&mut self, messages: impl IntoIterator<Item = String>) {
        self.messages.extend(messages);
    }

    /// Add data to the result
    pub fn add_data(&mut self, key: impl Into<String>, value: serde_json::Value) {
        self.data.insert(key.into(), value);
    }

    /// Reserve capacity for expected updates
    pub fn reserve_capacity(&mut self, trait_updates: usize, messages: usize) {
        self.trait_updates.reserve(trait_updates);
        self.messages.reserve(messages);
    }

    /// Check if the action was successful
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// Check if the action failed
    pub fn is_failure(&self) -> bool {
        !self.success
    }
}

/// An action represents stateless logic that reads traits and returns updates
#[async_trait]
pub trait Action: Send + Sync {
    /// Get the name of this action
    fn name(&self) -> &str;

    /// Get the description of this action
    fn description(&self) -> &str;

    /// Execute the action with the given context
    async fn execute(&self, context: ActionContext) -> Result<ActionResult>;

    /// Get the required trait names for this action
    fn required_traits(&self) -> Vec<String> {
        Vec::new()
    }

    /// Get the optional trait names for this action
    fn optional_traits(&self) -> Vec<String> {
        Vec::new()
    }
}

/// A simple action that can be created from a closure
pub struct SimpleAction<F> {
    name: String,
    description: String,
    required_traits: Vec<String>,
    optional_traits: Vec<String>,
    executor: F,
}

impl<F> SimpleAction<F>
where
    F: Fn(ActionContext) -> Result<ActionResult> + Send + Sync,
{
    /// Create a new simple action
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        executor: F,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            required_traits: Vec::new(),
            optional_traits: Vec::new(),
            executor,
        }
    }

    /// Set required traits for this action
    pub fn with_required_traits(mut self, traits: Vec<String>) -> Self {
        self.required_traits = traits;
        self
    }

    /// Set optional traits for this action
    pub fn with_optional_traits(mut self, traits: Vec<String>) -> Self {
        self.optional_traits = traits;
        self
    }
}

#[async_trait]
impl<F> Action for SimpleAction<F>
where
    F: Fn(ActionContext) -> Result<ActionResult> + Send + Sync,
{
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    async fn execute(&self, context: ActionContext) -> Result<ActionResult> {
        (self.executor)(context)
    }

    fn required_traits(&self) -> Vec<String> {
        self.required_traits.clone()
    }

    fn optional_traits(&self) -> Vec<String> {
        self.optional_traits.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simple_action() {
        let action = SimpleAction::new(
            "test_action",
            "A test action",
            |_context| {
                let mut result = ActionResult::success();
                result.add_message("Test action executed");
                Ok(result)
            },
        );

        let context = ActionContext::new();
        let result = action.execute(context).await.unwrap();

        assert!(result.is_success());
        assert_eq!(result.messages.len(), 1);
        assert_eq!(result.messages[0], "Test action executed");
    }

    #[test]
    fn test_action_context() {
        let mut context = ActionContext::new();
        let test_object = Object::new("test", "type");

        context.add_object("test_obj", test_object);
        context.add_parameter("param", serde_json::json!("value"));
        context.add_metadata("key", "value");

        assert!(context.get_object("test_obj").is_some());
        assert!(context.get_parameter("param").is_some());
        assert_eq!(context.get_metadata("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_action_result() {
        let mut result = ActionResult::success();
        result.add_message("Test message");
        result.add_data("key", serde_json::json!("value"));

        assert!(result.is_success());
        assert_eq!(result.messages.len(), 1);
        assert_eq!(result.data.len(), 1);
    }
} 