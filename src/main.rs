
use std::collections::HashSet;

use bevy::{asset::RenderAssetUsages, camera::{visibility::RenderLayers, Viewport}, prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages}, window::WindowResolution};
use render_test2::{core::CorePipelinePlugin, mesh::{TestRenderComponent, TestRenderPlugin}};
// use bevy::render::view::RenderLayers;
// use render_test2::{render::camera::CameraMyTest, TestRenderComponent, TestRenderPlugin};
// use bevy::color::palettes::basic::SILVER;

/*

* get_color_attachment returns a shared color tex?
** is it automatically created and clear col applied?
*** same with depth buf?

*/
fn main() {
    let mut app = App::new();

    app
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin::default())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "render test".into(),
                        resolution: WindowResolution::new(1100, 600)
                        // .sc
                            .with_scale_factor_override(1.0)
                            ,
                        resizable: true,
                        ..Default::default()
                    }),
                    ..Default::default()
            }),
            CorePipelinePlugin,
            TestRenderPlugin,
        ))

        .add_systems(Startup, (
            setup_ui,
            setup_2d,
            // setup_3d,
        ))
        .add_systems(Update, ( update_input, ))
        ;

    app.run();
}

pub fn setup_ui(
    // mut commands: Commands,
    window: Single<&mut Window>
) {
    println!("Scale is {} {} {}",
        window.scale_factor(),
        window.resolution.scale_factor(),
        window.resolution.base_scale_factor()
    );

    // commands.spawn((
    //     CameraMyTest{},
    //     // Camera2d::default(),
    //     // // Camera3d::default(),
    //     // // CameraTest::default(),
    //     // // // RenderLayers::from_layers(&[0]),
    //     // Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    //     Camera {
    //         order: 0,
    //         clear_color: ClearColorConfig::Custom(Color::srgb(0.2, 0.2, 0.6)),
    //         // viewport: Some(Viewport {
    //         //     physical_position: UVec2::new(0, 0),
    //         //     physical_size: UVec2::new(500, 500), //not size, actually x2,y2
    //         //     ..Default::default()
    //         // }),
    //         ..Default::default()
    //     },
    // ));

    // // commands.spawn((
    // //     Camera3d::default(),
    // //     // CameraTest::default(),
    // //     // RenderLayers::from_layers(&[0]),
    // //     Camera {
    // //         order: 1,
    // //         clear_color: ClearColorConfig::Custom(Color::srgb(0.2, 0.6, 0.2)),
    // //         viewport: Some(Viewport {
    // //             physical_position: UVec2::new(600, 0),
    // //             physical_size: UVec2::new(500, 500), //not size, actually x2,y2
    // //             // depth: 0.0..1.0,
    // //             ..Default::default()
    // //         }),
    // //         ..Default::default()
    // //     },
    // // ));

    // commands.spawn((
    //     TestRenderComponent{ col: Color::srgb(1.0,0.2,0.6), x: 0.0, y: 0.0, w: 50.0, h: 50.0, },
    //     // RenderLayers::layer(0),
    // ));
}

pub fn setup_2d(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 500,
        height: 500,
        ..default()
    };
    let mut image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[255,255,255,255],
        // TextureFormat::Bgra8UnormSrgb, //Rgba8Unorm
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    // You need to set these texture usage flags in order to use the image as a render target
    image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

    let _image_handle = images.add(image);

    commands.spawn((
        render_test2::core::core_my::CameraMy::default(),
        // Projection::Orthographic(OrthographicProjection::default_2d()),

        Camera {
            // target: image_handle.clone().into(),
            clear_color: Color::WHITE.into(),
            order: 0,
            // clear_color: ClearColorConfig::Custom(Color::srgb(0.2, 0.1, 0.5)),
            viewport: Some(Viewport {
                physical_position: UVec2::new(0, 0),
                physical_size: UVec2::new(500, 500),
                ..Default::default()
            }),
            ..Default::default()
        },
        RenderLayers::layer(0),
        // Transform::from_xyz( 0.0, 0.0, 999.0, ),
    ));
    commands.spawn((
        render_test2::core::core_my::CameraMy::default(),
        // Projection::Orthographic(OrthographicProjection::default_2d()),
        Camera {
            order: 1,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.2, 0.7, 0.1)),
            // clear_color: Color::WHITE.into(),
            viewport: Some(Viewport {
                physical_position: UVec2::new(500, 0),
                physical_size: UVec2::new(500, 500),
                ..Default::default()
            }),
            ..Default::default()
        },
        RenderLayers::layer(1),
    ));

    //
    commands.spawn((
        TestRenderComponent{
            col: Color::srgb(1.0,0.0,0.0),
            // col: Color::srgb(0.0,0.0,1.0),
            // col:Color::WHITE.into(),
            x: 0.0, y: 0.0, w: 50.0, h: 50.0,
            // handle:Some(asset_server.load("bevy_logo_dark_big.png")),
            handle:None,
        },
        // RenderLayers::layer(1),
        RenderLayers::from_layers(&[0]),
        Transform::from_xyz( 0.0, 0.0, 0.0, ),
    ));
    commands.spawn((
        TestRenderComponent{
            // col: Color::srgb(1.0,0.0,0.0),
            col: Color::srgb(0.0,1.0,0.0),
            // col:Color::WHITE.into(),
            x: 50.0, y: 50.0, w: 50.0, h: 50.0,
            // handle:Some(asset_server.load("bevy_logo_dark_big.png")),
            handle:None,
        },
        // RenderLayers::layer(1),
        RenderLayers::from_layers(&[0]),
        Transform::from_xyz( 0.0, 0.0, 0.0, ),
    ));
    commands.spawn((
        TestRenderComponent{
            // col: Color::srgb(1.0,0.0,0.0),
            col: Color::srgb(0.0,0.0,1.0),
            // col:Color::WHITE.into(),
            x: 100.0, y: 100.0, w: 50.0, h: 50.0,
            // handle:Some(asset_server.load("bevy_logo_dark_big.png")),
            handle:None,
        },
        // RenderLayers::layer(1),
        RenderLayers::from_layers(&[0]),
        Transform::from_xyz( 0.0, 0.0, 0.0, ),
    ));
    // commands.spawn((
    //     TestRenderComponent{
    //         // col: Color::srgb(1.0,0.0,0.0),
    //         col: Color::srgb(0.0,1.0,1.0),
    //         // col:Color::WHITE.into(),
    //         x: 450.0, y: 450.0, w: 50.0, h: 50.0,
    //         // handle:Some(asset_server.load("bevy_logo_dark_big.png")),
    //         handle:None,
    //     },
    //     // RenderLayers::layer(1),
    //     RenderLayers::from_layers(&[0]),
    //     Transform::from_xyz( 0.0, 0.0, 0.0, ),
    // ));
    // commands.spawn((
    //     TestRenderComponent{
    //         // col: Color::srgb(1.0,0.0,0.0),
    //         col: Color::srgb(1.0,1.0,0.0),
    //         // col:Color::WHITE.into(),
    //         x: 0.0, y: 230.0, w: 50.0, h: 50.0,
    //         // handle:Some(asset_server.load("bevy_logo_dark_big.png")),
    //         handle:None,
    //     },
    //     // RenderLayers::layer(1),
    //     RenderLayers::from_layers(&[0]),
    //     Transform::from_xyz( 0.0, 0.0, 0.0, ),
    // ));

    //


    commands.spawn((
        TestRenderComponent{
            // col: Color::srgb(1.0,0.2,0.6),
            x: 0.0, y: 0.0, w: 400.0, h: 400.0,
            col:Color::WHITE.into(),
            // handle:Some(image_handle),
            handle:Some(asset_server.load("bevy_logo_dark_big.png")),
            // handle:None,
        },
        // RenderLayers::layer(1),
        RenderLayers::from_layers(&[1]),
        Transform::from_xyz( 0.0, 0.0, 0.0, ),
    ));
    // let num_shapes = 5;

    // let shape=meshes.add(Rectangle::new(50.0, 50.0));
    // const X_EXTENT: f32 = 300.;

    // for i in 0..num_shapes {
    //     let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

    //     // commands.spawn((
    //     //     Mesh2d(shape.clone()),
    //     //     MeshMaterial2d(materials.add(color)),
    //     //     Transform::from_xyz(
    //     //         -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
    //     //         0.0,
    //     //         0.0,
    //     //     ),
    //     // ));
    // }
}

// pub fn setup_3d(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     commands.spawn((
//         Camera3d::default(),
//         Transform::from_xyz(0.0, 17., 24.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
//         Camera {
//             order: 3,
//             clear_color: ClearColorConfig::Custom(Color::srgb(0.1, 0.9, 0.1)),
//             viewport: Some(Viewport {
//                 physical_position: UVec2::new(0, 500),
//                 physical_size: UVec2::new(500, 500), //not size, actually x2,y2
//                 ..Default::default()
//             }),
//             ..Default::default()
//         },
//     ));

//     let shape=meshes.add(Cuboid::default());

//     const SHAPES_X_EXTENT: f32 = 14.0;
//     const Z_EXTENT: f32 = 5.0;
//     let num_shapes=5;

//     for i in 0 .. num_shapes {
//         let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);
//         let material = materials.add(StandardMaterial {base_color:color, ..default() });

//         commands.spawn((
//             Mesh3d(shape.clone()),
//             MeshMaterial3d(material.clone()),
//             Transform::from_xyz(
//                 -SHAPES_X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * SHAPES_X_EXTENT,
//                 2.0,
//                 Z_EXTENT / 2.,
//             )
//         ));
//     }

//     commands.spawn((
//         PointLight {
//             shadows_enabled: true,
//             intensity: 10_000_000.,
//             range: 100.0,
//             shadow_depth_bias: 0.2,
//             ..default()
//         },
//         Transform::from_xyz(8.0, 16.0, 8.0),
//     ));

//     // ground plane
//     commands.spawn((
//         Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0).subdivisions(10))),
//         MeshMaterial3d(materials.add(Color::from(SILVER))),
//     ));
// }

fn update_input(
    mut key_events: MessageReader<bevy::input::keyboard::KeyboardInput>,
    mut exit: MessageWriter<AppExit>,
    mut last_pressed:Local<HashSet<KeyCode>>,
) {
    for ev in key_events.read() {
        //
        if ev.state==bevy::input::ButtonState::Pressed && !last_pressed.contains(&ev.key_code) {
            if ev.key_code==KeyCode::Escape || ev.key_code==KeyCode::F4 {
                exit.write(AppExit::Success);
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
