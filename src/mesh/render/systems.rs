
// use bevy::ecs::prelude::*;
use std::collections::HashMap;



use bevy::asset::AssetEvent;
use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::image::Image;
use bevy::math::FloatOrd;
use bevy::prelude::{EventReader, Msaa};
use bevy::render::render_asset::RenderAssets;
use bevy::render::texture::GpuImage;
use bevy::render::{render_phase::*, Extract};
use bevy::render::renderer::{RenderDevice, RenderQueue};
use bevy::render::sync_world::{MainEntity, TemporaryRenderEntity};
use bevy::render::view::{ExtractedView, RenderLayers, ViewUniforms};
use bevy::ecs::system::*;


use bevy::render::render_resource::*;


use super::draws::DrawMesh;
use super::dummy_image::create_dummy_image;
use super::pipelines::*;
use super::components::*;
use super::resources::*;

use crate::core::core_my::TransparentMy;
use super::super::TestRenderComponent;


//systems

pub fn dummy_image_setup(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    mesh2d_pipeline: Res<MyUiPipeline>,
    mut image_bind_groups: ResMut<MyUiImageBindGroups>,
    mut init:Local<bool>,
) {

    if *init {
        return;
    }

    *init=true;


    let gpu_image=create_dummy_image(&render_device,&render_queue);

    let bind_group=render_device.create_bind_group(
        "my_ui_material_bind_group",
        &mesh2d_pipeline.image_layout, &[
            BindGroupEntry {binding: 0, resource: BindingResource::TextureView(&gpu_image.texture_view),},
            BindGroupEntry {binding: 1, resource: BindingResource::Sampler(&gpu_image.sampler),},
        ]
    );

    image_bind_groups.values.insert(None, bind_group);
}





pub fn extract_images(
    // mut commands: Commands,
    uinode_query: Extract<Query<(
        Entity,
        &TestRenderComponent,
    )> >,
    mut image_asset_events: Extract<EventReader<AssetEvent<Image>>>,

    render_device: Res<RenderDevice>,
    mesh2d_pipeline: Res<MyUiPipeline>,
    mut image_bind_groups: ResMut<MyUiImageBindGroups>,
    gpu_images: Res<RenderAssets<GpuImage>>,
) {

    for event in image_asset_events.read()
    {
        match event {
            AssetEvent::Removed { id } | AssetEvent::Modified { id } => {
                image_bind_groups.values.remove(&Some(id.clone()));//.unwrap();
            }
            _ =>{}
        }
    }

    for (_entity, test,  ) in uinode_query.iter() {
        let image_id=test.handle.clone().map(|x|x.id());
        // let image_id=test.handle.id();
        //
        if image_bind_groups.values.contains_key(&image_id) {
            continue;
        }

        let Some(image_id)=image_id else {
            continue;
        };

        // let gpu_image=image_id.and_then(|image_id|gpu_images.get(image_id));
        let gpu_image=gpu_images.get(image_id);

        let bind_group=gpu_image.map(|gpu_image|render_device.create_bind_group(
            "my_ui_material_bind_group",
            &mesh2d_pipeline.image_layout, &[
                BindGroupEntry {binding: 0, resource: BindingResource::TextureView(&gpu_image.texture_view),},
                BindGroupEntry {binding: 1, resource: BindingResource::Sampler(&gpu_image.sampler),},
            ]
        ));

        if let Some(bind_group)=bind_group {
            image_bind_groups.values.insert(Some(image_id), bind_group);
        }
    }
}

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
            image: test.handle.clone(),
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

    let default_render_layers = RenderLayers::layer(0);

    for (
        //_view_entiy
        _main_entity,
        extracted_view,
        msaa,
        view_render_layers,
    ) in views.iter() {
        let Some(transparent_phase) = render_phases.get_mut(&extracted_view.retained_view_entity) else {
            //skip transparent phases that aren't for my camera
            continue;
        };
        let view_render_layers=view_render_layers.unwrap_or(&default_render_layers);


        // if let Some(render_layers)=render_layers {
        //     for x in render_layers.iter() {

        //     }

        // }

        let pipeline = pipelines.specialize(&pipeline_cache, &colored_mesh2d_pipeline,MyUiPipelineKey{ msaa_samples: msaa.samples() });

        for element in extracted_elements.elements.iter() {

            let element_render_layers=element.render_layers.as_ref().unwrap_or(&default_render_layers);

            if element_render_layers.intersection(view_render_layers).iter().count()==0 {
                continue;
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


pub fn prepare_views(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
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
}



pub fn prepare_uinodes(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    extracted_elements : Res<MyUiExtractedElements>,
    mut ui_meta: ResMut<MyUiMeta>,
) {


    //
    ui_meta.vertices.clear();

    //
    let mut batches = HashMap::<Entity,MyUiBatch>::new();

    for element in extracted_elements.elements.iter() {
        let mut batch = MyUiBatch { range :0..0, image_handle: element.image.clone() };
        batch.range.start=ui_meta.vertices.len() as u32;

        let pos = vec![
            [element.x,element.y2,0.0], [element.x2,element.y2,0.0], [element.x,element.y,0.0],
            [element.x,element.y,0.0], [element.x2,element.y2,0.0],[element.x2,element.y,0.0],
        ];

        let tex=vec![
            [0.0,1.0],[1.0,1.0],[0.0,0.0],
            [0.0,0.0],[1.0,1.0],[1.0,0.0],
        ];

        let col=element.color.to_linear();
        for i in 0..6 {
            ui_meta.vertices.push(MyUiVertex {
                position: pos[i],
                color : [col.red,col.green,col.blue,col.alpha],
                uv: tex[i],

            });
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
