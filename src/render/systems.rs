
use std::collections::{HashMap, HashSet};

use bevy::ecs::entity::EntityHashSet;
use bevy::ecs::prelude::*;
use bevy::math::{FloatOrd, Mat4, UVec2, UVec4 };
use bevy::prelude::{Camera, Camera2d, Camera3d, GlobalTransform};
use bevy::window::{Window,PrimaryWindow};

use bevy::render::Extract;
use bevy::render::render_resource::*;
use bevy::render::render_phase::{DrawFunctions, PhaseItemExtraIndex, ViewSortedRenderPhases};
use bevy::render::sync_world::{RenderEntity, TemporaryRenderEntity};
use bevy::render::renderer::*;
use bevy::render::view::*;

use super::draw::*;
use super::phase::*;
use super::pipeline::*;
use super::resources::*;
use super::components::*;
use super::camera::*;
use super::super::components::*;

pub fn extract_camera_view(
    mut commands: Commands,
    mut render_phases: ResMut<ViewSortedRenderPhases<MyTransparentUi>>,
    // camera_query: Extract<Query<(RenderEntity, &Camera), With<CameraTest>, >>,
    camera_query: Extract<Query<(RenderEntity, &Camera), Or<(With<Camera2d>,With<Camera3d>)>, >>,
    // mut live_camera_entities: Local<EntityHashSet>,
    mut live_camera_entities: Local<HashSet<RetainedViewEntity>>,

) {
    //what are MainEntity and RenderEntity?
    //why does viewport xy not being zero, not render scene at its topleft?
    //  probably something to do with using Camera2d/3d, maybe should use own

    const UI_CAMERA_TRANSFORM_OFFSET: f32 = -0.1;
    const UI_CAMERA_FAR: f32 = 1000.0;

    live_camera_entities.clear();

    for (camera_render_entity, camera) in &camera_query {
        if !camera.is_active {
            let mut entity_commands = commands.get_entity(camera_render_entity).expect("Camera entity wasn't synced.");
            entity_commands.remove::<MyCameraView>();
            continue;
        }

        /// The ID of the subview associated with a camera on which UI is to be drawn.
        ///
        /// When UI is present, cameras extract to two views: the main 2D/3D one and a
        /// UI one. The main 2D or 3D camera gets subview 0, and the corresponding UI
        /// camera gets this subview, 1.
        const MYUI_CAMERA_SUBVIEW: u32 = 1;
        let retained_view_entity = RetainedViewEntity::new(camera_render_entity.into(), None, MYUI_CAMERA_SUBVIEW); //needs main entity (not render entity)?

        if let Some(physical_viewport_rect) = camera.physical_viewport_rect() {
            let projection_matrix = Mat4::orthographic_rh(
                0.0,
                physical_viewport_rect.width() as f32,
                physical_viewport_rect.height() as f32,
                0.0,
                0.0,
                UI_CAMERA_FAR,
            );
            println!("size {:?} {:?} {:?} {:?}",physical_viewport_rect,physical_viewport_rect.size(),physical_viewport_rect.width(),physical_viewport_rect.height());

            println!("projection_matrix {projection_matrix:?}");

            let view_entity = commands.spawn((
                ExtractedView {
                    clip_from_view: projection_matrix,
                    world_from_view: GlobalTransform::from_xyz(0.0, 0.0, UI_CAMERA_FAR + UI_CAMERA_TRANSFORM_OFFSET,),
                    clip_from_world: None,
                    hdr: camera.hdr,
                    viewport: UVec4::from((
                        // physical_viewport_rect.min,
                        UVec2::ZERO,
                        physical_viewport_rect.size(),
                    )),
                    color_grading: Default::default(),
                    retained_view_entity, //added
                },
                TemporaryRenderEntity,
            )).id();

            commands.get_entity(camera_render_entity).expect("Camera entity wasn't synced.")
                .insert(MyCameraView(view_entity));

            println!("camera_render_entity0 {camera_render_entity}");
            render_phases.insert_or_clear(retained_view_entity); //camera_render_entity

            live_camera_entities.insert(retained_view_entity); //camera_render_entity
        }
    }

    render_phases.retain(|camera_entity, _| live_camera_entities.contains(camera_entity));
}

pub fn extract_uinodes(
    mut commands: Commands,
    uinode_query: Extract<Query<(Entity,&TestRenderComponent,)> >,
    mut extracted_elements : ResMut<MyUiExtractedElements>,
    // default_ui_camera: Extract<MyDefaultUiCamera>,
    // cameras: Extract<Query<(RenderEntity, &MyCameraView), With<CameraTest>, >>,
    // mapping: Extract<Query<RenderEntity>>,
) {

    extracted_elements.elements.clear();


    // let Some(camera_entity) = default_ui_camera.get() else {return;};

    // let Ok(render_camera_entity) = mapping.get(camera_entity) else { return; };

    // let camera_entity=render_camera_entity;

    for (_entity, test, ) in uinode_query.iter() {
        extracted_elements.elements.push(MyUiExtractedElement{
            entity:commands.spawn((TemporaryRenderEntity,)).id(), //is this needed? instead spawn entity later?
            // camera_entity,
            x: test.x,
            y: test.y,
            x2: test.x+test.w,
            y2: test.y+test.h,
            color: test.col,
            depth: 0,
        });
    }
}

pub fn queue_uinodes(
    transparent_draw_functions: Res<DrawFunctions<MyTransparentUi>>,

    colored_mesh2d_pipeline: Res<MyUiPipeline>,
    mut pipelines: ResMut<SpecializedRenderPipelines<MyUiPipeline>>,
    pipeline_cache: Res<PipelineCache>,

    extracted_elements : Res<MyUiExtractedElements>,
    views: Query<(Entity, &ExtractedView)>,

    render_camera_query: Query<(Entity, &MyCameraView),  >,

    mut render_phases: ResMut<ViewSortedRenderPhases<MyTransparentUi>>,
) {

    let draw_colored_mesh2d = transparent_draw_functions.read().get_id::<DrawMesh>().unwrap();
    let pipeline = pipelines.specialize(&pipeline_cache, &colored_mesh2d_pipeline,MyUiPipelineKey{});

    // Iterate each view (a camera is a view)

    for element in extracted_elements.elements.iter() {
        // let Ok((view_entity, _view)) = views.get_mut(element.camera_entity) else {
        //     continue;
        // };
        // for (view_entity,_view) in views.iter()
        // for (camera_render_entity,_camera_view) in render_camera_query.iter()
        for (view_entiy,extracted_view) in views.iter()
        {
            let Some(transparent_phase) = render_phases.get_mut(&extracted_view.retained_view_entity) else {continue;};

            transparent_phase.add(MyTransparentUi {
                entity: element.entity, //what is it used for?
                draw_function: draw_colored_mesh2d,
                pipeline,
                sort_key: FloatOrd(element.depth as f32),
                batch_range: 0..1,
                extra_index: PhaseItemExtraIndex::None,
            });

            println!("camera_render_entity1 {:?}",extracted_view.retained_view_entity);
        }

    }
}

pub fn prepare_uinodes(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    extracted_elements : Res<MyUiExtractedElements>,
    mut ui_meta: ResMut<MyUiMeta>,
    mesh2d_pipeline: Res<MyUiPipeline>,
    view_uniforms: Res<ViewUniforms>,
    extracted_views: Query<Entity, With<ExtractedView>>,
) {

    if let Some(view_binding) = view_uniforms.uniforms.binding() {
        for view_entity in extracted_views.iter() {
            let view_bind_group = render_device.create_bind_group(
                "my_mesh2d_view_bind_group",&mesh2d_pipeline.view_layout,&[BindGroupEntry {
                    binding: 0,
                    resource: view_binding.clone(),
                }],);

            commands.entity(view_entity).insert(MyViewBindGroup { value: view_bind_group, });
        }
    }

    //
    ui_meta.vertices.clear();

    //
    let mut batches = HashMap::<Entity,MyUiBatch>::new();

    for element in extracted_elements.elements.iter() {
        let mut batch = MyUiBatch { range :0..0, };
        batch.range.start=ui_meta.vertices.len() as u32;

        let pos = vec![
            [element.x,element.y2,0.0], [element.x2,element.y2,0.0], [element.x,element.y,0.0],
            [element.x,element.y,0.0], [element.x2,element.y2,0.0],[element.x2,element.y,0.0],
        ];

        let col=element.color.to_linear();
        for i in 0..6 {
            ui_meta.vertices.push(MyUiVertex { position: pos[i], color : [col.red,col.green,col.blue,col.alpha], });
        }

        batch.range.end=ui_meta.vertices.len() as u32;
        batches.insert(element.entity,batch);
    }


    for (entity, batch) in batches.iter() {
        commands.entity(*entity).insert(batch.clone());
        // commands.spawn(batch.clone());
    }

    ui_meta.vertices.write_buffer(&render_device, &render_queue);
}
/*

    //
    for element in extracted_elements.elements.iter() {
        let pos = vec![
            [element.x,element.y2,0.0], [element.x2,element.y2,0.0], [element.x,element.y,0.0],
            [element.x,element.y,0.0], [element.x2,element.y2,0.0],[element.x2,element.y,0.0],
        ];

        let col=element.color.to_linear();

        for i in 0..6 {
            ui_meta.vertices.push(MyUiVertex { position: pos[i], color : [col.red,col.green,col.blue,col.alpha], });
        }
    }


    if !ui_meta.vertices.is_empty() {
        commands.spawn(MyUiBatch { range :0..(ui_meta.vertices.len() as u32), });
    }

    ui_meta.vertices.write_buffer(&render_device, &render_queue);
*/