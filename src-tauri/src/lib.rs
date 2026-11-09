mod settings;
mod util;

use std::sync::{
    atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering},
    Mutex,
};
use std::time::Duration;

use tauri::{
    image::Image,
    menu::{Menu, MenuItem, PredefinedMenuItem, Submenu},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    webview::{NewWindowResponse, WebviewBuilder},
    AppHandle, LogicalPosition, LogicalSize, Manager, Runtime, Url, WebviewUrl,
    WebviewWindowBuilder, WindowEvent,
};
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_opener::OpenerExt;
use tauri_plugin_window_state::StateFlags;

use settings::{Settings, SettingsState};
use util::{
    clamp_zoom, is_in_app_url, is_openable_externally, is_sign_in_url, title_for_url,
    unread_from_title,
};

const CHAT_URL: &str = "https://chat.google.com/";
// Intentional: Google blocks some embedded-browser user agents. Retest before changing.
const SAFARI_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.4 Safari/605.1.15";
const COLOR_ICON: &[u8] = include_bytes!("../icons/google-chat-color.png");
const DARK_ICON: &[u8] = include_bytes!("../icons/google-chat-dark.png");
const WHITE_ICON: &[u8] = include_bytes!("../icons/google-chat-white.png");
const PEEK_ACTION_HOST: &str = "peek-action.tauri.internal";

static CHILD_WINDOW_COUNTER: AtomicU64 = AtomicU64::new(1);
/// Bumped on every resize; a deferred reposition only runs if it is still the latest.
static RESIZE_GENERATION: AtomicU64 = AtomicU64::new(0);
static LAST_UNREAD: AtomicI64 = AtomicI64::new(0);
/// Hide-to-tray is only safe when a tray icon exists to bring the window back.
static TRAY_READY: AtomicBool = AtomicBool::new(false);

// Holds the real URL of the current peek webview.
// On Windows/WebView2, on_navigation fires on a background thread and
// the navigation cancel is async — by the time expand_peek runs,
// peek.url() already returns the sentinel URL, not the real content URL.
struct PeekUrl(Mutex<Option<Url>>);

/// Injected (as an initialization script) into peek webviews to render a
/// floating toolbar. Buttons navigate to sentinel URLs that `on_navigation`
/// intercepts on the Rust side.
const PEEK_TOOLBAR_JS: &str = r#"
(function() {
    if (window.top !== window) return;
    function inject() {
        if (document.getElementById('_peek_toolbar') || !document.body) return;
        var bar = document.createElement('div');
        bar.id = '_peek_toolbar';
        bar.setAttribute('role', 'toolbar');
        bar.style.cssText = 'position:fixed;bottom:0;left:0;right:0;height:44px;background:rgb(32,33,36);display:flex;align-items:center;justify-content:flex-end;padding:0 14px;gap:10px;z-index:2147483647;border-top:1px solid rgba(255,255,255,0.08);font-family:-apple-system,BlinkMacSystemFont,"Segoe UI",system-ui,sans-serif;box-sizing:border-box;';
        function button(label, title, bg, fg, action) {
            var b = document.createElement('button');
            b.textContent = label;
            b.title = title;
            b.style.cssText = 'background:' + bg + ';color:' + fg + ';border:none;padding:7px 16px;border-radius:20px;cursor:pointer;font-size:12px;font-weight:600;';
            b.onclick = function(e) { e.preventDefault(); window.location.href = 'https://peek-action.tauri.internal/' + action; };
            return b;
        }
        bar.appendChild(button('↗ Pop Out', 'Open in a separate window', '#8ab4f8', '#202124', 'expand'));
        bar.appendChild(button('✕ Close', 'Close (Esc)', 'rgba(255,255,255,0.12)', '#e8eaed', 'close'));
        document.body.appendChild(bar);
        document.body.style.paddingBottom = '44px';
    }
    document.addEventListener('keydown', function(e) {
        if (e.key === 'Escape' && !e.defaultPrevented) window.location.href = 'https://peek-action.tauri.internal/close';
    });
    document.addEventListener('DOMContentLoaded', inject);
    window.addEventListener('load', inject);
})();
"#;

/// Main-window script: shows an offline banner with Retry, and reloads when
/// the connection returns.
const OFFLINE_JS: &str = r#"
(function() {
    if (window.top !== window) return;
    function show() {
        if (document.getElementById('_gcd_offline') || !document.body) return;
        var b = document.createElement('div');
        b.id = '_gcd_offline';
        b.setAttribute('role', 'alert');
        b.style.cssText = 'position:fixed;top:12px;left:50%;transform:translateX(-50%);z-index:2147483647;background:#202124;color:#e8eaed;padding:10px 16px;border-radius:8px;font:13px system-ui,sans-serif;box-shadow:0 2px 8px rgba(0,0,0,.4);display:flex;gap:12px;align-items:center;';
        b.textContent = 'You are offline. Messages will sync when you reconnect.';
        var r = document.createElement('button');
        r.textContent = 'Retry';
        r.style.cssText = 'background:#8ab4f8;color:#202124;border:none;border-radius:14px;padding:4px 12px;cursor:pointer;font-weight:600;';
        r.onclick = function() { location.reload(); };
        b.appendChild(r);
        document.body.appendChild(b);
    }
    window.addEventListener('offline', show);
    window.addEventListener('online', function() {
        if (document.getElementById('_gcd_offline')) location.reload();
    });
    document.addEventListener('DOMContentLoaded', function() { if (!navigator.onLine) show(); });
})();
"#;

pub fn run() {
    let app = tauri::Builder::default()
        // Must be registered first: a second launch focuses the existing window.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            show_main(app)
        }))
        .plugin(
            tauri_plugin_window_state::Builder::default()
                // Never restore a hidden (tray) state, and only track the main window.
                .with_state_flags(StateFlags::all() - StateFlags::VISIBLE)
                .with_filter(|label| label == "main")
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .manage(PeekUrl(Mutex::new(None)))
        .setup(|app| {
            let handle = app.handle().clone();
            let prefs = settings::load(&handle);
            app.manage(SettingsState(Mutex::new(prefs.clone())));

            let app_new_window = handle.clone();
            let main = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External(CHAT_URL.parse().expect("valid Google Chat URL")),
            )
            .title("Google Chat")
            .inner_size(1280.0, 860.0)
            .min_inner_size(600.0, 400.0)
            .resizable(true)
            .user_agent(SAFARI_USER_AGENT)
            .initialization_script(OFFLINE_JS)
            .on_new_window(move |url, _| {
                eprintln!("[gcd] new window requested: {url}");
                let app = app_new_window.clone();
                if is_sign_in_url(&url) {
                    // Sign-in must run in the main window, not the peek overlay.
                    tauri::async_runtime::spawn(async move { navigate_main(&app, url) });
                } else if is_in_app_url(&url) {
                    // Defer: creating a webview inside this callback is unsafe on
                    // macOS and Windows.
                    tauri::async_runtime::spawn(async move { create_peek_overlay(&app, url) });
                } else {
                    open_externally(&app, &url);
                }
                NewWindowResponse::Deny
            })
            .on_page_load(|webview, payload| {
                eprintln!("[gcd] main page {:?}: {}", payload.event(), payload.url());
                let _ = webview;
            })
            .on_document_title_changed({
                let app = handle.clone();
                move |_, title| update_unread(&app, &title)
            })
            .build()?;

            if (prefs.zoom - 1.0).abs() > f64::EPSILON {
                let _ = main.set_zoom(prefs.zoom);
            }

            let app_events = handle.clone();
            main.on_window_event(move |event| match event {
                WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => {
                    schedule_peek_reposition(&app_events)
                }
                WindowEvent::CloseRequested { api, .. } if TRAY_READY.load(Ordering::Relaxed) => {
                    api.prevent_close();
                    if let Some(w) = app_events.get_webview_window("main") {
                        let _ = w.hide();
                    }
                }
                _ => {}
            });

            app.set_menu(build_menu(&handle)?)?;
            match build_tray(&handle) {
                Ok(()) => TRAY_READY.store(true, Ordering::Relaxed),
                Err(e) => eprintln!("tray unavailable, closing the window will quit: {e}"),
            }
            apply_icon(&handle, &prefs);
            Ok(())
        })
        .on_menu_event(|app, event| handle_menu(app, event.id().as_ref()))
        .build(tauri::generate_context!())
        .expect("failed to build Google Chat desktop app");

    app.run(|_app, _event| {
        #[cfg(target_os = "macos")]
        if let tauri::RunEvent::Reopen { .. } = _event {
            show_main(_app);
        }
    });
}

fn build_menu(app: &AppHandle) -> tauri::Result<Menu<tauri::Wry>> {
    #[cfg(target_os = "macos")]
    let (back_key, fwd_key) = ("Cmd+[", "Cmd+]");
    #[cfg(not(target_os = "macos"))]
    let (back_key, fwd_key) = ("Alt+Left", "Alt+Right");

    let sep = || PredefinedMenuItem::separator(app);

    let reload = MenuItem::with_id(app, "reload", "Reload", true, Some("CmdOrCtrl+R"))?;
    let back = MenuItem::with_id(app, "back", "Back", true, Some(back_key))?;
    let forward = MenuItem::with_id(app, "forward", "Forward", true, Some(fwd_key))?;
    let zoom_in = MenuItem::with_id(app, "zoom-in", "Zoom In", true, Some("CmdOrCtrl+="))?;
    let zoom_out = MenuItem::with_id(app, "zoom-out", "Zoom Out", true, Some("CmdOrCtrl+-"))?;
    let zoom_reset =
        MenuItem::with_id(app, "zoom-reset", "Actual Size", true, Some("CmdOrCtrl+0"))?;
    let quit = MenuItem::with_id(app, "quit", "Quit Google Chat", true, Some("CmdOrCtrl+Q"))?;

    let icon_menu = Submenu::with_items(
        app,
        "App Icon",
        true,
        &[
            &MenuItem::with_id(app, "icon-color", "Color", true, None::<&str>)?,
            &MenuItem::with_id(app, "icon-dark", "Dark", true, None::<&str>)?,
            &MenuItem::with_id(app, "icon-white", "White", true, None::<&str>)?,
        ],
    )?;

    #[cfg(target_os = "macos")]
    let app_menu = Submenu::with_items(
        app,
        "Google Chat",
        true,
        &[
            &PredefinedMenuItem::about(app, None, None)?,
            &sep()?,
            &PredefinedMenuItem::hide(app, None)?,
            &PredefinedMenuItem::hide_others(app, None)?,
            &PredefinedMenuItem::show_all(app, None)?,
            &sep()?,
            &quit,
        ],
    )?;
    #[cfg(not(target_os = "macos"))]
    let app_menu = Submenu::with_items(app, "File", true, &[&quit])?;

    let edit_menu = Submenu::with_items(
        app,
        "Edit",
        true,
        &[
            &PredefinedMenuItem::undo(app, None)?,
            &PredefinedMenuItem::redo(app, None)?,
            &sep()?,
            &PredefinedMenuItem::cut(app, None)?,
            &PredefinedMenuItem::copy(app, None)?,
            &PredefinedMenuItem::paste(app, None)?,
            &sep()?,
            &PredefinedMenuItem::select_all(app, None)?,
        ],
    )?;

    let view_menu = Submenu::with_items(
        app,
        "View",
        true,
        &[
            &reload,
            &back,
            &forward,
            &sep()?,
            &zoom_in,
            &zoom_out,
            &zoom_reset,
            &sep()?,
            &icon_menu,
        ],
    )?;

    let window_menu = Submenu::with_items(
        app,
        "Window",
        true,
        &[
            &PredefinedMenuItem::minimize(app, None)?,
            &PredefinedMenuItem::maximize(app, None)?,
            &sep()?,
            &PredefinedMenuItem::close_window(app, None)?,
        ],
    )?;

    Menu::with_items(app, &[&app_menu, &edit_menu, &view_menu, &window_menu])
}

fn build_tray(app: &AppHandle) -> tauri::Result<()> {
    let menu = Menu::with_items(
        app,
        &[
            &MenuItem::with_id(app, "show", "Show Google Chat", true, None::<&str>)?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?,
        ],
    )?;
    let mut builder = TrayIconBuilder::with_id("main")
        .tooltip("Google Chat")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                toggle_main(tray.app_handle());
            }
        });
    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }
    builder.build(app)?;
    Ok(())
}

fn handle_menu(app: &AppHandle, id: &str) {
    let main = app.get_webview_window("main");
    match id {
        "reload" => {
            if let Some(w) = main {
                let _ = w.reload();
            }
        }
        "back" => {
            if let Some(w) = main {
                let _ = w.eval("history.back()");
            }
        }
        "forward" => {
            if let Some(w) = main {
                let _ = w.eval("history.forward()");
            }
        }
        "zoom-in" | "zoom-out" | "zoom-reset" => {
            let s = settings::update(app, |s| {
                s.zoom = match id {
                    "zoom-in" => clamp_zoom(s.zoom + util::ZOOM_STEP),
                    "zoom-out" => clamp_zoom(s.zoom - util::ZOOM_STEP),
                    _ => 1.0,
                }
            });
            if let Some(w) = main {
                let _ = w.set_zoom(s.zoom);
            }
        }
        "icon-color" | "icon-dark" | "icon-white" => {
            let s = settings::update(app, |s| s.icon = id.trim_start_matches("icon-").into());
            apply_icon(app, &s);
        }
        "show" => show_main(app),
        "quit" => app.exit(0),
        _ => {}
    }
}

fn show_main<R: Runtime>(app: &AppHandle<R>) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.unminimize();
        let _ = w.show();
        let _ = w.set_focus();
    }
}

fn toggle_main(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        if w.is_visible().unwrap_or(false) && w.is_focused().unwrap_or(false) {
            let _ = w.hide();
        } else {
            show_main(app);
        }
    }
}

// Link policy is applied only to new-window requests (how Chat opens message
// links). In-page navigations are never filtered: on macOS the navigation
// handler also fires for iframes and `javascript:` links, and filtering them
// breaks Google sign-in.

/// Loads `url` in the main window (used for Google sign-in pages).
fn navigate_main(app: &AppHandle, url: Url) {
    let app2 = app.clone();
    let _ = app.run_on_main_thread(move || {
        close_peek(&app2);
        if let Some(w) = app2.get_webview_window("main") {
            if let Err(e) = w.navigate(url) {
                eprintln!("[gcd] failed to open sign-in page: {e}");
            }
        }
    });
}

fn open_externally(app: &AppHandle, url: &Url) {
    if !is_openable_externally(url) {
        eprintln!("[gcd] ignored link with unsupported scheme: {url}");
        return;
    }
    eprintln!("[gcd] opening in browser: {url}");
    // Navigation callbacks run on a webview thread on Windows; don't block it.
    let app = app.clone();
    let url = url.to_string();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = app.opener().open_url(url, None::<&str>) {
            eprintln!("failed to open link externally: {e}");
        }
    });
}

/// Sets the dock/taskbar badge and tray tooltip from the page title, and
/// notifies when new messages arrive while the window is not focused.
fn update_unread(app: &AppHandle, title: &str) {
    let count = unread_from_title(title);
    let previous = LAST_UNREAD.swap(count, Ordering::Relaxed);
    if count == previous {
        return;
    }
    let app = app.clone();
    let _ = app.clone().run_on_main_thread(move || {
        let Some(w) = app.get_webview_window("main") else {
            return;
        };
        let _ = w.set_badge_count((count > 0).then_some(count));
        if let Some(tray) = app.tray_by_id("main") {
            let tip = if count > 0 {
                format!("Google Chat — {count} unread")
            } else {
                "Google Chat".into()
            };
            let _ = tray.set_tooltip(Some(tip));
        }
        let focused = w.is_focused().unwrap_or(false) && w.is_visible().unwrap_or(false);
        if count > previous && !focused {
            let _ = app
                .notification()
                .builder()
                .title("Google Chat")
                .body(format!(
                    "{count} unread conversation{}",
                    if count == 1 { "" } else { "s" }
                ))
                .show();
        }
    });
}

/// Creates a child webview overlay inside the main window, centred.
/// This is a true in-window PiP — no separate OS window.
fn create_peek_overlay(app: &AppHandle, url: Url) {
    if let Some(existing) = app.get_webview("peek") {
        let _ = existing.close();
    }
    let Some(main_window) = app.get_window("main") else {
        return;
    };
    let (size, pos) = peek_size_and_position_logical(app);

    // Store the real URL before creating the webview (see `PeekUrl`).
    if let Ok(mut guard) = app.state::<PeekUrl>().0.lock() {
        *guard = Some(url.clone());
    }

    let app_for_nav = app.clone();
    let app_for_new = app.clone();
    let builder = WebviewBuilder::new("peek", WebviewUrl::External(url))
        .user_agent(SAFARI_USER_AGENT)
        .initialization_script(PEEK_TOOLBAR_JS)
        .on_navigation(move |nav_url| {
            if nav_url.host_str() == Some(PEEK_ACTION_HOST) {
                // On Windows/WebView2 this runs on a background thread and window
                // ops would deadlock, so defer the work to the async runtime.
                let app = app_for_nav.clone();
                match nav_url.path() {
                    "/expand" => {
                        tauri::async_runtime::spawn(async move { expand_peek(&app) });
                    }
                    "/close" => {
                        tauri::async_runtime::spawn(async move { close_peek(&app) });
                    }
                    _ => {}
                }
                return false;
            }
            true
        })
        .on_new_window(move |url, _| {
            if is_in_app_url(&url) {
                let app = app_for_new.clone();
                tauri::async_runtime::spawn(async move { open_child_window(&app, url) });
            } else {
                open_externally(&app_for_new, &url);
            }
            NewWindowResponse::Deny
        });

    if let Err(e) = main_window.add_child(builder, pos, size) {
        eprintln!("failed to create peek overlay: {e}");
    }
}

/// Pops the peek webview out into a standalone window, then closes the overlay.
fn expand_peek(app: &AppHandle) {
    let Some(peek) = app.get_webview("peek") else {
        return;
    };
    // Read (and clear) the real URL from state rather than peek.url(); see `PeekUrl`.
    let url = app
        .state::<PeekUrl>()
        .0
        .lock()
        .ok()
        .and_then(|mut g| g.take());
    let _ = peek.close();
    if let Some(url) = url {
        open_child_window(app, url);
    }
}

/// Standalone window for Google Workspace content (Docs, Drive, Meet, ...).
fn open_child_window(app: &AppHandle, url: Url) {
    let label = format!(
        "expanded-{}",
        CHILD_WINDOW_COUNTER.fetch_add(1, Ordering::Relaxed)
    );
    let app_new = app.clone();

    // Creating a new WebviewWindow (instead of navigate()) avoids corrupting
    // WebView2 internal state on Windows.
    let result = WebviewWindowBuilder::new(app, label, WebviewUrl::External(url.clone()))
        .title(title_for_url(&url))
        .inner_size(1180.0, 820.0)
        .min_inner_size(480.0, 360.0)
        .resizable(true)
        .user_agent(SAFARI_USER_AGENT)
        .on_new_window(move |new_url, _| {
            if is_in_app_url(&new_url) {
                let app = app_new.clone();
                tauri::async_runtime::spawn(async move { open_child_window(&app, new_url) });
            } else {
                open_externally(&app_new, &new_url);
            }
            NewWindowResponse::Deny
        })
        .build();

    match result {
        // WebView2 can silently swallow close(); force-release the process.
        #[cfg(target_os = "windows")]
        Ok(win) => {
            let win_clone = win.clone();
            win.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = win_clone.destroy();
                }
            });
        }
        #[cfg(not(target_os = "windows"))]
        Ok(_) => {}
        Err(e) => eprintln!("failed to open window: {e}"),
    }
}

fn close_peek(app: &AppHandle) {
    if let Ok(mut g) = app.state::<PeekUrl>().0.lock() {
        *g = None;
    }
    if let Some(peek) = app.get_webview("peek") {
        let _ = peek.close();
    }
}

/// Debounced: a resize drag fires many events; only reposition once it settles.
fn schedule_peek_reposition(app: &AppHandle) {
    if app.get_webview("peek").is_none() {
        return;
    }
    let generation = RESIZE_GENERATION.fetch_add(1, Ordering::Relaxed) + 1;
    let app = app.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(30));
        if RESIZE_GENERATION.load(Ordering::Relaxed) != generation {
            return;
        }
        let app2 = app.clone();
        let _ = app.run_on_main_thread(move || {
            if let Some(peek) = app2.get_webview("peek") {
                let (size, pos) = peek_size_and_position_logical(&app2);
                let _ = peek.set_size(size);
                let _ = peek.set_position(pos);
            }
        });
    });
}

fn peek_size_and_position_logical(app: &AppHandle) -> (LogicalSize<f64>, LogicalPosition<f64>) {
    let fallback = (
        LogicalSize::new(800.0, 600.0),
        LogicalPosition::new(100.0, 100.0),
    );
    let Some(main_window) = app.get_window("main") else {
        return fallback;
    };
    let Ok(phys) = main_window.inner_size() else {
        return fallback;
    };
    let scale = main_window.scale_factor().unwrap_or(1.0);
    let (x, y, w, h) = util::peek_geometry(phys.width as f64 / scale, phys.height as f64 / scale);
    (LogicalSize::new(w, h), LogicalPosition::new(x, y))
}

fn apply_icon(app: &AppHandle, prefs: &Settings) {
    let bytes = match prefs.icon.as_str() {
        "dark" => DARK_ICON,
        "white" => WHITE_ICON,
        _ => COLOR_ICON,
    };
    let Some(window) = app.get_webview_window("main") else {
        return;
    };
    if let Ok(icon) = Image::from_bytes(bytes) {
        let _ = window.set_icon(icon);
    }
    #[cfg(target_os = "macos")]
    set_macos_dock_icon(bytes);
}

#[cfg(target_os = "macos")]
fn set_macos_dock_icon(icon_bytes: &[u8]) {
    use objc2::{AllocAnyThread, MainThreadMarker};
    use objc2_app_kit::{NSApplication, NSImage};
    use objc2_foundation::NSData;

    let Some(mtm) = MainThreadMarker::new() else {
        return;
    };
    let data = NSData::with_bytes(icon_bytes);
    let Some(image) = NSImage::initWithData(NSImage::alloc(), &data) else {
        return;
    };
    let ns_app = NSApplication::sharedApplication(mtm);
    // SAFETY: called on the main thread with a valid, retained NSImage.
    unsafe { ns_app.setApplicationIconImage(Some(&image)) };
}
