use bevy::prelude::*;
use bevy_ui_exact_image::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ExactImagePlugin)
        .add_systems(Startup, spawn_example)
        .run();
}

fn spawn_example(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_assets: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());
    let texture_atlas_image = asset_server.load("orientation_big.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_atlas_image.clone(),
        Vec2::splat(256.),
        2,
        2,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlas_assets.add(texture_atlas);

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(100.),
                height: Val::Px(100.),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn(ExactAtlasImageBundle {
                image: ExactAtlasImage {
                    atlas: texture_atlas_handle.clone(),
                    index: 1,
                    color: Color::WHITE,
                    size: ExactSize::Exactly(Vec2::new(256., 128.)),
                    ..Default::default()
                },
                style: Style {
                    width: Val::Px(256.),
                    height: Val::Px(128.),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::RED),
                ..Default::default()
            });
        });
}
