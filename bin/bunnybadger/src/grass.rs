use bevy::prelude::*;

pub struct GrassSevice<'a, 'w, 's> {
    width: f32,
    height: f32,
    texture: Handle<Image>,
    commands: &'a mut Commands<'w, 's>,
}

impl<'a, 'w, 's> GrassSevice<'a, 'w, 's> {
    pub fn new(asset_server: &Res<AssetServer>, commands: &'a mut Commands<'w, 's>) -> Self {
        Self {
            width: 100.,
            height: 100.,
            texture: asset_server.load("images/grass.png"),
            commands,
        }
    }

    pub fn spawn(&mut self) {
        let start_x: f32 = -270.;
        let start_y: f32 = -190.;
        for yi in 0..5 {
            let current_y = start_y + yi as f32 * self.height;
            for xi in 0..7 {
                self.commands.spawn(SpriteBundle {
                    texture: self.texture.clone(),
                    transform: Transform::from_xyz(
                        start_x + xi as f32 * self.width,
                        current_y,
                        -1.,
                    ),
                    ..default()
                });
            }
        }
    }
}
