use tauri::{
    image::Image,
    menu::{Menu, MenuItem, PredefinedMenuItem, Submenu},
    webview::NewWindowResponse,
    Manager, WebviewUrl, WebviewWindowBuilder,
};

const CHAT_URL: &str = "https://chat.google.com/";
const SAFARI_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.4 Safari/605.1.15";
const COLOR_ICON: &[u8] = include_bytes!("../icons/google-chat-color.png");
const DARK_ICON: &[u8] = include_bytes!("../icons/google-chat-dark.png");
const WHITE_ICON: &[u8] = include_bytes!("../icons/google-chat-white.png");

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();
            let nested_app_handle = app.handle().clone();
            let navigation_app_handle = app.handle().clone();

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
            .on_new_window(move |url, features| {
                let title = title_for_url(&url);
                let window = WebviewWindowBuilder::new(
                    &app_handle,
                    child_window_label(&url),
                    WebviewUrl::External(url.clone()),
                )
                .title(title)
                .inner_size(1180.0, 820.0)
                .min_inner_size(820.0, 560.0)
                .resizable(true)
                .user_agent(SAFARI_USER_AGENT)
                .window_features(features)
                .on_new_window({
                    let nested_app_handle = nested_app_handle.clone();
                    move |url, features| {
                        let title = title_for_url(&url);
                        let window = WebviewWindowBuilder::new(
                            &nested_app_handle,
                            child_window_label(&url),
                            WebviewUrl::External(url.clone()),
                        )
                        .title(title)
                        .inner_size(1180.0, 820.0)
                        .min_inner_size(820.0, 560.0)
                        .resizable(true)
                        .user_agent(SAFARI_USER_AGENT)
                        .window_features(features)
                        .build()
                        .expect("failed to open child webview window");

                        NewWindowResponse::Create { window }
                    }
                })
                .build()
                .expect("failed to open webview window");

                NewWindowResponse::Create { window }
            })
            .on_navigation(move |url| {
                if should_open_in_child_window(url) {
                    open_internal_window(&navigation_app_handle, url.clone());
                    false
                } else {
                    true
                }
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
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.eval("window.location.reload()");
                }
            }
            "back" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.eval("history.back()");
                }
            }
            "forward" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.eval("history.forward()");
                }
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

fn set_main_window_icon(app: &tauri::AppHandle, icon_bytes: &[u8]) {
    let Some(window) = app.get_webview_window("main") else {
        return;
    };

    if let Ok(icon) = Image::from_bytes(icon_bytes) {
        let _ = window.set_icon(icon);
    }
}

fn should_open_in_child_window(url: &tauri::Url) -> bool {
    matches!(
        url.host_str(),
        Some(
            "docs.google.com"
                | "drive.google.com"
                | "mail.google.com"
                | "gmail.com"
                | "calendar.google.com"
                | "meet.google.com"
                | "contacts.google.com"
                | "keep.google.com"
                | "tasks.google.com"
                | "jamboard.google.com"
        )
    )
}

fn open_internal_window(app: &tauri::AppHandle, url: tauri::Url) {
    let app_handle = app.clone();

    let _ = WebviewWindowBuilder::new(
        app,
        child_window_label(&url),
        WebviewUrl::External(url.clone()),
    )
    .title(title_for_url(&url))
    .inner_size(1180.0, 820.0)
    .min_inner_size(820.0, 560.0)
    .resizable(true)
    .user_agent(SAFARI_USER_AGENT)
    .on_new_window(move |url, features| {
        let title = title_for_url(&url);
        let window = WebviewWindowBuilder::new(
            &app_handle,
            child_window_label(&url),
            WebviewUrl::External(url.clone()),
        )
        .title(title)
        .inner_size(1180.0, 820.0)
        .min_inner_size(820.0, 560.0)
        .resizable(true)
        .user_agent(SAFARI_USER_AGENT)
        .window_features(features)
        .build()
        .expect("failed to open child webview window");

        NewWindowResponse::Create { window }
    })
    .build();
}

fn child_window_label(url: &tauri::Url) -> String {
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

    format!("workspace-{sanitized}")
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
