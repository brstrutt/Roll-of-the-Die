use bevy::prelude::*;

#[derive(Component)]
pub struct SubSpritesheet {
    pub spritesheet_indices: Vec<usize>,
    pub current_index: usize,
}

impl SubSpritesheet {
    pub fn next_sprite_index(&mut self) -> usize {
        self.current_index = (self.current_index + 1) % self.spritesheet_indices.len();
        return self.spritesheet_indices[self.current_index];
    }
}