
use bevy::prelude::*;

use super::render;

#[derive(Default)]
pub struct TestRenderPlugin;

impl Plugin for TestRenderPlugin {
    fn build(&self, _app: &mut App) {

    }
    fn finish(&self, app: &mut App) {
        render::setup(app);
    }
}