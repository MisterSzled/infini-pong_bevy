use bevy::{
        prelude::*,
        sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

const WALL_WIDTH: f32 = 1600.0;
const WALL_HEIGHT: f32 = 10.0;

use crate::components::collideable::Collideable;

pub fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
) {
        // Make the walls
        commands.spawn((
                Name::new("bottom_wall"),
                Collideable::new(WALL_WIDTH, WALL_HEIGHT),
                MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Rectangle::new(WALL_WIDTH, WALL_HEIGHT))),
                        material: materials.add(Color::hsl(180., 0.95, 0.57)),
                        transform: Transform::from_xyz(0.0, -445., 0.0),
                        ..default()
                },
        ));
        commands.spawn((
                Name::new("top_wall"),
                Collideable::new(WALL_WIDTH, WALL_HEIGHT),
                MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Rectangle::new(WALL_WIDTH, WALL_HEIGHT))),
                        material: materials.add(Color::hsl(180., 0.95, 0.57)),
                        transform: Transform::from_xyz(0.0, 395., 0.0),
                        ..default()
                },
        ));

        commands.spawn((
                Name::new("paddle_top_wall"),
                Collideable::new(20., WALL_HEIGHT),
                MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Rectangle::new(20., WALL_HEIGHT))),
                        material: materials.add(Color::hsl(180., 0.95, 0.57)),
                        transform: Transform::from_xyz(750.0, 300., 0.0),
                        ..default()
                },
        ));
        commands.spawn((
                Name::new("paddle_top_wall"),
                Collideable::new(20., WALL_HEIGHT),
                MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Rectangle::new(20., WALL_HEIGHT))),
                        material: materials.add(Color::hsl(180., 0.95, 0.57)),
                        transform: Transform::from_xyz(750.0, -350., 0.0),
                        ..default()
                },
        ));
}
