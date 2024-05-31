use crate::AppStates;
use bevy::prelude::*;

mod initiate;
mod movement;

pub struct Player;

impl Plugin for Player {
        fn build(&self, app: &mut App) {
                app.add_systems(
                        OnEnter(AppStates::InGame),
                        initiate::setup,
                );
                app.add_systems(
                        Update,
                        movement::move_player.run_if(
                                in_state(AppStates::InGame)
                        ),
                );
        }
}
