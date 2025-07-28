
// use bevy::ecs::prelude::*;
use std::collections::HashMap;



use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::math::FloatOrd;
use bevy::prelude::Msaa;
use bevy::render::{render_phase::*, Extract};
use bevy::render::renderer::{RenderDevice, RenderQueue};
use bevy::render::sync_world::{MainEntity, TemporaryRenderEntity};
use bevy::render::view::{ExtractedView, RenderLayers, ViewUniforms};
use bevy::ecs::system::*;


use bevy::render::render_resource::*;


use super::draws::DrawMesh;
use super::pipelines::*;
use super::components::*;
use super::resources::*;

use crate::core::core_my::TransparentMy;
use super::super::TestRenderComponent;


//systems

pub fn extract_uinodes(
    mut commands: Commands,
    uinode_query: Extract<Query<(
        Entity,
        &TestRenderComponent,
        Option<&RenderLayers>,
    )> >,
    mut extracted_elements : ResMut<MyUiExtractedElements>,
    // default_ui_camera: Extract<MyDefaultUiCamera>,
    // cameras: Extract<Query<(RenderEntity, &MyCameraView), With<CameraTest>, >>,
    // mapping: Extract<Query<RenderEntity>>,
) {

    extracted_elements.elements.clear();


    // let Some(camera_entity) = default_ui_camera.get() else {return;};

    // let Ok(render_camera_entity) = mapping.get(camera_entity) else { return; };

    // let camera_entity=render_camera_entity;

    for (entity, test, render_layers, ) in uinode_query.iter() {

        extracted_elements.elements.push(MyUiExtractedElement{
            entity:commands.spawn((TemporaryRenderEntity,)).id(), //is this needed? instead spawn entity later?
            main_entity:entity.into(),
            // camera_entity,
            x: test.x,
            y: test.y,
            x2: test.x+test.w,
            y2: test.y+test.h,
            color: test.col,
            depth: 0,
            render_layers: render_layers.cloned(),
        });
    }
}

//MainTransparentPass2dNode
pub fn queue_uinodes(
    // transparent_draw_functions: Res<DrawFunctions<MyTransparentUi>>,
    transparent_draw_functions: Res<DrawFunctions<TransparentMy>>,

    colored_mesh2d_pipeline: Res<MyUiPipeline>,
    mut pipelines: ResMut<SpecializedRenderPipelines<MyUiPipeline>>,
    pipeline_cache: Res<PipelineCache>,

    extracted_elements : Res<MyUiExtractedElements>,
    views: Query<(
        // Entity, &ExtractedView
        &MainEntity,
        &ExtractedView,
        &Msaa,
        Option<&RenderLayers>,
    )>,

    // // render_camera_query: Query<(Entity, &MyCameraView),  >,

    // mut render_phases: ResMut<ViewSortedRenderPhases<MyTransparentUi>>,
    mut render_phases: ResMut<ViewSortedRenderPhases<TransparentMy>>,
) {

    let draw_colored_mesh2d = transparent_draw_functions.read().get_id::<DrawMesh>().unwrap();

    // Iterate each view (a camera is a view)

    // let Ok((view_entity, _view)) = views.get_mut(element.camera_entity) else {
    //     continue;
    // };
    // for (view_entity,_view) in views.iter()
    // for (camera_render_entity,_camera_view) in render_camera_query.iter()
    for (
        //_view_entiy
        _main_entity,
        extracted_view,
        msaa,
        render_layers,
    ) in views.iter() {
        let Some(transparent_phase) = render_phases.get_mut(&extracted_view.retained_view_entity) else {
            //skip transparent phases that aren't for my camera
            continue;
        };

        // if let Some(render_layers)=render_layers {
        //     for x in render_layers.iter() {

        //     }

        // }

        let pipeline = pipelines.specialize(&pipeline_cache, &colored_mesh2d_pipeline,MyUiPipelineKey{ msaa_samples: msaa.samples() });

        for element in extracted_elements.elements.iter() {
            if let Some(render_layers)=render_layers {
                let intersects_count=element.render_layers.as_ref().map(|x|x.intersection(render_layers).iter().count()).unwrap_or(0);

                if intersects_count==0 {
                    continue;
                }
            }

            transparent_phase.add(TransparentMy {
                entity: (element.entity,element.main_entity), //what is it used for?
                draw_function: draw_colored_mesh2d,
                pipeline,
                sort_key: FloatOrd(element.depth as f32),
                batch_range: 0..1,
                extra_index: PhaseItemExtraIndex::None,
            });
            // println!("camera_render_entity1 {:?}",extracted_view.retained_view_entity);
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
