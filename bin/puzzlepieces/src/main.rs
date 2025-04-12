use bevy::prelude::*;
use puzzlepieces::{
    board::{Board, Cell, EmptyCell},
    common::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH, SIZE_TILE},
};
use rand::{seq::IndexedRandom, Rng};

// Components for the shuffle button
#[derive(Component)]
struct ShuffleButton;

#[derive(Resource)]
struct GameState {
    board_state: BoardState,
    size: usize,
    current_texture: usize,
    textures: Vec<(Handle<Image>, Handle<TextureAtlasLayout>)>,
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
        .add_systems(Update, (Board::update, button_system))
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

    // for row in 0..3 {
    //     for col in 0..3 {
    //         let index = row * (size as usize) + col;
    //         if index == board_state.blank_cell_idx {
    //             commands.spawn((
    //                 EmptyCell,
    //                 Transform::from_xyz(
    //                     -SIZE_TILE + col as f32 * SIZE_TILE,
    //                     SIZE_TILE - row as f32 * SIZE_TILE,
    //                     0.0,
    //                 ),
    //             ));
    //             continue;
    //         }
    //
    //         commands.spawn((
    //             Sprite {
    //                 image: texture.clone(),
    //                 texture_atlas: Some(TextureAtlas {
    //                     layout: texture_atlas_layout.clone(),
    //                     index: board_state.state[row * (size as usize) + col],
    //                 }),
    //                 custom_size: Some(Vec2::new(218.0, 218.0)),
    //                 ..Default::default()
    //             },
    //             Transform::from_xyz(
    //                 -SIZE_TILE + col as f32 * SIZE_TILE,
    //                 SIZE_TILE - row as f32 * SIZE_TILE,
    //                 0.0,
    //             ),
    //             Cell,
    //         ));
    //     }
    // }
}

pub struct BoardState {
    pub state: Vec<usize>,
    pub blank_cell_idx: usize,
    pub size: usize,
}

impl BoardState {
    pub fn new(size: usize) -> Self {
        let max = size * size;
        Self {
            state: (0..max).collect(),
            blank_cell_idx: max - 1,
            size,
        }
    }

    pub fn move_left(&mut self) {
        if self.blank_cell_idx % self.size != 0 {
            self.state
                .swap(self.blank_cell_idx, self.blank_cell_idx - 1);
            self.blank_cell_idx -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.blank_cell_idx % self.size != self.size - 1 {
            self.state
                .swap(self.blank_cell_idx, self.blank_cell_idx + 1);
            self.blank_cell_idx += 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.blank_cell_idx >= self.size {
            self.state
                .swap(self.blank_cell_idx, self.blank_cell_idx - self.size);
            self.blank_cell_idx -= self.size;
        }
    }

    pub fn move_down(&mut self) {
        if self.blank_cell_idx <= self.size - 3 {
            self.state
                .swap(self.blank_cell_idx, self.blank_cell_idx + self.size);
            self.blank_cell_idx += self.size;
        }
    }

    pub fn shuffle(&mut self, times: usize) {
        for _ in 0..times {
            let mut rng = rand::rng();
            let direction = rng.random_range(0..4);
            match direction {
                0 => self.move_left(),
                1 => self.move_right(),
                2 => self.move_up(),
                _ => self.move_down(),
            }
        }
    }
}

// Function to spawn the board
fn spawn_board(
    commands: &mut Commands,
    board_state: &BoardState,
    size: usize,
    texture: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
) {
    for row in 0..size {
        for col in 0..size {
            let index = row * size + col;
            if index == board_state.blank_cell_idx {
                commands.spawn((
                    EmptyCell,
                    Transform::from_xyz(
                        -SIZE_TILE + col as f32 * SIZE_TILE,
                        SIZE_TILE - row as f32 * SIZE_TILE,
                        0.0,
                    ),
                ));
                continue;
            }

            commands.spawn((
                Sprite {
                    image: texture.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: board_state.state[row * size + col],
                    }),
                    custom_size: Some(Vec2::new(218.0, 218.0)),
                    ..Default::default()
                },
                Transform::from_xyz(
                    -SIZE_TILE + col as f32 * SIZE_TILE,
                    SIZE_TILE - row as f32 * SIZE_TILE,
                    0.0,
                ),
                Cell,
            ));
        }
    }
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
