#![feature(async_await)]

use future_executor::{futures::TimerFuture, executor::new_executor_and_spawner};
use std::time::Duration;

fn main() {
    let (executor, spawner) = new_executor_and_spawner();
    spawner.spawn(async {
        println!("Waiting for the future timer...");
        TimerFuture::new(Duration::from_secs(3)).await;
        println!("Tick!, done");
    });

    drop(spawner);
    executor.run();
}