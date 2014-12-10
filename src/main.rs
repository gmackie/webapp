extern crate http;
extern crate serialize;
extern crate nickel;

use http::status::NotFound;
use std::io::net::ip::Ipv4Addr;
use nickel::{
      Nickel, NickelError, ErrorWithStatusCode,
      Action, Continue, Halt, Request,
      Response, IntoErrorHandler, HttpRouter,
};
fn main() {
    let mut server = Nickel::new();
    let mut router = Nickel::router();
    
    fn a_handler (_request: &Request, response: &mut Response) { 
        response.send("What the...a Rust-powered web app?"); 
    }

    router.get("/", a_handler);
    server.utilize(router);

    fn custom_404 (err: &NickelError, _req: &Request, response: &mut Response) -> Result<Action, NickelError> {
        match err.kind {
            ErrorWithStatusCode(NotFound) => {
                response.content_type("html")
                        .status_code(NotFound)
                        .send("<h1>Call the police!<h1>");
                Ok(Halt)
            },
            _ => Ok(Continue)
        }
    }

    server.handle_error(IntoErrorHandler::from_fn(custom_404));
    server.listen(Ipv4Addr(0, 0, 0, 0), 8080);
}
