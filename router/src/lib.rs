use std::collections::HashMap;
use std::rc::Rc;

pub struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

pub struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

type BoxedCallback = Box<Fn(&Request) -> Response>;

pub struct BasicRouter {
    routes: HashMap<String, BoxedCallback>,
}

impl BasicRouter {
    /// Create an empty router
    pub fn new() -> BasicRouter {
        BasicRouter {
            routes: HashMap::new(),
        }
    }

    /// Add a route to the router
    pub fn add_route<C>(&mut self, url: &str, callback: C)
    where
        C: Fn(&Request) -> Response + 'static,
    {
        self.routes.insert(url.to_string(), Box::new(callback));
    }

    fn handle_request(&self, request: &Request) -> Response {
        match self.routes.get(&request.url) {
            None => not_found_response(request),
            Some(callback) => callback(request),
        }
    }
}

fn not_found_response(request: &Request) -> Response {
    let r = Response {
        code: 404,
        body: "Not found".as_bytes().to_vec(),
        headers: HashMap::new(),
    };

    println!(
        "The request with url {} returned 404 Not Found",
        &request.url
    );
    r
}

#[test]
fn test_router() {

    let mut router = BasicRouter::new();

    use std::cell::RefCell;

    let mut callback_called = Rc::new(RefCell::new(false));

    let assertion = callback_called.clone();

    router.add_route("/hello", move |request| {

        println!(
            "Handler working {} request {} with content {}",
            request.method,
            request.url,
            String::from_utf8_lossy(&request.body[..])
        );

        *callback_called.borrow_mut() = true;

        let mut response = String::from("Hello ");
        response.push_str(&String::from_utf8_lossy(&request.body[..]));

        Response {
            code: 200,
            headers: HashMap::new(),
            body: response.as_bytes().to_vec()
        }
    });

    let resp = router.handle_request(&Request {
        url: "/hello".to_string(),
        method: "POST".to_string(),
        body: "This is superman!".as_bytes().to_vec(),
        headers: HashMap::new(),
    });

    assert_eq!(*assertion.borrow(), true);
    assert_eq!(String::from_utf8(resp.body).unwrap(), "Hello This is superman!");

}
