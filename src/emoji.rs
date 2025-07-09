use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Gitmoji data structure matching the JSON schema
#[derive(Debug, Deserialize, Serialize)]
struct GitmojiData {
    gitmojis: Vec<Gitmoji>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Gitmoji {
    emoji: String,
    code: String,
    description: String,
    name: String,
}

/// Emoji lookup functionality
pub struct EmojiLookup;

impl EmojiLookup {
    /// Convert emoji code to Unicode emoji
    pub fn code_to_unicode(code: &str) -> Option<&'static str> {
        EMOJI_MAP.get(code).copied()
    }

    /// Get all available emoji codes
    pub fn all_codes() -> Vec<&'static str> {
        EMOJI_MAP.keys().copied().collect()
    }

    /// Load emoji mapping from gitmojis.json file
    fn load_from_json() -> Result<HashMap<&'static str, &'static str>, Box<dyn std::error::Error>> {
        let json_content = include_str!("../fixtures/gitmojis.json");
        let gitmoji_data: GitmojiData = serde_json::from_str(json_content)?;

        let mut map = HashMap::new();
        for gitmoji in gitmoji_data.gitmojis {
            // We need to use Box::leak to convert String to &'static str
            // This is safe for our use case since this data lives for the program duration
            let code: &'static str = Box::leak(gitmoji.code.into_boxed_str());
            let emoji: &'static str = Box::leak(gitmoji.emoji.into_boxed_str());
            map.insert(code, emoji);
        }

        Ok(map)
    }

    /// Fallback emoji mapping if JSON loading fails
    fn default_emoji_map() -> HashMap<&'static str, &'static str> {
        let mut map = HashMap::new();

        // Generated from gitmojis.json
        map.insert(":art:", "🎨");
        map.insert(":zap:", "⚡️");
        map.insert(":fire:", "🔥");
        map.insert(":bug:", "🐛");
        map.insert(":ambulance:", "🚑️");
        map.insert(":sparkles:", "✨");
        map.insert(":memo:", "📝");
        map.insert(":rocket:", "🚀");
        map.insert(":lipstick:", "💄");
        map.insert(":tada:", "🎉");
        map.insert(":white_check_mark:", "✅");
        map.insert(":lock:", "🔒️");
        map.insert(":closed_lock_with_key:", "🔐");
        map.insert(":bookmark:", "🔖");
        map.insert(":rotating_light:", "🚨");
        map.insert(":construction:", "🚧");
        map.insert(":green_heart:", "💚");
        map.insert(":arrow_down:", "⬇️");
        map.insert(":arrow_up:", "⬆️");
        map.insert(":pushpin:", "📌");
        map.insert(":construction_worker:", "👷");
        map.insert(":chart_with_upwards_trend:", "📈");
        map.insert(":recycle:", "♻️");
        map.insert(":heavy_plus_sign:", "➕");
        map.insert(":heavy_minus_sign:", "➖");
        map.insert(":wrench:", "🔧");
        map.insert(":hammer:", "🔨");
        map.insert(":globe_with_meridians:", "🌐");
        map.insert(":pencil2:", "✏️");
        map.insert(":poop:", "💩");
        map.insert(":rewind:", "⏪️");
        map.insert(":twisted_rightwards_arrows:", "🔀");
        map.insert(":package:", "📦️");
        map.insert(":alien:", "👽️");
        map.insert(":truck:", "🚚");
        map.insert(":page_facing_up:", "📄");
        map.insert(":boom:", "💥");
        map.insert(":bento:", "🍱");
        map.insert(":wheelchair:", "♿️");
        map.insert(":bulb:", "💡");
        map.insert(":beers:", "🍻");
        map.insert(":speech_balloon:", "💬");
        map.insert(":card_file_box:", "🗃️");
        map.insert(":loud_sound:", "🔊");
        map.insert(":mute:", "🔇");
        map.insert(":busts_in_silhouette:", "👥");
        map.insert(":children_crossing:", "🚸");
        map.insert(":building_construction:", "🏗️");
        map.insert(":iphone:", "📱");
        map.insert(":clown_face:", "🤡");
        map.insert(":egg:", "🥚");
        map.insert(":see_no_evil:", "🙈");
        map.insert(":camera_flash:", "📸");
        map.insert(":alembic:", "⚗️");
        map.insert(":mag:", "🔍️");
        map.insert(":label:", "🏷️");
        map.insert(":seedling:", "🌱");
        map.insert(":triangular_flag_on_post:", "🚩");
        map.insert(":goal_net:", "🥅");
        map.insert(":dizzy:", "💫");
        map.insert(":wastebasket:", "🗑️");
        map.insert(":passport_control:", "🛂");
        map.insert(":adhesive_bandage:", "🩹");
        map.insert(":monocle_face:", "🧐");
        map.insert(":coffin:", "⚰️");
        map.insert(":test_tube:", "🧪");
        map.insert(":necktie:", "👔");
        map.insert(":stethoscope:", "🩺");
        map.insert(":bricks:", "🧱");
        map.insert(":technologist:", "🧑‍💻");
        map.insert(":money_with_wings:", "💸");
        map.insert(":thread:", "🧵");
        map.insert(":safety_vest:", "🦺");
        map.insert(":airplane:", "✈️");

        map
    }
}

/// Comprehensive gitmoji mapping - first tries loading from gitmojis.json, then falls back to default
/// Maps emoji codes (like ":sparkles:") to Unicode characters
pub static EMOJI_MAP: std::sync::LazyLock<HashMap<&'static str, &'static str>> =
    std::sync::LazyLock::new(|| {
        // First try to load from JSON
        if let Ok(map) = EmojiLookup::load_from_json() {
            map
        } else {
            // Fall back to hardcoded mapping
            EmojiLookup::default_emoji_map()
        }
    });
