#![feature(proc_macro_hygiene, decl_macro)]

// use macros from rocket
#[macro_use] extern crate rocket;

// internal imports
mod routes;
mod catchers;
mod basic_auth;
mod survey_form;

use rocket_contrib::templates::Template;

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .register(catchers![catchers::catch_401])
        .mount("/", routes![routes::index, routes::survey])
        .launch();
}

