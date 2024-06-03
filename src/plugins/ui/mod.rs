use crate::AppStates;
use bevy::prelude::*;

mod initiate;
mod listener;

pub struct UI;

impl Plugin for UI {
        fn build(&self, app: &mut App) {
                app.add_systems(
                        OnEnter(AppStates::InGame),
                        initiate::setup,
                );
                app.add_systems(
                        Update,
                        listener::listen.run_if(
                                in_state(AppStates::InGame)
                        ),
                );
        }
}
