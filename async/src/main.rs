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

type HttpsConnector = hyper_rustls::HttpsConnector<hyper::client::HttpConnector>;

struct RequestExecutor {
    core: Core,
    client: Client<HttpsConnector, Body>,
}

impl RequestExecutor {
    pub fn new() -> RequestExecutor {
        RequestExecutor {
            core: Core::new().unwrap(),
            client: Client::builder().build(HttpsConnector::new(4)),
        }
    }
    #[cfg(feature = "rustls")]
    type HttpsConnector = hyper_rustls::HttpsConnector<hyper::client::HttpConnector>;
    pub fn get<S, T>(&mut self, url: S) -> Result<T, hyper::error::Error>
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

        self.core.run(work)
    }
}

fn main() {
    let mut req_executor = RequestExecutor::new();

    let result: Todo = req_executor
        .get("https://jsonplaceholder.typicode.com/todos/12")
        .unwrap();

    println!("{:?}", result);

    let result2: Todo = req_executor
        .get("https://jsonplaceholder.typicode.com/todos/12")
        .unwrap();


    println!("{:?}", result2);
}
