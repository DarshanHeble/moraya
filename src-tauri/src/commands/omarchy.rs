use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct OmarchyColors {
    pub accent: String,
    pub background: String,
    pub foreground: String,
    pub cursor: String,
    pub selection_background: String,
    pub selection_foreground: String,
    pub color0: String,
    pub color1: String,
    pub color2: String,
    pub color3: String,
    pub color4: String,
    pub color5: String,
    pub color6: String,
    pub color7: String,
    pub color8: String,
    pub color9: String,
    pub color10: String,
    pub color11: String,
    pub color12: String,
    pub color13: String,
    pub color14: String,
    pub color15: String,
}

#[tauri::command]
pub fn get_omarchy_colors() -> Result<Option<OmarchyColors>, String> {
    let home = std::env::var("HOME").map_err(|_| "Could not find HOME directory")?;
    let path = PathBuf::from(home).join(".config/omarchy/current/theme/colors.toml");

    if !path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read colors.toml: {}", e))?;
    
    // Simple TOML-like parsing for Omarchy's flat colors.toml
    // Since we don't want to add a full TOML parser dependency if possible, 
    // or if we have one (serde_json is there, toml might be in dependencies).
    // Let's check Cargo.toml for 'toml' crate.
    
    // Actually, tauri already uses toml for config.
    // Let's just use toml crate if it's in the tree.
    
    parse_omarchy_toml(&content).map(Some)
}

fn parse_omarchy_toml(content: &str) -> Result<OmarchyColors, String> {
    let mut map = std::collections::HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim().trim_matches('"').trim_matches('\'');
            map.insert(key.to_string(), value.to_string());
        }
    }

    let get = |k: &str| map.get(k).cloned().unwrap_or_else(|| "#000000".to_string());

    Ok(OmarchyColors {
        accent: get("accent"),
        background: get("background"),
        foreground: get("foreground"),
        cursor: get("cursor"),
        selection_background: get("selection_background"),
        selection_foreground: get("selection_foreground"),
        color0: get("color0"),
        color1: get("color1"),
        color2: get("color2"),
        color3: get("color3"),
        color4: get("color4"),
        color5: get("color5"),
        color6: get("color6"),
        color7: get("color7"),
        color8: get("color8"),
        color9: get("color9"),
        color10: get("color10"),
        color11: get("color11"),
        color12: get("color12"),
        color13: get("color13"),
        color14: get("color14"),
        color15: get("color15"),
    })
}
