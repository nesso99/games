use bevy::{ecs::query::QueryFilter, prelude::*};

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

impl Board {
    pub fn update(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut empty_cell_query: Query<&mut Transform, WithWithoutFilter<EmptyCell, Cell>>,
        mut cell_query: Query<&mut Transform, WithWithoutFilter<Cell, EmptyCell>>,
    ) {
        let mut empty_cell_transform: Mut<'_, Transform> = empty_cell_query.single_mut();

        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            if empty_cell_transform.translation.y == -220.0 {
                return;
            }

            for mut cell_transform in cell_query.iter_mut() {
                if cell_transform.translation.x == empty_cell_transform.translation.x
                    && cell_transform.translation.y + 220.0 == empty_cell_transform.translation.y
                {
                    cell_transform.translation.y += 220.0;
                    break;
                }
            }
            empty_cell_transform.translation.y -= 220.0;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            if empty_cell_transform.translation.y == 220.0 {
                return;
            }

            for mut cell_transform in cell_query.iter_mut() {
                if cell_transform.translation.x == empty_cell_transform.translation.x
                    && cell_transform.translation.y - 220.0 == empty_cell_transform.translation.y
                {
                    cell_transform.translation.y -= 220.0;
                    break;
                }
            }
            empty_cell_transform.translation.y += 220.0;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            if empty_cell_transform.translation.x == -220.0 {
                return;
            }

            for mut cell_transform in cell_query.iter_mut() {
                if cell_transform.translation.y == empty_cell_transform.translation.y
                    && cell_transform.translation.x + 220.0 == empty_cell_transform.translation.x
                {
                    cell_transform.translation.x += 220.0;
                    break;
                }
            }
            empty_cell_transform.translation.x -= 220.0;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            if empty_cell_transform.translation.x == 220.0 {
                return;
            }

            for mut cell_transform in cell_query.iter_mut() {
                if cell_transform.translation.y == empty_cell_transform.translation.y
                    && cell_transform.translation.x - 220.0 == empty_cell_transform.translation.x
                {
                    cell_transform.translation.x -= 220.0;
                    break;
                }
            }
            empty_cell_transform.translation.x += 220.0;
        }
    }
}
