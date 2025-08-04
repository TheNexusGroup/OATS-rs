//! OATS - Objects • Actions • Traits • Systems
//! 
//! Universal architecture pattern for infinite scale across any domain.
//! 
//! This library provides the core abstractions and implementations for the OATS pattern:
//! - **Objects**: Identity containers that compose traits
//! - **Actions**: Stateless logic that reads traits and returns updates
//! - **Traits**: Immutable state containing domain data
//! - **Systems**: Orchestration that coordinates actions and manages resources

pub mod objects;
pub mod actions;
pub mod traits;
pub mod systems;
pub mod error;

// Re-export main types for convenience
pub use objects::Object;
pub use actions::Action;
pub use traits::Trait;
pub use systems::System;
pub use error::OatsError;

/// Result type for OATS operations
pub type Result<T> = std::result::Result<T, OatsError>;

/// Core OATS system that orchestrates all components
pub struct OatsSystem {
    objects: Vec<Object>,
    actions: Vec<Box<dyn Action>>,
    systems: Vec<Box<dyn System>>,
}

impl OatsSystem {
    /// Create a new OATS system
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            actions: Vec::new(),
            systems: Vec::new(),
        }
    }

    /// Add an object to the system
    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    /// Add an action to the system
    pub fn add_action(&mut self, action: Box<dyn Action>) {
        self.actions.push(action);
    }

    /// Add a system to the system
    pub fn add_system(&mut self, system: Box<dyn System>) {
        self.systems.push(system);
    }

    /// Get all objects in the system
    pub fn objects(&self) -> &[Object] {
        &self.objects
    }

    /// Get all actions in the system
    pub fn actions(&self) -> &[Box<dyn Action>] {
        &self.actions
    }

    /// Get all systems in the system
    pub fn systems(&self) -> &[Box<dyn System>] {
        &self.systems
    }
}

impl Default for OatsSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oats_system_creation() {
        let system = OatsSystem::new();
        assert_eq!(system.objects().len(), 0);
        assert_eq!(system.actions().len(), 0);
        assert_eq!(system.systems().len(), 0);
    }

    #[test]
    fn test_oats_system_default() {
        let system = OatsSystem::default();
        assert_eq!(system.objects().len(), 0);
        assert_eq!(system.actions().len(), 0);
        assert_eq!(system.systems().len(), 0);
    }
} 