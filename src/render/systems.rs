
use std::collections::HashMap;

use bevy::ecs::entity::EntityHashSet;
use bevy::ecs::prelude::*;
use bevy::math::{FloatOrd, Mat4, URect, UVec4, };
use bevy::prelude::{Camera, Camera2d, Camera3d, GlobalTransform};
use bevy::render::sync_world::{RenderEntity, TemporaryRenderEntity};
use bevy::window::{Window,PrimaryWindow};

use bevy::render::Extract;
use bevy::render::render_resource::*;
use bevy::render::render_phase::{DrawFunctions, PhaseItemExtraIndex, ViewSortedRenderPhases};
use bevy::render::renderer::*;
use bevy::render::view::*;

use crate::TestComponent;

use super::draw::*;
use super::phase::*;
use super::pipeline::*;
use super::resources::*;
use super::components::*;
use super::camera::*;

pub fn extract_default_ui_camera_view(
    mut commands: Commands,
    mut transparent_render_phases: ResMut<ViewSortedRenderPhases<MyTransparentUi>>,
    query: Extract<Query<(RenderEntity, &Camera), Or<(With<Camera2d>, With<Camera3d>)>>>,
    mut live_entities: Local<EntityHashSet>,
) {
    /// Extracts all UI elements associated with a camera into the render world.

    const UI_CAMERA_TRANSFORM_OFFSET: f32 = -0.1;
    const UI_CAMERA_FAR: f32 = 1000.0;

    live_entities.clear();

    let scale = 1.0;//ui_scale.0.recip();
    for (entity, camera) in &query {
        // ignore inactive cameras
        if !camera.is_active {
            let mut entity_commands = commands.get_entity(entity).expect("Camera entity wasn't synced.");
            entity_commands.remove::<MyDefaultCameraView>();
            continue;
        }

        if let (Some(logical_size),Some(URect {min: physical_origin,..}), Some(physical_size),) = (
            camera.logical_viewport_size(),
            camera.physical_viewport_rect(),
            camera.physical_viewport_size(),
        ) {
            let projection_matrix = Mat4::orthographic_rh(0.0, logical_size.x * scale, logical_size.y * scale, 0.0, 0.0, UI_CAMERA_FAR,);
            let default_camera_view = commands
                .spawn((ExtractedView {
                    clip_from_view: projection_matrix,
                    world_from_view: GlobalTransform::from_xyz(0.0, 0.0, UI_CAMERA_FAR + UI_CAMERA_TRANSFORM_OFFSET,),
                    clip_from_world: None,
                    hdr: camera.hdr,
                    viewport: UVec4::new( physical_origin.x, physical_origin.y, physical_size.x, physical_size.y, ),
                    color_grading: Default::default(),
                },TemporaryRenderEntity)).id();

            let mut entity_commands = commands.get_entity(entity).expect("Camera entity wasn't synced.");
            entity_commands.insert(MyDefaultCameraView(default_camera_view));
            transparent_render_phases.insert_or_clear(entity);

            live_entities.insert(entity);
        }
    }

    transparent_render_phases.retain(|entity, _| live_entities.contains(entity));
}

pub fn extract_uinodes(
    windows: Extract<Query<&Window, With<PrimaryWindow>>>,
    mut commands: Commands,
    uinode_query: Extract<Query<(Entity,&TestComponent,)> >,
    mut extracted_elements : ResMut<MyUiExtractedElements>,
    default_ui_camera: Extract<MyDefaultUiCamera>,
    mapping: Extract<Query<RenderEntity>>,
) {

    extracted_elements.elements.clear();

    let scale_factor = windows
        .get_single()
        .map(|window| window.resolution.scale_factor() as f32)
        .unwrap_or(1.0);

    let _inv_scale_factor = 1. / scale_factor;

    let Some(camera_entity) = default_ui_camera.get() else {return;};

    let Ok(render_camera_entity) = mapping.get(camera_entity) else {
        return;
    };

    let camera_entity=render_camera_entity;

    for (_entity, test, ) in uinode_query.iter() {
        extracted_elements.elements.push(MyUiExtractedElement{
            entity:commands.spawn((TemporaryRenderEntity,)).id(),
            camera_entity,
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
    mut views: Query<(Entity, &ExtractedView)>,
    mut transparent_render_phases: ResMut<ViewSortedRenderPhases<MyTransparentUi>>,
) {

    let draw_colored_mesh2d = transparent_draw_functions.read().get_id::<DrawMesh>().unwrap();
    let pipeline = pipelines.specialize(&pipeline_cache, &colored_mesh2d_pipeline,MyUiPipelineKey{});

    // Iterate each view (a camera is a view)

    for element in extracted_elements.elements.iter() {
        let Ok((view_entity, _view)) = views.get_mut(element.camera_entity) else {
            continue;
        };

        let Some(transparent_phase) = transparent_render_phases.get_mut(&view_entity) else {
            continue;
        };

        transparent_phase.add(MyTransparentUi {
            entity: element.entity,
            draw_function: draw_colored_mesh2d,
            pipeline,
            sort_key: FloatOrd(element.depth as f32),
            batch_range: 0..1,
            extra_index: PhaseItemExtraIndex::NONE,
        });
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
        for entity in extracted_views.iter() {
            let view_bind_group = render_device.create_bind_group(
                "my_mesh2d_view_bind_group",&mesh2d_pipeline.view_layout,&[BindGroupEntry {
                    binding: 0,
                    resource: view_binding.clone(),
                }],);

            commands.entity(entity).insert(MyViewBindGroup { value: view_bind_group, });
        }
    }

    //
    ui_meta.vertices.clear();

    //
    let mut batches = HashMap::<Entity,MyUiBatch>::new();

    for element in extracted_elements.elements.iter() {
        let mut batch = MyUiBatch { range :0..0, };
        batch.range.start=ui_meta.vertices.len() as u32;

        let v_pos = vec![
            [element.x,element.y2,0.0], [element.x2,element.y2,0.0], [element.x,element.y,0.0],
            [element.x,element.y,0.0], [element.x2,element.y2,0.0],[element.x2,element.y,0.0],
        ];

        for i in 0..6 {
            let c=element.color.to_linear();
            ui_meta.vertices.push(MyUiVertex { position: v_pos[i], color : [c.red,c.green,c.blue,c.alpha], });
        }

        batch.range.end=ui_meta.vertices.len() as u32;
        batches.insert(element.entity,batch);
    }


    for (entity, batch) in batches.iter() {
        commands.entity(*entity).insert(batch.clone());
    }

    ui_meta.vertices.write_buffer(&render_device, &render_queue);
}
