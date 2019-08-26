#![feature(async_await, async_closure)]
use async_std::task;
use async_std::fs::{File, create_dir};
use futures::io::Error;
use futures::AsyncWriteExt;
use futures::future::join_all;
use async_std::io;
use std::path::PathBuf;

fn main() {
    task::block_on(async {
        let tv_shows = vec!["Suits", "Game of thrones", "Black Mirror", "The Wire", "24h", "The 100", "Stranger Things"];
        let mut tasks = vec![];
        create_dir("./files").await.unwrap();
        for show in tv_shows {
            tasks.push(async move {
                println!("Writing file with tv show: {}", show);
                let path = PathBuf::from(format!("./files/{}.txt", show));
                let mut f = File::create(path).await.unwrap();
                f.write(show.as_bytes()).await.unwrap();
            });
        }
        futures::future::join_all(tasks).await;
    });
}