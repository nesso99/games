use bevy::{ecs::query::QueryFilter, prelude::*};
use rand::Rng;

use crate::common::SIZE_TILE;

pub struct Board;

#[derive(Component, Debug)]
pub struct Cell;

#[derive(Component, Debug)]
pub struct EmptyCell;

#[derive(QueryFilter)]
pub struct WithWithoutFilter<T: Component, P: Component> {
    _c: With<T>,
    _d: Without<P>,
}

#[derive(Resource)]
pub struct GameState {
    pub board_state: BoardState,
    pub size: usize,
    pub current_texture: usize,
    pub textures: Vec<(Handle<Image>, Handle<TextureAtlasLayout>)>,
    pub win_message_visible: bool,
}

impl Board {
    pub fn update(
        mut commands: Commands,
        mut game_state: ResMut<GameState>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        cell_query: Query<Entity, With<Cell>>,
        empty_cell_query: Query<Entity, With<EmptyCell>>,
    ) {
        let mut is_moved = false;
        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            game_state.board_state.move_up();
            is_moved = true;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            game_state.board_state.move_down();
            is_moved = true;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            game_state.board_state.move_right();
            is_moved = true;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            game_state.board_state.move_left();
            is_moved = true;
        }

        if is_moved {
            // TODO: rerender optimization
            // Remove existing cells
            for entity in cell_query.iter() {
                commands.entity(entity).despawn();
            }
            for entity in empty_cell_query.iter() {
                commands.entity(entity).despawn();
            }

            // Choose a random texture
            let (texture, texture_atlas_layout) =
                game_state.textures[game_state.current_texture].clone();

            // Spawn new board
            spawn_board(
                &mut commands,
                &game_state.board_state,
                game_state.size,
                texture,
                texture_atlas_layout,
            );
        }
    }
}

pub struct BoardState {
    pub state: Vec<usize>,
    pub blank_cell_idx: usize,
    pub size: usize,
    pub max: usize,
}

impl BoardState {
    pub fn new(size: usize) -> Self {
        let max = size * size;
        Self {
            state: (0..max).collect(),
            blank_cell_idx: max - 1,
            size,
            max,
        }
    }

    pub fn move_right(&mut self) {
        if self.blank_cell_idx % self.size != 0 {
            self.state
                .swap(self.blank_cell_idx, self.blank_cell_idx - 1);
            self.blank_cell_idx -= 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.blank_cell_idx % self.size != self.size - 1 {
            self.state
                .swap(self.blank_cell_idx, self.blank_cell_idx + 1);
            self.blank_cell_idx += 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.blank_cell_idx >= self.size {
            self.state
                .swap(self.blank_cell_idx, self.blank_cell_idx - self.size);
            self.blank_cell_idx -= self.size;
        }
    }

    pub fn move_up(&mut self) {
        if self.blank_cell_idx <= self.max - 1 - self.size {
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

    // Check if the puzzle is solved
    pub fn is_solved(&self) -> bool {
        // The puzzle is solved when all tiles are in order (0, 1, 2, ...)
        // and the blank cell is at the end
        for i in 0..self.state.len() - 1 {
            if self.state[i] != i {
                return false;
            }
        }

        // Also check that the blank cell is in the last position
        self.blank_cell_idx == self.state.len() - 1
    }
}

// Function to spawn the board
pub fn spawn_board(
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
