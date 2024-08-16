use bevy::{
        math::bounding::{Aabb2d, IntersectsVolume, RayCast2d},
        math::Dir2,
        prelude::*,
};

#[derive(Component, Debug, Clone, Copy)]
pub struct Collideable {
        pub width: f32,
        pub height: f32,
        pub x_offset: f32,
        pub y_offset: f32,
}

#[derive(Debug)]
pub struct Directions {
        pub up: bool,
        pub down: bool,
        pub left: bool,
        pub right: bool,
}

impl Collideable {
        pub fn new(width: f32, height: f32, x_offset: f32, y_offset: f32) -> Collideable {
                Collideable {
                        width: width as f32,
                        height: height as f32,
                        x_offset: x_offset as f32,
                        y_offset: y_offset as f32,
                }
        }

        pub fn get_colliding_sides(
                self,
                self_pos: Vec3,
                collidables_query: Query<(&Collideable, &Transform)>,
        ) -> Directions {
                let subject_aabb = Aabb2d {
                        min: Vec2 {
                                x: self_pos.x - (self.width / 2.) + self.x_offset,
                                y: self_pos.y - (self.height / 2.) + self.y_offset,
                        },
                        max: Vec2 {
                                x: self_pos.x + (self.width / 2.) + self.x_offset,
                                y: self_pos.y + (self.height / 2.) + self.y_offset,
                        },
                };

                let mut colliding_sides = Directions {
                        up: false,
                        down: false,
                        left: false,
                        right: false,
                };

                for (collidable, transform) in collidables_query.iter() {
                        let center = transform.translation;

                        let collidable_aabb = Aabb2d {
                                min: Vec2 {
                                        x: center.x - (collidable.width / 2.) + collidable.x_offset,
                                        y: center.y - (collidable.height / 2.) + collidable.y_offset,
                                },
                                max: Vec2 {
                                        x: center.x + (collidable.width / 2.) + collidable.x_offset,
                                        y: center.y + (collidable.height / 2.) + collidable.y_offset,
                                },
                        };

                        if subject_aabb.intersects(&collidable_aabb) {
                                let pos_x_ray = RayCast2d::new(
                                        Vec2 {
                                                x: self_pos.x + self.x_offset,
                                                y: self_pos.y + self.y_offset,
                                        },
                                        Dir2::X,
                                        50.,
                                );
                                let neg_x_ray = RayCast2d::new(
                                        Vec2 {
                                                x: self_pos.x + self.x_offset,
                                                y: self_pos.y + self.y_offset,
                                        },
                                        Dir2::NEG_X,
                                        50.,
                                );

                                let pos_y_ray = RayCast2d::new(
                                        Vec2 {
                                                x: self_pos.x + self.x_offset,
                                                y: self_pos.y + self.y_offset,
                                        },
                                        Dir2::Y,
                                        50.,
                                );
                                let neg_y_ray = RayCast2d::new(
                                        Vec2 {
                                                x: self_pos.x + self.x_offset,
                                                y: self_pos.y + self.y_offset,
                                        },
                                        Dir2::NEG_Y,
                                        50.,
                                );

                                let pos_x_intersection =
                                        pos_x_ray.aabb_intersection_at(&collidable_aabb);
                                let neg_x_intersection =
                                        neg_x_ray.aabb_intersection_at(&collidable_aabb);

                                let pos_y_intersection =
                                        pos_y_ray.aabb_intersection_at(&collidable_aabb);
                                let neg_y_intersection =
                                        neg_y_ray.aabb_intersection_at(&collidable_aabb);

                                if Option::is_some(&pos_x_intersection) {
                                        colliding_sides.right = true;
                                }
                                if Option::is_some(&neg_x_intersection) {
                                        colliding_sides.left = true;
                                }
                                if Option::is_some(&pos_y_intersection) {
                                        colliding_sides.up = true;
                                }
                                if Option::is_some(&neg_y_intersection) {
                                        colliding_sides.down = true;
                                }
                        }
                }

                colliding_sides
        }
}
