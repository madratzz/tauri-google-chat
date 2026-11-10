//! Tiny persisted preferences (icon variant, zoom, title bar theme) stored as JSON in the app config dir.

use std::{fs, path::PathBuf, sync::Mutex};

use serde_json::{json, Value};
use tauri::{AppHandle, Manager};

use crate::util::clamp_zoom;

#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    pub icon: String,
    pub zoom: f64,
    /// "system", "light" or "dark".
    pub title_bar: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            icon: "color".into(),
            zoom: 1.0,
            title_bar: "system".into(),
        }
    }
}

pub struct SettingsState(pub Mutex<Settings>);

fn path(app: &AppHandle) -> Option<PathBuf> {
    app.path()
        .app_config_dir()
        .ok()
        .map(|d| d.join("settings.json"))
}

pub fn load(app: &AppHandle) -> Settings {
    let mut s = Settings::default();
    let Some(v) = path(app)
        .and_then(|p| fs::read_to_string(p).ok())
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
    else {
        return s;
    };
    if let Some(icon) = v.get("icon").and_then(Value::as_str) {
        if matches!(icon, "color" | "dark" | "white") {
            s.icon = icon.into();
        }
    }
    if let Some(t) = v.get("title_bar").and_then(Value::as_str) {
        if matches!(t, "system" | "light" | "dark") {
            s.title_bar = t.into();
        }
    }
    if let Some(z) = v.get("zoom").and_then(Value::as_f64) {
        s.zoom = clamp_zoom(z);
    }
    s
}

/// Applies `f` to the stored settings and writes them to disk (best effort).
pub fn update(app: &AppHandle, f: impl FnOnce(&mut Settings)) -> Settings {
    let state = app.state::<SettingsState>();
    let snapshot = match state.0.lock() {
        Ok(mut s) => {
            f(&mut s);
            s.clone()
        }
        Err(_) => return Settings::default(),
    };
    if let Some(p) = path(app) {
        if let Some(dir) = p.parent() {
            let _ = fs::create_dir_all(dir);
        }
        let body = json!({ "icon": snapshot.icon, "zoom": snapshot.zoom, "title_bar": snapshot.title_bar });
        if let Err(e) = fs::write(&p, body.to_string()) {
            eprintln!("failed to save settings: {e}");
        }
    }
    snapshot
}
