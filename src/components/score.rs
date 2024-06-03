use bevy::prelude::*;

#[derive(Component)]
pub struct Score {
        pub player: u8,
        pub enemy: u8
}

impl Default for Score {
        fn default() -> Self {
            Score {
                player: 0,
                enemy: 0
            }
        }
}