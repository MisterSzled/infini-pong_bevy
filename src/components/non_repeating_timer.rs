use bevy::prelude::*;

#[derive(Component)]
pub struct NonRepeatingTimer {
    pub timer: Timer,
}