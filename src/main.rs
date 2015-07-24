extern crate iron;
extern crate mount;
extern crate staticfile;

use std::path::Path;

use iron::prelude::*;
use iron::status;
use mount::Mount;
use staticfile::Static;

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!")))
}

fn main() {
    let mut mount = Mount::new();

    mount.mount("/", Static::new(Path::new("static")));
    mount.mount("/hello", hello_world);

    Iron::new(mount).http("localhost:3000").unwrap();
}
