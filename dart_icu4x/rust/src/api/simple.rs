#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

use serde::Serialize;
use icu_collections::codepointinvlist::CodePointInversionList;
use icu_properties::CodePointMapData;
use icu_properties::CodePointSetData;
use unicode_names2::name;
use unicode_blocks::find_unicode_block;
use icu_properties::props::*;

#[derive(Serialize)]
pub struct UnicodeCharProperties {
    pub character: String,
    pub code_point: u32,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub unicode_value: Option<String>,
    #[serde(default)]
    pub general_category: Option<String>,
    #[serde(default)]
    pub block: Option<String>,
    #[serde(default)]
    pub plane: Option<String>,
    #[serde(default)]
    pub script: Option<String>,
    #[serde(default)]
    pub bidi_class: Option<String>,
    #[serde(default)]
    pub east_asian_width: Option<String>,
    #[serde(default)]
    pub line_break: Option<String>,
    #[serde(default)]
    pub word_break: Option<String>,
    #[serde(default)]
    pub sentence_break: Option<String>,
    #[serde(default)]
    pub grapheme_cluster_break: Option<String>,
    #[serde(default)]
    pub hangul_syllable_type: Option<String>,
    #[serde(default)]
    pub joining_type: Option<String>,
    #[serde(default)]
    pub is_alphabetic: Option<bool>,
    #[serde(default)]
    pub is_uppercase: Option<bool>,
    #[serde(default)]
    pub is_lowercase: Option<bool>,
    #[serde(default)]
    pub is_white_space: Option<bool>,
    #[serde(default)]
    pub is_math: Option<bool>,
    #[serde(default)]
    pub is_dash: Option<bool>,
    #[serde(default)]
    pub is_diacritic: Option<bool>,
    #[serde(default)]
    pub is_emoji: Option<bool>,
    #[serde(default)]
    pub is_emoji_presentation: Option<bool>,
    #[serde(default)]
    pub is_emoji_modifier: Option<bool>,
    #[serde(default)]
    pub is_emoji_modifier_base: Option<bool>,
}

#[flutter_rust_bridge::frb(sync)]
pub fn get_unicode_char_properties(
    search: Option<String>,
    offset: usize,
    limit: usize,
) -> Vec<UnicodeCharProperties> {
    let general_category_map = CodePointMapData::<GeneralCategory>::new();
    let script_map = CodePointMapData::<Script>::new();
    let bidi_class_map = CodePointMapData::<BidiClass>::new();
    let east_asian_width_map = CodePointMapData::<EastAsianWidth>::new();
    let line_break_map = CodePointMapData::<LineBreak>::new();
    let word_break_map = CodePointMapData::<WordBreak>::new();
    let sentence_break_map = CodePointMapData::<SentenceBreak>::new();
    let grapheme_cluster_break_map = CodePointMapData::<GraphemeClusterBreak>::new();
    let hangul_syllable_type_map = CodePointMapData::<HangulSyllableType>::new();
    let joining_type_map = CodePointMapData::<JoiningType>::new();
    let alphabetic = CodePointSetData::new::<Alphabetic>();
    let uppercase = CodePointSetData::new::<Uppercase>();
    let lowercase = CodePointSetData::new::<Lowercase>();
    let white_space = CodePointSetData::new::<WhiteSpace>();
    let math = CodePointSetData::new::<Math>();
    let dash = CodePointSetData::new::<Dash>();
    let diacritic = CodePointSetData::new::<Diacritic>();
    let emoji = CodePointSetData::new::<Emoji>();
    let emoji_presentation = CodePointSetData::new::<EmojiPresentation>();
    let emoji_modifier = CodePointSetData::new::<EmojiModifier>();
    let emoji_modifier_base = CodePointSetData::new::<EmojiModifierBase>();

    let mut results: Vec<char> = Vec::new();

    if let Some(ref s) = search {
        if !s.is_empty() {
            // If search is a single character, search for that specific character
            let mut chars = s.chars();
            if let Some(c) = chars.next() {
                if chars.next().is_none() {
                    results.push(c);
                } else {
                    // Search is multiple characters, filter across all fields
                    results = CodePointInversionList::all()
                        .iter_chars()
                        .filter(|&c| {
                            let char_str = c.to_string();
                            let code_point_str = (c as u32).to_string();
                            let unicode_value = format!("U+{:04X}", c as u32);
                            let general_category = format!("{:?}", general_category_map.get(c));
                            
                            s.to_lowercase().contains(&char_str.to_lowercase()) ||
                            code_point_str.contains(s) ||
                            unicode_value.to_lowercase().contains(&s.to_lowercase()) ||
                            general_category.to_lowercase().contains(&s.to_lowercase())
                        })
                        .collect();
                }
            } else {
                // Empty search string, treat as no search
                results = CodePointInversionList::all()
                    .iter_chars()
                    .collect();
            }
        } else {
            // search is Some("") (empty string), treat as no search
            results = CodePointInversionList::all()
                .iter_chars()
                .collect();
        }
    } else {
        // search is None
        results = CodePointInversionList::all()
            .iter_chars()
            .collect();
    }

    results
        .into_iter()
        .skip(offset)
        .take(limit)
        .map(|c| UnicodeCharProperties {
            character: c.to_string(),
            code_point: c as u32,
            name: Some(name(c).map_or("UNKNOWN".to_string(), |n| n.to_string())),
            unicode_value: Some(format!("U+{:04X}", c as u32)),
            block: Some(find_unicode_block(c).map(|b| b.name()).unwrap_or("UNKNOWN").to_string()),
            plane: Some(get_plane_name(c as u32).to_string()),
            general_category: Some(format!("{:?}", general_category_map.get(c))),
            script: Some(format!("{:?}", script_map.get(c))),
            bidi_class: Some(format!("{:?}", bidi_class_map.get(c))),
            east_asian_width: Some(format!("{:?}", east_asian_width_map.get(c))),
            line_break: Some(format!("{:?}", line_break_map.get(c))),
            word_break: Some(format!("{:?}", word_break_map.get(c))),
            sentence_break: Some(format!("{:?}", sentence_break_map.get(c))),
            grapheme_cluster_break: Some(format!("{:?}", grapheme_cluster_break_map.get(c))),
            hangul_syllable_type: Some(format!("{:?}", hangul_syllable_type_map.get(c))),
            joining_type: Some(format!("{:?}", joining_type_map.get(c))),
            is_alphabetic: Some(alphabetic.contains(c)),
            is_uppercase: Some(uppercase.contains(c)),
            is_lowercase: Some(lowercase.contains(c)),
            is_white_space: Some(white_space.contains(c)),
            is_math: Some(math.contains(c)),
            is_dash: Some(dash.contains(c)),
            is_diacritic: Some(diacritic.contains(c)),
            is_emoji: Some(emoji.contains(c)),
            is_emoji_presentation: Some(emoji_presentation.contains(c)),
            is_emoji_modifier: Some(emoji_modifier.contains(c)),
            is_emoji_modifier_base: Some(emoji_modifier_base.contains(c)),
        }).collect()
}

/// Get the name of the plane for a given code point.
///
/// This function returns a string that describes the plane of the given code point.
/// The plane is determined by the first 16 bits of the code point.
///
/// # Arguments
///
/// * `code_point` - The code point to get the plane name for.
///
/// # Returns
///
/// A string that describes the plane of the given code point.
fn get_plane_name(code_point: u32) -> &'static str {
    match code_point {
        0x0000..=0xFFFF => "Basic Multilingual Plane [1]",
        0x10000..=0x1FFFF => "Supplementary Multilingual Plane [2]",
        0x20000..=0x2FFFF => "Supplementary Ideographic Plane [3]",
        0x30000..=0x3FFFF => "Tertiary Ideographic Plane [4]",
        0x40000..=0x4FFFF => "Unassigned [5]",
        0x50000..=0x5FFFF => "Unassigned [6]",
        0x60000..=0x6FFFF => "Unassigned [7]",
        0x70000..=0x7FFFF => "Unassigned [8]",
        0x80000..=0x8FFFF => "Unassigned [9]",
        0x90000..=0x9FFFF => "Unassigned [10]",
        0xA0000..=0xAFFFF => "Unassigned [11]",
        0xB0000..=0xBFFFF => "Supplementary Special-purpose Plane [14]",
        0xC0000..=0xCFFFF => "Unassigned [13]",
        0xD0000..=0xDFFFF => "Unassigned [14]",
        0xE0000..=0xEFFFF => "Supplementary Private Use Area-A [15]",
        0xF0000..=0xFFFFF => "Supplementary Private Use Area-B [16]",
        0x100000..=0x10FFFF => "Supplementary Private Use Area-B [17]",
        _ => "Unassigned",
    }
}


use icu_casemap::CaseMapper;
use icu_locale_core::LanguageIdentifier;

#[derive(Serialize)]
pub struct CaseMappingResult {
    pub original: String,
    pub mapped: String,
    pub has_mapping: bool,
}

#[flutter_rust_bridge::frb(sync)]
pub fn get_character_case_mapping(character: String) -> CaseMappingResult {
    let casemapper = CaseMapper::new();
    let langid: LanguageIdentifier = "und".parse().unwrap();
    
    let mut chars = character.chars();
    let c = match chars.next() {
        Some(ch) if chars.next().is_none() => ch,
        _ => return CaseMappingResult {
            original: character.clone(),
            mapped: character,
            has_mapping: false,
        },
    };
    
    let original = c.to_string();
    let upper = casemapper.uppercase_to_string(&original, &langid).into_owned();
    let lower = casemapper.lowercase_to_string(&original, &langid).into_owned();
    
    // Check if the character has a case mapping (upper or lower is different from original)
    let has_mapping = upper != original || lower != original;
    
    // Return the appropriate mapping (prefer uppercase if different, otherwise lowercase)
    let mapped = if upper != original {
        upper
    } else if lower != original {
        lower
    } else {
        original.clone()
    };
    
    CaseMappingResult {
        original,
        mapped,
        has_mapping,
    }
}