use std::sync::atomic::{AtomicU64, Ordering};

use tauri::{
    image::Image,
    menu::{Menu, MenuItem, PredefinedMenuItem, Submenu},
    webview::{NewWindowResponse, PageLoadEvent},
    Manager, WebviewUrl, WebviewWindowBuilder,
};

const CHAT_URL: &str = "https://chat.google.com/";
const SAFARI_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.4 Safari/605.1.15";
const COLOR_ICON: &[u8] = include_bytes!("../icons/google-chat-color.png");
const DARK_ICON: &[u8] = include_bytes!("../icons/google-chat-dark.png");
const WHITE_ICON: &[u8] = include_bytes!("../icons/google-chat-white.png");
static CHILD_WINDOW_COUNTER: AtomicU64 = AtomicU64::new(1);

const PEEK_WIDTH: f64 = 520.0;
const PEEK_HEIGHT: f64 = 420.0;
const PEEK_MARGIN: f64 = 24.0;

/// JavaScript injected into peek windows that renders a floating toolbar
/// with "Pop Out" and "Close" buttons. Button clicks navigate to sentinel
/// URLs that are intercepted by `on_navigation` on the Rust side.
const PEEK_TOOLBAR_JS: &str = r#"
(function() {
    if (document.getElementById('_peek_toolbar')) return;
    function inject() {
        if (document.getElementById('_peek_toolbar') || !document.body) return;
        var bar = document.createElement('div');
        bar.id = '_peek_toolbar';
        bar.style.cssText = 'position:fixed;bottom:0;left:0;right:0;height:44px;background:rgba(32,33,36,0.96);backdrop-filter:blur(16px);-webkit-backdrop-filter:blur(16px);display:flex;align-items:center;justify-content:flex-end;padding:0 14px;gap:10px;z-index:2147483647;border-top:1px solid rgba(255,255,255,0.08);font-family:-apple-system,BlinkMacSystemFont,"Segoe UI",system-ui,sans-serif;box-sizing:border-box;';

        var expandBtn = document.createElement('button');
        expandBtn.textContent = '\u2197 Pop Out';
        expandBtn.style.cssText = 'background:#8ab4f8;color:#202124;border:none;padding:7px 16px;border-radius:20px;cursor:pointer;font-size:12px;font-weight:600;letter-spacing:0.3px;transition:all 0.15s ease;outline:none;';
        expandBtn.onmouseenter = function() { this.style.background='#aecbfa'; this.style.transform='scale(1.04)'; };
        expandBtn.onmouseleave = function() { this.style.background='#8ab4f8'; this.style.transform='scale(1)'; };
        expandBtn.onclick = function(e) { e.preventDefault(); window.location.href='https://peek-action.tauri.internal/expand'; };

        var closeBtn = document.createElement('button');
        closeBtn.textContent = '\u2715 Close';
        closeBtn.style.cssText = 'background:rgba(255,255,255,0.08);color:#e8eaed;border:1px solid rgba(255,255,255,0.1);padding:7px 16px;border-radius:20px;cursor:pointer;font-size:12px;font-weight:500;letter-spacing:0.3px;transition:all 0.15s ease;outline:none;';
        closeBtn.onmouseenter = function() { this.style.background='rgba(234,67,53,0.8)'; this.style.borderColor='rgba(234,67,53,0.6)'; };
        closeBtn.onmouseleave = function() { this.style.background='rgba(255,255,255,0.08)'; this.style.borderColor='rgba(255,255,255,0.1)'; };
        closeBtn.onclick = function(e) { e.preventDefault(); window.location.href='https://peek-action.tauri.internal/close'; };

        bar.appendChild(expandBtn);
        bar.appendChild(closeBtn);
        document.body.appendChild(bar);
        document.body.style.paddingBottom = '44px';
    }
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', inject);
    }
    inject();
    window.addEventListener('load', inject);
})();
"#;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();

            WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External(CHAT_URL.parse().expect("valid Google Chat URL")),
            )
            .title("Google Chat")
            .inner_size(1280.0, 860.0)
            .min_inner_size(960.0, 640.0)
            .resizable(true)
            .user_agent(SAFARI_USER_AGENT)
            .on_new_window(move |url, _| {
                create_peek_window(&app_handle, url);
                NewWindowResponse::Deny
            })
            .build()?;

            let reload = MenuItem::with_id(app, "reload", "Reload", true, Some("CmdOrCtrl+R"))?;
            let back = MenuItem::with_id(app, "back", "Back", true, Some("CmdOrCtrl+["))?;
            let forward = MenuItem::with_id(app, "forward", "Forward", true, Some("CmdOrCtrl+]"))?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, Some("CmdOrCtrl+Q"))?;
            let icon_color = MenuItem::with_id(app, "icon-color", "Color", true, None::<&str>)?;
            let icon_dark = MenuItem::with_id(app, "icon-dark", "Dark", true, None::<&str>)?;
            let icon_white = MenuItem::with_id(app, "icon-white", "White", true, None::<&str>)?;
            let separator_one = PredefinedMenuItem::separator(app)?;
            let separator_two = PredefinedMenuItem::separator(app)?;
            let edit_undo = PredefinedMenuItem::undo(app, None)?;
            let edit_redo = PredefinedMenuItem::redo(app, None)?;
            let edit_separator_one = PredefinedMenuItem::separator(app)?;
            let edit_cut = PredefinedMenuItem::cut(app, None)?;
            let edit_copy = PredefinedMenuItem::copy(app, None)?;
            let edit_paste = PredefinedMenuItem::paste(app, None)?;
            let edit_separator_two = PredefinedMenuItem::separator(app)?;
            let edit_select_all = PredefinedMenuItem::select_all(app, None)?;

            let app_menu = Submenu::with_items(
                app,
                "Google Chat",
                true,
                &[
                    &reload,
                    &separator_one,
                    &back,
                    &forward,
                    &separator_two,
                    &quit,
                ],
            )?;

            let edit_menu = Submenu::with_items(
                app,
                "Edit",
                true,
                &[
                    &edit_undo,
                    &edit_redo,
                    &edit_separator_one,
                    &edit_cut,
                    &edit_copy,
                    &edit_paste,
                    &edit_separator_two,
                    &edit_select_all,
                ],
            )?;

            let icon_menu =
                Submenu::with_items(app, "Icon", true, &[&icon_color, &icon_dark, &icon_white])?;

            let menu = Menu::with_items(app, &[&app_menu, &edit_menu, &icon_menu])?;
            app.set_menu(menu)?;

            Ok(())
        })
        .on_menu_event(|app, event| match event.id().as_ref() {
            "reload" => {
                let window = get_active_window(app);
                let _ = window.eval("window.location.reload()");
            }
            "back" => {
                let window = get_active_window(app);
                let _ = window.eval("history.back()");
            }
            "forward" => {
                let window = get_active_window(app);
                let _ = window.eval("history.forward()");
            }
            "icon-color" => set_main_window_icon(app, COLOR_ICON),
            "icon-dark" => set_main_window_icon(app, DARK_ICON),
            "icon-white" => set_main_window_icon(app, WHITE_ICON),
            "quit" => app.exit(0),
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("failed to run Google Chat desktop app");
}

/// Creates a small always-on-top "peek" window positioned at the bottom-right
/// of the main window. The peek window has an injected toolbar with "Pop Out"
/// (expand to full window) and "Close" buttons.
fn create_peek_window(app: &tauri::AppHandle, url: tauri::Url) {
    let (peek_x, peek_y) = peek_position(app);
    let label = child_window_label(&url);
    let label_for_nav = label.clone();
    let app_for_nav = app.clone();
    let app_for_newwin = app.clone();
    let label_for_newwin = label.clone();

    let result = WebviewWindowBuilder::new(
        app,
        &label,
        WebviewUrl::External(url.clone()),
    )
    .title(title_for_url(&url))
    .inner_size(PEEK_WIDTH, PEEK_HEIGHT)
    .position(peek_x, peek_y)
    .min_inner_size(320.0, 240.0)
    .resizable(true)
    .always_on_top(true)
    .user_agent(SAFARI_USER_AGENT)
    .on_navigation(move |nav_url| {
        if nav_url.host_str() == Some("peek-action.tauri.internal") {
            match nav_url.path() {
                "/expand" => expand_peek_window(&app_for_nav, &label_for_nav),
                "/close" => {
                    if let Some(w) = app_for_nav.get_webview_window(&label_for_nav) {
                        let _ = w.close();
                    }
                }
                _ => {}
            }
            return false;
        }
        true
    })
    .on_page_load(move |webview, payload| {
        if matches!(payload.event(), PageLoadEvent::Finished) {
            let _ = webview.eval(PEEK_TOOLBAR_JS);
        }
    })
    .on_new_window(move |url, _| {
        // Links clicked inside the peek window navigate within it
        if let Some(peek_win) = app_for_newwin.get_webview_window(&label_for_newwin) {
            let _ = peek_win.navigate(url);
        }
        NewWindowResponse::Deny
    })
    .build();

    if let Ok(ref win) = result {
        // Inject toolbar immediately for fast-loading pages
        let _ = win.eval(PEEK_TOOLBAR_JS);
    }

    if let Err(e) = result {
        eprintln!("failed to create peek window: {e}");
    }
}

/// Takes a peek window label, removes always_on_top, and resizes it to
/// a full-size workspace window.
fn expand_peek_window(app: &tauri::AppHandle, label: &str) {
    let Some(window) = app.get_webview_window(label) else {
        return;
    };

    let _ = window.set_always_on_top(false);
    let _ = window.set_size(tauri::LogicalSize::new(1180.0, 820.0));
    let _ = window.center();

    // Remove the peek toolbar from the expanded window
    let _ = window.eval(
        r#"(function(){
            var tb = document.getElementById('_peek_toolbar');
            if (tb) tb.remove();
            document.body.style.paddingBottom = '';
        })();"#,
    );
}

/// Calculates the position for a peek window at the bottom-right of the main window.
fn peek_position(app: &tauri::AppHandle) -> (f64, f64) {
    if let Some(main_win) = app.get_webview_window("main") {
        if let (Ok(pos), Ok(size)) = (main_win.outer_position(), main_win.outer_size()) {
            return (
                pos.x as f64 + size.width as f64 - PEEK_WIDTH - PEEK_MARGIN,
                pos.y as f64 + size.height as f64 - PEEK_HEIGHT - PEEK_MARGIN,
            );
        }
    }
    (200.0, 200.0)
}

fn get_active_window(app: &tauri::AppHandle) -> tauri::WebviewWindow {
    app.webview_windows()
        .into_values()
        .find(|w| w.is_focused().unwrap_or(false))
        .unwrap_or_else(|| app.get_webview_window("main").expect("main window exists"))
}

fn set_main_window_icon(app: &tauri::AppHandle, icon_bytes: &[u8]) {
    let Some(window) = app.get_webview_window("main") else {
        return;
    };

    if let Ok(icon) = Image::from_bytes(icon_bytes) {
        let _ = window.set_icon(icon);
    }
}

fn child_window_label(url: &tauri::Url) -> String {
    let id = CHILD_WINDOW_COUNTER.fetch_add(1, Ordering::Relaxed);
    let sanitized: String = url
        .as_str()
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '-'
            }
        })
        .take(80)
        .collect();

    format!("peek-{id}-{sanitized}")
}

fn title_for_url(url: &tauri::Url) -> &'static str {
    match url.host_str() {
        Some("docs.google.com") => "Google Docs",
        Some("drive.google.com") => "Google Drive",
        Some("mail.google.com" | "gmail.com") => "Gmail",
        Some("calendar.google.com") => "Google Calendar",
        Some("meet.google.com") => "Google Meet",
        Some("contacts.google.com") => "Google Contacts",
        Some("keep.google.com") => "Google Keep",
        Some("tasks.google.com") => "Google Tasks",
        Some("jamboard.google.com") => "Google Jamboard",
        _ => "Google Workspace",
    }
}
