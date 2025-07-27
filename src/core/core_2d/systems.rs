
use bevy::platform::collections::{HashMap, HashSet};
use bevy::render::{
    batching::gpu_preprocessing::GpuPreprocessingMode,
    view::{ExtractedView, RetainedViewEntity},
};


use bevy::ecs::prelude::*;
use bevy::render::{
    camera::{Camera, ExtractedCamera},
    render_phase::{
        ViewBinnedRenderPhases,
        ViewSortedRenderPhases,
    },
    render_resource::{
        Extent3d, TextureDescriptor, TextureDimension,
        TextureUsages,
    },
    renderer::RenderDevice,
    texture::TextureCache,
    view::{Msaa, ViewDepthTexture},
    Extract,
};

use super::{phases::*, CORE_2D_DEPTH_FORMAT};
use super::camera::*;

pub fn extract_core_2d_camera_phases(
    mut transparent_2d_phases: ResMut<ViewSortedRenderPhases<Transparent2d>>,
    // mut my_render_phases: ResMut<ViewSortedRenderPhases<MyTransparentUi>>,
    mut opaque_2d_phases: ResMut<ViewBinnedRenderPhases<Opaque2d>>,
    mut alpha_mask_2d_phases: ResMut<ViewBinnedRenderPhases<AlphaMask2d>>,
    cameras_2d: Extract<Query<(Entity, &Camera), With<Camera2d>>>,
    mut live_entities: Local<HashSet<RetainedViewEntity>>,
) {
    live_entities.clear();

    for (main_entity, camera) in &cameras_2d {
        if !camera.is_active {
            continue;
        }

        // This is the main 2D camera, so we use the first subview index (0).
        let retained_view_entity = RetainedViewEntity::new(main_entity.into(), None, 0);

        transparent_2d_phases.insert_or_clear(retained_view_entity);
        // my_render_phases.insert_or_clear(retained_view_entity); //
        opaque_2d_phases.prepare_for_new_frame(retained_view_entity, GpuPreprocessingMode::None);
        alpha_mask_2d_phases
            .prepare_for_new_frame(retained_view_entity, GpuPreprocessingMode::None);

        live_entities.insert(retained_view_entity);
    }

    // Clear out all dead views.
    transparent_2d_phases.retain(|camera_entity, _| live_entities.contains(camera_entity));
    // my_render_phases.retain(|camera_entity, _| live_entities.contains(camera_entity));
    opaque_2d_phases.retain(|camera_entity, _| live_entities.contains(camera_entity));
    alpha_mask_2d_phases.retain(|camera_entity, _| live_entities.contains(camera_entity));
}

pub fn prepare_core_2d_depth_textures(
    mut commands: Commands,
    mut texture_cache: ResMut<TextureCache>,
    render_device: Res<RenderDevice>,
    transparent_2d_phases: Res<ViewSortedRenderPhases<Transparent2d>>,
    // my_render_phases: Res<ViewSortedRenderPhases<MyTransparentUi>>,
    opaque_2d_phases: Res<ViewBinnedRenderPhases<Opaque2d>>,
    views_2d: Query<(Entity, &ExtractedCamera, &ExtractedView, &Msaa), (With<Camera2d>,)>,
) {
    let mut textures = <HashMap<_, _>>::default();
    for (view, camera, extracted_view, msaa) in &views_2d {
        if !opaque_2d_phases.contains_key(&extracted_view.retained_view_entity)
            || !transparent_2d_phases.contains_key(&extracted_view.retained_view_entity)
            // || !my_render_phases.contains_key(&extracted_view.retained_view_entity)

        {
            continue;
        };

        let Some(physical_target_size) = camera.physical_target_size else {
            continue;
        };

        let cached_texture = textures
            .entry(camera.target.clone())
            .or_insert_with(|| {
                // The size of the depth texture
                let size = Extent3d {
                    depth_or_array_layers: 1,
                    width: physical_target_size.x,
                    height: physical_target_size.y,
                };

                let descriptor = TextureDescriptor {
                    label: Some("view_depth_texture"),
                    size,
                    mip_level_count: 1,
                    sample_count: msaa.samples(),
                    dimension: TextureDimension::D2,
                    format: CORE_2D_DEPTH_FORMAT,
                    usage: TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                };

                texture_cache.get(&render_device, descriptor)
            })
            .clone();

        commands
            .entity(view)
            .insert(ViewDepthTexture::new(cached_texture, Some(0.0)));
    }
}
