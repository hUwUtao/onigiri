use bevy::{prelude::*};

mod resource;

#[derive(Component, Deref, DerefMut)]
pub struct AnimateAtlas(Timer);

// fn serialize_scene(world: &mut World) {
//     let type_registry = world.resource::<AppTypeRegistry>();
//     let scene = DynamicScene::from_world(&world, type_registry);
//     let ser = SceneSerializer::new(&scene, type_registry);
// }

pub fn setup(
    // mut registry: ResMut<InspectableRegistry>,
    mut commands: Commands,
    server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {

    // registry.register::<USceneFormat>();

    let texture_handle = resource::resource(&server, 0);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_handle = atlases.add(texture_atlas);
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        AnimateAtlas(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
