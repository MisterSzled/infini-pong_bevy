use bevy::{
        prelude::*,
        render::mesh::Mesh2d,
        sprite::Sprite
};

use crate::resources::config::config::Config;
use crate::materials::dungeon_map::DungeonMap;
use crate::components::collideable::Collideable;

const WALL_WIDTH: f32 = 1600.0;
const WALL_HEIGHT: f32 = 10.0;

pub fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        config: Res<Config>,
        dungeon_map: Res<DungeonMap>,
) {
        // Make the walls
        commands.spawn((
                Name::new("bottom_wall"),
                Collideable::new(WALL_WIDTH, WALL_HEIGHT, 0., 16.),
                Mesh2d(meshes.add(Rectangle::new(WALL_WIDTH, WALL_HEIGHT))),
                MeshMaterial2d(materials.add(Color::srgba(180., 0.95, 0.57, 0.))),
                Transform::from_xyz(
                        0.0, 
                        -445. + 64., 
                        0.0),
        ));

        commands.spawn((
                Name::new("top_wall"),
                Collideable::new(WALL_WIDTH, WALL_HEIGHT, 0., 0.),
                Mesh2d(meshes.add(Rectangle::new(WALL_WIDTH, WALL_HEIGHT))),
                MeshMaterial2d(materials.add(Color::srgba(180., 0.95, 0.57, 0.))),
                Transform::from_xyz(
                        0.0,
                        (config.window_height / 2.) - (WALL_HEIGHT / 2.) - config.in_game_ui_height - 32.,
                        0.0,
                )
        ));

        commands.spawn((
                Name::new("paddle_top_wall"),
                Collideable::new(50., WALL_HEIGHT, 0., 0.),
                Sprite::from_atlas_image(
                        dungeon_map.spritesheet.image_handle.clone(), 
                        TextureAtlas {
                                index: 90 as usize,
                                layout: dungeon_map.spritesheet.atlas_handle.clone(),
                        }
                ),

                Transform {
                        scale: dungeon_map.spritesheet.scale,
                        translation: Vec3 {
                                x: 750.,
                                y: (config.window_height / 2.) - (WALL_HEIGHT / 2.) - config.in_game_ui_height - 32. - 100.,
                                z: 1.,
                        },
                        ..default()
                },
        ));
        commands.spawn((
                Name::new("paddle_bottom_wall"),
                Collideable::new(50., WALL_HEIGHT, 0., 32.),

                Sprite::from_atlas_image(
                        dungeon_map.spritesheet.image_handle.clone(), 
                        TextureAtlas {
                                index: 90 as usize,
                                layout: dungeon_map.spritesheet.atlas_handle.clone(),
                        }
                ),
                Transform {
                        scale: dungeon_map.spritesheet.scale,
                        translation: Vec3 {
                                x: 750.,
                                y: -350. + 64.,
                                z: 1.,
                        },
                        ..default()
                }
        ));
}
