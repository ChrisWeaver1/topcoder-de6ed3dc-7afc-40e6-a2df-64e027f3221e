extern crate reqwest;
extern crate futures;
extern crate async_trait;
extern crate serde_json;
extern crate serde;

use crate::Translation;
use std::collections::HashMap;
use crate::LanguageOptions;
use crate::Translator;

use serde::Deserialize;

pub struct Google {
    pub api_key: String,
}

#[derive(Deserialize)]
pub struct GoogleResponse {
}

impl Google {
    pub fn new(api_key: String) -> Google {
        Google { api_key: api_key }
    }

    pub fn lang_to_string(lang: LanguageOptions) -> Option<String> {
        return match lang {
            LanguageOptions::CS => Some(String::from("CS")),
            LanguageOptions::DA => Some(String::from("DA")),
            LanguageOptions::DE => Some(String::from("DE")),
            LanguageOptions::EL => Some(String::from("EL")),
            LanguageOptions::EN => Some(String::from("EN")),
            LanguageOptions::ENGB => Some(String::from("EN-GB")),
            LanguageOptions::ENUS => Some(String::from("EN-US")),
            LanguageOptions::ES => Some(String::from("ES")),
            LanguageOptions::ET => Some(String::from("ET")),
            LanguageOptions::FI => Some(String::from("FI")),
            LanguageOptions::FR => Some(String::from("FR")),
            LanguageOptions::IT => Some(String::from("IT")),
            LanguageOptions::NL => Some(String::from("NL")),
            LanguageOptions::PL => Some(String::from("PL")),
            LanguageOptions::PT => Some(String::from("PT")),
            LanguageOptions::RU => Some(String::from("RU")),
            LanguageOptions::AUTO => None,
        }
    }

    pub async fn request(&self, _text: String, _to_language: Option<String>, _from_language: Option<String>) -> Result<String, String>
    {
        
        return Err(format!("err: {}", String::from("Unexpected result")));
    }
}

#[async_trait::async_trait]
impl Translator for Google {

    async fn translate(&self, text: String, to_language: LanguageOptions, from_language: Option<LanguageOptions>) -> Result<Translation, String> {
        let to_lang = Google::lang_to_string(to_language);
        let from_lang = Google::lang_to_string(from_language.unwrap_or(LanguageOptions::AUTO));

        let result = self.request(text.clone(), to_lang, from_lang).await;

        if result.is_err() {
            let error_result = result.err().unwrap();
            return Err(error_result);
        }

        let ok_result = result.ok().unwrap();
        
        let t = Translation::new(Some(ok_result), Some(to_language), from_language, Some(text));

        return Ok(t);
    }
}
