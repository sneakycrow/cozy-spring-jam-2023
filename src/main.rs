use bevy::DefaultPlugins;
use bevy::prelude::App;

mod components;
mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(plugins::HelloPlugin)
        .run();
}

