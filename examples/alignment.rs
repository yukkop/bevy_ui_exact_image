use bevy::prelude::*;
use bevy_ui_exact_image::*;

fn spawn_example(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(ExactImageBundle {
        image: ExactImage {
            texture: assets.load("orientation.png"),
            size: ExactSize::Texture,
            alignment: ImageAlignment::BottomLeft,
            rotation: None,
            ..Default::default()
        },
        style: Style {
            width: Val::Px(128.),
            height: Val::Px(128.),
            ..Default::default()
        },
        background_color: BackgroundColor(Color::RED),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .insert_resource(UiScale(2.))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    // window: WindowDescriptor {
                    //     scale_factor_override: Some(2.0),
                    //     ..Default::default()
                    // },
                    ..Default::default()
                }),
        )
        .add_plugins(ExactImagePlugin)
        .add_systems(Startup, spawn_example)
        .run();
}
