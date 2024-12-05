use bevy::prelude::*;

use crate::components::ball::Ball;
use crate::components::collideable::Collideable;
use crate::components::velocity::Velocity;
use crate::materials::dungeon_map::DungeonMap;

pub const BALL_DIAMETER: f32 = 15.0;

pub fn setup(
        mut commands: Commands,

        dungeon_map: Res<DungeonMap>,
) {
        commands.spawn((
                Name::new("Ball"),
                Ball,
                Velocity::new(),
                Collideable::new(BALL_DIAMETER, BALL_DIAMETER, 0., 0.),

                Transform {
                        scale: dungeon_map.spritesheet.scale,
                        translation: Vec3 {
                                x: 0.,
                                y: 0.,
                                z: 2.,
                        },
                        ..default()
                },
                Sprite::from_atlas_image(
                        dungeon_map.spritesheet.image_handle.clone(), 
                        TextureAtlas {
                                index: 86 as usize,
                                layout: dungeon_map.spritesheet.atlas_handle.clone(),
                        }
                )
        ));
}
