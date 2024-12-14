use bevy::prelude::*;

#[derive(Component)]
pub struct Lifetime(pub f32);

pub fn apply_lifetime(
    mut query: Query<(&mut Lifetime, Entity)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut lifetime, entity) in query.iter_mut() {
        lifetime.0 -= time.delta_secs();
        if lifetime.0 <= 0. {
            commands.entity(entity).despawn();
        }
    }
}
