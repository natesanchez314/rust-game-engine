use std::collections::HashMap;

use super::entity::Entity;

struct EntityManager {
    entities: HashMap<u32, Entity>
}