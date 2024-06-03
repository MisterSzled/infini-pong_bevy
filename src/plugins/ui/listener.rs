use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0., 0., 0.);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn listen(
        mut interaction_query: Query<
                (
                        &Interaction,
                        &mut BackgroundColor,
                        &mut BorderColor,
                        &Children,
                ),
                (Changed<Interaction>, With<Button>),
        >,
        mut text_query: Query<&mut Text>,
) {
        for (interaction, mut color, mut border_color, children) in &mut interaction_query {
                let mut text = text_query.get_mut(children[0]).unwrap();
                match *interaction {
                        Interaction::Pressed => {
                                text.sections[0].value = "You Clicked".to_string();
                                *color = PRESSED_BUTTON.into();
                        }
                        Interaction::Hovered => {
                                text.sections[0].value = "You hovered".to_string();
                                *color = HOVERED_BUTTON.into();
                        }
                        Interaction::None => {
                                text.sections[0].value = "Score: 0 - 0".to_string();
                                *color = NORMAL_BUTTON.into();
                        }
                }
        }
}
