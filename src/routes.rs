use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rocket::request::{FromForm, Form, FormItems};

use crate::basic_auth;

#[get("/")]
pub fn index(auth: basic_auth::BasicAuth) -> Template {
    let mut context: HashMap<&str, &str> = HashMap::new();
    context.insert("name", &auth.username);
    return Template::render("index", context);
}

#[derive(FromForm)]
pub struct SurveyForm {
    name: String,
    age: String,
    gender: String,
    favoriteFruit: String,
}

#[post("/survey", format="application/x-www-form-urlencoded", data="<form>")]
pub fn survey(auth: basic_auth::BasicAuth, form: Form<SurveyForm>) -> Template {
    let mut context: HashMap<&str, &str> = HashMap::new();
    context.insert("name", &auth.username);

    println!("name: {}, age: {}, gender: {}, fruit: {}", form.name, form.age, form.gender, form.favoriteFruit);
    return Template::render("survey", context);
}