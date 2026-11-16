//! Pure helpers kept free of Tauri runtime types so they can be unit tested.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

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

/// Country Google domains used during sign-in, e.g. `google.co.uk`, `accounts.google.com.pk`.
fn is_google_country_domain(host: &str) -> bool {
    let labels: Vec<&str> = host.split('.').collect();
    let Some(i) = labels.iter().rposition(|l| *l == "google") else {
        return false;
    };
    let tld = &labels[i + 1..];
    let is_cc = |l: &str| l.len() == 2 && l.chars().all(|c| c.is_ascii_lowercase());
    match tld {
        [cc] => is_cc(cc),
        [sld, cc] => matches!(*sld, "co" | "com") && is_cc(cc),
        _ => false,
    }
}

/// Whether a URL may be shown inside an app webview.
pub fn is_in_app_url(url: &Url) -> bool {
    match url.scheme() {
        "about" | "blob" | "data" => true,
        "http" | "https" => url.host_str().is_some_and(|host| {
            let host = host.to_ascii_lowercase();
            GOOGLE_EXACT.contains(&host.as_str())
                || GOOGLE_SUFFIXES.iter().any(|s| host_matches(&host, s))
                || is_google_country_domain(&host)
        }),
        _ => false,
    }
}

/// Google account sign-in / account-chooser pages, which must load in the main window.
pub fn is_sign_in_url(url: &Url) -> bool {
    let host = url.host_str().unwrap_or("").to_ascii_lowercase();
    host == "accounts.google.com"
        || (is_in_app_url(url)
            && ["/servicelogin", "/signin", "/accountchooser"]
                .iter()
                .any(|p| url.path().to_ascii_lowercase().contains(p)))
}

/// The marketing page chat.google.com redirects to when signed out.
pub fn is_signed_out_landing(url: &Url) -> bool {
    url.host_str() == Some("workspace.google.com")
        && url.path().trim_end_matches('/').ends_with("/products/chat")
}

/// File name for a download: the webview's suggestion, else the URL's last
/// path segment, else "download". Never contains path separators.
pub fn download_file_name(suggested: Option<&OsStr>, url: &Url) -> String {
    let from_url = || {
        url.path_segments()
            .and_then(|mut s| s.next_back())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
    };
    let raw = suggested
        .map(|s| s.to_string_lossy().into_owned())
        .filter(|s| !s.is_empty())
        .or_else(from_url)
        .unwrap_or_else(|| "download".into());
    let clean: String = raw
        .chars()
        .map(|c| {
            if matches!(c, '/' | '\\' | ':') {
                '_'
            } else {
                c
            }
        })
        .collect();
    let clean = clean.trim_start_matches('.').to_string();
    if clean.is_empty() {
        "download".into()
    } else {
        clean
    }
}

/// `dir/name`, or `dir/name (2).ext`, `(3)`… if taken.
pub fn unique_path(dir: &Path, name: &str, exists: impl Fn(&Path) -> bool) -> PathBuf {
    let first = dir.join(name);
    if !exists(&first) {
        return first;
    }
    let p = Path::new(name);
    let stem = p
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    let ext = p
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();
    (2..)
        .map(|n| dir.join(format!("{stem} ({n}){ext}")))
        .find(|c| !exists(c))
        .expect("unbounded range")
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
        assert!(is_in_app_url(&u("https://accounts.google.com.pk/x")));
        assert!(is_in_app_url(&u("https://www.google.co.uk/")));
        assert!(is_in_app_url(&u("https://google.de/")));
        assert!(!is_in_app_url(&u("https://google.evil.com/")));
        assert!(!is_in_app_url(&u("https://google.co.evil/")));
        assert!(!is_in_app_url(&u("https://github.com/")));
        assert!(!is_in_app_url(&u("file:///etc/passwd")));
        assert!(is_openable_externally(&u("mailto:a@b.c")));
        assert!(!is_openable_externally(&u("file:///etc/passwd")));
    }

    #[test]
    fn sign_in() {
        assert!(is_sign_in_url(&u(
            "https://accounts.google.com/ServiceLogin?x=1"
        )));
        assert!(is_sign_in_url(&u(
            "https://accounts.google.com/v3/signin/identifier"
        )));
        assert!(is_sign_in_url(&u(
            "https://www.google.com/a/x/ServiceLogin"
        )));
        assert!(!is_sign_in_url(&u("https://chat.google.com/room/abc")));
        assert!(!is_sign_in_url(&u("https://evil.com/signin")));
    }

    #[test]
    fn signed_out_landing() {
        assert!(is_signed_out_landing(&u(
            "https://workspace.google.com/products/chat/"
        )));
        assert!(is_signed_out_landing(&u(
            "https://workspace.google.com/intl/en/products/chat/"
        )));
        assert!(!is_signed_out_landing(&u(
            "https://workspace.google.com/pricing"
        )));
        assert!(!is_signed_out_landing(&u("https://chat.google.com/")));
    }

    #[test]
    fn downloads() {
        let url = u("https://chat.google.com/api/get_attachment_url?x=1");
        assert_eq!(
            download_file_name(Some(OsStr::new("report.pdf")), &url),
            "report.pdf"
        );
        assert_eq!(
            download_file_name(None, &u("https://x.google.com/a/b/file.png")),
            "file.png"
        );
        assert_eq!(
            download_file_name(Some(OsStr::new("../etc/x")), &url),
            "_etc_x"
        );
        assert_eq!(
            download_file_name(None, &u("https://x.google.com/")),
            "download"
        );
        let dir = Path::new("/dl");
        let taken = [PathBuf::from("/dl/a.pdf"), PathBuf::from("/dl/a (2).pdf")];
        assert_eq!(
            unique_path(dir, "b.pdf", |p| taken.iter().any(|t| t == p)),
            PathBuf::from("/dl/b.pdf")
        );
        assert_eq!(
            unique_path(dir, "a.pdf", |p| taken.iter().any(|t| t == p)),
            PathBuf::from("/dl/a (3).pdf")
        );
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
