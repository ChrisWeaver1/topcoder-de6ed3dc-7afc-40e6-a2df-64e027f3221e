use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rocket::request::{Form};

use crate::basic_auth;
use crate::survey_form;

#[get("/")]
pub fn index(auth: basic_auth::BasicAuth) -> Template {
    let mut context: HashMap<&str, &str> = HashMap::new();
    context.insert("name", &auth.username);
    return Template::render("index", context);
}

#[post("/survey", format="application/x-www-form-urlencoded", data="<form>")]
pub fn survey(_auth: basic_auth::BasicAuth, form: Form<survey_form::SurveyForm>) -> Template {
    let context: HashMap<&str, &str> = HashMap::new();

    println!("name: {:?}, age: {:?}, gender: {:?}, fruit: {}", form.name.0, form.age, form.gender, form.fruit);
    return Template::render("survey", context);
}