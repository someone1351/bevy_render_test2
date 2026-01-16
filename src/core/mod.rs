
// #![allow(unused_imports)]

mod camera;
mod passes;
mod graph;
mod systems;
mod phases;
mod plugin;

pub use camera::*;
pub use passes::*;
pub use plugin::*;
pub use phases::*;

use bevy::render::render_resource::TextureFormat;

pub const CORE_2D_DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;
