use iron::prelude::*;
use iron::status;
use std::fs;

#[macro_use]
extern crate mime;

fn main() {
    println!("Serving on http://localhost:3000");
    match Iron::new(get_form).http("localhost:3000") {
        Ok(v) => println!("Server started"),
        Err(e) => {
            panic!("Server could not start: {}", e);
        }
    }
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(read_index_file());
    Ok(response)
}

fn read_index_file() -> String {
    fs::read_to_string("index.html").unwrap()
}
