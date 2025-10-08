use bevy::{ecs::world::CommandQueue, prelude::*};
use tokio::{runtime::Runtime, sync::mpsc::Sender};

#[derive(Resource)]
pub struct CqSender(pub Sender<CommandQueue>);

#[derive(Resource)]
pub struct AsyncPool(pub Runtime);

impl Default for AsyncPool {
    fn default() -> Self {
        Self(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
        )
    }
}
