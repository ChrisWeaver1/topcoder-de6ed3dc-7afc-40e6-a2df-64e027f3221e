
use std::collections::HashMap;
use rocket_contrib::templates::Template;

use crate::basic_auth;

#[get("/")]
pub fn index(auth: basic_auth::BasicAuth) -> Template {
    let mut context: HashMap<&str, &str> = HashMap::new();
    context.insert("name", &auth.username);
    return Template::render("index", context);
}