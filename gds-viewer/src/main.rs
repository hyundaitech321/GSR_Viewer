// GDS Recorded File Viewer (beta) - desktop wrapper.
// Designed and built by Jerry Lawson (Gerald T. Lawson), Hyundai Techline, Team High Voltage.
// INTERNAL USE ONLY - DO NOT SHARE.
//
// The viewer itself is one HTML page (assets/viewer.html) that is compiled into this program and shown in a
// native window through the Windows WebView2 runtime. Nothing is uploaded anywhere.
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use std::borrow::Cow;
use std::path::PathBuf;

use base64::Engine;
use tao::dpi::LogicalSize;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tao::window::{Fullscreen, WindowBuilder};
use wry::http::{header::CONTENT_TYPE, Response};
use wry::{PageLoadEvent, WebContext, WebViewBuilder};

const VIEWER_HTML: &str = include_str!("../assets/viewer.html");
#[cfg(windows)]
const ORIGIN: &str = "http://app.localhost";
#[cfg(not(windows))]
const ORIGIN: &str = "app://localhost";

/// Recordings named on the command line (file association, drag onto the exe, or `gds-viewer.exe a.msr b.gsr`).
fn script_for_args() -> Option<String> {
    let mut parts = Vec::new();
    for arg in std::env::args_os().skip(1).take(2) {
        let path = PathBuf::from(&arg);
        let Ok(bytes) = std::fs::read(&path) else { continue };
        let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        let b64 = base64::engine::general_purpose::STANDARD.encode(bytes);
        let name_js = name.replace('\\', "\\\\").replace('\'', "\\'");
        parts.push(format!("mk('{}','{}')", name_js, b64));
    }
    if parts.is_empty() {
        return None;
    }
    Some(format!(
        "(async()=>{{const mk=(n,b)=>new File([Uint8Array.from(atob(b),c=>c.charCodeAt(0))],n);\
         try{{await loadFiles([{}]);}}catch(e){{console.error(e);}}}})();",
        parts.join(",")
    ))
}

fn data_dir() -> Option<PathBuf> {
    let base = std::env::var_os("LOCALAPPDATA").or_else(|| std::env::var_os("TEMP"))?;
    let dir = PathBuf::from(base).join("GDSRecordedFileViewer").join("webview");
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir)
}

/// Messages sent to the event loop from the page-load and page-script callbacks.
enum Msg {
    Loaded,
    Fullscreen(bool),
}

fn main() -> wry::Result<()> {
    let event_loop = EventLoopBuilder::<Msg>::with_user_event().build();
    let window = WindowBuilder::new()
        .with_title("GDS Recorded File Viewer (beta) - Internal use only")
        .with_inner_size(LogicalSize::new(1440.0, 900.0))
        .with_min_inner_size(LogicalSize::new(720.0, 480.0))
        .build(&event_loop)
        .expect("could not create the window");

    // Keeps the remembered settings (panel state, saved labels) in the user's profile, not next to the exe.
    let mut context = WebContext::new(data_dir());

    let proxy = event_loop.create_proxy();
    let ipc_proxy = event_loop.create_proxy();
    let startup = script_for_args();

    let builder = WebViewBuilder::new_with_web_context(&mut context)
        .with_custom_protocol("app".into(), |_id, _request| {
            Response::builder()
                .header(CONTENT_TYPE, "text/html; charset=utf-8")
                .body(Cow::Borrowed(VIEWER_HTML.as_bytes()))
                .unwrap()
        })
        .with_url(format!("{}/", ORIGIN))
        // Stay inside the viewer: no browsing away to other sites.
        .with_navigation_handler(|url| url.starts_with(ORIGIN) || url.starts_with("blob:") || url.starts_with("about:"))
        // "Save picture" and "Export CSV" are downloads: ask where to put them.
        .with_download_started_handler(|_url, path| {
            let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_else(|| "download".into());
            let mut dialog = rfd::FileDialog::new().set_file_name(name);
            if let Some(dir) = path.parent() {
                dialog = dialog.set_directory(dir);
            }
            match dialog.save_file() {
                Some(chosen) => {
                    *path = chosen;
                    true
                }
                None => false,
            }
        })
        // The page's "Full screen" button asks the native window to go full screen (and back).
        .with_ipc_handler(move |req| match req.body().as_str() {
            "fullscreen:on" => { let _ = ipc_proxy.send_event(Msg::Fullscreen(true)); }
            "fullscreen:off" => { let _ = ipc_proxy.send_event(Msg::Fullscreen(false)); }
            _ => {}
        })
        .with_on_page_load_handler(move |event, _url| {
            if let PageLoadEvent::Finished = event {
                // wake the event loop; the webview is not reachable from in here
                let _ = proxy.send_event(Msg::Loaded);
            }
        });

    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "ios", target_os = "android"))]
    let webview = builder.build(&window)?;
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "ios", target_os = "android")))]
    let webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        builder.build_gtk(vbox)?
    };

    let mut pending = startup;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            // The page has finished loading: hand it any recordings named on the command line (once).
            Event::UserEvent(Msg::Loaded) => {
                if let Some(js) = pending.take() {
                    let _ = webview.evaluate_script(&js);
                }
            }
            Event::UserEvent(Msg::Fullscreen(on)) => {
                window.set_fullscreen(if on { Some(Fullscreen::Borderless(None)) } else { None });
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}
