use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy_svg::prelude::*;

mod components;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(SvgPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let svg = asset_server.load("logo.svg");
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Svg2dBundle {
        svg,
        origin: Origin::Center,
        ..Default::default()
    });
}