use crate::api::types::{Ayah, SearchResult, Surah};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct QuranJsonSurah {
    id: u32,
    name: String,
    transliteration: String,
    translation: String,
    #[serde(rename = "type")]
    type_name: String,
    total_verses: u32,
    verses: Vec<QuranJsonAyah>,
}

#[derive(Deserialize)]
struct QuranJsonAyah {
    id: u32,
    text: String,
    translation: String,
}

// Bundle the Quran data at compile time
const QURAN_DATA: &str = include_str!("../../data/quran_en.json");

static QURAN: Lazy<HashMap<u32, Surah>> = Lazy::new(|| {
    let raw_surahs: Vec<QuranJsonSurah> =
        serde_json::from_str(QURAN_DATA).expect("Failed to parse bundled Quran data");
    
    raw_surahs
        .into_iter()
        .map(|s| {
            let verses = s.verses.iter().map(|v| Ayah {
                id: v.id,
                text: v.text.clone(),
                translation: v.translation.clone(),
                surah_id: s.id,
            }).collect();

            let surah = Surah {
                id: s.id,
                name: s.name,
                transliteration: s.transliteration,
                translation: s.translation,
                type_name: s.type_name,
                total_verses: s.total_verses,
                verses,
            };
            (s.id, surah)
        })
        .collect()
});

pub fn get_surah(id: u32) -> Option<&'static Surah> {
    QURAN.get(&id)
}

pub fn get_ayah(surah_id: u32, ayah_num: u32) -> Option<Ayah> {
    let surah = QURAN.get(&surah_id)?;
    surah.verses.get((ayah_num - 1) as usize).cloned()
}

pub fn search(query: &str) -> Vec<SearchResult> {
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();

    let mut surah_ids: Vec<u32> = QURAN.keys().cloned().collect();
    surah_ids.sort();

    for id in surah_ids {
        let surah = QURAN.get(&id).unwrap();
        for ayah in &surah.verses {
            if ayah.translation.to_lowercase().contains(&query_lower) {
                results.push(SearchResult {
                    surah_id: surah.id,
                    surah_name: surah.transliteration.clone(),
                    ayah_num: ayah.id,
                    text: ayah.text.clone(),
                    translation: ayah.translation.clone(),
                });
            }
        }
    }

    results.truncate(50);
    results
}

pub fn random_ayah() -> Ayah {
    use std::time::{SystemTime, UNIX_EPOCH};

    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as usize;

    let surah_id = (seed % 114) + 1;
    let surah = QURAN.get(&(surah_id as u32)).unwrap();
    let ayah_idx = seed % surah.verses.len();
    surah.verses[ayah_idx].clone()
}
