mod keyboard;
mod network;
mod qrcode_gen;
mod server;

use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Duration;
use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    Emitter, Manager,
};

pub static SERVER_STATE: Lazy<Arc<Mutex<ServerState>>> =
    Lazy::new(|| Arc::new(Mutex::new(ServerState::default())));

#[derive(Default, Clone)]
pub struct ServerState {
    pub local_ip: String,
    pub port: u16,
    pub connected_clients: usize,
    pub cached_ips: Vec<String>,
}

#[derive(Clone, serde::Serialize)]
struct ServerInfo {
    ip: String,
    port: u16,
    url: String,
    qrcode_data_url: String,
    ips: Vec<String>,
}

fn build_server_info_unlocked(state: &ServerState) -> Result<ServerInfo, String> {
    let url = format!("http://{}:{}", state.local_ip, state.port);
    let qr = qrcode_gen::generate_qr_data_url(&url).map_err(|e| e.to_string())?;
    let ips = if state.cached_ips.is_empty() {
        network::get_all_local_ips()
    } else {
        state.cached_ips.clone()
    };
    Ok(ServerInfo {
        ip: state.local_ip.clone(),
        port: state.port,
        url,
        qrcode_data_url: qr,
        ips,
    })
}

#[tauri::command]
fn get_server_info() -> Result<ServerInfo, String> {
    let state = SERVER_STATE.lock();
    build_server_info_unlocked(&state)
}

/// Rescans interfaces and updates [`SERVER_STATE`]. Returns the new [`ServerInfo`] and whether IP list / primary changed.
fn scan_and_store_network() -> Result<(ServerInfo, bool), String> {
    let primary = network::get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string());
    let mut ips = network::get_all_local_ips();
    if ips.is_empty() {
        ips.push(primary.clone());
    }

    let changed = {
        let mut s = SERVER_STATE.lock();
        let ch = s.local_ip != primary || s.cached_ips != ips;
        s.local_ip = primary.clone();
        s.cached_ips = ips.clone();
        ch
    };

    let state = SERVER_STATE.lock();
    let info = build_server_info_unlocked(&state)?;
    Ok((info, changed))
}

fn sync_network_from_os(app_handle: &tauri::AppHandle, force_emit: bool) -> Result<(), String> {
    let (info, changed) = scan_and_store_network()?;
    if changed || force_emit {
        let _ = app_handle.emit("network-info-changed", &info);
    }
    Ok(())
}

#[tauri::command]
fn refresh_network_info() -> Result<ServerInfo, String> {
    let (info, _) = scan_and_store_network()?;
    Ok(info)
}

#[tauri::command]
fn generate_qr(url: String) -> Result<String, String> {
    qrcode_gen::generate_qr_data_url(&url).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_local_ips() -> Vec<String> {
    network::get_all_local_ips()
}

#[tauri::command]
fn check_accessibility() -> bool {
    #[cfg(target_os = "macos")]
    {
        extern "C" {
            fn AXIsProcessTrusted() -> bool;
        }
        unsafe { AXIsProcessTrusted() }
    }
    #[cfg(not(target_os = "macos"))]
    {
        true
    }
}

#[tauri::command]
fn open_accessibility_settings() {
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
            .spawn();
    }
}

#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub fn run() {
    env_logger::init();

    let port: u16 = 5678;
    let local_ip = network::get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string());
    let initial_ips = {
        let mut ips = network::get_all_local_ips();
        if ips.is_empty() {
            ips.push(local_ip.clone());
        }
        ips
    };

    {
        let mut state = SERVER_STATE.lock();
        state.local_ip = local_ip.clone();
        state.port = port;
        state.cached_ips = initial_ips;
    }

    let server_local_ip = local_ip.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
        rt.block_on(async {
            server::start_server(&server_local_ip, port).await;
        });
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_server_info,
            get_local_ips,
            generate_qr,
            refresh_network_info,
            check_accessibility,
            open_accessibility_settings,
            get_app_version,
        ])
        .setup(|app| {
            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let show = MenuItemBuilder::with_id("show", "Show Window").build(app)?;
            let refresh = MenuItemBuilder::with_id("refresh_net", "Refresh Network").build(app)?;
            let menu = MenuBuilder::new(app)
                .items(&[&show, &refresh, &quit])
                .build()?;

            let _tray = TrayIconBuilder::new()
                .icon(Image::from_path("icons/icon.png").unwrap_or_else(|_| {
                    app.default_window_icon().unwrap().clone()
                }))
                .menu(&menu)
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "refresh_net" => {
                        let h = app.app_handle().clone();
                        let _ = sync_network_from_os(&h, true);
                    }
                    _ => {}
                })
                .build(app)?;

            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                server::set_app_handle(app_handle);
            });

            let poll_handle = app.handle().clone();
            std::thread::spawn(move || loop {
                std::thread::sleep(Duration::from_secs(5));
                let _ = sync_network_from_os(&poll_handle, false);
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
