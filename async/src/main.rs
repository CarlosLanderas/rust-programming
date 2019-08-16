#[macro_use]
extern crate serde_derive;

use futures::future::ok;
use futures::prelude::*;
use hyper::{client::Client, Body, Uri};
use std::vec::Vec;
use tokio_core::reactor::Core;
use serde::de::DeserializeOwned;


#[derive(Deserialize, Serialize, Debug)]
struct Todo {
    id: usize,
    #[serde(rename = "userId")]
    user_id: usize,
    title: String,
    completed: bool,
}

#[derive(Deserialize, Debug)]
struct Image {
    #[serde(rename = "albumId")]
    album_id : usize,
    id: usize,
    title: String,
    url: String,
    #[serde(rename = "thumbnailUrl")]
    thumbnail_url: String
}

type ResponseResult<T> = Result<T, hyper::error::Error>;
type HttpsConnector = hyper_rustls::HttpsConnector<hyper::client::HttpConnector>;

struct RequestExecutor {    
    client: Client<HttpsConnector, Body>,
}

impl RequestExecutor {
    pub fn new() -> RequestExecutor {        
        RequestExecutor {            
            client: Client::builder().build(HttpsConnector::new(4)),
        }
    }    
    
    pub fn get<S, T>(&mut self, url: S) -> ResponseResult<T>
    where
        S: Into<String>,
        T: DeserializeOwned,
    {
        
        let work = self
            .client
            .get(url.into().parse::<Uri>().unwrap())
            .and_then(|resp| {
                resp.into_body()
                    .fold(Vec::new(), |mut v, chunk| {
                        v.extend(&chunk[..]);
                        ok::<_, hyper::Error>(v)
                    })
                    .map(move |chunks| serde_json::from_slice::<T>(&chunks).unwrap())
            });

        Core::new().unwrap().run(work)
    }
}

fn main() {
    let mut req_executor = RequestExecutor::new();

    let todo : ResponseResult<Todo> = req_executor.get("https://jsonplaceholder.typicode.com/todos/1");
    let todo3 : ResponseResult<Todo>= req_executor.get("https://jsonplaceholder.typicode.com/todos/3");
    let todo7 : ResponseResult<Todo> = req_executor.get("https://jsonplaceholder.typicode.com/todos/7");
    let todo15 : ResponseResult<Todo> = req_executor.get("https://jsonplaceholder.typicode.com/todos/15");
    
    println!("{:?}", todo.unwrap());
    println!("{:?}", todo3.unwrap());
    println!("{:?}", todo7.unwrap());
    println!("{:?}", todo15.unwrap());

    let photo10 : ResponseResult<Image> = req_executor.get("https://jsonplaceholder.typicode.com/photos/10");
    let photo12 : ResponseResult<Image> = req_executor.get("https://jsonplaceholder.typicode.com/photos/12");
    let photo20 : ResponseResult<Image> = req_executor.get("https://jsonplaceholder.typicode.com/photos/20");

    println!("{:?}", photo10.unwrap());
    println!("{:?}", photo12.unwrap());
    println!("{:?}", photo20.unwrap());
    
}
