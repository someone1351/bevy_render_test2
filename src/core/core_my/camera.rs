// use crate::{
//     core_2d::graph::Core2d,
//     // tonemapping::{DebandDither, Tonemapping},
// };
use bevy::ecs::prelude::*;
use bevy::reflect::{std_traits::ReflectDefault, Reflect};
use bevy::render::{
    camera::{Camera, CameraProjection, CameraRenderGraph, OrthographicProjection, Projection},
    extract_component::ExtractComponent,
    primitives::Frustum,
};
use bevy::transform::prelude::{GlobalTransform, Transform};

use super::graph::CoreMy;

/// A 2D camera component. Enables the 2D render graph for a [`Camera`].
#[derive(Component, Default, Reflect, Clone, ExtractComponent)]
#[extract_component_filter(With<Camera>)]
#[reflect(Component, Default, Clone)]
#[require(
    Camera,
    // DebandDither,
    CameraRenderGraph::new(CoreMy),
    Projection::Orthographic(OrthographicProjection::default_2d()),
    Frustum = OrthographicProjection::default_2d().compute_frustum(&GlobalTransform::from(Transform::default())),
    // Tonemapping::None,
)]
pub struct CameraMy;
