use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ayah {
    pub id: u32,
    pub text: String,
    pub translation: String,
    pub surah_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Surah {
    pub id: u32,
    pub name: String,
    pub transliteration: String,
    pub translation: String,
    pub type_name: String,
    pub total_verses: u32,
    pub verses: Vec<Ayah>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResult {
    pub surah_id: u32,
    pub surah_name: String,
    pub ayah_num: u32,
    pub text: String,
    pub translation: String,
}
