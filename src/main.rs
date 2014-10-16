extern crate nickel;

use std::io::net::ip::Ipv4Addr;
use nickel::{ Nickel, Request, Response, HttpRouter };

fn main() {
    let mut server = Nickel::new();
    let mut router = Nickel::router();
    
    fn a_handler (_request: &Request, response: &mut Response) { 
        response.send("What the...a Rust-powered web app?"); 
    }

    router.get("/", a_handler);
    server.utilize(router);
    server.listen(Ipv4Addr(0, 0, 0, 0), 8080);
}
