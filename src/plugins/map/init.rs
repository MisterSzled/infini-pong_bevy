use bevy::prelude::*;

use crate::materials::dungeon_map::DungeonMap;
use crate::resources::config::config::Config;
use crate::resources::levels::level::Level;

pub fn setup(
        mut commands: Commands,
        levels: Res<Level>,
        dungeon_map: Res<DungeonMap>,
        config: Res<Config>,
) {
        let mut start_y: f32 = 0.0 + (config.window_height / 2.) - config.in_game_ui_height - ((dungeon_map.spritesheet.grid_size / 2.) * dungeon_map.spritesheet.scale[1]);

        for row in levels.level_data.clone() {
                let mut start_x: f32 = 0.0 - (config.window_width / 2.) + ((dungeon_map.spritesheet.grid_size / 2.) * dungeon_map.spritesheet.scale[0]);
                for material_id in row {
                        commands.spawn(SpriteSheetBundle {
                                transform: Transform {
                                        scale: dungeon_map.spritesheet.scale,
                                        translation: Vec3 {
                                                x: start_x,
                                                y: start_y,
                                                z: 0.,
                                        },
                                        ..default()
                                },
                                texture: dungeon_map.spritesheet.image_handle.clone(),
                                atlas: TextureAtlas {
                                        index: material_id as usize,
                                        layout: dungeon_map.spritesheet.atlas_handle.clone(),
                                },
                                ..default()
                        });
                        start_x += dungeon_map.spritesheet.grid_size * dungeon_map.spritesheet.scale[0];
                }
                start_y -= dungeon_map.spritesheet.grid_size * dungeon_map.spritesheet.scale[1];
        }
}
