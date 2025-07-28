
use bevy::color::Color;
use bevy::ecs::resource::Resource;

use bevy::prelude::Entity;
use bevy::render::render_resource::{BufferUsages, RawBufferVec};
use bevy::render::sync_world::MainEntity;
use bevy::render::view::RenderLayers;



//render resources

#[derive(Clone,Debug)]
pub struct MyUiExtractedElement{
    pub x:f32,
    pub y:f32,
    pub x2:f32,
    pub y2:f32,
    pub color : Color,
    pub depth:u32,
    pub entity:Entity,
    pub main_entity:MainEntity,
    // pub camera_entity:Entity,
    pub render_layers:Option<RenderLayers>,
}

#[derive(Resource,Default,Debug)]
pub struct MyUiExtractedElements {
    pub elements : Vec<MyUiExtractedElement>,
}


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