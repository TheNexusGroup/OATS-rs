use oats::{
    Object, Trait, Action, System,
    traits::TraitData,
    actions::{ActionContext, ActionResult},
    systems::{SystemManager, Priority},
};
use std::collections::HashMap;

// Custom actions for the basic example
struct IncrementTraitAction {
    trait_name: String,
    increment: f64,
}

impl IncrementTraitAction {
    fn new(trait_name: impl Into<String>, increment: f64) -> Self {
        Self {
            trait_name: trait_name.into(),
            increment,
        }
    }
}

#[async_trait::async_trait]
impl Action for IncrementTraitAction {
    fn name(&self) -> &str {
        "increment_trait"
    }

    fn description(&self) -> &str {
        "Increments a numeric trait by a specified amount"
    }

    async fn execute(&self, context: ActionContext) -> Result<ActionResult, oats::OatsError> {
        let target_object = context
            .get_object("target")
            .ok_or_else(|| oats::OatsError::action_failed("Target object not found"))?;

        let current_trait = target_object
            .get_trait(&self.trait_name)
            .ok_or_else(|| oats::OatsError::trait_not_found(&self.trait_name))?;

        let current_value = current_trait
            .data()
            .as_number()
            .ok_or_else(|| oats::OatsError::action_failed("Trait is not numeric"))?;

        let new_value = current_value + self.increment;
        let new_trait = Trait::new(&self.trait_name, TraitData::Number(new_value));

        let mut result = ActionResult::success();
        result.add_trait_update(new_trait);
        result.add_message(format!(
            "Incremented {} from {} to {}",
            self.trait_name, current_value, new_value
        ));

        Ok(result)
    }

    fn required_traits(&self) -> Vec<String> {
        vec![self.trait_name.clone()]
    }
}

struct SetTraitAction {
    trait_name: String,
    value: TraitData,
}

impl SetTraitAction {
    fn new(trait_name: impl Into<String>, value: TraitData) -> Self {
        Self {
            trait_name: trait_name.into(),
            value,
        }
    }
}

#[async_trait::async_trait]
impl Action for SetTraitAction {
    fn name(&self) -> &str {
        "set_trait"
    }

    fn description(&self) -> &str {
        "Sets a trait to a specific value"
    }

    async fn execute(&self, _context: ActionContext) -> Result<ActionResult, oats::OatsError> {
        let new_trait = Trait::new(&self.trait_name, self.value.clone());

        let mut result = ActionResult::success();
        result.add_trait_update(new_trait);
        result.add_message(format!("Set {} to {:?}", self.trait_name, self.value));

        Ok(result)
    }
}

// Custom systems for the basic example
struct HealthSystem {
    name: String,
    description: String,
    stats: oats::systems::SystemStats,
}

impl HealthSystem {
    fn new() -> Self {
        Self {
            name: "health_system".to_string(),
            description: "Manages health-related traits for objects".to_string(),
            stats: oats::systems::SystemStats::default(),
        }
    }
}

#[async_trait::async_trait]
impl System for HealthSystem {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    async fn process(&mut self, objects: Vec<Object>, _priority: Priority) -> Result<Vec<ActionResult>, oats::OatsError> {
        let mut results = Vec::new();
        let start_time = std::time::Instant::now();

        for object in objects {
            // Only process objects with health traits
            if object.has_trait("health") {
                let health_action = IncrementTraitAction::new("health", -1.0); // Natural health decay
                let mut context = ActionContext::new();
                context.add_object("target", object);

                match health_action.execute(context).await {
                    Ok(result) => {
                        results.push(result);
                        self.stats.actions_executed += 1;
                    }
                    Err(e) => {
                        self.stats.errors += 1;
                        let error_result = ActionResult::failure(format!("Health action failed: {}", e));
                        results.push(error_result);
                    }
                }
            }
            self.stats.objects_processed += 1;
        }

        self.stats.total_processing_time_ms += start_time.elapsed().as_millis() as u64;
        self.stats.last_processed = Some(chrono::Utc::now());

        Ok(results)
    }

    fn get_stats(&self) -> oats::systems::SystemStats {
        self.stats.clone()
    }
}

struct PositionSystem {
    name: String,
    description: String,
    stats: oats::systems::SystemStats,
}

impl PositionSystem {
    fn new() -> Self {
        Self {
            name: "position_system".to_string(),
            description: "Manages position-related traits for objects".to_string(),
            stats: oats::systems::SystemStats::default(),
        }
    }
}

#[async_trait::async_trait]
impl System for PositionSystem {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    async fn process(&mut self, objects: Vec<Object>, _priority: Priority) -> Result<Vec<ActionResult>, oats::OatsError> {
        let mut results = Vec::new();
        let start_time = std::time::Instant::now();

        for object in objects {
            // Only process objects with position traits
            if object.has_trait("position") {
                // Simple position update logic
                let position_data = TraitData::Object(HashMap::new()); // Simplified for example
                let position_action = SetTraitAction::new("position", position_data);
                let mut context = ActionContext::new();
                context.add_object("target", object);

                match position_action.execute(context).await {
                    Ok(result) => {
                        results.push(result);
                        self.stats.actions_executed += 1;
                    }
                    Err(e) => {
                        self.stats.errors += 1;
                        let error_result = ActionResult::failure(format!("Position action failed: {}", e));
                        results.push(error_result);
                    }
                }
            }
            self.stats.objects_processed += 1;
        }

        self.stats.total_processing_time_ms += start_time.elapsed().as_millis() as u64;
        self.stats.last_processed = Some(chrono::Utc::now());

        Ok(results)
    }

    fn get_stats(&self) -> oats::systems::SystemStats {
        self.stats.clone()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 OATS Basic Example");
    println!("=====================\n");

    // Create some objects with traits
    println!("1. Creating objects with traits...");
    
    let mut player = Object::new("player_1", "character");
    let health_trait = Trait::new("health", TraitData::Number(100.0));
    let position_trait = Trait::new("position", TraitData::Object(HashMap::new()));
    player.add_trait(health_trait);
    player.add_trait(position_trait);

    let mut enemy = Object::new("enemy_1", "character");
    let enemy_health = Trait::new("health", TraitData::Number(50.0));
    enemy.add_trait(enemy_health);

    println!("   Created player: {} (type: {})", player.name(), player.object_type());
    println!("   Created enemy: {} (type: {})", enemy.name(), enemy.object_type());
    println!("   Player traits: {:?}", player.trait_names());
    println!("   Enemy traits: {:?}", enemy.trait_names());

    // Create actions
    println!("\n2. Creating actions...");
    
    let heal_action = IncrementTraitAction::new("health", 25.0);
    let damage_action = IncrementTraitAction::new("health", -10.0);
    let set_position_action = SetTraitAction::new("position", TraitData::String("(10, 20)".to_string()));

    println!("   Created heal action: {}", heal_action.name());
    println!("   Created damage action: {}", damage_action.name());
    println!("   Created set position action: {}", set_position_action.name());

    // Test actions directly
    println!("\n3. Testing actions directly...");
    
    let mut context = ActionContext::new();
    context.add_object("target", player.clone());
    
    let heal_result = heal_action.execute(context).await?;
    println!("   Heal action result: {}", heal_result.is_success());
    if let Some(message) = heal_result.messages.first() {
        println!("   Message: {}", message);
    }

    // Create systems
    println!("\n4. Creating systems...");
    
    let health_system = HealthSystem::new();
    let position_system = PositionSystem::new();

    println!("   Created health system: {}", health_system.name());
    println!("   Created position system: {}", position_system.name());

    // Create system manager and register everything
    println!("\n5. Setting up system manager...");
    
    let mut manager = SystemManager::new();
    manager.add_system(Box::new(health_system));
    manager.add_system(Box::new(position_system));

    // Register objects
    manager.register_object(player).await;
    manager.register_object(enemy).await;

    println!("   Registered {} systems", manager.systems().len());
    println!("   Registered objects: player_1, enemy_1");

    // Process objects through systems
    println!("\n6. Processing objects through systems...");
    
    let results = manager.process_all(Priority::Normal).await?;
    println!("   Processed {} action results", results.len());

    for (i, result) in results.iter().enumerate() {
        println!("   Result {}: {}", i + 1, if result.is_success() { "SUCCESS" } else { "FAILURE" });
        for message in &result.messages {
            println!("     Message: {}", message);
        }
    }

    // Get system statistics
    println!("\n7. System statistics:");
    
    let stats = manager.get_all_stats();
    for (system_name, stat) in stats {
        println!("   {}:", system_name);
        println!("     Objects processed: {}", stat.objects_processed);
        println!("     Actions executed: {}", stat.actions_executed);
        println!("     Errors: {}", stat.errors);
        println!("     Total processing time: {}ms", stat.total_processing_time_ms);
    }

    println!("\n✅ Basic OATS example completed successfully!");
    Ok(())
} 