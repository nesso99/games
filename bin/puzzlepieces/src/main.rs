use bevy::prelude::*;
use puzzlepieces::{
    board::{spawn_board, Board, BoardState, Cell, EmptyCell, GameState},
    common::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH},
};
use rand::Rng;

// Components for the shuffle button
#[derive(Component)]
struct ShuffleButton;

#[derive(Component)]
struct WinMessage;

// System to check if the puzzle is solved
fn check_win_condition(
    mut game_state: ResMut<GameState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut win_message_query: Query<&mut Visibility, With<WinMessage>>,
) {
    // Only check after a move has been made
    if keyboard_input.just_released(KeyCode::ArrowUp)
        || keyboard_input.just_released(KeyCode::ArrowDown)
        || keyboard_input.just_released(KeyCode::ArrowLeft)
        || keyboard_input.just_released(KeyCode::ArrowRight)
    {
        // Check if the puzzle is solved
        if game_state.board_state.is_solved() && !game_state.win_message_visible {
            // Show win message
            if let Ok(mut visibility) = win_message_query.get_single_mut() {
                *visibility = Visibility::Visible;
                game_state.win_message_visible = true;
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "puzzlepieces".into(),
                resolution: (RESOLUTION_WIDTH, RESOLUTION_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }),))
        .add_systems(Startup, setup)
        .add_systems(Update, (Board::update, button_system, check_win_condition))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    // let texture = asset_server.load("images/1.jpg");
    // let layout = TextureAtlasLayout::from_grid(UVec2::new(150, 150), 3, 3, None, None);
    // let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let size = 3;

    let texture1 = asset_server.load("images/1.jpg");
    let layout1 =
        TextureAtlasLayout::from_grid(UVec2::new(450 / size, 450 / size), size, size, None, None);
    let texture_atlas_layout1 = texture_atlas_layouts.add(layout1);

    let texture2 = asset_server.load("images/2.jpg");
    let layout2 =
        TextureAtlasLayout::from_grid(UVec2::new(248 / size, 248 / size), size, size, None, None);
    let texture_atlas_layout2 = texture_atlas_layouts.add(layout2);

    let textures = vec![
        (texture1, texture_atlas_layout1),
        (texture2, texture_atlas_layout2),
    ];
    let current_texture = rand::rng().random_range(0..textures.len());
    let (texture, texture_atlas_layout) = textures[current_texture].clone();

    let mut board_state = BoardState::new(size as usize);
    board_state.shuffle(100);

    // Spawn the board
    spawn_board(
        &mut commands,
        &board_state,
        size as usize,
        texture,
        texture_atlas_layout,
    );

    // Store game state as a resource
    commands.insert_resource(GameState {
        board_state,
        size: size as usize,
        current_texture,
        textures,
        win_message_visible: false,
    });

    // Create win message (initially hidden)
    commands
        .spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Px(100.0),
                position_type: PositionType::Absolute,
                top: Val::Px(RESOLUTION_HEIGHT / 2.0 - 50.0),
                left: Val::Px(RESOLUTION_WIDTH / 2.0 - 150.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.6, 0.2)),
            Visibility::Hidden,
            WinMessage,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Puzzle Solved!"),
                TextFont {
                    font_size: 32.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ));
        });

    commands
        .spawn((
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(50.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(20.0),
                right: Val::Px(20.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            ShuffleButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Shuffle"),
                TextFont {
                    font_size: 20.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

// System to handle button interactions
#[allow(clippy::type_complexity)]
fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ShuffleButton>),
    >,
    mut game_state: ResMut<GameState>,
    cell_query: Query<Entity, With<Cell>>,
    empty_cell_query: Query<Entity, With<EmptyCell>>,
    mut win_message_query: Query<&mut Visibility, With<WinMessage>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Change button color when pressed
                *color = Color::srgb(0.35, 0.35, 0.35).into();

                // Shuffle the board state
                game_state.board_state.shuffle(100);

                // Remove existing cells
                for entity in cell_query.iter() {
                    commands.entity(entity).despawn();
                }
                for entity in empty_cell_query.iter() {
                    commands.entity(entity).despawn();
                }

                // Choose a random texture
                let current_texture = rand::rng().random_range(0..game_state.textures.len());
                game_state.current_texture = current_texture;
                let (texture, texture_atlas_layout) = game_state.textures[current_texture].clone();

                // Spawn new board
                spawn_board(
                    &mut commands,
                    &game_state.board_state,
                    game_state.size,
                    texture,
                    texture_atlas_layout,
                );

                // Hide win message when shuffling
                if game_state.win_message_visible {
                    game_state.win_message_visible = false;
                    if let Ok(mut visibility) = win_message_query.get_single_mut() {
                        *visibility = Visibility::Hidden;
                    }
                }
            }
            Interaction::Hovered => {
                // Change button color when hovered
                *color = Color::srgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                // Reset button color
                *color = Color::srgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}
