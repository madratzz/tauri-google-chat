use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem, Submenu},
    webview::NewWindowResponse,
    WebviewUrl, WebviewWindowBuilder,
    Manager,
};

const CHAT_URL: &str = "https://chat.google.com/";
const SAFARI_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.4 Safari/605.1.15";

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
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
            .on_new_window(|url, _features| {
                open_external_url(url.as_str());
                NewWindowResponse::Deny
            })
            .on_navigation(|url| {
                if should_open_externally(url) {
                    open_external_url(url.as_str());
                    false
                } else {
                    true
                }
            })
            .build()?;

            let reload = MenuItem::with_id(app, "reload", "Reload", true, Some("CmdOrCtrl+R"))?;
            let back = MenuItem::with_id(app, "back", "Back", true, Some("CmdOrCtrl+["))?;
            let forward =
                MenuItem::with_id(app, "forward", "Forward", true, Some("CmdOrCtrl+]"))?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, Some("CmdOrCtrl+Q"))?;
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
                &[&reload, &separator_one, &back, &forward, &separator_two, &quit],
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

            let menu = Menu::with_items(app, &[&app_menu, &edit_menu])?;
            app.set_menu(menu)?;

            Ok(())
        })
        .on_menu_event(|app, event| {
            let Some(window) = app.get_webview_window("main") else {
                return;
            };

            match event.id().as_ref() {
                "reload" => {
                    let _ = window.eval("window.location.reload()");
                }
                "back" => {
                    let _ = window.eval("history.back()");
                }
                "forward" => {
                    let _ = window.eval("history.forward()");
                }
                "quit" => app.exit(0),
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("failed to run Google Chat desktop app");
}

fn should_open_externally(url: &tauri::Url) -> bool {
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

fn open_external_url(url: &str) {
    #[cfg(target_os = "macos")]
    let command = ("open", vec![url]);

    #[cfg(target_os = "windows")]
    let command = ("cmd", vec!["/C", "start", "", url]);

    #[cfg(all(unix, not(target_os = "macos")))]
    let command = ("xdg-open", vec![url]);

    let _ = std::process::Command::new(command.0)
        .args(command.1)
        .spawn();
}
