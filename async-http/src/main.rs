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

#[derive(Deserialize, Debug)]
struct User {
    id: usize,
    name: String,
    username: String,
    email: String,
}

fn main() {
    task::block_on(async {
        let post_fut  = surf::get("https://jsonplaceholder.typicode.com/todos/1").recv_json::<Post>();
        let post2_fut = surf::get("https://jsonplaceholder.typicode.com/todos/2").recv_json::<Post>();
        let user_fut = surf::get("https://jsonplaceholder.typicode.com/users/9").recv_json::<User>();
        let user2_fut = surf::get("https://jsonplaceholder.typicode.com/users/10").recv_json::<User>();
        let (result1, result2, result3, result4) = join!(post_fut, post2_fut, user_fut, user2_fut);
        println!("{:?}", result1.unwrap());
        println!("{:?}", result2.unwrap());
        println!("{:?}", result3.unwrap());
        println!("{:?}", result4.unwrap());
    });
}
