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

pub struct DeepL {
    pub api_key: String,
}

#[derive(Deserialize)]
pub struct DeepLResponse {
    pub translations: Vec<DeepLTranslationResponse>
}

#[derive(Deserialize)]
pub struct DeepLTranslationResponse {
    pub text : String,
    pub detected_source_language: Option<String>
}

impl DeepL {
    // new DeepL instance
    pub fn new(api_key: String) -> DeepL {
        DeepL { api_key: api_key }
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

    pub async fn request(&self, text: String, to_language: Option<String>, from_language: Option<String>) -> Result<String, String>
    {
        let mut form = HashMap::new();
        form.insert("auth_key", self.api_key.clone());
        form.insert("text", text);
        form.insert("target_lang", to_language.unwrap_or(String::from("EN-GB")));
        if from_language != None {
            form.insert("source_lang", from_language.unwrap_or_default());
        }

        // https://api.deepl.com/v2/translate"
        // requires DeepL Pro api access which is a paid service

        let client = reqwest::Client::new();
        let res = client.post("http://localhost:8080/v2/translate")
            .form(&form)
            .send()
            .await;

        if res.is_err() {
            let err = res.err().unwrap();
            return Err(format!("err: {}", err.to_string()));
        }

        let success = res.ok().unwrap();

        let status = success.status();
        if status.as_u16() >= 400 {
            return Err(format!("err: {}", status));
        }
        else if status.as_u16() == 200 {
            //let text = success.text().await.ok().unwrap();
            let body : reqwest::Result<DeepLResponse> = success.json().await;
            if body.is_err() {
                println!("{:?}", body.err());
                return Err(String::from("Failed to parse request body"));
            }
            return Ok(format!("{}: {}", status, body.unwrap().translations[0].text));
        }

        return Err(format!("err: {}", String::from("Unexpected result")));
    }
}

#[async_trait::async_trait]
impl Translator for DeepL {

    async fn translate(&self, text: String, to_language: LanguageOptions, from_language: Option<LanguageOptions>) -> Result<Translation, String> {
        let to_lang = DeepL::lang_to_string(to_language);
        let from_lang = DeepL::lang_to_string(from_language.unwrap_or(LanguageOptions::AUTO));

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
