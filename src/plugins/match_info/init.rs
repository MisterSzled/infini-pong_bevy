use bevy::prelude::*;

use crate::components::score::Score;

pub fn setup(
        mut commands: Commands,
) {
        commands.spawn((
                Name::new("match_info"),
                Score::default()
        ));
}
