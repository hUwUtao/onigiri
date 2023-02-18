mod utils;

use bevy::prelude::*;
use utils::panic_hook;
use wasm_bindgen::prelude::*;

// use onigiri::setup;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    panic_hook();
    alert("Hello, standalone!");
    entry();
}

pub fn entry() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    registry: Res<AppTypeRegistry>,
    // mut scene_spawner: ResMut<SceneSpawner>,
    mut scenes: ResMut<Assets<DynamicScene>>,
) {
    // Load the scene from a file
    let scene = core::scene::deserialize_dynamic_scene(&registry, "../map00.bin");

    let scene_handle = scenes.add(scene);

    // spawn the scene and get the instance id
    // let instance_id = scene_spawner.spawn_dynamic(scene_handle);

    commands.spawn(DynamicSceneBundle {
        scene: scene_handle.clone(),
        ..Default::default()
    });

    // store the instance id as a resource
    // commands.insert_resource(instance_id);
}
