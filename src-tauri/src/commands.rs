use crate::models::{
    AppConfig, BackupInfo, DolphinInfo, GameInfo, GeckoCode, LaunchResult, LaunchStatus, ModEntry,
};
use roxmltree::Document;
use serde_json::Value;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::{AppHandle, Manager, State};
use uuid::Uuid;
use zip::ZipArchive;

#[derive(Default)]
pub struct ProcessState(pub Mutex<Option<Child>>);

fn app_data_root(app: &AppHandle) -> Result<PathBuf, String> {
    let path = app.path().app_data_dir().map_err(|error| error.to_string())?;
    fs::create_dir_all(&path).map_err(|error| error.to_string())?;
    Ok(path)
}

fn config_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app_data_root(app)?.join("config.json"))
}

fn write_replacing(path: &Path, content: &[u8]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let temp = path.with_extension("tmp");
    fs::write(&temp, content).map_err(|error| error.to_string())?;
    if cfg!(windows) && path.exists() {
        fs::remove_file(path).map_err(|error| error.to_string())?;
    }
    fs::rename(temp, path).map_err(|error| error.to_string())
}

fn read_config(app: &AppHandle) -> Result<AppConfig, String> {
    let path = config_path(app)?;
    if !path.exists() {
        let config = AppConfig::default();
        write_config(app, &config)?;
        return Ok(config);
    }
    let content = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut config: AppConfig = serde_json::from_str(&content).map_err(|error| error.to_string())?;
    if config.profiles.is_empty() {
        config = AppConfig::default();
        write_config(app, &config)?;
    }
    if !config.profiles.iter().any(|profile| profile.id == config.active_profile_id) {
        config.active_profile_id = config.profiles[0].id.clone();
        write_config(app, &config)?;
    }
    Ok(config)
}

fn write_config(app: &AppHandle, config: &AppConfig) -> Result<(), String> {
    let path = config_path(app)?;
    let content = serde_json::to_vec_pretty(config).map_err(|error| error.to_string())?;
    write_replacing(&path, &content)
}

fn valid_profile_id(value: &str) -> bool {
    Uuid::parse_str(value).is_ok()
}

fn profile_user_dir(app: &AppHandle, profile_id: &str) -> Result<PathBuf, String> {
    if !valid_profile_id(profile_id) {
        return Err("Invalid profile identifier.".to_string());
    }
    let path = app_data_root(app)?.join("DolphinProfiles").join(profile_id).join("User");
    fs::create_dir_all(&path).map_err(|error| error.to_string())?;
    Ok(path)
}

fn resolve_dolphin_path(value: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(value);
    if !path.exists() {
        return Err("Dolphin path does not exist.".to_string());
    }
    if path.is_dir() {
        #[cfg(target_os = "macos")]
        {
            let candidate = path.join("Contents/MacOS/Dolphin");
            if candidate.is_file() {
                return Ok(candidate);
            }
        }
        return Err("Select the Dolphin executable inside the application bundle, not a folder.".to_string());
    }
    if !path.is_file() {
        return Err("The selected Dolphin path is not a file.".to_string());
    }
    Ok(path)
}

fn inspect_dolphin_path(value: &str) -> Result<DolphinInfo, String> {
    let executable = resolve_dolphin_path(value)?;
    let output = Command::new(&executable)
        .arg("--version")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|error| format!("Could not start Dolphin: {error}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Dolphin --version failed: {}", stderr.trim()));
    }
    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let version = text.lines().next().unwrap_or("Dolphin version detected").to_string();
    Ok(DolphinInfo {
        executable: executable.to_string_lossy().into_owned(),
        version,
    })
}

fn inspect_game_path(value: &str) -> Result<GameInfo, String> {
    let path = PathBuf::from(value);
    if !path.is_file() {
        return Err("The selected game image does not exist or is not a file.".to_string());
    }
    let metadata = fs::metadata(&path).map_err(|error| error.to_string())?;
    if metadata.len() < 1024 * 1024 {
        return Err("The selected image is unexpectedly small. Choose a complete Wii disc image.".to_string());
    }
    let extension = path.extension().and_then(|item| item.to_str()).unwrap_or("").to_ascii_lowercase();
    let supported = ["iso", "rvz", "wbfs", "wia", "gcz", "gcm", "ciso"];
    if !supported.contains(&extension.as_str()) {
        return Err("Unsupported image format. Supported: ISO, RVZ, WBFS, WIA, GCZ, GCM and CISO.".to_string());
    }

    let mut game_id = None;
    if extension == "iso" || extension == "gcm" {
        let mut file = File::open(&path).map_err(|error| error.to_string())?;
        let mut header = [0u8; 6];
        if file.read_exact(&mut header).is_ok() {
            if let Ok(id) = std::str::from_utf8(&header) {
                if id.chars().all(|character| character.is_ascii_alphanumeric()) {
                    game_id = Some(id.to_ascii_uppercase());
                }
            }
        }
    }

    let is_wsr = game_id.as_ref().map(|id| id.starts_with("RZT"));
    let note = match is_wsr {
        Some(true) => "Disc header matches the Wii Sports Resort RZT game-ID family.".to_string(),
        Some(false) => format!("Detected game ID {}; it does not match the known Wii Sports Resort RZT family.", game_id.as_deref().unwrap_or("unknown")),
        None => "Image format is supported. This format does not expose a raw game ID to the launcher; confirm the title in Dolphin before launching.".to_string(),
    };

    Ok(GameInfo {
        path: path.to_string_lossy().into_owned(),
        extension,
        size_bytes: metadata.len(),
        game_id,
        is_wii_sports_resort: is_wsr,
        note,
    })
}

#[tauri::command]
pub fn load_config(app: AppHandle) -> Result<AppConfig, String> {
    read_config(&app)
}

#[tauri::command]
pub fn save_config(app: AppHandle, mut config: AppConfig) -> Result<(), String> {
    if config.profiles.is_empty() {
        return Err("At least one profile is required.".to_string());
    }
    if config.profiles.len() > 100 {
        return Err("A maximum of 100 profiles is supported.".to_string());
    }
    for profile in &config.profiles {
        if !valid_profile_id(&profile.id) {
            return Err("A profile has an invalid ID. Re-create that profile.".to_string());
        }
        if profile.name.trim().is_empty() || profile.name.len() > 60 {
            return Err("Profile names must contain 1–60 characters.".to_string());
        }
        if !(1..=12).contains(&profile.graphics.internal_resolution) {
            return Err("Internal resolution must be between 1× and 12×.".to_string());
        }
        if ![1, 2, 4, 8, 16].contains(&profile.graphics.msaa) {
            return Err("MSAA must be off/1×, 2×, 4×, 8× or 16×.".to_string());
        }
        if !["real", "emulated"].contains(&profile.controller.mode.as_str()) {
            return Err("Controller mode must be real or emulated.".to_string());
        }
    }
    if !config.profiles.iter().any(|profile| profile.id == config.active_profile_id) {
        config.active_profile_id = config.profiles[0].id.clone();
    }
    config.schema_version = 1;
    write_config(&app, &config)
}

#[tauri::command]
pub fn inspect_dolphin(path: String) -> Result<DolphinInfo, String> {
    inspect_dolphin_path(&path)
}

#[tauri::command]
pub fn inspect_game(path: String) -> Result<GameInfo, String> {
    inspect_game_path(&path)
}

#[tauri::command]
pub fn launch_game(
    app: AppHandle,
    state: State<'_, ProcessState>,
    dolphin_path: String,
    game_path: String,
    profile_id: String,
) -> Result<LaunchResult, String> {
    let dolphin = inspect_dolphin_path(&dolphin_path)?;
    let game = inspect_game_path(&game_path)?;
    if game.is_wii_sports_resort == Some(false) {
        return Err("The selected image's disc ID is not in the Wii Sports Resort RZT family.".to_string());
    }
    let config = read_config(&app)?;
    let profile = config.profiles.iter().find(|profile| profile.id == profile_id)
        .ok_or_else(|| "The selected profile no longer exists.".to_string())?;
    let user_dir = profile_user_dir(&app, &profile_id)?;
    let game_id = game.game_id.as_deref().or(config.game_id.as_deref())
        .ok_or_else(|| "A six-character Wii Sports Resort Game ID is required. Confirm it in Settings before launching this image.".to_string())?;
    if !game_id.to_ascii_uppercase().starts_with("RZT") {
        return Err("The Game ID must begin with RZT for Wii Sports Resort.".to_string());
    }
    write_controller_source(&user_dir, game_id, &profile.controller.mode)?;
    fs::create_dir_all(user_dir.join("ResourcePacks")).map_err(|error| error.to_string())?;
    fs::create_dir_all(user_dir.join("Load/Riivolution")).map_err(|error| error.to_string())?;

    let mut child_guard = state.0.lock().map_err(|_| "Emulator process lock is unavailable.".to_string())?;
    if let Some(child) = child_guard.as_mut() {
        match child.try_wait().map_err(|error| error.to_string())? {
            None => return Err("Dolphin is already running from this launcher.".to_string()),
            Some(_) => { child_guard.take(); }
        }
    }

    let graphics = &profile.graphics;
    let anisotropy_mode = match graphics.anisotropy {
        1 => 0,
        2 => 1,
        4 => 2,
        8 => 3,
        16 => 4,
        _ => -1,
    };
    let mut arguments = vec![
        "--user".to_string(),
        user_dir.to_string_lossy().into_owned(),
        "--exec".to_string(),
        game.path.clone(),
        format!("--config=GFX.Settings.InternalResolution={}", graphics.internal_resolution),
        "--config=GFX.Settings.MaxInternalResolution=12".to_string(),
        format!("--config=GFX.Settings.MSAA={}", graphics.msaa),
        format!("--config=GFX.Enhancements.MaxAnisotropy={}", anisotropy_mode),
        "--config=GFX.Enhancements.ForceTextureFiltering=2".to_string(),
        format!("--config=GFX.Hacks.VertexRounding={}", graphics.vertex_rounding),
        format!("--config=GFX.Settings.ShowFPS={}", graphics.show_fps),
        format!("--config=GFX.Settings.BorderlessFullscreen={}", graphics.borderless_fullscreen),
    ];
    if profile.mods.iter().any(|item| item.kind == "resource_pack" && item.enabled) {
        arguments.push("--config=GFX.Settings.HiresTextures=True".to_string());
        arguments.push("--config=GFX.Settings.CacheHiresTextures=True".to_string());
    }
    if profile.gecko_codes.iter().any(|code| code.enabled) {
        arguments.push("--config=Core.EnableCheats=True".to_string());
    }

    let executable = PathBuf::from(&dolphin.executable);
    let mut command = Command::new(&executable);
    command.args(&arguments).stdout(Stdio::null()).stderr(Stdio::null());
    if let Some(parent) = executable.parent() {
        command.current_dir(parent);
    }
    let child = command.spawn().map_err(|error| format!("Could not launch Dolphin: {error}"))?;
    let pid = child.id();
    *child_guard = Some(child);

    Ok(LaunchResult {
        pid,
        user_data_path: user_dir.to_string_lossy().into_owned(),
        arguments,
    })
}

#[tauri::command]
pub fn launch_dolphin_ui(
    app: AppHandle,
    state: State<'_, ProcessState>,
    dolphin_path: Option<String>,
    profile_id: String,
) -> Result<LaunchResult, String> {
    let path = dolphin_path.ok_or_else(|| "Select a Dolphin executable first.".to_string())?;
    let dolphin = inspect_dolphin_path(&path)?;
    let user_dir = profile_user_dir(&app, &profile_id)?;
    let mut child_guard = state.0.lock().map_err(|_| "Emulator process lock is unavailable.".to_string())?;
    if let Some(child) = child_guard.as_mut() {
        match child.try_wait().map_err(|error| error.to_string())? {
            None => return Err("Dolphin is already running.".to_string()),
            Some(_) => { child_guard.take(); }
        }
    }
    let arguments = vec!["--user".to_string(), user_dir.to_string_lossy().into_owned()];
    let child = Command::new(&dolphin.executable)
        .args(&arguments).stdout(Stdio::null()).stderr(Stdio::null()).spawn()
        .map_err(|error| error.to_string())?;
    let pid = child.id();
    *child_guard = Some(child);
    Ok(LaunchResult { pid, user_data_path: user_dir.to_string_lossy().into_owned(), arguments })
}

#[tauri::command]
pub fn poll_launch_status(state: State<'_, ProcessState>) -> Result<LaunchStatus, String> {
    let mut guard = state.0.lock().map_err(|_| "Emulator process lock is unavailable.".to_string())?;
    if let Some(child) = guard.as_mut() {
        match child.try_wait().map_err(|error| error.to_string())? {
            None => Ok(LaunchStatus { running: true, pid: Some(child.id()), exit_code: None }),
            Some(status) => {
                let code = status.code();
                guard.take();
                Ok(LaunchStatus { running: false, pid: None, exit_code: code })
            }
        }
    } else {
        Ok(LaunchStatus { running: false, pid: None, exit_code: None })
    }
}

fn copy_dir(source: &Path, destination: &Path) -> Result<(), String> {
    fs::create_dir_all(destination).map_err(|error| error.to_string())?;
    for entry in fs::read_dir(source).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let metadata = fs::symlink_metadata(&source_path).map_err(|error| error.to_string())?;
        if metadata.file_type().is_symlink() {
            return Err("Mod archives may not contain symbolic links.".to_string());
        }
        if metadata.is_dir() {
            copy_dir(&source_path, &destination_path)?;
        } else if metadata.is_file() {
            if let Some(parent) = destination_path.parent() {
                fs::create_dir_all(parent).map_err(|error| error.to_string())?;
            }
            fs::copy(&source_path, &destination_path).map_err(|error| error.to_string())?;
        }
    }
    Ok(())
}

fn find_named_file(root: &Path, name: &str) -> Result<Option<PathBuf>, String> {
    if !root.is_dir() { return Ok(None); }
    for entry in fs::read_dir(root).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(found) = find_named_file(&path, name)? { return Ok(Some(found)); }
        } else if path.file_name().and_then(|value| value.to_str()).map(|value| value.eq_ignore_ascii_case(name)).unwrap_or(false) {
            return Ok(Some(path));
        }
    }
    Ok(None)
}

fn find_first_xml(root: &Path) -> Result<Option<PathBuf>, String> {
    if !root.is_dir() { return Ok(None); }
    for entry in fs::read_dir(root).map_err(|error| error.to_string())? {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.is_dir() {
            if let Some(found) = find_first_xml(&path)? { return Ok(Some(found)); }
        } else if path.extension().and_then(|value| value.to_str()).map(|value| value.eq_ignore_ascii_case("xml")).unwrap_or(false) {
            return Ok(Some(path));
        }
    }
    Ok(None)
}

fn extract_safe_zip(archive_path: &Path, destination: &Path) -> Result<(), String> {
    let file = File::open(archive_path).map_err(|error| error.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|error| format!("Invalid ZIP archive: {error}"))?;
    if archive.len() > 20_000 { return Err("Archive contains too many files.".to_string()); }
    fs::create_dir_all(destination).map_err(|error| error.to_string())?;
    let mut total_size: u64 = 0;
    for index in 0..archive.len() {
        let mut item = archive.by_index(index).map_err(|error| error.to_string())?;
        let relative = item.enclosed_name().ok_or_else(|| "Archive contains an unsafe path.".to_string())?.to_path_buf();
        if item.unix_mode().map(|mode| mode & 0o170000 == 0o120000).unwrap_or(false) {
            return Err("Archive contains a symbolic link.".to_string());
        }
        total_size = total_size.saturating_add(item.size());
        if total_size > 8 * 1024 * 1024 * 1024 { return Err("Archive expands beyond the 8 GiB safety limit.".to_string()); }
        let output = destination.join(relative);
        if item.is_dir() {
            fs::create_dir_all(&output).map_err(|error| error.to_string())?;
        } else {
            if let Some(parent) = output.parent() { fs::create_dir_all(parent).map_err(|error| error.to_string())?; }
            let mut out = File::create(&output).map_err(|error| error.to_string())?;
            std::io::copy(&mut item, &mut out).map_err(|error| error.to_string())?;
            out.flush().map_err(|error| error.to_string())?;
        }
    }
    Ok(())
}

fn mod_storage_paths(user_dir: &Path, profile_id: &str, kind: &str, mod_id: &str) -> Result<(PathBuf, PathBuf), String> {
    if !valid_profile_id(profile_id) || !valid_profile_id(mod_id) {
        return Err("Invalid profile or mod identifier.".to_string());
    }
    match kind {
        "riivolution" => Ok((
            user_dir.join("Load/Riivolution").join(mod_id),
            user_dir.join("ModStash").join(profile_id).join("Riivolution").join(mod_id),
        )),
        "resource_pack" => Ok((
            user_dir.join("ResourcePacks").join(format!("{mod_id}.zip")),
            user_dir.join("ModStash").join(profile_id).join("ResourcePacks").join(format!("{mod_id}.zip")),
        )),
        _ => Err("Unsupported modification type.".to_string()),
    }
}

#[tauri::command]
pub fn import_mod_archive(
    app: AppHandle,
    profile_id: String,
    archive_path: String,
    kind: String,
) -> Result<ModEntry, String> {
    if !valid_profile_id(&profile_id) { return Err("Invalid profile identifier.".to_string()); }
    if !["riivolution", "resource_pack"].contains(&kind.as_str()) { return Err("Unsupported mod type.".to_string()); }
    let archive_path = PathBuf::from(archive_path);
    if !archive_path.is_file() || archive_path.extension().and_then(|value| value.to_str()).unwrap_or("").to_ascii_lowercase() != "zip" {
        return Err("Choose a ZIP archive.".to_string());
    }
    let user_dir = profile_user_dir(&app, &profile_id)?;
    let id = Uuid::new_v4().to_string();
    let staging = app_data_root(&app)?.join("ModStaging").join(&id);
    if staging.exists() { fs::remove_dir_all(&staging).map_err(|error| error.to_string())?; }
    extract_safe_zip(&archive_path, &staging)?;

    let (name, version, game_prefix, xml_path, pack_root) = if kind == "riivolution" {
        let xml = find_named_file(&staging, "riivolution.xml")?
            .or(find_named_file(&staging, "patch.xml")?)
            .or(find_first_xml(&staging)?)
            .ok_or_else(|| "No Riivolution XML file was found in the archive.".to_string())?;
        let xml_text = fs::read_to_string(&xml).map_err(|error| error.to_string())?;
        let document = Document::parse(&xml_text).map_err(|error| format!("Riivolution XML is invalid: {error}"))?;
        if document.root_element().tag_name().name() != "wiidisc" {
            return Err("The XML root is not a Riivolution wiidisc document.".to_string());
        }
        let game_prefix = document.descendants().find(|node| node.has_tag_name("id")).and_then(|node| node.attribute("game")).map(str::to_string);
        let xml_name = document.descendants().find(|node| node.has_tag_name("name")).and_then(|node| node.text()).map(|value| value.trim().to_string());
        let fallback = archive_path.file_stem().and_then(|value| value.to_str()).unwrap_or("Riivolution patch").to_string();
        let relative = xml.strip_prefix(&staging).map_err(|error| error.to_string())?.to_string_lossy().into_owned();
        (xml_name.unwrap_or(fallback), None, game_prefix, Some(relative), staging.clone())
    } else {
        let manifest_path = find_named_file(&staging, "manifest.json")?
            .ok_or_else(|| "No manifest.json was found. Select a Dolphin Resource Pack ZIP.".to_string())?;
        if manifest_path.parent() != Some(staging.as_path()) {
            return Err("Dolphin Resource Pack manifest.json must be at the root of the ZIP archive.".to_string());
        }
        let manifest_text = fs::read_to_string(&manifest_path).map_err(|error| error.to_string())?;
        let manifest: Value = serde_json::from_str(&manifest_text).map_err(|error| format!("Invalid resource pack manifest: {error}"))?;
        let name = manifest.get("name").and_then(Value::as_str).ok_or_else(|| "Resource pack manifest is missing its name.".to_string())?.to_string();
        let version = manifest.get("version").and_then(Value::as_str).map(str::to_string);
        let root = manifest_path.parent().ok_or_else(|| "Resource pack root could not be determined.".to_string())?.to_path_buf();
        (name, version, None, None, root)
    };

    let (active_path, _) = mod_storage_paths(&user_dir, &profile_id, &kind, &id)?;
    if let Some(parent) = active_path.parent() { fs::create_dir_all(parent).map_err(|error| error.to_string())?; }
    if kind == "resource_pack" {
        fs::copy(&archive_path, &active_path).map_err(|error| error.to_string())?;
    } else {
        copy_dir(&pack_root, &active_path)?;
    }
    let _ = fs::remove_dir_all(&staging);

    let entry = ModEntry { id, name, kind, enabled: true, game_prefix, xml_path, version };
    let mut config = read_config(&app)?;
    let profile = config.profiles.iter_mut().find(|profile| profile.id == profile_id).ok_or_else(|| "Profile not found.".to_string())?;
    profile.mods.push(entry.clone());
    write_config(&app, &config)?;
    Ok(entry)
}

#[tauri::command]
pub fn set_mod_enabled(app: AppHandle, profile_id: String, mod_id: String, enabled: bool) -> Result<(), String> {
    let mut config = read_config(&app)?;
    let profile = config.profiles.iter_mut().find(|profile| profile.id == profile_id).ok_or_else(|| "Profile not found.".to_string())?;
    let item = profile.mods.iter_mut().find(|item| item.id == mod_id).ok_or_else(|| "Mod not found.".to_string())?;
    if item.enabled == enabled { return Ok(()); }
    let user_dir = profile_user_dir(&app, &profile_id)?;
    let (active, disabled) = mod_storage_paths(&user_dir, &profile_id, &item.kind, &item.id)?;
    if enabled {
        if !disabled.exists() { return Err("Disabled mod folder is missing.".to_string()); }
        if let Some(parent) = active.parent() { fs::create_dir_all(parent).map_err(|error| error.to_string())?; }
        fs::rename(&disabled, &active).map_err(|error| format!("Could not enable mod: {error}"))?;
    } else {
        if !active.exists() { return Err("Installed mod folder is missing.".to_string()); }
        if let Some(parent) = disabled.parent() { fs::create_dir_all(parent).map_err(|error| error.to_string())?; }
        fs::rename(&active, &disabled).map_err(|error| format!("Could not disable mod: {error}"))?;
    }
    item.enabled = enabled;
    write_config(&app, &config)
}

#[tauri::command]
pub fn remove_mod(app: AppHandle, profile_id: String, mod_id: String) -> Result<(), String> {
    let mut config = read_config(&app)?;
    let profile = config.profiles.iter_mut().find(|profile| profile.id == profile_id).ok_or_else(|| "Profile not found.".to_string())?;
    let index = profile.mods.iter().position(|item| item.id == mod_id).ok_or_else(|| "Mod not found.".to_string())?;
    let item = profile.mods.remove(index);
    let user_dir = profile_user_dir(&app, &profile_id)?;
    let (active, disabled) = mod_storage_paths(&user_dir, &profile_id, &item.kind, &item.id)?;
    for path in [active, disabled] {
        if path.exists() {
            if item.kind == "resource_pack" { fs::remove_file(path).map_err(|error| error.to_string())?; }
            else { fs::remove_dir_all(path).map_err(|error| error.to_string())?; }
        }
    }
    write_config(&app, &config)
}

fn set_ini_value(content: &str, section: &str, key: &str, value: &str) -> String {
    let mut lines: Vec<String> = content.lines().map(str::to_string).collect();
    let header = format!("[{section}]");
    let section_start = lines.iter().position(|line| line.trim().eq_ignore_ascii_case(&header));
    match section_start {
        Some(start) => {
            let end = lines.iter().enumerate().skip(start + 1)
                .find(|(_, line)| {
                    let trimmed = line.trim();
                    trimmed.starts_with('[') && trimmed.ends_with(']')
                })
                .map(|(index, _)| index)
                .unwrap_or(lines.len());
            let key_prefix = format!("{key}=");
            let key_prefix_spaced = format!("{key} =");
            let existing: Vec<usize> = (start + 1..end)
                .filter(|index| {
                    let trimmed = lines[*index].trim();
                    trimmed.starts_with(&key_prefix) || trimmed.starts_with(&key_prefix_spaced)
                })
                .collect();
            if let Some(first) = existing.first().copied() {
                lines[first] = format!("{key} = {value}");
                for duplicate in existing.into_iter().skip(1).rev() {
                    lines.remove(duplicate);
                }
            } else {
                lines.insert(end, format!("{key} = {value}"));
            }
        }
        None => {
            if !lines.is_empty() && !lines.last().unwrap().is_empty() {
                lines.push(String::new());
            }
            lines.push(header);
            lines.push(format!("{key} = {value}"));
        }
    }
    let mut result = lines.join("\n");
    result.push('\n');
    result
}

fn write_controller_source(user_dir: &Path, game_id: &str, mode: &str) -> Result<(), String> {
    let game_id = game_id.to_ascii_uppercase();
    if game_id.len() != 6 || !game_id.chars().all(|character| character.is_ascii_alphanumeric()) {
        return Err("Game ID must contain six letters or digits before applying the controller profile.".to_string());
    }
    let source = match mode {
        "real" => "2",
        "emulated" => "1",
        _ => return Err("Controller source must be real or emulated.".to_string()),
    };
    let path = user_dir.join("GameSettings").join(format!("{game_id}.ini"));
    let existing = if path.exists() { fs::read_to_string(&path).map_err(|error| error.to_string())? } else { String::new() };
    let updated = set_ini_value(&existing, "Controls", "WiimoteSource0", source);
    write_replacing(&path, updated.as_bytes())
}

fn validate_gecko_code(code: &str) -> Result<String, String> {
    let mut lines = Vec::new();
    for line in code.lines().map(str::trim).filter(|line| !line.is_empty()) {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() != 2 || tokens.iter().any(|token| token.len() != 8 || !token.chars().all(|character| character.is_ascii_hexdigit())) {
            return Err("Each Gecko code line must contain exactly two eight-character hexadecimal values.".to_string());
        }
        lines.push(tokens.join(" "));
    }
    if lines.is_empty() { return Err("The Gecko code is empty.".to_string()); }
    Ok(lines.join("\n"))
}

fn update_gecko_section(path: &Path, codes: &[GeckoCode]) -> Result<(), String> {
    let mut content = if path.exists() { fs::read_to_string(path).map_err(|error| error.to_string())? } else { String::new() };
    let start_marker = "; WSR Enhanced managed Gecko codes begin";
    let end_marker = "; WSR Enhanced managed Gecko codes end";
    if let (Some(start), Some(end)) = (content.find(start_marker), content.find(end_marker)) {
        let end = end + end_marker.len();
        content.replace_range(start..end, "");
    }
    let mut block = String::from(start_marker);
    for code in codes.iter().filter(|code| code.enabled) {
        block.push_str("\n$");
        block.push_str(&code.name.replace('\n', " ").replace('\r', " "));
        block.push('\n');
        block.push_str(&validate_gecko_code(&code.code)?);
        block.push('\n');
    }
    block.push_str("\n");
    block.push_str(end_marker);
    if let Some(section_start) = content.find("[Gecko]") {
        let after_header = section_start + "[Gecko]".len();
        let section_end = content[after_header..].find("\n[").map(|offset| after_header + offset).unwrap_or(content.len());
        content.insert_str(section_end, &format!("\n{block}\n"));
    } else {
        content.push_str("\n\n[Gecko]\n");
        content.push_str(&block);
        content.push('\n');
    }
    write_replacing(path, content.as_bytes())
}

#[tauri::command]
pub fn save_gecko_codes(app: AppHandle, profile_id: String, game_id: String, codes: Vec<GeckoCode>) -> Result<(), String> {
    if !valid_profile_id(&profile_id) { return Err("Invalid profile identifier.".to_string()); }
    let game_id = game_id.to_ascii_uppercase();
    if game_id.len() != 6 || !game_id.chars().all(|character| character.is_ascii_alphanumeric()) {
        return Err("Game ID must contain six letters or digits.".to_string());
    }
    for code in &codes { validate_gecko_code(&code.code)?; }
    let user_dir = profile_user_dir(&app, &profile_id)?;
    let path = user_dir.join("GameSettings").join(format!("{game_id}.ini"));
    update_gecko_section(&path, &codes)?;
    let mut config = read_config(&app)?;
    let profile = config.profiles.iter_mut().find(|profile| profile.id == profile_id).ok_or_else(|| "Profile not found.".to_string())?;
    profile.gecko_codes = codes;
    write_config(&app, &config)
}

fn timestamp() -> String {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos().to_string()
}

fn copy_if_exists(source: &Path, destination: &Path) -> Result<(), String> {
    if source.is_dir() { copy_dir(source, destination)?; }
    Ok(())
}

fn create_backup(app: &AppHandle, profile_id: &str) -> Result<BackupInfo, String> {
    let user_dir = profile_user_dir(app, profile_id)?;
    let id = timestamp();
    let backup_path = app_data_root(app)?.join("Backups").join(profile_id).join(&id);
    fs::create_dir_all(&backup_path).map_err(|error| error.to_string())?;
    for relative in ["Wii/title", "StateSaves", "Screenshots"] {
        let source = user_dir.join(relative);
        let destination = backup_path.join(relative);
        if source.is_dir() {
            copy_dir(&source, &destination)?;
        } else {
            fs::create_dir_all(&destination).map_err(|error| error.to_string())?;
        }
    }
    let manifest = serde_json::json!({"profileId":profile_id,"createdAtUnixNanos":id,"contents":["Wii/title","StateSaves","Screenshots"]});
    fs::write(backup_path.join("backup.json"), serde_json::to_vec_pretty(&manifest).map_err(|error| error.to_string())?)
        .map_err(|error| error.to_string())?;
    Ok(BackupInfo { id, path: backup_path.to_string_lossy().into_owned() })
}

#[tauri::command]
pub fn backup_profile_data(app: AppHandle, profile_id: String) -> Result<BackupInfo, String> {
    create_backup(&app, &profile_id)
}

#[tauri::command]
pub fn list_backups(app: AppHandle, profile_id: String) -> Result<Vec<BackupInfo>, String> {
    if !valid_profile_id(&profile_id) { return Err("Invalid profile identifier.".to_string()); }
    let root = app_data_root(&app)?.join("Backups").join(&profile_id);
    if !root.exists() { return Ok(Vec::new()); }
    let mut backups = Vec::new();
    for entry in fs::read_dir(root).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        if entry.path().is_dir() {
            backups.push(BackupInfo { id: entry.file_name().to_string_lossy().into_owned(), path: entry.path().to_string_lossy().into_owned() });
        }
    }
    backups.sort_by(|left, right| right.id.cmp(&left.id));
    Ok(backups)
}

#[tauri::command]
pub fn restore_backup(app: AppHandle, profile_id: String, backup_id: String) -> Result<(), String> {
    if !valid_profile_id(&profile_id) || !backup_id.chars().all(|character| character.is_ascii_alphanumeric() || character == '-') {
        return Err("Invalid backup identifier.".to_string());
    }
    let _safety_backup = create_backup(&app, &profile_id)?;
    let backup_path = app_data_root(&app)?.join("Backups").join(&profile_id).join(&backup_id);
    if !backup_path.join("backup.json").is_file() { return Err("Backup was not found or is incomplete.".to_string()); }
    let user_dir = profile_user_dir(&app, &profile_id)?;
    for relative in ["Wii/title", "StateSaves", "Screenshots"] {
        let source = backup_path.join(relative);
        if source.is_dir() {
            let destination = user_dir.join(relative);
            if destination.exists() { fs::remove_dir_all(&destination).map_err(|error| error.to_string())?; }
            copy_dir(&source, &destination)?;
        }
    }
    Ok(())
}

#[tauri::command]
pub fn get_profile_data_path(app: AppHandle, profile_id: String) -> Result<String, String> {
    Ok(profile_user_dir(&app, &profile_id)?.to_string_lossy().into_owned())
}

#[tauri::command]
pub fn open_folder(path: String) -> Result<(), String> {
    let folder = PathBuf::from(path);
    fs::create_dir_all(&folder).map_err(|error| error.to_string())?;
    #[cfg(target_os = "macos")]
    let mut command = Command::new("open");
    #[cfg(target_os = "windows")]
    let mut command = Command::new("explorer");
    #[cfg(target_os = "linux")]
    let mut command = Command::new("xdg-open");
    command.arg(folder).spawn().map_err(|error| error.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_non_uuid_profile_paths() {
        assert!(!valid_profile_id("../../outside"));
        assert!(!valid_profile_id("profile"));
    }

    #[test]
    fn accepts_generated_uuid_profile_ids() {
        assert!(valid_profile_id(&Uuid::new_v4().to_string()));
    }

    #[test]
    fn resource_pack_paths_are_zip_files_and_riivolution_paths_are_folders() {
        let profile_id = Uuid::new_v4().to_string();
        let mod_id = Uuid::new_v4().to_string();
        let root = Path::new("/tmp/wsr-user");
        let (pack_active, pack_disabled) = mod_storage_paths(root, &profile_id, "resource_pack", &mod_id).unwrap();
        assert_eq!(pack_active, root.join("ResourcePacks").join(format!("{mod_id}.zip")));
        assert_eq!(pack_disabled, root.join("ModStash").join(&profile_id).join("ResourcePacks").join(format!("{mod_id}.zip")));
        let (patch_active, patch_disabled) = mod_storage_paths(root, &profile_id, "riivolution", &mod_id).unwrap();
        assert_eq!(patch_active, root.join("Load/Riivolution").join(&mod_id));
        assert_eq!(patch_disabled, root.join("ModStash").join(&profile_id).join("Riivolution").join(&mod_id));
    }

    #[test]
    fn updates_gameini_values_without_discarding_other_settings() {
        let original = "[Controls]\\nPadType0 = 6\\nWiimoteSource0 = 1\\n\\n[Core]\\nEnableCheats = True\\n";
        let updated = set_ini_value(original, "Controls", "WiimoteSource0", "2");
        assert!(updated.contains("PadType0 = 6"));
        assert!(updated.contains("WiimoteSource0 = 2"));
        assert!(updated.contains("EnableCheats = True"));
        assert_eq!(updated.matches("WiimoteSource0").count(), 1);
    }

    #[test]
    fn validates_gecko_code_lines() {
        assert_eq!(validate_gecko_code("04000000 1234ABCD").unwrap(), "04000000 1234ABCD");
        assert!(validate_gecko_code("04ZZ0000 1234ABCD").is_err());
        assert!(validate_gecko_code("1234567").is_err());
        assert!(validate_gecko_code("12345678").is_err());
        assert!(validate_gecko_code("12345678 12345678 12345678").is_err());
    }
}
