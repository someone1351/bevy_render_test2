mod camera;
mod passes;
mod graph;
mod systems;

pub use camera::*;
use systems::*;

use graph::setup_graph;
pub use passes::*;


use bevy::render::render_resource::TextureFormat;


use bevy::app::{App, Plugin};
use bevy::ecs::prelude::*;
use bevy::render::{
    extract_component::ExtractComponentPlugin,
    render_phase::{
        sort_phase_system, DrawFunctions, ViewBinnedRenderPhases,
        ViewSortedRenderPhases,
    },
    ExtractSchedule, Render, RenderApp, RenderSet,
};

pub mod phases;

pub use phases::*;


pub const CORE_2D_DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;
pub struct CoreMyPlugin;

impl Plugin for CoreMyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CameraMy>()
            .add_plugins(ExtractComponentPlugin::<CameraMy>::default());

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app
            .init_resource::<DrawFunctions<Opaque2d>>()
            .init_resource::<DrawFunctions<AlphaMask2d>>()
            .init_resource::<DrawFunctions<Transparent2d>>()
            // .init_resource::<DrawFunctions<MyTransparentUi>>() //
            // .init_resource::<ViewSortedRenderPhases<MyTransparentUi>>() //
            .init_resource::<ViewSortedRenderPhases<Transparent2d>>()
            .init_resource::<ViewBinnedRenderPhases<Opaque2d>>()
            .init_resource::<ViewBinnedRenderPhases<AlphaMask2d>>()
            .add_systems(ExtractSchedule, extract_core_2d_camera_phases)
            .add_systems(
                Render,
                (
                    sort_phase_system::<Transparent2d>.in_set(RenderSet::PhaseSort),
                    // sort_phase_system::<MyTransparentUi>.in_set(RenderSet::PhaseSort), //
                    prepare_core_2d_depth_textures.in_set(RenderSet::PrepareResources),
                ),
            );
        setup_graph(render_app);

    }
}
