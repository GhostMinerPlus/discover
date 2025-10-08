use bevy::prelude::*;
use discover_plugin::DiscoverPlugin;

fn main() {
    env_logger::init();

    App::new().add_plugins(DiscoverPlugin).run();
}
