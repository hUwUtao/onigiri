use bevy::{prelude::*};

// mod lib;
use onigiri::*;
// #[cfg(debug_assertions)]
use bevy_editor_pls::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_startup_system(setup)
        .add_system(animate_atlas)
        // .add_system(serialize_scene)
        .add_plugin(EditorPlugin)
        .run();

    // #[cfg(not(debug_assertions))]
    // app.run();
}

fn animate_atlas(
    time: Res<Time>,
    atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimateAtlas,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, handler) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = atlases.get(handler).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}