

pub struct QuranReference {
    pub surah_id: u32,
    pub ayah_num: Option<u32>,
}

pub fn parse(input: &str) -> Result<QuranReference, String> {
    let input = input.trim();
    if input.is_empty() {
        return Err("Empty reference".to_string());
    }

    if let Some((surah_str, ayah_str)) = input.split_once(':') {
        let surah_id: u32 = surah_str.trim().parse().map_err(|_| "Invalid surah number")?;
        let ayah_num: u32 = ayah_str.trim().parse().map_err(|_| "Invalid ayah number")?;
        return Ok(QuranReference { surah_id, ayah_num: Some(ayah_num) });
    }

    // Try parsing as just a surah number or surah name
    if let Ok(id) = input.parse::<u32>() {
        return Ok(QuranReference { surah_id: id, ayah_num: None });
    }

    // Handle surah names
    // This part would need a mapping of names to IDs.
    // For now, let's keep it simple.
    
    Err("Unsupported reference format".to_string())
}
