use bevy::prelude::*;

use crate::common::MainCamera;

#[derive(Component)]
struct StartButton;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    StartScreen,
    InGame,
}

pub struct StartScreenPlugin;

impl Plugin for StartScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::StartScreen), setup_start_screen)
            .add_systems(
                Update,
                button_system.run_if(in_state(GameState::StartScreen)),
            )
            .add_systems(OnExit(GameState::StartScreen), cleanup_start_screen);
    }
}

#[derive(Component)]
struct StartScreenUI;

fn setup_start_screen(mut commands: Commands, camera_query: Query<Entity, With<MainCamera>>) {
    if camera_query.is_empty() {
        commands.spawn((Camera2d, MainCamera));
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
            StartScreenUI,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    StartButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Play Game"),
                        TextFont {
                            font_size: 40.0,
                            ..Default::default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });
        });
}

fn button_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StartButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(GameState::InGame);
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

fn cleanup_start_screen(mut commands: Commands, ui_query: Query<Entity, With<StartScreenUI>>) {
    for entity in &ui_query {
        commands.entity(entity).despawn_recursive();
    }
}
