use bevy::prelude::*;

use crate::AppStates;

mod controller;
mod init;

pub struct Enemy;

impl Plugin for Enemy {
        fn build(&self, app: &mut App) {
                app.add_systems(
                        OnEnter(AppStates::InGame), 
                        init::setup);

                app.add_systems(
                        Update,
                        controller::update_controller.run_if(in_state(AppStates::InGame)),
                );
        }
}
