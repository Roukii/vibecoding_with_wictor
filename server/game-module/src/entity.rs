use crate::map::{Map, Vec2};
use std::collections::HashMap;

/// Entity types in the game
#[derive(Clone, Debug, PartialEq)]
pub enum EntityType {
    Player,
    Monster,
    NPC,
    Item,
    Projectile,
}

/// Entity state for game logic
#[derive(Clone, Debug, PartialEq)]
pub enum EntityState {
    Idle,
    Moving,
    Attacking,
    Dead,
    Interacting,
}

/// Combat statistics for entities
#[derive(Clone, Debug)]
pub struct CombatStats {
    pub health: u32,
    pub max_health: u32,
    pub attack_damage: u32,
    pub defense: u32,
    pub speed: f64,
    pub attack_range: f64,
    pub attack_cooldown: f64,
    pub last_attack_time: f64,
}

impl Default for CombatStats {
    fn default() -> Self {
        Self {
            health: 100,
            max_health: 100,
            attack_damage: 10,
            defense: 0,
            speed: 1.0,
            attack_range: 1.0,
            attack_cooldown: 1.0,
            last_attack_time: 0.0,
        }
    }
}

/// Entity structure for game logic
#[derive(Clone, Debug)]
pub struct Entity {
    pub id: u64,
    pub entity_type: EntityType,
    pub position: Vec2,
    pub direction: f64,
    pub state: EntityState,
    pub combat_stats: Option<CombatStats>,
    pub inventory: Vec<u64>, // Item entity IDs
    pub target_entity_id: Option<u64>,
    pub move_speed: f64,
    pub created_at: f64,
    pub last_update: f64,
}

impl Entity {
    /// Create a new player entity
    pub fn new_player(id: u64, position: Vec2, created_at: f64) -> Self {
        Self {
            id,
            entity_type: EntityType::Player,
            position,
            direction: 0.0,
            state: EntityState::Idle,
            combat_stats: Some(CombatStats::default()),
            inventory: Vec::new(),
            target_entity_id: None,
            move_speed: 2.0,
            created_at,
            last_update: created_at,
        }
    }

    /// Create a new monster entity
    pub fn new_monster(id: u64, position: Vec2, created_at: f64) -> Self {
        let mut combat_stats = CombatStats::default();
        combat_stats.health = 50;
        combat_stats.max_health = 50;
        combat_stats.attack_damage = 15;
        combat_stats.attack_range = 1.5;
        combat_stats.speed = 1.5;

        Self {
            id,
            entity_type: EntityType::Monster,
            position,
            direction: 0.0,
            state: EntityState::Idle,
            combat_stats: Some(combat_stats),
            inventory: Vec::new(),
            target_entity_id: None,
            move_speed: 1.5,
            created_at,
            last_update: created_at,
        }
    }

    /// Create a new NPC entity
    pub fn new_npc(id: u64, position: Vec2, created_at: f64) -> Self {
        Self {
            id,
            entity_type: EntityType::NPC,
            position,
            direction: 0.0,
            state: EntityState::Idle,
            combat_stats: None,
            inventory: Vec::new(),
            target_entity_id: None,
            move_speed: 1.0,
            created_at,
            last_update: created_at,
        }
    }

    /// Create a new item entity
    pub fn new_item(id: u64, position: Vec2, created_at: f64) -> Self {
        Self {
            id,
            entity_type: EntityType::Item,
            position,
            direction: 0.0,
            state: EntityState::Idle,
            combat_stats: None,
            inventory: Vec::new(),
            target_entity_id: None,
            move_speed: 0.0,
            created_at,
            last_update: created_at,
        }
    }

    /// Check if the entity is alive
    pub fn is_alive(&self) -> bool {
        match &self.combat_stats {
            Some(stats) => stats.health > 0,
            None => true, // Non-combat entities are always "alive"
        }
    }

    /// Check if the entity can attack
    pub fn can_attack(&self, current_time: f64) -> bool {
        if let Some(stats) = &self.combat_stats {
            current_time - stats.last_attack_time >= stats.attack_cooldown
        } else {
            false
        }
    }

    /// Take damage and return true if entity died
    pub fn take_damage(&mut self, damage: u32) -> bool {
        if let Some(ref mut stats) = self.combat_stats {
            let actual_damage = damage.saturating_sub(stats.defense);
            stats.health = stats.health.saturating_sub(actual_damage);

            if stats.health == 0 {
                self.state = EntityState::Dead;
                return true;
            }
        }
        false
    }

    /// Heal the entity
    pub fn heal(&mut self, amount: u32) {
        if let Some(ref mut stats) = self.combat_stats {
            stats.health = (stats.health + amount).min(stats.max_health);
        }
    }

    /// Calculate distance to another entity
    pub fn distance_to(&self, other: &Entity) -> f64 {
        let dx = self.position.x - other.position.x;
        let dy = self.position.y - other.position.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Calculate distance to a position
    pub fn distance_to_position(&self, position: &Vec2) -> f64 {
        let dx = self.position.x - position.x;
        let dy = self.position.y - position.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Check if entity is in range to attack another entity
    pub fn in_attack_range(&self, target: &Entity) -> bool {
        if let Some(stats) = &self.combat_stats {
            self.distance_to(target) <= stats.attack_range
        } else {
            false
        }
    }

    /// Add item to inventory
    pub fn add_to_inventory(&mut self, item_id: u64) -> bool {
        if self.inventory.len() < 20 {
            // Max inventory size
            self.inventory.push(item_id);
            true
        } else {
            false
        }
    }

    /// Remove item from inventory
    pub fn remove_from_inventory(&mut self, item_id: u64) -> bool {
        if let Some(pos) = self.inventory.iter().position(|&id| id == item_id) {
            self.inventory.remove(pos);
            true
        } else {
            false
        }
    }
}

/// Entity manager for handling game logic
pub struct EntityManager {
    entities: HashMap<u64, Entity>,
    next_id: u64,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            next_id: 1,
        }
    }

    /// Add an entity to the manager
    pub fn add_entity(&mut self, mut entity: Entity) -> u64 {
        if entity.id == 0 {
            entity.id = self.next_id;
            self.next_id += 1;
        }
        let id = entity.id;
        self.entities.insert(id, entity);
        id
    }

    /// Remove an entity from the manager
    pub fn remove_entity(&mut self, id: u64) -> Option<Entity> {
        self.entities.remove(&id)
    }

    /// Get an entity by ID
    pub fn get_entity(&self, id: u64) -> Option<&Entity> {
        self.entities.get(&id)
    }

    /// Get a mutable reference to an entity by ID
    pub fn get_entity_mut(&mut self, id: u64) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    /// Get all entities
    pub fn get_all_entities(&self) -> &HashMap<u64, Entity> {
        &self.entities
    }

    /// Get entities by type
    pub fn get_entities_by_type(&self, entity_type: EntityType) -> Vec<&Entity> {
        self.entities
            .values()
            .filter(|entity| entity.entity_type == entity_type)
            .collect()
    }

    /// Get entities within a certain range of a position
    pub fn get_entities_in_range(&self, position: &Vec2, range: f64) -> Vec<&Entity> {
        self.entities
            .values()
            .filter(|entity| entity.distance_to_position(position) <= range)
            .collect()
    }

    /// Move an entity and validate against map boundaries
    pub fn move_entity(
        &mut self,
        id: u64,
        new_position: Vec2,
        map: Option<&Map>,
    ) -> Result<(), String> {
        let entity = self.get_entity_mut(id).ok_or("Entity not found")?;

        if entity.state == EntityState::Dead {
            return Err("Cannot move dead entity".to_string());
        }

        // Validate movement against map boundaries and walkability
        let x = new_position.x as usize;
        let y = new_position.y as usize;

        if let Some(map) = map {
            if !map.is_walkable(x, y) {
                return Err("Position is not walkable".to_string());
            }
        }

        entity.position = new_position;
        entity.state = EntityState::Moving;
        Ok(())
    }

    /// Attack one entity with another
    pub fn attack_entity(
        &mut self,
        attacker_id: u64,
        target_id: u64,
        current_time: f64,
    ) -> Result<AttackResult, String> {
        // Check if attacker exists and can attack
        let attacker_stats = {
            let attacker = self.get_entity(attacker_id).ok_or("Attacker not found")?;
            if !attacker.can_attack(current_time) {
                return Err("Attacker is on cooldown".to_string());
            }
            attacker
                .combat_stats
                .as_ref()
                .ok_or("Attacker has no combat stats")?
                .clone()
        };

        // Check if target exists and is in range
        let target_position = {
            let target = self.get_entity(target_id).ok_or("Target not found")?;
            if !target.is_alive() {
                return Err("Target is already dead".to_string());
            }
            target.position.clone()
        };

        // Verify range
        let attacker = self.get_entity(attacker_id).unwrap();
        if !attacker.in_attack_range(self.get_entity(target_id).unwrap()) {
            return Err("Target is out of range".to_string());
        }

        // Update attacker's last attack time
        if let Some(attacker) = self.get_entity_mut(attacker_id) {
            if let Some(ref mut stats) = attacker.combat_stats {
                stats.last_attack_time = current_time;
            }
            attacker.state = EntityState::Attacking;
        }

        // Apply damage to target
        let target_died = if let Some(target) = self.get_entity_mut(target_id) {
            target.take_damage(attacker_stats.attack_damage)
        } else {
            false
        };

        Ok(AttackResult {
            damage_dealt: attacker_stats.attack_damage,
            target_died,
            target_position,
        })
    }

    /// Handle entity interactions (e.g., picking up items)
    pub fn interact_entities(
        &mut self,
        entity_id: u64,
        target_id: u64,
    ) -> Result<InteractionResult, String> {
        let entity = self.get_entity(entity_id).ok_or("Entity not found")?;
        let target = self.get_entity(target_id).ok_or("Target not found")?;

        match (&entity.entity_type, &target.entity_type) {
            (EntityType::Player, EntityType::Item) => {
                // Player picking up item
                if entity.distance_to(target) <= 1.5 {
                    if let Some(player) = self.get_entity_mut(entity_id) {
                        if player.add_to_inventory(target_id) {
                            self.remove_entity(target_id);
                            return Ok(InteractionResult::ItemPickedUp);
                        } else {
                            return Err("Inventory is full".to_string());
                        }
                    }
                } else {
                    return Err("Too far from item".to_string());
                }
            }
            (EntityType::Player, EntityType::NPC) => {
                // Player talking to NPC
                if entity.distance_to(target) <= 2.0 {
                    return Ok(InteractionResult::NPCInteraction);
                } else {
                    return Err("Too far from NPC".to_string());
                }
            }
            _ => return Err("Invalid interaction".to_string()),
        }

        Err("Interaction failed".to_string())
    }

    /// Update entity AI and behaviors
    pub fn update_entities(&mut self, delta_time: f64, current_time: f64) {
        let entity_ids: Vec<u64> = self.entities.keys().cloned().collect();

        for entity_id in entity_ids {
            if let Some(entity) = self.entities.get_mut(&entity_id) {
                entity.last_update = current_time;

                match entity.entity_type {
                    EntityType::Monster => {
                        self.update_monster_ai(entity_id, delta_time, current_time);
                    }
                    EntityType::Player => {
                        // Reset state to idle if not actively doing something
                        if entity.state == EntityState::Moving
                            || entity.state == EntityState::Attacking
                        {
                            entity.state = EntityState::Idle;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    /// Update monster AI behavior
    fn update_monster_ai(&mut self, monster_id: u64, _delta_time: f64, current_time: f64) {
        // Find the nearest player
        let monster_position = if let Some(monster) = self.get_entity(monster_id) {
            if monster.state == EntityState::Dead {
                return;
            }
            monster.position.clone()
        } else {
            return;
        };

        // Get all players and find the nearest one
        let players = self.get_entities_by_type(EntityType::Player);
        let nearest_player_info = players
            .iter()
            .filter(|player| player.is_alive())
            .map(|player| {
                (
                    player.id,
                    player.position.clone(),
                    player.distance_to_position(&monster_position),
                )
            })
            .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));

        if let Some((player_id, player_position, distance)) = nearest_player_info {
            // If player is close enough, attack
            if distance <= 1.5
                && self
                    .get_entity(monster_id)
                    .unwrap()
                    .can_attack(current_time)
            {
                let _ = self.attack_entity(monster_id, player_id, current_time);
            }
            // If player is within detection range but not attack range, move towards player
            else if distance <= 8.0 && distance > 1.5 {
                if let Some(monster) = self.get_entity_mut(monster_id) {
                    let dx = player_position.x - monster.position.x;
                    let dy = player_position.y - monster.position.y;
                    let length = (dx * dx + dy * dy).sqrt();

                    if length > 0.0 {
                        let move_distance = monster.move_speed * 0.1; // Adjust for frame rate
                        let new_x = monster.position.x + (dx / length) * move_distance;
                        let new_y = monster.position.y + (dy / length) * move_distance;

                        monster.position.x = new_x;
                        monster.position.y = new_y;
                        monster.state = EntityState::Moving;
                        monster.target_entity_id = Some(player_id);
                    }
                }
            } else {
                // Reset to idle if no target
                if let Some(monster) = self.get_entity_mut(monster_id) {
                    monster.state = EntityState::Idle;
                    monster.target_entity_id = None;
                }
            }
        }
    }

    /// Remove all dead entities
    pub fn cleanup_dead_entities(&mut self) -> Vec<u64> {
        let dead_entity_ids: Vec<u64> = self
            .entities
            .iter()
            .filter(|(_, entity)| !entity.is_alive())
            .map(|(id, _)| *id)
            .collect();

        for id in &dead_entity_ids {
            self.entities.remove(id);
        }

        dead_entity_ids
    }

    /// Get entity count by type
    pub fn count_entities_by_type(&self, entity_type: EntityType) -> usize {
        self.entities
            .values()
            .filter(|entity| entity.entity_type == entity_type)
            .count()
    }
}

/// Result of an attack action
#[derive(Clone, Debug)]
pub struct AttackResult {
    pub damage_dealt: u32,
    pub target_died: bool,
    pub target_position: Vec2,
}

/// Result of an interaction action
#[derive(Clone, Debug)]
pub enum InteractionResult {
    ItemPickedUp,
    NPCInteraction,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_creation() {
        let entity = Entity::new_player(1, Vec2 { x: 0.0, y: 0.0 }, 0.0);
        assert_eq!(entity.id, 1);
        assert_eq!(entity.entity_type, EntityType::Player);
        assert!(entity.is_alive());
    }

    #[test]
    fn test_entity_damage() {
        let mut entity = Entity::new_player(1, Vec2 { x: 0.0, y: 0.0 }, 0.0);
        let died = entity.take_damage(50);
        assert!(!died);
        assert_eq!(entity.combat_stats.as_ref().unwrap().health, 50);

        let died = entity.take_damage(50);
        assert!(died);
        assert_eq!(entity.state, EntityState::Dead);
    }

    #[test]
    fn test_entity_manager() {
        let mut manager = EntityManager::new();
        let entity = Entity::new_player(0, Vec2 { x: 0.0, y: 0.0 }, 0.0);
        let id = manager.add_entity(entity);

        assert!(manager.get_entity(id).is_some());
        assert_eq!(manager.count_entities_by_type(EntityType::Player), 1);
    }
}
