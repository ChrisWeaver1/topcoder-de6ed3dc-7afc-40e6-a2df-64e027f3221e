#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
//pub extern crate tera;

mod basic_auth_middleware;

use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket::{Request, Response, catch};
use rocket::response::{Result};
use rocket::http::{Header, Status};

#[get("/")]
fn index(auth: basic_auth_middleware::BasicAuth) -> Template {
    let mut context: HashMap<&str, &str> = HashMap::new();
    context.insert("name", &auth.username);
    return Template::render("index", context);
}

#[catch(401)]
fn basic_auth<'a>(_req: &Request) -> Result<'a> {
    // build a respose
    return Response::build()
        // add header
        .header(Header::new("WWW-Authenticate", "Basic realm=\"topcoder-challenge\""))
        // set status code
        .status(Status::Unauthorized)
        // wrap response as result
        .ok();
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .register(catchers![basic_auth])
        .mount("/", routes![index])
        .launch();
}

