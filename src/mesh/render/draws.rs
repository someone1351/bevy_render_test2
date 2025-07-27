

use bevy::render::render_phase::*;
use bevy::render::view::ViewUniformOffset;
use bevy::ecs::system::{lifetimeless::*, *};

use bevy::render::render_phase::SetItemPipeline;

// use super::{MyUiBatch, MyUiMeta, MyViewBindGroup};
use super::components::*;
use super::resources::*;


//render phase


pub struct SetViewBindGroup<const I: usize>;

impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetViewBindGroup<I> {
    type Param = ();//SQuery<(Read<ViewUniformOffset>, Read<MyViewBindGroup>)>;
    type ViewQuery = (Read<ViewUniformOffset>, Read<MyViewBindGroup>);//Read<ViewUniformOffset>;
    type ItemQuery = ();
    #[inline]
    fn render<'w>(
        _item: &P,
        (view_uniform,mesh2d_view_bind_group): (&'w ViewUniformOffset,&'w MyViewBindGroup), //view
        _entity: Option<()>, //item
        _view_query: SystemParamItem<'w, '_, Self::Param>, //param
        pass: &mut TrackedRenderPass<'w>,

    ) -> RenderCommandResult {
        pass.set_bind_group(
            I,
            &mesh2d_view_bind_group.value,
            &[view_uniform.offset]
        );
        RenderCommandResult::Success
    }
}

pub struct SetDrawBuf;

impl<P: PhaseItem> RenderCommand<P> for SetDrawBuf {
    type Param = SRes<MyUiMeta> ;//(, SQuery<>);
    type ViewQuery = ();
    type ItemQuery = Read<MyUiBatch>;

    #[inline]
    fn render<'w>(
        _item: &P,
        _view : (), //view
        batch : Option<&'w MyUiBatch>, //item
        ui_meta : SystemParamItem<'w, '_, Self::Param>, //param
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let Some(batch) = batch else {return RenderCommandResult::Failure("...");};
        pass.set_vertex_buffer(0, ui_meta.into_inner().vertices.buffer().unwrap().slice(..));
        pass.draw(batch.range.clone(), 0..1);
        RenderCommandResult::Success
    }
}

pub type DrawMesh = (
    SetItemPipeline,
    SetViewBindGroup<0>,
    SetDrawBuf,
);
