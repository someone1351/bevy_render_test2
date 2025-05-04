use bevy::{color::Color, ecs::prelude::*};

#[derive(Component, Debug, Clone,Copy)]

pub struct TestComponent {
    pub col : Color,
    pub x : f32,
    pub y : f32,
    pub w : f32,
    pub h : f32,
}
