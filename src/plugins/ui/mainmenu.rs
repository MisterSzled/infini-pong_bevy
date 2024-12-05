use bevy::{prelude::*, text::FontSmoothing};

use crate::AppStates;

const NORMAL_BUTTON: Color = Color::srgb(0.0, 0.0, 0.0);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn(Node {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                }
        )
        .with_children(|parent| {
                parent.spawn((
                        Button,
                        Node {
                                width: Val::Px(500.0),
                                height: Val::Px(250.),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                        },
                        BorderColor(Color::BLACK),
                        BackgroundColor(NORMAL_BUTTON.into()),
                ))
                .with_children(|parent| {
                        parent.spawn((
                                Text::new("Press Start"),
                                TextFont {
                                        font: asset_server.load("fonts/Retro Gaming.ttf"),
                                        font_size: 50.0,
                                        font_smoothing: FontSmoothing::AntiAliased
                                },
                                TextColor(Color::srgb(0.4, 0.9, 0.9))
                        ));
                });
        });
}

pub fn listen(
        mut commands: Commands,
        mut interaction_query: Query<
                (&Interaction, &mut BackgroundColor, &Children, Entity),
                (Changed<Interaction>, With<Button>),
        >,
        mut state: ResMut<NextState<AppStates>>,
) {
        for (interaction, mut color, _, entity) in &mut interaction_query {
                match *interaction {
                        Interaction::Pressed => {
                                commands.entity(entity).despawn_recursive();
                                state.set(AppStates::InGame);
                        }
                        Interaction::Hovered => {
                                *color = NORMAL_BUTTON.into();
                        }
                        Interaction::None => {
                                *color = NORMAL_BUTTON.into();
                        }
                }
        }
}
