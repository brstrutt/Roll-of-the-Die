use bevy::prelude::*;

#[derive(Component)]
pub struct SubSpritesheet {
    pub spritesheet_indices: Vec<usize>
}

impl SubSpritesheet {
    pub fn get_sprite_index(&self, index: usize) -> usize {
        return self.spritesheet_indices[index % self.spritesheet_indices.len()];
    }
}