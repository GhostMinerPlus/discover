use bevy::prelude::*;
use sender_plugin::SenderPlugin;

mod sys;

pub struct DiscoverPlugin;

impl Plugin for DiscoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SenderPlugin)
            .add_systems(Startup, sys::start_server);
    }
}
