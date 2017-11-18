extern crate hyper;
extern crate gotham;
extern crate mime;
#[macro_use]
extern crate gotham_derive;
#[macro_use]
extern crate log;

mod boot;
mod controllers;

use hyper::server::Http;
use gotham::handler::NewHandlerService;

fn main() {
    // the address we will use. parse() knows what type to parse the string into based on type inference
    // we use addr later as a SockAddr
    let addr = "127.0.0.1:7878".parse().unwrap();

    let server = Http::new()
        .bind(&addr, NewHandlerService::new(boot::router::router()))
        .unwrap();

    println!("Listening on http://{}", server.local_addr().unwrap());
    server.run().unwrap();
}