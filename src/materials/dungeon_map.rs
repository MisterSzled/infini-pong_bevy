use bevy::prelude::*;

use crate::resources::textures::sprite_sheet::SpriteSheet;

#[derive(Resource)]
pub struct DungeonMap {
        pub spritesheet: SpriteSheet,

        pub top_right: usize,
        pub top_a: usize,
        pub top_b: usize,
        pub top_c: usize,
        pub top_d: usize,
        pub top_left: usize,
}

impl DungeonMap {
        pub fn new(loaded_sheet_data: SpriteSheet) -> Self {
                DungeonMap {
                        spritesheet: loaded_sheet_data,

                        top_left: 0,
                        top_a: 1,
                        top_b: 2,
                        top_c: 3,
                        top_d: 4,
                        top_right: 5,
                }
        }

        pub fn get_tile_by_id(id: f32) -> usize {
                0
        }
}