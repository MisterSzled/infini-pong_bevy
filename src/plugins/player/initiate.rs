use bevy::{
        prelude::*,
        sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::components::collideable::Collideable;
use crate::components::player::Player;

pub const PADDLE_WIDTH: f32 = 20.0;
pub const PADDLE_HEIGHT: f32 = 60.0;

pub fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
) {
        // Make the player paddle
        commands.spawn((
                Name::new("Player Paddle"),
                Player,
                Collideable::new(PADDLE_WIDTH, PADDLE_HEIGHT),
                MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT))),
                        material: materials.add(Color::hsl(360., 0.95, 0.7)),
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        ..default()
                },
        ));
}
