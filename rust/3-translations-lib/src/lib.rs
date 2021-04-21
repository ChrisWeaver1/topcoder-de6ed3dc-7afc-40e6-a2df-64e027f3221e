extern crate tokio;
extern crate async_trait;

use std::fmt::Display;

pub mod deepl;
pub mod google;

#[async_trait::async_trait]
pub trait Translator {
    async fn translate(&self, text: String, to_language: LanguageOptions, from_language: Option<LanguageOptions>) -> Result<Translation, String>;
}

pub struct Translation {
    pub translation: String,
    pub to_language: LanguageOptions,
    pub from_language: LanguageOptions,
    pub initial_text: String
} 

impl Translation {
    fn new(
        translation: Option<String>, 
        to_language: Option<LanguageOptions>, 
        from_language: Option<LanguageOptions>,
        initial_text: Option<String>) 
        -> Translation 
    {
        return Translation { 
            translation: translation.unwrap_or_default(),
            to_language: to_language.unwrap_or(LanguageOptions::AUTO),
            from_language: from_language.unwrap_or(LanguageOptions::AUTO),
            initial_text: initial_text.unwrap_or_default(),
        };
    }
}

impl Display for Translation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "{}", self.translation)
    }
}

#[derive(Clone, Copy)]
pub enum LanguageOptions {
    CS,
    DA,
    DE,
    EL,
    ENGB,
    ENUS,
    EN,
    ES,
    ET,
    FI,
    FR,
    IT,
    NL,
    PL,
    PT,
    RU,
    AUTO
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn translate_test() {
        let client = deepl::DeepL::new(String::from("asfewg2g23"));
        let to_lang = deepl::DeepL::lang_to_string(LanguageOptions::PL);
        let from_lang = deepl::DeepL::lang_to_string(LanguageOptions::AUTO);
        let text = String::from("test sentence");
        let expected = format!("{} -> {} [ {} ]", from_lang.unwrap_or_default(), to_lang.unwrap_or_default(), text);

        let result =  client.translate(text, LanguageOptions::PL, None).await;

        if result.is_err() {
            panic!("Error was not expected")
        }
        
        let translation = result.ok().unwrap();
        println!("{}", translation);
        assert_eq!(expected, translation.translation);
    }
}



