use crate::AppStates;
use bevy::prelude::*;

mod initiate;

pub struct UI;

impl Plugin for UI {
        fn build(&self, app: &mut App) {
                app.add_systems(
                        OnEnter(AppStates::MainMenu),
                        initiate::setup,
                );
        }
}
