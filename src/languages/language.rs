use serde::Deserialize;
use std::cmp::Ord;

#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Language {
    #[serde(rename = "nome")]
    pub name: String,
    #[serde(rename = "dicas")]
    pub clues: [String; 3]
}
