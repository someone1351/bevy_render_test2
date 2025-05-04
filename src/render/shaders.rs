
use bevy::{asset::{load_internal_asset, Handle}, prelude::Shader};


pub const COLORED_MESH2D_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(5312396983770130001);

pub fn setup_shaders(app: &mut bevy::app::App) {
    load_internal_asset!(app, COLORED_MESH2D_SHADER_HANDLE, "mesh2d_col.wgsl", Shader::from_wgsl);
}