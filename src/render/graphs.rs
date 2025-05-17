
use bevy::core_pipeline::{core_2d::graph::Node2d,core_3d::graph::Node3d};
use bevy::render::render_graph::*;
use bevy::prelude::*;

use super::pass::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct MyUiPassNodeLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderSubGraph)]
struct MyUiSubGraphLabel;

fn get_ui_graph(render_app: &mut SubApp) -> RenderGraph {
    let ui_pass_node = MyUiPassNode::new(render_app.world_mut());

    // let mut ui_graph_2d = RenderGraph::default();
    // let mut ui_graph_3d = RenderGraph::default();
    let mut ui_graph = RenderGraph::default();
    ui_graph.add_node(MyUiPassNodeLabel, ui_pass_node);

    // ui_graph_2d.add_node( MyUiPassNodeLabel, MyUiPassNode::new(render_app.world_mut()));
    // ui_graph_3d.add_node( MyUiPassNodeLabel, MyUiPassNode::new(render_app.world_mut()));
    ui_graph
}

pub fn setup_graph2d(render_app: &mut SubApp) {
    let ui_graph_2d = get_ui_graph(render_app);
    let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();

    if let Some(graph_2d) = render_graph.get_sub_graph_mut(bevy::core_pipeline::core_2d::graph::Core2d) {
        graph_2d.add_sub_graph(MyUiSubGraphLabel,ui_graph_2d);
        graph_2d.add_node(MyUiPassNodeLabel,RunGraphOnViewNode::new(MyUiSubGraphLabel),);
        graph_2d.add_node_edge(Node2d::EndMainPass, MyUiPassNodeLabel);
    }
}

pub fn setup_graph3d(render_app: &mut SubApp) {
    let ui_graph_3d = get_ui_graph(render_app);
    let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();

    if let Some(graph_3d) = render_graph.get_sub_graph_mut(bevy::core_pipeline::core_3d::graph::Core3d) {
        graph_3d.add_sub_graph(MyUiSubGraphLabel , ui_graph_3d);
        graph_3d.add_node(MyUiPassNodeLabel,RunGraphOnViewNode::new(MyUiSubGraphLabel),);
        graph_3d.add_node_edge(Node3d::EndMainPass, MyUiPassNodeLabel);
    }
}
