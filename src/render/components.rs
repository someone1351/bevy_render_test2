
use bevy::reflect::Reflect;
use bevy::render::render_resource::BindGroup;
use bevy::ecs::prelude::*;
use core::ops::Range;

#[derive(Component, Default, Debug, Clone)]
pub struct MyUiBatch {
    pub range: Range<u32>,
}

#[derive(Component)]
pub struct MyViewBindGroup {
    pub value: BindGroup,
}

#[derive(Component)]
pub struct MyDefaultCameraView(pub Entity);

#[derive(Component, Clone, Debug, Reflect, Eq, PartialEq)]
pub struct MyTargetCamera(pub Entity);
