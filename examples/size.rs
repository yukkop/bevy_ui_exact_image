use bevy::prelude::*;
use bevy_ui_exact_image::*;

fn spawn_example(mut commands: Commands, assets: Res<AssetServer>) {
    let node_width = Val::Px(128.0);
    let node_height = Val::Px(192.0);
    let alignment = ImageAlignment::TopCenter;
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                width: node_width,
                height: node_height,
                overflow: Overflow::clip(),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn((ExactImageBundle {
                image: ExactImage {
                    texture: assets.load("orientation.png"),
                    size: ExactSize::FillNode,
                    alignment,
                    ..Default::default()
                },
                style: Style {
                    width: node_width,
                    height: node_height,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::RED),
                ..Default::default()
            },));
            builder.spawn((ExactImageBundle {
                image: ExactImage {
                    texture: assets.load("orientation.png"),
                    size: ExactSize::Texture,
                    alignment,
                    ..Default::default()
                },
                style: Style {
                    width: node_width,
                    height: node_height,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::RED),
                ..Default::default()
            },));

            builder.spawn((ExactImageBundle {
                image: ExactImage {
                    texture: assets.load("orientation.png"),
                    size: ExactSize::AttemptPreserveAspectRatio,
                    alignment,
                    ..Default::default()
                },
                style: Style {
                    width: node_width,
                    height: node_height,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::RED),
                ..Default::default()
            },));

            builder.spawn((ExactImageBundle {
                image: ExactImage {
                    texture: assets.load("orientation.png"),
                    size: ExactSize::ForcePreserveAspectRatio,
                    alignment,
                    ..Default::default()
                },
                style: Style {
                    width: node_width,
                    height: node_height,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::RED),
                ..Default::default()
            },));

            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: node_width,
                        height: node_height,
                        overflow: Overflow::clip(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|builder| {
                    builder.spawn((ExactImageBundle {
                        image: ExactImage {
                            texture: assets.load("orientation.png"),
                            size: ExactSize::ForcePreserveAspectRatio,
                            alignment,
                            ..Default::default()
                        },
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            width: node_width,
                            height: node_height,
                            ..Default::default()
                        },
                        ..Default::default()
                    },));
                });
        });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(ExactImagePlugin)
        .add_systems(Startup, spawn_example)
        .run();
}
