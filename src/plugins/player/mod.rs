use crate::AppStates;
use bevy::prelude::*;

mod init;
mod movement;

pub struct Player;

impl Plugin for Player {
        fn build(&self, app: &mut App) {
                app.add_systems(
                        OnEnter(AppStates::InGame),
                        init::setup,
                );
                app.add_systems(
                        Update,
                        movement::move_player.run_if(
                                in_state(AppStates::InGame)
                        ),
                );
        }
}
