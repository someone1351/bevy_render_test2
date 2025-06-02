
pub mod core_2d;
// pub mod fullscreen_vertex_shader;
// pub mod upscaling;

// pub use skybox::Skybox;

/// The core pipeline prelude.
///
// /// This includes the most common types in this crate, re-exported for your convenience.
// pub mod prelude {
//     #[doc(hidden)]
//     pub use crate::{core_2d::Camera2d,
//         // core_3d::Camera3d
//     };
// }

// use crate::{
//     blit::BlitPlugin,
//     // bloom::BloomPlugin,
//     // contrast_adaptive_sharpening::CasPlugin,
//     core_2d::Core2dPlugin,
//     // core_3d::Core3dPlugin,
//     // deferred::copy_lighting_id::CopyDeferredLightingIdPlugin,
//     // dof::DepthOfFieldPlugin,
//     // experimental::mip_generation::MipGenerationPlugin,
//     fullscreen_vertex_shader::FULLSCREEN_SHADER_HANDLE,
//     // fxaa::FxaaPlugin,
//     // motion_blur::MotionBlurPlugin,
//     // msaa_writeback::MsaaWritebackPlugin,
//     // post_process::PostProcessingPlugin,
//     // prepass::{DeferredPrepass, DepthPrepass, MotionVectorPrepass, NormalPrepass},
//     // smaa::SmaaPlugin,
//     // tonemapping::TonemappingPlugin,
//     upscaling::UpscalingPlugin,
// };
use bevy::app::{App, Plugin};
use bevy::asset::load_internal_asset;
use bevy::render::prelude::Shader;
use core_2d::Core2dPlugin;
// use oit::OrderIndependentTransparencyPlugin;

#[derive(Default)]
pub struct CorePipelinePlugin;

impl Plugin for CorePipelinePlugin {
    fn build(&self, app: &mut App) {
        // load_internal_asset!(
        //     app,
        //     FULLSCREEN_SHADER_HANDLE,
        //     "fullscreen_vertex_shader/fullscreen.wgsl",
        //     Shader::from_wgsl
        // );

        app
            .add_plugins((
                Core2dPlugin,
            ))
            // .add_plugins((
            //     BlitPlugin,
            //     UpscalingPlugin,
            // ))
            ;
    }
}
