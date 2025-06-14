use bevy::prelude::*;

use crate::plugins::start_screen::GameState;
use bevy_kira_audio::prelude::*;

#[derive(Component)]
struct ResumeButton;
#[derive(Component)]
struct ExitButton;
#[derive(Component)]
struct PauseMenuUI;
#[derive(Component)]
struct VolumeButton {
    pub index: u8, // 0-10
    pub is_hovered: bool,
}

#[derive(Resource, Default)]
pub struct Volume(pub f64); // 0.0 - 1.0

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Volume>()
            .add_systems(OnEnter(GameState::PauseMenu), setup_pause_menu)
            .add_systems(
                Update,
                (
                    button_system,
                    keyboard_pause,
                    volume_button_system,
                    volume_highlight_system,
                )
                    .run_if(in_state(GameState::PauseMenu)),
            )
            .add_systems(OnExit(GameState::PauseMenu), cleanup_pause_menu);
    }
}

fn setup_pause_menu(
    mut commands: Commands,
    camera_query: Query<Entity, With<crate::common::MainCamera>>,
    volume: Res<Volume>,
) {
    if camera_query.is_empty() {
        commands.spawn((Camera2d, crate::common::MainCamera));
    }

    commands.insert_resource(Volume(0.5));

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            PauseMenuUI,
        ))
        .with_children(|parent| {
            // Resume Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    ResumeButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Resume"),
                        TextFont {
                            font_size: 40.0,
                            ..Default::default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });
            // Exit Button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    ExitButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Exit"),
                        TextFont {
                            font_size: 40.0,
                            ..Default::default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });
            // Volume "slider" as 11 buttons (0-10)
            parent
                .spawn(Node {
                    width: Val::Px(340.0),
                    height: Val::Px(60.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Music Volume"),
                        TextFont {
                            font_size: 24.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                    ));
                    // Button row
                    parent
                        .spawn(Node {
                            width: Val::Px(320.0),
                            height: Val::Px(34.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Row,
                            ..default()
                        })
                        .with_children(|parent| {
                            for i in 0u8..=10 {
                                let highlight = (volume.0 * 10.0).round() as u8 == i;
                                parent
                                    .spawn((
                                        Button,
                                        Node {
                                            width: Val::Px(24.0),
                                            height: Val::Px(24.0),
                                            margin: UiRect::horizontal(Val::Px(2.0)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BorderColor(Color::BLACK),
                                        BackgroundColor(if highlight {
                                            Color::srgb(1.0, 1.0, 0.0)
                                        } else {
                                            Color::srgb(0.2, 0.2, 0.2)
                                        }),
                                        VolumeButton {
                                            index: i,
                                            is_hovered: false,
                                        },
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn((
                                            Text::new(i.to_string()),
                                            TextFont {
                                                font_size: 22.0,
                                                ..default()
                                            },
                                            TextColor(Color::BLACK),
                                        ));
                                    });
                            }
                        });
                });
        });
}

fn volume_button_system(
    mut interaction_query: Query<(&Interaction, &mut VolumeButton), Changed<Interaction>>,
    mut volume: ResMut<Volume>,
    audio: Res<Audio>,
) {
    let mut changed = false;
    for (interaction, mut btn) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let v = btn.index as f64 / 10.0;
                if (volume.0 - v).abs() > 0.01 {
                    volume.0 = v;
                    changed = true;
                }
                // *bg = BackgroundColor(Color::srgb(1.0, 1.0, 0.0)); // immediate feedback
            }
            Interaction::Hovered => {
                btn.is_hovered = true;
                // *bg = BackgroundColor(Color::srgb(1.0, 0.5, 0.0));
            }
            Interaction::None => {
                btn.is_hovered = false;
                // let highlight = (volume.0 * 10.0).round() as u8 == btn.0;
                // *bg = BackgroundColor(if highlight {
                //     Color::srgb(1.0, 1.0, 0.0)
                // } else {
                //     Color::srgb(0.2, 0.2, 0.2)
                // });
            }
        }
    }
    if changed {
        audio.set_volume(volume.0);
    }
}

fn volume_highlight_system(
    mut volume_button_query: Query<(&VolumeButton, &mut BackgroundColor)>,
    volume: Res<Volume>,
) {
    let actual_volume = (volume.0 * 10.0).round() as u8;
    for (btn, mut bg) in &mut volume_button_query {
        if btn.index <= actual_volume {
            *bg = BackgroundColor(Color::srgb(1.0, 1.0, 0.0)); // Highlight the current volume
        } else {
            *bg = BackgroundColor(Color::srgb(0.2, 0.2, 0.2)); // Default color for other buttons
        }

        if btn.is_hovered {
            *bg = BackgroundColor(Color::srgb(1.0, 0.5, 0.0)); // Highlight on hover
        }
    }
}

#[allow(clippy::type_complexity)]
fn button_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&ResumeButton>,
            Option<&ExitButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, mut color, resume, exit) in &mut interaction_query {
        // Only react for resume/exit, not for volume dots (they use their own system)
        if resume.is_none() && exit.is_none() {
            continue;
        }
        match *interaction {
            Interaction::Pressed => {
                if resume.is_some() {
                    next_state.set(GameState::InGame);
                } else if exit.is_some() {
                    app_exit_events.write(AppExit::Success);
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.25, 0.25, 0.25));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
            }
        }
    }
}

fn keyboard_pause(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::InGame);
    }
}

fn cleanup_pause_menu(mut commands: Commands, ui_query: Query<Entity, With<PauseMenuUI>>) {
    for entity in &ui_query {
        commands.entity(entity).despawn();
    }
}
