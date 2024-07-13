use bevy::prelude::*;
use puzzlepieces::{
    board::{Board, Cell, EmptyCell},
    common::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH},
};

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
        .add_systems(Update, (Board::update).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((Camera2dBundle::default(),));

    let texture = asset_server.load("images/1.jpg");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(150, 150), 3, 3, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    for row in 0..3 {
        for col in 0..3 {
            if row == 2 && col == 2 {
                commands.spawn((
                    TransformBundle {
                        local: Transform::from_xyz(
                            -220.0 + col as f32 * 220.0,
                            220.0 - row as f32 * 220.0,
                            0.0,
                        ),
                        ..default()
                    },
                    EmptyCell,
                ));
                continue;
            }

            commands.spawn((
                SpriteBundle {
                    texture: texture.clone(),
                    transform: Transform::from_xyz(
                        -220.0 + col as f32 * 220.0,
                        220.0 - row as f32 * 220.0,
                        0.0,
                    ),
                    ..default()
                },
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: row * 3 + col,
                },
                Cell,
            ));
        }
    }
}
