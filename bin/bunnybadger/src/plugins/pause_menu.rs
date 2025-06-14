use bevy::prelude::*;

use crate::plugins::start_screen::GameState;

#[derive(Component)]
struct ResumeButton;
#[derive(Component)]
struct ExitButton;
#[derive(Component)]
struct PauseMenuUI;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::PauseMenu), setup_pause_menu)
            .add_systems(
                Update,
                (
                    button_system.run_if(in_state(GameState::PauseMenu)),
                    keyboard_pause.run_if(in_state(GameState::PauseMenu)),
                ),
            )
            .add_systems(OnExit(GameState::PauseMenu), cleanup_pause_menu);
    }
}

fn setup_pause_menu(
    mut commands: Commands,
    camera_query: Query<Entity, With<crate::common::MainCamera>>,
) {
    if camera_query.is_empty() {
        commands.spawn((Camera2d, crate::common::MainCamera));
    }
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
        });
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
