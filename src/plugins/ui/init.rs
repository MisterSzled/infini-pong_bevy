use bevy::{prelude::*, text::FontSmoothing};

const NORMAL_BUTTON: Color = Color::srgb(0., 0., 0.);

pub fn setup(
        mut commands: Commands, 
        asset_server: Res<AssetServer>,
) {
        commands.spawn(
                Node {
                        width: Val::Percent(100.),
                        height: Val::Percent(7.5),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                },
        )
        .with_children(|parent| {
                parent.spawn((
                        Button,
                        Node {
                            width: Val::Px(250.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BorderColor(Color::BLACK),
                        BackgroundColor(NORMAL_BUTTON.into()),
                ))
                .with_children(|parent| {
                        parent.spawn((
                                Text::new("Score: 0 - 0"),
                                TextFont {
                                        font: asset_server.load("fonts/Retro Gaming.ttf"),
                                        font_size: 25.0,
                                        font_smoothing: FontSmoothing::AntiAliased
                                },
                                TextColor(Color::srgb(0.4, 0.9, 0.9))
                        ));
                });
        });
}