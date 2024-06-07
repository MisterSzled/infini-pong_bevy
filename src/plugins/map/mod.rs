use crate::AppStates;
use bevy::prelude::*;

mod init;

pub struct Map;

impl Plugin for Map {
        fn build(&self, app: &mut App) {
                app.add_systems(
                        OnEnter(AppStates::InGame),
                        init::setup,
                );
        }
}