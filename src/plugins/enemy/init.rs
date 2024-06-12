use bevy::prelude::*;

use crate::components::collideable::Collideable;
use crate::components::enemy::Enemy;
use crate::materials::dungeon_map::DungeonMap;

pub fn setup(
        mut commands: Commands,
        dungeon_map: Res<DungeonMap>,
) {
        let paddle_height = dungeon_map.spritesheet.scale[1] * dungeon_map.spritesheet.grid_size * 2.;
        let paddle_width = dungeon_map.spritesheet.scale[0] * dungeon_map.spritesheet.grid_size * 0.25;

        // Make the player paddle
        commands.spawn((
                Name::new("Enemy Paddle"),
                Enemy,
                Collideable::new(
                        paddle_width, 
                        paddle_height + 32.,
                        -16.,
                        16. + 32.
                ),
                SpriteSheetBundle {
                        transform: Transform {
                                scale: dungeon_map.spritesheet.scale,
                                translation: Vec3 {
                                        x: 750.,
                                        y: -32.,
                                        z: 1.,
                                },
                                ..default()
                        },
                        texture: dungeon_map.spritesheet.image_handle.clone(),
                        atlas: TextureAtlas {
                                index: 56 as usize,
                                layout: dungeon_map.spritesheet.atlas_handle.clone(),
                        },
                        ..default()
                },
        ))
        .with_children(|parent| {
                parent.spawn((
                        SpriteSheetBundle {
                                transform: Transform {
                                        scale: Vec3::splat(1.),
                                        translation: Vec3 {
                                                x: 0.,
                                                y: 16.,
                                                z: 1.,
                                        },
                                        ..default()
                                },
                                texture: dungeon_map.spritesheet.image_handle.clone(),
                                atlas: TextureAtlas {
                                        index: 46 as usize,
                                        layout: dungeon_map.spritesheet.atlas_handle.clone(),
                                },
                                ..default()
                        },
                        
                )
        );
        });
}
