use bevy::prelude::*;
use puzzlepieces::{
    board::{Board, Cell, EmptyCell},
    common::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH, SIZE_TILE},
};
use rand::seq::IndexedRandom;

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

    let texture1 = asset_server.load("images/1.jpg");
    let layout1 = TextureAtlasLayout::from_grid(UVec2::new(450/3, 450/3), 3, 3, None, None);
    let texture_atlas_layout1 = texture_atlas_layouts.add(layout1);

    let texture2 = asset_server.load("images/2.jpg");
    let layout2 = TextureAtlasLayout::from_grid(UVec2::new(248/3, 248/3), 3, 3, None, None);
    let texture_atlas_layout2 = texture_atlas_layouts.add(layout2);

    let textures = [(texture1, texture_atlas_layout1),
        (texture2, texture_atlas_layout2)];
    let (texture, texture_atlas_layout) = textures.choose(&mut rand::rng()).unwrap();

    for row in 0..3 {
        for col in 0..3 {
            if row == 2 && col == 2 {
                commands.spawn((
                    Transform::from_xyz(
                        -220.0 + col as f32 * 220.0,
                        220.0 - row as f32 * 220.0,
                        0.0,
                    ),
                    EmptyCell,
                ));
                continue;
            }

            commands.spawn((
                Sprite {
                    image: texture.clone(),
                    texture_atlas: Some(TextureAtlas{
                        layout: texture_atlas_layout.clone(),
                        index: row * 3 + col,
                    }),
                    custom_size: Some(Vec2::new(218.0, 218.0)),
                    ..Default::default()
                },
                Transform::from_xyz(-SIZE_TILE + col as f32 * SIZE_TILE, SIZE_TILE - row as f32 * SIZE_TILE, 0.0),
                Cell,
            ));
        }
    }
}
