use bevy::prelude::*;

use crate::AppStates;

const NORMAL_BUTTON: Color = Color::rgb(0.0, 0.0, 0.0);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn(NodeBundle {
                style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                },
                ..default()
        })
        .with_children(|parent| {
                parent.spawn(ButtonBundle {
                        style: Style {
                                width: Val::Px(500.0),
                                height: Val::Px(250.),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                })
                .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                                "Press Start",
                                TextStyle {
                                        font: asset_server.load("fonts/Retro Gaming.ttf"),
                                        font_size: 50.0,
                                        color: Color::rgb(0.4, 0.9, 0.9),
                                },
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
        mut text_query: Query<&mut Text>,
        mut state: ResMut<NextState<AppStates>>,
) {
        for (interaction, mut color, children, entity) in &mut interaction_query {
                let mut text = text_query.get_mut(children[0]).unwrap();
                match *interaction {
                        Interaction::Pressed => {
                                text.sections[0].value = "Click To Start!".to_string();
                                commands.entity(entity).despawn_recursive();
                                state.set(AppStates::InGame);
                        }
                        Interaction::Hovered => {
                                text.sections[0].value = "Click To Start!".to_string();
                                *color = NORMAL_BUTTON.into();
                        }
                        Interaction::None => {
                                text.sections[0].value = "Click To Start".to_string();
                                *color = NORMAL_BUTTON.into();
                        }
                }
        }
}
