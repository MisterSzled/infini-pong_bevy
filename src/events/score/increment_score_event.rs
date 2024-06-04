use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct IncrementScoreEvent {
        pub player: bool,
        pub enemy: bool,
}
