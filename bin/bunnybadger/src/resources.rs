use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(Resource, Default, Clone)]
pub struct GameAssets {
    // Textures
    pub dude_texture: Handle<Image>,
    pub castle_texture: Handle<Image>,
    pub arrow_texture: Handle<Image>,
    pub badguy_texture: Handle<Image>,
    pub grass_texture: Handle<Image>,
    pub healthbar_texture: Handle<Image>,

    // Audio
    pub shoot_sound: Handle<AudioSource>,
    pub enemy_sound: Handle<AudioSource>,
    pub explode_sound: Handle<AudioSource>,
    pub background_music: Handle<AudioSource>,
}

impl GameAssets {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            // Textures
            dude_texture: asset_server.load("images/dude.png"),
            castle_texture: asset_server.load("images/castle.png"),
            arrow_texture: asset_server.load("images/bullet.png"),
            badguy_texture: asset_server.load("images/badguy_sheet.png"),
            grass_texture: asset_server.load("images/grass.png"),
            healthbar_texture: asset_server.load("images/healthbar.png"),

            // Audio
            shoot_sound: asset_server.load("audios/shoot.wav"),
            enemy_sound: asset_server.load("audios/enemy.wav"),
            explode_sound: asset_server.load("audios/explode.wav"),
            background_music: asset_server.load("audios/moonlight.wav"),
        }
    }
}
