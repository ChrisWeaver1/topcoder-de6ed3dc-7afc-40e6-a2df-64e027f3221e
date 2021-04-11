#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
pub extern crate tera;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
mod basic_auth_middleware;

use rocket::response::status::Custom;
use rocket::{Catcher, Request, Response, catch};
use rocket::response::{Result, Responder};
use rocket::http::{Header, Status};

#[get("/")]
fn index(auth: basic_auth_middleware::BasicAuth) -> Template {
    let mut context: HashMap<&str, &str> = HashMap::new();
    context.insert("name", "chirs");
    return Template::render("index", context);
}

#[catch(401)]
fn basic_auth<'a>(req: &Request) -> Result<'a> {
    return Response::build()
        .header(Header::new("WWW-Authenticate", "Basic realm=\"realm\""))
        .status(Status::Unauthorized)
        .ok();
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .register(catchers![basic_auth])
        .mount("/", routes![index])
        .launch();
}

