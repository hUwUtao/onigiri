use bevy::{asset::Asset, prelude::*};

fn resolver(path: &str) -> &str {
    path
}

fn load() -> [&'static str; 1] {
    return ["textures/atlas_gabe.png"]
}

pub fn resource<T: Asset>(server: &Res<AssetServer>, file: usize) -> bevy::prelude::Handle<T> {
    server.load(resolver(load()[file]))
}
