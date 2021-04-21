use rocket::request::{FromForm, FromFormValue};
use rocket::http::RawStr;

#[derive(FromForm)]
pub struct SurveyForm {
    pub name: Name,
    pub age: Age,
    pub gender: Gender,
    #[form(field = "favoriteFruit")]
    pub fruit: String,
}

#[derive(Debug)]
pub struct Age(pub u32);

impl<'v> FromFormValue<'v> for Age {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Age, &'v RawStr> {
        match form_value.parse::<u32>() {
            Ok(age) if age >= 17 && age <= 40 => Ok(Age(age)),
            _ => Err(form_value),
        }
    }
}

#[derive(Debug)]
pub struct Name(pub String);

impl<'v> FromFormValue<'v> for Name {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Name, &'v RawStr> {
        match form_value.parse::<String>() {
            Ok(name) if name.len() >= 1 && name.len() <= 20 => {
                return Ok(Name(name));
            },
            _ => Err(form_value),
        }
    }
}

#[derive(Debug)]
pub struct Gender(pub String);

impl<'v> FromFormValue<'v> for Gender {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Gender, &'v RawStr> {
        match form_value.parse::<String>() {
            Ok(gender) if gender == "male" || gender == "female" => Ok(Gender(gender)),
            _ => Err(form_value),
        }
    }
}