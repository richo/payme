extern crate iron;
extern crate mount;
extern crate staticfile;
extern crate urlencoded;

use std::path::Path;
use std::io::Read;

use iron::prelude::*;
use iron::status;
use mount::Mount;
use staticfile::Static;
use urlencoded::UrlEncodedBody;

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!")))
}

fn charge(req: &mut Request) -> IronResult<Response> {
    let mut buf: Vec<u8> = vec![];
    match req.get_ref::<UrlEncodedBody>() {
        Ok(ref hashmap) => println!("Parsed POST request body string:\n {:?}", hashmap),
        Err(ref e) => println!("{:?}", e)
    };

    Ok(Response::with((status::Ok, "Got it")))

}

fn main() {
    let mut mount = Mount::new();

    mount.mount("/", Static::new(Path::new("static")));
    mount.mount("/hello", hello_world);
    mount.mount("/charge", charge);

    Iron::new(mount).http("localhost:3000").unwrap();
}
