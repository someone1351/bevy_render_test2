
// use bevy::ecs::prelude::*;

use bevy::app::App;


use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::render::{render_phase::*, ExtractSchedule, Render, RenderApp, RenderSet};



use bevy::render::render_resource::*;


use draws::DrawMesh;
use pipelines::MyUiPipeline;
use shaders::setup_shaders;

use crate::core::core_my::Transparent2d;

// use bevy::transform::components::GlobalTransform;

//use crate::core::core_2d::mypass::MyTransparentUi;

// use bevy::prelude::*;
//render component
pub mod pipelines;
pub mod shaders;
pub mod draws;

pub mod components;
pub mod resources;
pub mod systems;

use resources::*;
// use components::*;
use systems::*;


pub fn render_setup(app: &mut App) {
    let render_app = app.get_sub_app_mut(RenderApp).unwrap();

    render_app
        .init_resource::<MyUiMeta>()
        .init_resource::<MyUiExtractedElements>()
        .init_resource::<MyUiPipeline>()
        .init_resource::<SpecializedRenderPipelines<MyUiPipeline>>()
        // // .init_resource::<DrawFunctions<MyTransparentUi>>()
        // // .init_resource::<ViewSortedRenderPhases<MyTransparentUi>>()
        // .add_render_command::<MyTransparentUi, DrawMesh>()
        .add_render_command::<Transparent2d, DrawMesh>()
        .add_systems(ExtractSchedule,(
            // // extract_camera_view,
            extract_uinodes
        ).chain())
        .add_systems( Render,(
            queue_uinodes.in_set(RenderSet::Queue),
            // // sort_phase_system::<MyTransparentUi>.in_set(RenderSet::PhaseSort),
            prepare_uinodes.in_set(RenderSet::PrepareBindGroups),
        )) ;

    setup_shaders(app);

    // let render_app = app.get_sub_app_mut(RenderApp).unwrap();
    // // graphs::setup_graph2d(render_app);
    // // graphs::setup_graph3d(render_app);
    // graphs::setup_graph(render_app);

}



//camera


// #[derive(Component,Clone)]
// pub struct MyCameraView(pub Entity);

// pub fn extract_camera_view(
//     mut commands: Commands,
//     mut my_render_phases: ResMut<ViewSortedRenderPhases<MyTransparentUi>>,
//     // camera_query: Extract<Query<(RenderEntity, &Camera), With<CameraTest>, >>,
//     camera_query: Extract<Query<(RenderEntity, &Camera), With<Camera2d>, >>,
//     // camera_query: Extract<Query<(RenderEntity, &Camera), With<CameraMyTest>, >>,
//     // mut live_camera_entities: Local<EntityHashSet>,
//     mut live_camera_entities: Local<HashSet<RetainedViewEntity>>,

// ) {
//     //what are MainEntity and RenderEntity?
//     //why does viewport xy not being zero, not render scene at its topleft?
//     //  probably something to do with using Camera2d/3d, maybe should use own

//     const UI_CAMERA_TRANSFORM_OFFSET: f32 = -0.1;
//     const UI_CAMERA_FAR: f32 = 1000.0;

//     live_camera_entities.clear();

//     for (camera_render_entity, camera) in &camera_query {
//         if !camera.is_active {
//             let mut entity_commands = commands.get_entity(camera_render_entity).expect("Camera entity wasn't synced.");
//             entity_commands.remove::<MyCameraView>();
//             continue;
//         }

//         /// The ID of the subview associated with a camera on which UI is to be drawn.
//         ///
//         /// When UI is present, cameras extract to two views: the main 2D/3D one and a
//         /// UI one. The main 2D or 3D camera gets subview 0, and the corresponding UI
//         /// camera gets this subview, 1.
//         const MYUI_CAMERA_SUBVIEW: u32 = 1;
//         let retained_view_entity = RetainedViewEntity::new(camera_render_entity.into(), None, MYUI_CAMERA_SUBVIEW); //needs main entity (not render entity)?

//         if let Some(physical_viewport_rect) = camera.physical_viewport_rect() {
//             let projection_matrix = Mat4::orthographic_rh(
//                 0.0,
//                 physical_viewport_rect.width() as f32,
//                 physical_viewport_rect.height() as f32,
//                 0.0,
//                 0.0,
//                 UI_CAMERA_FAR,
//             );
//             // println!("size {:?} {:?} {:?} {:?}",physical_viewport_rect,physical_viewport_rect.size(),physical_viewport_rect.width(),physical_viewport_rect.height());

//             // println!("projection_matrix {projection_matrix:?}");

//             let view_entity = commands.spawn((
//                 ExtractedView {
//                     clip_from_view: projection_matrix,
//                     world_from_view: GlobalTransform::from_xyz(0.0, 0.0, UI_CAMERA_FAR + UI_CAMERA_TRANSFORM_OFFSET,),
//                     clip_from_world: None,
//                     hdr: camera.hdr,
//                     viewport: UVec4::from((
//                         // physical_viewport_rect.min,
//                         UVec2::ZERO,
//                         physical_viewport_rect.size(),
//                     )),
//                     color_grading: Default::default(),
//                     retained_view_entity, //added
//                 },
//                 TemporaryRenderEntity,
//             )).id();

//             commands.get_entity(camera_render_entity).expect("Camera entity wasn't synced.")
//                 .insert(MyCameraView(view_entity));

//             // println!("camera_render_entity0 {camera_render_entity}");
//             // println!("retained_view_entity0 {retained_view_entity:?}");
//             // println!("view_entity0 {view_entity}");

//             my_render_phases.insert_or_clear(retained_view_entity); //camera_render_entity

//             live_camera_entities.insert(retained_view_entity); //camera_render_entity
//         }
//     }

//     my_render_phases.retain(|camera_entity, _| live_camera_entities.contains(camera_entity));
// }
