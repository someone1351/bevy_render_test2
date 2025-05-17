
use bevy::prelude::*;
use bevy::render::render_resource::{BufferUsages, RawBufferVec};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MyUiVertex {
    pub position: [f32; 3],
    pub color: [f32; 4],//u32,
}

#[derive(Resource)]
pub struct MyUiMeta {
    pub vertices: RawBufferVec<MyUiVertex>,
}

impl Default for MyUiMeta {
    fn default() -> Self {
        Self {
            vertices: RawBufferVec::new(BufferUsages::VERTEX),
        }
    }
}

#[derive(Clone,Debug)]
pub struct MyUiExtractedElement{
    pub x:f32,
    pub y:f32,
    pub x2:f32,
    pub y2:f32,
    pub color : Color,
    pub depth:u32,
    pub entity:Entity,
    // pub camera_entity:Entity,
}

#[derive(Resource,Default,Debug)]
pub struct MyUiExtractedElements {
    pub elements : Vec<MyUiExtractedElement>,
}

