
use std::collections::HashSet;

use bevy::prelude::*;

use render_test::TestComponent;
use render_test::TestRenderPlugin as TestRenderPlugin;

fn main() {
    let mut app = App::new();

    app
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin::default())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "render test".into(),
                        resolution: (800.0, 600.0).into(),
                        resizable: true,
                        ..Default::default()
                    }),
                    ..Default::default()
            }),
            TestRenderPlugin,
        ))

        .add_systems(Startup, ( setup_ui, ))
        .add_systems(Update, ( update_input, ))
        ;

    app.run();
}

pub fn setup_ui(
    mut commands: Commands,
) {
    commands.spawn((Camera3d::default(),));
    commands.spawn((
        TestComponent{ col: Color::srgb(1.0,0.2,0.1), x: 0.0, y: 0.0, w: 50.0, h: 50.0, },
    ));
}

fn update_input(
    mut key_events: EventReader<bevy::input::keyboard::KeyboardInput>,
    mut exit: EventWriter<AppExit>,
    mut last_pressed:Local<HashSet<KeyCode>>,
) {
    for ev in key_events.read() {
        //
        if ev.state==bevy::input::ButtonState::Pressed && !last_pressed.contains(&ev.key_code) {
            if ev.key_code==KeyCode::Escape || ev.key_code==KeyCode::F4 {
                exit.send(AppExit::Success);
            }
        }

        //
        if ev.state==bevy::input::ButtonState::Pressed {
            last_pressed.insert(ev.key_code);
        } else if ev.state==bevy::input::ButtonState::Released {
            last_pressed.remove(&ev.key_code);
        }
    }
}
