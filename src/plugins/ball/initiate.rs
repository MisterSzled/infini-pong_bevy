use bevy::{
        prelude::*,
        sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use rand::Rng;

use crate::components::ball::Ball;
use crate::components::collideable::Collideable;
use crate::components::velocity::Velocity;

pub const BALL_DIAMETER: f32 = 20.0;

pub fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
) {
        commands.spawn((
                Name::new("Ball"),
                Ball,
                Velocity::new(),
                Collideable::new(BALL_DIAMETER, BALL_DIAMETER),
                MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Circle::new(BALL_DIAMETER))),
                        material: materials.add(Color::hsl(90., 0.95, 0.7)),
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        ..default()
                },
        ));
}
