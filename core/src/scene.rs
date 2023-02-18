use std::fs::File;

use bevy::{
    prelude::{AppTypeRegistry, Res, World},
    scene::{
        serde::{SceneDeserializer, SceneSerializer},
        DynamicScene,
    },
};
use bincode::Options;
use serde::{de::DeserializeSeed, Serialize};
// use serde::Deserialize;
// use nanoserde::SerBin;

// use crate::math::*;

// #[serde(crate = "serde")]
// #[derive(Deserialize)]
// #[derive(Reflect)]
// struct SceneSerializerWrapper {
//     // delegate to the SceneSerializer
//     // #[serde(serialize_with = "SceneSerializer")]
//     // #[serde(deserialize_with = "SceneDeserializer")]
//     scene_serializer: DynamicScene,
// }

use bincode::config::*;

fn bincode_withcfg() -> WithOtherTrailing<WithOtherIntEncoding<DefaultOptions, FixintEncoding>, AllowTrailing> {
    bincode::options()
        .with_fixint_encoding()
        .allow_trailing_bytes()
}

pub fn serialize_dynamic_scene(world: &World, path: &str) {
    let registry = world.get_resource::<AppTypeRegistry>().unwrap();
    let scene = DynamicScene::from_world(world, registry);
    // let serializable = SceneSerializer::new(&scene, registry);
    // let data = bincode::serialize(&serializable).unwrap();
    SceneSerializer {
        scene: &scene,
        registry: &registry,
    }
    .serialize(&mut bincode::Serializer::new(
        File::create(path).unwrap(),
        bincode_withcfg(),
    ))
    .unwrap();
    // std::fs::write(path, data).unwrap();
}

pub fn deserialize_dynamic_scene(registry: &Res<AppTypeRegistry>, path: &str) -> DynamicScene {
    let reader = File::open(path).unwrap();
    // let mut buffer = Vec::new();
    // reader.read_to_end(&mut buffer).expect("Failed to read file");
    SceneDeserializer {
        type_registry: &registry.read(),
    }
    .deserialize(&mut bincode::Deserializer::with_reader(
        reader,
        bincode_withcfg(),
    ))
    .unwrap()
}
