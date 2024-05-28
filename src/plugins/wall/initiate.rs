use bevy::{
        prelude::*,
        sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

const WALL_WIDTH: f32 = 1600.0;
const WALL_HEIGHT: f32 = 10.0;

#[derive(Component)]
struct Velocity(Vec2);

pub fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
) {
        // Make the walls
        commands.spawn((
                Name::new("bottom_wall"),
                Velocity(Vec2::new(0., 0.)),
                MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Rectangle::new(WALL_WIDTH, WALL_HEIGHT))),
                        material: materials.add(Color::hsl(180., 0.95, 0.57)),
                        transform: Transform::from_xyz(0.0, -445., 0.0),
                        ..default()
                },
        ));
        commands.spawn((
                Name::new("top_wall"),
                Velocity(Vec2::new(0., 0.)),
                MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Rectangle::new(WALL_WIDTH, WALL_HEIGHT))),
                        material: materials.add(Color::hsl(180., 0.95, 0.57)),
                        transform: Transform::from_xyz(0.0, 445., 0.0),
                        ..default()
                },
        ));
}
