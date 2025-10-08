use bevy::{ecs::world::CommandQueue, prelude::*};
use tokio::sync::mpsc;

pub mod res;

const MAX_CAPACITY: usize = 1024;

pub struct SenderPlugin;

impl Plugin for SenderPlugin {
    fn build(&self, app: &mut App) {
        let (sender, mut receiver) = mpsc::channel::<CommandQueue>(MAX_CAPACITY);

        app.init_resource::<res::AsyncPool>()
            .insert_resource(res::CqSender(sender))
            .set_runner(move |mut app| {
                let mut buffer = Vec::with_capacity(MAX_CAPACITY);

                loop {
                    log::debug!("update");
                    app.update();

                    if let Some(exit) = app.should_exit() {
                        return exit;
                    }

                    if let Err(e) = app.world().resource::<res::AsyncPool>().0.block_on(async {
                        if receiver.recv_many(&mut buffer, MAX_CAPACITY).await == 0 {
                            return Err("no more events");
                        }

                        Ok(())
                    }) {
                        log::error!("{e}");
                        return AppExit::error();
                    }

                    buffer.iter_mut().for_each(|cq| cq.apply(app.world_mut()));

                    buffer.clear();
                }
            });
    }
}
