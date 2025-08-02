
// use bevy::ecs::prelude::*;

use bevy::app::{App, Plugin};
use bevy::asset::Handle;
use bevy::color::Color;

use bevy::ecs::component::Component;
use bevy::image::Image;
use render::render_setup;



pub mod render;
//mod

//components

#[derive(Component, Debug, Clone,)]
pub struct TestRenderComponent {
    pub col : Color,
    pub x : f32,
    pub y : f32,
    pub w : f32,
    pub h : f32,
    pub handle : Option<Handle<Image>>,
}

//plugin

#[derive(Default)]
pub struct TestRenderPlugin;

impl Plugin for TestRenderPlugin {
    fn build(&self, _app: &mut App) {
        // app
        //     // .register_type::<CameraMyTest>()
        //     // .add_plugins((
        //     //     ExtractComponentPlugin::<CameraMyTest>::default(),
        //     // ))
        //     ;
    }
    fn finish(&self, app: &mut App) {
        render_setup(app);
        // println!("ok!");
    }
}

