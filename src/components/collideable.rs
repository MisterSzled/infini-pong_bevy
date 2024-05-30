use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Collideable {
        pub width: f32,
        pub height: f32,
}

impl Collideable {
        pub fn new(width: f32, height: f32) -> Collideable {
                Collideable {
                        width: width as f32,
                        height: height as f32,
                }
        }
}