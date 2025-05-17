use bevy::prelude::*;

#[derive(Component, Debug, Clone,Copy)]

pub struct TestRenderComponent {
    pub col : Color,
    pub x : f32,
    pub y : f32,
    pub w : f32,
    pub h : f32,
}

// #[derive(Component, Debug, Clone,Copy,Default)]

// pub struct CameraTest {
// }