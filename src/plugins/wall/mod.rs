use crate::AppStates;
use bevy::prelude::*;

mod initiate;

pub struct Wall;

impl Plugin for Wall {
        fn build(&self, app: &mut App) {
                app.add_systems(
                        OnEnter(AppStates::InGame),
                        initiate::setup,
                );
        }
}
