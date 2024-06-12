use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Velocity {
        pub x: f32,
        pub y: f32,
}

impl Velocity {
        pub fn new() -> Velocity {
                let mut rng = rand::thread_rng();

                let mut x: f32 = rng.gen_range(0.5..=2.5);
                let mut y: f32 = rng.gen_range(0.5..=2.5);

                if rng.gen_bool(0.5) {
                        x = -x;
                }
                if rng.gen_bool(0.5) {
                        y = -y;
                }

                // x = -10.;
                // y = 0.;

                Velocity { x, y }
        }
}
