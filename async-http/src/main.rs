#![feature(pin, async_await, futures_api)]
use async_std::io;
use async_std::task;
use serde_derive::Deserialize;

#[macro_use]
extern crate futures;

#[derive(Deserialize, Debug)]
struct Post {
    #[serde(rename = "userId")]
    user_id : usize,
    id: usize,
    title: String,
    completed: bool
}

fn main() {
    task::block_on(async {
        let post_fut  = surf::get("https://jsonplaceholder.typicode.com/todos/1").recv_json::<Post>();
        let post2_fut = surf::get("https://jsonplaceholder.typicode.com/todos/2").recv_json::<Post>();
        let (result1, result2 ) = join!(post_fut, post2_fut);
        println!("{:?}", result1.unwrap());
        println!("{:?}", result2.unwrap());
    });
}
