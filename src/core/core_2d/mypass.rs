use std::ops::Range;

use bevy::ecs::entity::Entity;
use bevy::ecs::query::QueryItem;
use bevy::ecs::world::World;
use bevy::math::FloatOrd;
use bevy::render::camera::ExtractedCamera;
use bevy::render::diagnostic::RecordDiagnostics;
use bevy::render::render_resource::CachedRenderPipelineId;
use bevy::render::render_resource::CommandEncoderDescriptor;
use bevy::render::render_resource::RenderPassDescriptor;
use bevy::render::renderer::RenderContext;
use bevy::render::render_graph::*;
use bevy::render::render_phase::*;
use bevy::render::sync_world::MainEntity;
use bevy::render::view::ExtractedView;
use bevy::render::view::ViewTarget;
use tracing::error;


//phase


pub struct MyTransparentUi {
    pub sort_key: FloatOrd,
    pub entity: Entity,
    pub pipeline: CachedRenderPipelineId,
    pub draw_function: DrawFunctionId,
    pub batch_range: Range<u32>, /// Range in the vertex buffer of this item
    pub extra_index: PhaseItemExtraIndex,
}

impl PhaseItem for MyTransparentUi {
    #[inline]
    fn entity(&self) -> Entity {
        self.entity
    }

    #[inline]
    fn draw_function(&self) -> DrawFunctionId {
        self.draw_function
    }

    #[inline]
    fn batch_range(&self) -> &Range<u32> {
        &self.batch_range
    }

    #[inline]
    fn batch_range_mut(&mut self) -> &mut Range<u32> {
        &mut self.batch_range
    }

    #[inline]
    fn extra_index(&self) -> PhaseItemExtraIndex {
        self.extra_index.clone()
    }

    #[inline]
    fn batch_range_and_extra_index_mut(&mut self) -> (&mut Range<u32>, &mut PhaseItemExtraIndex) {
        (&mut self.batch_range, &mut self.extra_index)
    }

    fn main_entity(&self) -> MainEntity {
        MainEntity::from(Entity::PLACEHOLDER) //what is main ntity?
    }
}

impl CachedRenderPipelinePhaseItem for MyTransparentUi {
    #[inline]
    fn cached_pipeline(&self) -> CachedRenderPipelineId {
        self.pipeline
    }
}

impl SortedPhaseItem for MyTransparentUi {
    type SortKey = FloatOrd;

    #[inline]
    fn sort_key(&self) -> Self::SortKey {
        self.sort_key
    }

    #[inline]
    fn sort(items: &mut [Self]) {
        items.sort_by_key(|item| item.sort_key());
    }

    fn indexed(&self) -> bool {
        false
    }
}

//pass
#[derive(Default)]
pub struct MyMainTransparentPass2dNode {}

impl ViewNode for MyMainTransparentPass2dNode {
    type ViewQuery = (
        &'static ExtractedCamera,
        &'static ExtractedView,
        &'static ViewTarget,
        // &'static ViewDepthTexture,
    );

    fn run<'w>(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        (camera, view, target,
            // depth
        ): QueryItem<'w, Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        let Some(transparent_phases) =
            world.get_resource::<ViewSortedRenderPhases<MyTransparentUi>>()
        else {
            return Ok(());
        };

        let view_entity = graph.view_entity();
        let Some(transparent_phase) = transparent_phases.get(&view.retained_view_entity) else {
            return Ok(());
        };

        let diagnostics = render_context.diagnostic_recorder();

        let color_attachments = [Some(target.get_color_attachment())];
        // NOTE: For the transparent pass we load the depth buffer. There should be no
        // need to write to it, but store is set to `true` as a workaround for issue #3776,
        // https://github.com/bevyengine/bevy/issues/3776
        // so that wgpu does not clear the depth buffer.
        // As the opaque and alpha mask passes run first, opaque meshes can occlude
        // transparent ones.
        // let depth_stencil_attachment = Some(depth.get_attachment(StoreOp::Store));

        render_context.add_command_buffer_generation_task(move |render_device| {
            // Command encoder setup
            let mut command_encoder =
                render_device.create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("my_main_transparent_pass_2d_command_encoder"),
                });

            // This needs to run at least once to clear the background color, even if there are no items to render
            {
                #[cfg(feature = "trace")]
                let _main_pass_2d = info_span!("my_main_transparent_pass_2d").entered();

                let render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
                    label: Some("my_main_transparent_pass_2d"),
                    color_attachments: &color_attachments,
                    depth_stencil_attachment:None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                let mut render_pass = TrackedRenderPass::new(&render_device, render_pass);

                let pass_span = diagnostics.pass_span(&mut render_pass, "my_main_transparent_pass_2d");

                if let Some(viewport) = camera.viewport.as_ref() {
                    render_pass.set_camera_viewport(viewport);
                }

                if !transparent_phase.items.is_empty() {
                    #[cfg(feature = "trace")]
                    let _transparent_main_pass_2d_span =
                        info_span!("transparent_main_pass_2d").entered();
                    if let Err(err) = transparent_phase.render(&mut render_pass, world, view_entity)
                    {
                        error!(
                            "Error encountered while rendering the transparent 2D phase {err:?}"
                        );
                    }
                }

                pass_span.end(&mut render_pass);
            }

            // WebGL2 quirk: if ending with a render pass with a custom viewport, the viewport isn't
            // reset for the next render pass so add an empty render pass without a custom viewport
            #[cfg(all(feature = "webgl", target_arch = "wasm32", not(feature = "webgpu")))]
            if camera.viewport.is_some() {
                #[cfg(feature = "trace")]
                let _my_reset_viewport_pass_2d = info_span!("my_reset_viewport_pass_2d").entered();
                let pass_descriptor = RenderPassDescriptor {
                    label: Some("my_reset_viewport_pass_2d"),
                    color_attachments: &[Some(target.get_color_attachment())],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                };

                command_encoder.begin_render_pass(&pass_descriptor);
            }

            command_encoder.finish()
        });

        Ok(())
    }
}
