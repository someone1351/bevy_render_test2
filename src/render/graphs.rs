
use super::pass::*;

use bevy::core_pipeline::{core_2d::graph::Node2d,core_3d::graph::Node3d};
use bevy::prelude::*;
use bevy::render::render_graph::*;
use bevy::render::RenderApp;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct MyUiPassNodeLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderSubGraph)]
struct MyUiSubGraphLabel;

pub fn setup_graph2d(app: &mut App) {
    let render_app = app.get_sub_app_mut(RenderApp).unwrap();
    let mut ui_graph_2d = RenderGraph::default();
    ui_graph_2d.add_node( MyUiPassNodeLabel, MyUiPassNode::new(render_app.world_mut()));
    let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();

    if let Some(graph_2d) = render_graph.get_sub_graph_mut(bevy::core_pipeline::core_2d::graph::Core2d) {
        graph_2d.add_sub_graph(MyUiSubGraphLabel,ui_graph_2d);
        graph_2d.add_node(MyUiPassNodeLabel,RunGraphOnViewNode::new(MyUiSubGraphLabel),);
        graph_2d.add_node_edge(Node2d::EndMainPass, MyUiPassNodeLabel);
    }
}

pub fn setup_graph3d(app: &mut App) {
    let render_app = app.get_sub_app_mut(RenderApp).unwrap();
    let mut ui_graph_3d = RenderGraph::default();
    ui_graph_3d.add_node( MyUiPassNodeLabel, MyUiPassNode::new(render_app.world_mut()));
    let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();

    if let Some(graph_3d) = render_graph.get_sub_graph_mut(bevy::core_pipeline::core_3d::graph::Core3d) {
        graph_3d.add_sub_graph(MyUiSubGraphLabel , ui_graph_3d);
        graph_3d.add_node(MyUiPassNodeLabel,RunGraphOnViewNode::new(MyUiSubGraphLabel),);
        graph_3d.add_node_edge(Node3d::EndMainPass, MyUiPassNodeLabel);
    }
}
