use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct AppConfig {
    pub schema_version: u32,
    pub dolphin_path: Option<String>,
    pub game_path: Option<String>,
    pub game_id: Option<String>,
    pub active_profile_id: String,
    pub profiles: Vec<Profile>,
}

impl Default for AppConfig {
    fn default() -> Self {
        let profile = Profile::new("Default".to_string());
        Self {
            schema_version: 1,
            dolphin_path: None,
            game_path: None,
            game_id: None,
            active_profile_id: profile.id.clone(),
            profiles: vec![profile],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub graphics: GraphicsSettings,
    pub controller: ControllerSettings,
    pub mods: Vec<ModEntry>,
    pub gecko_codes: Vec<GeckoCode>,
}

impl Profile {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            graphics: GraphicsSettings::default(),
            controller: ControllerSettings::default(),
            mods: Vec::new(),
            gecko_codes: Vec::new(),
        }
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self::new("Default".to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct GraphicsSettings {
    pub internal_resolution: u8,
    pub msaa: u8,
    pub anisotropy: u8,
    pub vertex_rounding: bool,
    pub show_fps: bool,
    pub borderless_fullscreen: bool,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self {
            internal_resolution: 3,
            msaa: 1,
            anisotropy: 4,
            vertex_rounding: false,
            show_fps: false,
            borderless_fullscreen: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ControllerSettings {
    pub mode: String,
    pub motion_plus: bool,
    pub nunchuk: bool,
    pub device_label: String,
    pub sensitivity: f32,
}

impl Default for ControllerSettings {
    fn default() -> Self {
        Self {
            mode: "real".to_string(),
            motion_plus: true,
            nunchuk: false,
            device_label: String::new(),
            sensitivity: 1.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModEntry {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub enabled: bool,
    pub game_prefix: Option<String>,
    pub xml_path: Option<String>,
    pub version: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeckoCode {
    pub name: String,
    pub code: String,
    pub enabled: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DolphinInfo {
    pub executable: String,
    pub version: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameInfo {
    pub path: String,
    pub extension: String,
    pub size_bytes: u64,
    pub game_id: Option<String>,
    pub is_wii_sports_resort: Option<bool>,
    pub note: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchResult {
    pub pid: u32,
    pub user_data_path: String,
    pub arguments: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchStatus {
    pub running: bool,
    pub pid: Option<u32>,
    pub exit_code: Option<i32>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupInfo {
    pub id: String,
    pub path: String,
}
