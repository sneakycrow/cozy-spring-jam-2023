use bevy::DefaultPlugins;
use bevy::prelude::App;

mod components;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SvgPlugin)
        .run();
}

