use bevy::prelude::*;
use crate::AppStates;

mod init;
mod controller;
mod reset;

pub struct Ball;

impl Plugin for Ball {
        fn build(&self, app: &mut App) {
                app.add_systems(
                        OnEnter(AppStates::InGame),
                        init::setup,
                );
                app.add_systems(
                        Update,
                        controller::update_controller.run_if(
                                in_state(AppStates::InGame)
                        ),
                );
                app.add_systems(
                        Update,
                        reset::reset_ball.run_if(
                                in_state(AppStates::InGame)
                        ),
                );
        }
}
