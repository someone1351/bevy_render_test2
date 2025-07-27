
pub mod core_2d;
pub mod upscaling;

use bevy::app::{App, Plugin};

use core_2d::Core2dPlugin;
use upscaling::UpscalingPlugin;

#[derive(Default)]
pub struct CorePipelinePlugin;

impl Plugin for CorePipelinePlugin {
    fn build(&self, app: &mut App) {


        app
            .add_plugins((
                Core2dPlugin,
                UpscalingPlugin,
            ))
            ;
    }
}
