use bevy::prelude::*;

use crate::components::score::Score;
use crate::events::score::increment_score_event::IncrementScoreEvent;

pub fn listen(
        mut score_query: Query<&mut Score>,
        mut increment_score_event: EventReader<IncrementScoreEvent>,
) {
        let mut score = score_query.single_mut();

        for score_event in increment_score_event.read() {
                if score_event.player {
                        score.player += 1;
                }
                if score_event.enemy {
                        score.enemy += 1;
                }
        }
}
