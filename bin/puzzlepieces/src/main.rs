use bevy::prelude::*;
use puzzlepieces::{
    board::{Board, Cell, EmptyCell},
    common::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH, SIZE_TILE},
};
use rand::{seq::IndexedRandom, Rng};

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
        .add_systems(Update, (Board::update,))
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

    let textures = [
        (texture1, texture_atlas_layout1),
        (texture2, texture_atlas_layout2),
    ];
    let (texture, texture_atlas_layout) = textures.choose(&mut rand::rng()).unwrap();

    let mut board_state = BoardState::new(size as usize);
    board_state.shuffle(100);

    for row in 0..3 {
        for col in 0..3 {
            let index = row * (size as usize) + col;
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
                        index: board_state.state[row * (size as usize) + col],
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
