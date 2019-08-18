#![feature(async_await)]
use async_std::io;
use async_std::task;
use futures_timer::Delay;
use rand::{thread_rng, Rng};
use std::sync::{Arc, Mutex};
use std::time::Duration;
fn main() {
    task::block_on(async {
        let mut tasks = vec![];
        for i in 1..10 {
            tasks.push(async move {
                let random_delay = thread_rng().gen_range(1, 5);
                Delay::new(Duration::from_secs(random_delay)).await;
                println!("Received {}", i);
            });
        }
        println!("Tasks triggered, waiting output");

        futures::future::join_all(tasks).await;
    });
}
