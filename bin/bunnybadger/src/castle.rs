use bevy::prelude::*;

pub struct CastleService<'a, 'w, 's> {
    // width: f32,
    height: f32,
    texture: Handle<Image>,
    commands: &'a mut Commands<'w, 's>,
}

impl<'a, 'w, 's> CastleService<'a, 'w, 's> {
    pub fn new(asset_server: &Res<AssetServer>, commands: &'a mut Commands<'w, 's>) -> Self {
        Self {
            // width: 109.,
            height: 105.,
            texture: asset_server.load("images/castle.png"),
            commands,
        }
    }

    pub fn spawn(&mut self) {
        for i in 0..4 {
            self.commands.spawn(SpriteBundle {
                texture: self.texture.clone(),
                transform: Transform::from_xyz(-260., -170. + i as f32 * self.height, 0.),
                ..default()
            });
        }
    }
}
