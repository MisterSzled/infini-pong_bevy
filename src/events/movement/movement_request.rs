use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct MovementRequest {
        pub up: bool,
        pub down: bool,
        pub left: bool,
        pub right: bool,
}
