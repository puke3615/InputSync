mod config;
mod keyboard;
mod network;
mod qrcode_gen;
mod scene;
mod server;

use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::sync::Arc;
use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    Manager,
};

pub static SERVER_STATE: Lazy<Arc<Mutex<ServerState>>> =
    Lazy::new(|| Arc::new(Mutex::new(ServerState::default())));

#[derive(Default, Clone)]
pub struct ServerState {
    pub local_ip: String,
    pub port: u16,
    pub connected_clients: usize,
}

#[derive(Clone, serde::Serialize)]
struct ServerInfo {
    ip: String,
    port: u16,
    url: String,
    qrcode_data_url: String,
}

#[tauri::command]
fn get_server_info() -> Result<ServerInfo, String> {
    let state = SERVER_STATE.lock();
    let url = format!("http://{}:{}", state.local_ip, state.port);
    let qr = qrcode_gen::generate_qr_data_url(&url).map_err(|e| e.to_string())?;
    Ok(ServerInfo {
        ip: state.local_ip.clone(),
        port: state.port,
        url,
        qrcode_data_url: qr,
    })
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
fn open_config_dir() -> Result<(), String> {
    let dir = config::config_dir_path();
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&dir)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&dir)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&dir)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
fn get_scenes() -> Vec<scene::Scene> {
    scene::get_scenes()
}

#[tauri::command]
fn save_scene(new_scene: scene::Scene) -> Result<Vec<scene::Scene>, String> {
    if let Some(existing) = scene::find_scene(&new_scene.id) {
        if existing.builtin {
            return Err("Cannot modify built-in scene".to_string());
        }
    }
    let mut s = new_scene;
    s.builtin = false;
    scene::save_scene(s);
    let scenes = scene::get_scenes();
    config::save_scenes(&scenes);
    server::broadcast_scenes();
    Ok(scenes)
}

#[tauri::command]
fn delete_scene(id: String) -> Vec<scene::Scene> {
    let deleted = scene::delete_scene(&id);
    log::info!("delete_scene id={}, success={}", id, deleted);
    let scenes = scene::get_scenes();
    config::save_scenes(&scenes);
    server::broadcast_scenes();
    scenes
}

#[tauri::command]
fn export_scene_file(id: String) -> Result<String, String> {
    let scene = scene::find_scene(&id).ok_or_else(|| "Scene not found".to_string())?;
    let json = serde_json::to_string_pretty(&scene).map_err(|e| e.to_string())?;
    let safe_name: String = scene
        .name
        .chars()
        .map(|c| if c == '/' || c == '\\' || c == ':' { '_' } else { c })
        .collect();
    let filename = format!("TalkType-{}.json", safe_name);

    let path = rfd::FileDialog::new()
        .set_title("Export Scene")
        .set_file_name(&filename)
        .add_filter("JSON", &["json"])
        .save_file();

    match path {
        Some(p) => {
            std::fs::write(&p, &json).map_err(|e| e.to_string())?;
            Ok(p.display().to_string())
        }
        None => Err("Cancelled".to_string()),
    }
}

#[tauri::command]
fn import_scene_data(json: String, overwrite_id: Option<String>) -> Result<Vec<scene::Scene>, String> {
    let mut s: scene::Scene =
        serde_json::from_str(&json).map_err(|e| format!("Parse failed: {}", e))?;
    s.builtin = false;
    if let Some(oid) = overwrite_id {
        if let Some(existing) = scene::find_scene(&oid) {
            if existing.builtin {
                return Err("Cannot overwrite built-in scene".to_string());
            }
        }
        s.id = oid;
    } else {
        s.id = format!(
            "import-{}",
            &uuid::Uuid::new_v4().to_string()[..8]
        );
    }
    scene::save_scene(s);
    let scenes = scene::get_scenes();
    config::save_scenes(&scenes);
    server::broadcast_scenes();
    Ok(scenes)
}

pub fn run() {
    env_logger::init();

    // Load config and initialize scenes
    let app_config = config::load_config();
    scene::init(app_config.scenes);
    log::info!("Loaded {} scenes", scene::get_scenes().len());

    let port: u16 = 5678;
    let local_ip = network::get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string());

    {
        let mut state = SERVER_STATE.lock();
        state.local_ip = local_ip.clone();
        state.port = port;
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
            check_accessibility,
            open_accessibility_settings,
            open_config_dir,
            get_app_version,
            get_scenes,
            save_scene,
            delete_scene,
            export_scene_file,
            import_scene_data,
        ])
        .setup(|app| {
            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let show = MenuItemBuilder::with_id("show", "Show Window").build(app)?;
            let menu = MenuBuilder::new(app).items(&[&show, &quit]).build()?;

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
                    _ => {}
                })
                .build(app)?;

            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                server::set_app_handle(app_handle);
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
