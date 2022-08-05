use bevy::{prelude::*};
use player::PlayerPlugin;

mod player;

// region:    --- Asset Constants

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);

const SPRITE_SCALE: f32 = 0.5;

// endregion: --- Asset Constants 

// region: --- Asset Constants

// region: --- Resources
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

pub struct GameTexture {
    player: Handle<Image>,
}
// endregion: --- Resources

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource(WindowDescriptor {
        title: "Bevy test".to_string(),
        width: 598.0,
        height: 680.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(PlayerPlugin)
    .add_startup_system(setup_system)
    .run()
}

fn setup_system(mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Capture window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    // position window 
    window.set_position(IVec2::new(0, 0));

    // add WinSize resource
    let win_size = WinSize {w: win_w, h: win_h,};
    commands.insert_resource(win_size);

    // add GameTextures resource
    let game_textures = GameTexture {
        player: asset_server.load(PLAYER_SPRITE)
    };
    commands.insert_resource(game_textures)
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

