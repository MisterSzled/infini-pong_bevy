use crate::AppStates;
use bevy::prelude::*;

mod init;

pub struct MatchInfo;

impl Plugin for MatchInfo {
        fn build(&self, app: &mut App) {
                app.add_systems(
                        OnEnter(AppStates::InGame),
                        init::setup,
                );
        }
}
