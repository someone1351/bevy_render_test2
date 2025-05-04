
use bevy::prelude::*;

use super::render;

#[derive(Default)]
pub struct UiDisplayPlugin;

impl Plugin for UiDisplayPlugin {
    fn build(&self, _app: &mut App) {

    }
    fn finish(&self, app: &mut App) {
        render::setup(app);
    }
}