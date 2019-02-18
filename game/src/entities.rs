use mint::{Point2};
use std::collections::HashMap;

use crate::console::{Color};

pub type EntityID = u64;

pub struct EntityManager {
    next_id: EntityID,
    pub visual: HashMap<EntityID, Visual>,
    pub physics: HashMap<EntityID, Physics>,
}

#[derive(Debug)]
pub struct Physics {
    pub position: Point2<usize>,
    pub durability: u32,
    pub hardness: u32,
}

#[derive(Debug)]
pub struct Visual {
    pub glyph: char,
    pub foreground: Color,
}

impl EntityManager {
    pub fn create_entity(&mut self) -> EntityID {
        let next = self.next_id;
        self.next_id += 1;
        next
    }

    pub fn add_visual(&mut self, entity_id: EntityID, visual: Visual) {
        self.visual.insert(entity_id, visual);
    }

    pub fn get_visual(&self, entity_id: &EntityID) -> Option<&Visual> {
        self.visual.get(entity_id)
    }

    pub fn add_physics(&mut self, entity_id: EntityID, physics: Physics) {
        self.physics.insert(entity_id, physics);
    }

    pub fn get_physics(&self, entity_id: &EntityID) -> Option<&Physics> {
        self.physics.get(entity_id)
    }

    pub fn delete_entity(&mut self, entity_id: &EntityID) {
        self.visual.remove(entity_id);
        self.physics.remove(entity_id);
    }
}

impl Default for EntityManager {
    fn default() -> EntityManager {
        EntityManager {
            next_id: 1,
            visual: HashMap::default(),
            physics: HashMap::default(),
        }
    }
}
