use crate::components::score::Score;
use bevy::prelude::*;

pub fn listen(
    score_query: Query<&Score, Changed<Score>>,
    mut text_query: Query<(&mut Text, &TextFont, &TextColor)>,
) {
        for score_data in score_query.iter() {
                let score_text = format!("Score {} - {}", score_data.player, score_data.enemy);

                for (mut text, _, _) in text_query.iter_mut() {
                        text.0 = score_text.clone();
                };
        }
}
