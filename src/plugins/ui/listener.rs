use bevy::prelude::*;

use crate::components::score::Score;

pub fn listen(mut score_query: Query<&Score, Changed<Score>>, mut text_query: Query<&mut Text>) {
        for score_data in score_query.iter_mut() {
                let score_text = format!("Score {} - {}", score_data.player, score_data.enemy);
                let mut text = text_query.single_mut();

                text.sections[0].value = score_text;
        }
}
