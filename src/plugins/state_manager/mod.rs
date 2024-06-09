use crate::AppStates;
use bevy::prelude::*;

mod setup;

pub struct StateManager;

impl Plugin for StateManager {
        fn build(&self, app: &mut App) {
                // app.add_systems(
                //         Startup,
                //         setup::setup,
                // );
                app.add_systems(
                        Update,
                        setup::setup.run_if(in_state(AppStates::Setup)),
                );
        }
}