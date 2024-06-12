use bevy::prelude::*;

use crate::resources::textures::sprite_sheet::SpriteSheet;

#[derive(Resource)]
pub struct DungeonMap {
        pub spritesheet: SpriteSheet,
}

impl DungeonMap {
        pub fn new(loaded_sheet_data: SpriteSheet) -> Self {
                DungeonMap {
                        spritesheet: loaded_sheet_data,
                }
        }
}