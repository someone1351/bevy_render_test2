
use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::render::render_phase::*;
use bevy::render::{Render,RenderApp, RenderSet, ExtractSchedule,};

pub mod systems;
pub mod pipeline;
pub mod phase;
pub mod draw;
pub mod pass;
pub mod resources;
pub mod components;
pub mod camera;
pub mod graphs;
pub mod shaders;

use systems::*;
use pipeline::*;
use phase::*;
use draw::*;
use resources::*;
// use components::*;
// use camera::*;


pub fn setup(app: &mut bevy::app::App) {
    let render_app = app.get_sub_app_mut(RenderApp).unwrap();

    render_app
        .init_resource::<MyUiMeta>()
        .init_resource::<MyUiExtractedElements>()
        .init_resource::<MyUiPipeline>()
        .init_resource::<SpecializedRenderPipelines<MyUiPipeline>>()
        .init_resource::<DrawFunctions<MyTransparentUi>>()
        .init_resource::<ViewSortedRenderPhases<MyTransparentUi>>()
        .add_render_command::<MyTransparentUi, DrawMesh>()
        .add_systems(ExtractSchedule,(
            extract_default_ui_camera_view,
            extract_uinodes
        ).chain())
        .add_systems( Render,(
            queue_uinodes.in_set(RenderSet::Queue),
            sort_phase_system::<MyTransparentUi>.in_set(RenderSet::PhaseSort),
            prepare_uinodes.in_set(RenderSet::PrepareBindGroups),
        )) ;

    shaders::setup_shaders(app);
    graphs::setup_graph2d(app);
    graphs::setup_graph3d(app);
}

