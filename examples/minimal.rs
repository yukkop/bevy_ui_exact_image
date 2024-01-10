use bevy::prelude::*;
use bevy_ui_exact_image::prelude::*;

fn spawn_example(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((ExactImageBundle {
        image: ExactImage {
            texture: assets.load("orientation.png"),
            color: Color::WHITE,
            size: ExactSize::Exactly(Vec2::new(300., 200.)),
            alignment: ImageAlignment::BottomCenter,
            rotation: None,
        },
        style: Style {
            width: Val::Px(400.),
            height: Val::Px(400.),
            ..Default::default()
        },
        background_color: BackgroundColor(Color::RED),
        ..Default::default()
    },));
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(ExactImagePlugin)
        .add_systems(Startup, spawn_example)
        .run();
}
