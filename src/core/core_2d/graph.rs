


use bevy::render::render_graph::{EmptyNode, RenderGraphApp, ViewNodeRunner};


use bevy::{app::SubApp, render::render_graph::{RenderLabel, RenderSubGraph}};

use super::super::upscaling::UpscalingNode;

use super::passes::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderSubGraph)]
pub struct Core2d;

// pub mod input {
//     pub const VIEW_ENTITY: &str = "view_entity";
// }

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub enum Node2d {
    // MsaaWriteback,
    StartMainPass,
    // MainOpaquePass,
    MainTransparentPass,
    EndMainPass,
    Upscaling,
    // EndMainPassPostProcessing,
}

pub fn setup_graph(render_app:&mut SubApp) {

    render_app
        .add_render_sub_graph(Core2d)
        .add_render_graph_node::<EmptyNode>(Core2d, Node2d::StartMainPass)
        // // .add_render_graph_node::<ViewNodeRunner<MainOpaquePass2dNode>>(Core2d,Node2d::MainOpaquePass,)
        .add_render_graph_node::<ViewNodeRunner<MainTransparentPass2dNode>>(Core2d,Node2d::MainTransparentPass,)
        // .add_render_graph_node::<ViewNodeRunner<MyMainTransparentPass2dNode>>(Core2d,Node2d::MyMainTransparentPass)
        .add_render_graph_node::<EmptyNode>(Core2d, Node2d::EndMainPass)
        // // // .add_render_graph_node::<ViewNodeRunner<TonemappingNode>>(Core2d, Node2d::Tonemapping)
        // // // .add_render_graph_node::<EmptyNode>(Core2d, Node2d::EndMainPassPostProcessing)
        .add_render_graph_node::<ViewNodeRunner<UpscalingNode>>(Core2d, Node2d::Upscaling)
        .add_render_graph_edges(
            Core2d,
            (
                Node2d::StartMainPass,
                // // Node2d::MainOpaquePass,
                Node2d::MainTransparentPass,
                // Node2d::MyMainTransparentPass,
                Node2d::EndMainPass,
                // // // Node2d::Tonemapping,
                // // // Node2d::EndMainPassPostProcessing,
                Node2d::Upscaling,
            ),
        );
}