//! Pure helpers kept free of Tauri runtime types so they can be unit tested.

use tauri::Url;

/// Hosts that stay inside the app. Everything else opens in the system browser.
const GOOGLE_SUFFIXES: &[&str] = &[
    "google.com",
    "googleusercontent.com",
    "gstatic.com",
    "googleapis.com",
    "googlevideo.com",
    "gmail.com",
];
/// Non-google.com hosts that are part of Google's sign-in redirect chain.
const GOOGLE_EXACT: &[&str] = &["accounts.youtube.com"];

fn host_matches(host: &str, suffix: &str) -> bool {
    host == suffix || host.ends_with(&format!(".{suffix}"))
}

/// Whether a URL may be shown inside an app webview.
pub fn is_in_app_url(url: &Url) -> bool {
    match url.scheme() {
        "about" | "blob" | "data" => true,
        "http" | "https" => url.host_str().is_some_and(|host| {
            let host = host.to_ascii_lowercase();
            GOOGLE_EXACT.contains(&host.as_str())
                || GOOGLE_SUFFIXES.iter().any(|s| host_matches(&host, s))
        }),
        _ => false,
    }
}

/// Whether an out-of-app URL is safe to hand to the OS opener.
pub fn is_openable_externally(url: &Url) -> bool {
    matches!(url.scheme(), "http" | "https" | "mailto" | "tel")
}

pub fn title_for_url(url: &Url) -> &'static str {
    match url.host_str() {
        Some("docs.google.com") => "Google Docs",
        Some("drive.google.com") => "Google Drive",
        Some("mail.google.com" | "gmail.com") => "Gmail",
        Some("calendar.google.com") => "Google Calendar",
        Some("meet.google.com") => "Google Meet",
        Some("contacts.google.com") => "Google Contacts",
        Some("keep.google.com") => "Google Keep",
        Some("tasks.google.com") => "Google Tasks",
        _ => "Google Workspace",
    }
}

/// Extracts an unread count from a page title such as `Chat (3)` or `(12) Google Chat`.
pub fn unread_from_title(title: &str) -> i64 {
    let mut rest = title;
    while let Some(start) = rest.find('(') {
        let after = &rest[start + 1..];
        if let Some(end) = after.find(')') {
            let inner = after[..end].trim().trim_end_matches('+');
            if let Ok(n) = inner.parse::<i64>() {
                return n.max(0);
            }
            rest = &after[end + 1..];
        } else {
            break;
        }
    }
    0
}

/// Peek overlay geometry (x, y, w, h) in logical pixels: 85% of the window,
/// centred, but never smaller than a usable minimum nor larger than the window.
pub fn peek_geometry(win_w: f64, win_h: f64) -> (f64, f64, f64, f64) {
    const MIN_W: f64 = 480.0;
    const MIN_H: f64 = 360.0;
    let w = (win_w * 0.85).max(MIN_W).min(win_w);
    let h = (win_h * 0.85).max(MIN_H).min(win_h);
    ((win_w - w) / 2.0, (win_h - h) / 2.0, w, h)
}

pub const ZOOM_MIN: f64 = 0.5;
pub const ZOOM_MAX: f64 = 2.0;
pub const ZOOM_STEP: f64 = 0.1;

pub fn clamp_zoom(z: f64) -> f64 {
    if !z.is_finite() {
        return 1.0;
    }
    ((z.clamp(ZOOM_MIN, ZOOM_MAX)) * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn u(s: &str) -> Url {
        s.parse().unwrap()
    }

    #[test]
    fn allowlist() {
        assert!(is_in_app_url(&u("https://chat.google.com/")));
        assert!(is_in_app_url(&u("https://accounts.google.com/x")));
        assert!(is_in_app_url(&u("https://lh3.googleusercontent.com/a")));
        assert!(is_in_app_url(&u("https://accounts.youtube.com/a")));
        assert!(is_in_app_url(&u("about:blank")));
        assert!(!is_in_app_url(&u("https://evilgoogle.com/")));
        assert!(!is_in_app_url(&u("https://google.com.evil.io/")));
        assert!(!is_in_app_url(&u("https://github.com/")));
        assert!(!is_in_app_url(&u("file:///etc/passwd")));
        assert!(is_openable_externally(&u("mailto:a@b.c")));
        assert!(!is_openable_externally(&u("file:///etc/passwd")));
    }

    #[test]
    fn titles() {
        assert_eq!(
            title_for_url(&u("https://docs.google.com/d")),
            "Google Docs"
        );
        assert_eq!(
            title_for_url(&u("https://x.google.com/")),
            "Google Workspace"
        );
    }

    #[test]
    fn unread() {
        assert_eq!(unread_from_title("Chat"), 0);
        assert_eq!(unread_from_title("Chat (3)"), 3);
        assert_eq!(unread_from_title("(12) Google Chat"), 12);
        assert_eq!(unread_from_title("Team (eng) (99+)"), 99);
    }

    #[test]
    fn geometry() {
        let (x, y, w, h) = peek_geometry(1000.0, 800.0);
        assert_eq!((w, h), (850.0, 680.0));
        assert_eq!((x, y), (75.0, 60.0));
        let (x, _, w, h) = peek_geometry(500.0, 300.0);
        assert_eq!((w, h), (480.0, 300.0));
        assert_eq!(x, 10.0);
    }

    #[test]
    fn zoom() {
        assert_eq!(clamp_zoom(5.0), ZOOM_MAX);
        assert_eq!(clamp_zoom(f64::NAN), 1.0);
        assert_eq!(clamp_zoom(1.1000001), 1.1);
    }
}
