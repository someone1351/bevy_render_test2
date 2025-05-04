use bevy::ecs::prelude::*;
use bevy::image::BevyDefault;
use bevy::render::renderer::RenderDevice;
use bevy::render::render_resource::*;
use bevy::render::view::ViewUniform;

use super::shaders;

#[derive(Resource,Clone)]
pub struct MyUiPipeline {
    pub view_layout: BindGroupLayout,
}

impl FromWorld for MyUiPipeline {
    fn from_world(world: &mut World) -> Self {
        MyUiPipeline {
             view_layout : create_view_layout(world),
        }
    }
}

impl SpecializedRenderPipeline for MyUiPipeline {
    type Key = MyUiPipelineKey;

    fn specialize(&self, _key: Self::Key) -> RenderPipelineDescriptor {
        let vertex_buffer_layout = VertexBufferLayout::from_vertex_formats(
            VertexStepMode::Vertex,
            vec![
                VertexFormat::Float32x3,// position
                VertexFormat::Float32x4,// color
            ],
        );

        RenderPipelineDescriptor {
            vertex: VertexState {
                shader: shaders::COLORED_MESH2D_SHADER_HANDLE,
                entry_point: "vertex".into(),
                shader_defs: Vec::new(),
                buffers: vec![vertex_buffer_layout],
            },
            fragment: Some(FragmentState {
                shader: shaders::COLORED_MESH2D_SHADER_HANDLE,
                shader_defs: Vec::new(),
                entry_point: "fragment".into(),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            layout: vec![
                self.view_layout.clone(), // Bind group 0 is the view uniform
            ],
            primitive: PrimitiveState {
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
            },
            depth_stencil:None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            label: Some("my_colored_mesh2d_pipeline".into()),
            push_constant_ranges: Vec::new(),
            zero_initialize_workgroup_memory: false,
        }
    }
}

#[derive(PartialEq,Eq, Hash, Clone)]
pub struct MyUiPipelineKey { }

fn create_view_layout(world: &mut World) -> BindGroupLayout {
    let render_device = world.resource::<RenderDevice>();

    render_device.create_bind_group_layout(
        Some("my_mesh2d_view_layout"),
        &[
            BindGroupLayoutEntry { // View
                binding: 0,
                visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: Some(ViewUniform::min_size()),
                },
                count: None,
            },
        ]
    )
}
