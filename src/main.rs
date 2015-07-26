extern crate stripe;
extern crate iron;
extern crate mount;
extern crate staticfile;
extern crate urlencoded;

use std::path::Path;
use std::io::Read;
use std::env;

use iron::prelude::*;
use iron::status;
use mount::Mount;
use staticfile::Static;
use urlencoded::UrlEncodedBody;

use stripe::connection::Connection;
use stripe::customer::Customer;

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!")))
}

fn get_conn() -> Connection {
    let secret_key: String = env::var("STRIPE_SECRET_KEY").ok().expect("No STRIPE_SECRET_KEY set");
    return Connection::new(secret_key);
}

fn charge(req: &mut Request) -> IronResult<Response> {
    let mut buf: Vec<u8> = vec![];
    match req.get_ref::<UrlEncodedBody>() {
        Ok(ref hashmap) => {
            let email = hashmap.get("stripeEmail").unwrap()[0].clone();
            let token = hashmap.get("stripeToken").unwrap()[0].clone();

            let customer = Customer::create(get_conn(), email, token);
        },
        Err(ref e) => return Ok(Response::with((status::InternalServerError, e.to_string()))),
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
