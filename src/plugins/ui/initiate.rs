use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0., 0., 0.);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn(NodeBundle {
                style: Style {
                        width: Val::Percent(100.),
                        height: Val::Px(40.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                },
                ..default()
        })
        .with_children(|parent| {
                parent.spawn(ButtonBundle {
                        style: Style {
                                width: Val::Px(250.0),
                                height: Val::Px(40.0),
                                border: UiRect::all(Val::Px(2.5)),
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
                                "Score: 0 - 0",
                                TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 40.0,
                                        color: Color::rgb(0.4, 0.9, 0.9),
                                },
                        ));
                });
        });
}
