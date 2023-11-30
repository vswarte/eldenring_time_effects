use std::collections::HashMap;
use std::fs;
use std::path;
use std::sync;
use serde::Deserialize;

const CONFIG_FILE: &str = "./time_speffects.toml";

static mut CONFIG: sync::OnceLock<Config> = sync::OnceLock::new();
static mut EFFECT_TABLE: sync::OnceLock<HashMap<u32, ConfigEffectEntry>> = sync::OnceLock::new();

#[derive(Clone, Default, Deserialize)]
pub struct Config {
    pub effects: Vec<ConfigEffectEntry>,
}

#[derive(Clone, Deserialize)]
pub struct ConfigEffectEntry {
    pub effect_id: u32,
    pub effect_type: EffectEntryType,
}

#[derive(Clone, Deserialize)]
#[serde(tag = "type")]
pub enum EffectEntryType {
    WorldTime { multiplier: f32 },
    ChrTime { multiplier: f32 },
}

fn get_config_file() -> Config {
    unsafe {
        CONFIG.get_or_init(|| read_config_file().unwrap_or_else(|| Config::default())).clone()
    }
}

fn read_config_file() -> Option<Config> {
    path::absolute(path::PathBuf::from(CONFIG_FILE))
        .map(|p| fs::read_to_string(p).ok()).ok()
        .flatten()
        .map(|f| toml::from_str(f.as_str()).ok())
        .flatten()
}

fn get_effect_table() -> &'static HashMap<u32, ConfigEffectEntry>  {
    unsafe {
        EFFECT_TABLE.get_or_init(|| {
            get_config_file().effects.into_iter()
                .map(|e| (e.effect_id, e))
                .collect()
        })
    }
}

pub fn get_effect_mapping(sp_effect_id: u32) -> Option<&'static ConfigEffectEntry> {
    get_effect_table().get(&sp_effect_id)
}