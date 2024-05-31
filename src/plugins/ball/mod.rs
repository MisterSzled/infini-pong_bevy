use bevy::prelude::*;
use crate::AppStates;

mod initiate;
mod controller;

pub struct Ball;

impl Plugin for Ball {
        fn build(&self, app: &mut App) {
                app.add_systems(
                        OnEnter(AppStates::InGame),
                        initiate::setup,
                );
                app.add_systems(
                        Update,
                        controller::update_controller.run_if(
                                in_state(AppStates::InGame)
                        ),
                );
        }
}
