pub struct Entity {
    pub(crate) id: u32,
}

impl Entity {
    pub fn entity(id: u32) -> Self {
        Self {
            id
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}