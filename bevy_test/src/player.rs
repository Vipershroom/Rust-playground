use bevy::prelude::*;
use crate::{GameTexture, WinSize, PLAYER_SIZE, SPRITE_SCALE};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
    }
} 

fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTexture>,
    win_size: Res<WinSize>,
) {
    // add player
    let bottom = -win_size.h / 2.0;
    commands.spawn_bundle(SpriteBundle {
        texture: game_textures.player.clone(),
        transform: Transform {
            translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2.0 * SPRITE_SCALE + 5., 10.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}