use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::{Html, IntoResponse, Json},
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use once_cell::sync::{Lazy, OnceCell};
use std::sync::atomic::{AtomicUsize, Ordering};
use tauri::AppHandle;
use tauri::Emitter;
use tokio::sync::broadcast;
use tower_http::cors::CorsLayer;

use crate::keyboard;
use crate::scene;

static APP_HANDLE: OnceCell<AppHandle> = OnceCell::new();
static CLIENT_COUNT: AtomicUsize = AtomicUsize::new(0);
static SCENE_TX: Lazy<broadcast::Sender<String>> = Lazy::new(|| {
    let (tx, _) = broadcast::channel(16);
    tx
});

pub fn set_app_handle(handle: AppHandle) {
    let _ = APP_HANDLE.set(handle);
}

fn emit_client_count() {
    let count = CLIENT_COUNT.load(Ordering::Relaxed);
    if let Some(handle) = APP_HANDLE.get() {
        let _ = handle.emit("client-count", serde_json::json!({ "count": count }));
    }
}

pub async fn start_server(_ip: &str, port: u16) {
    let app = Router::new()
        .route("/", get(serve_mobile_page))
        .route("/ws", get(ws_handler))
        .route("/api/info", get(serve_info))
        .route("/api/qrcode", get(serve_qrcode))
        .route("/manifest.json", get(serve_manifest))
        .layer(CorsLayer::permissive());

    let addr = format!("0.0.0.0:{}", port);
    log::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind server");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}

async fn serve_info() -> Json<serde_json::Value> {
    let state = crate::SERVER_STATE.lock();
    let url = format!("http://{}:{}", state.local_ip, state.port);
    let qr = crate::qrcode_gen::generate_qr_data_url(&url).unwrap_or_default();
    Json(serde_json::json!({
        "name": "InputSync",
        "ip": state.local_ip,
        "port": state.port,
        "url": url,
        "qrcode": qr,
        "version": "1.0.0"
    }))
}

async fn serve_qrcode() -> impl IntoResponse {
    let state = crate::SERVER_STATE.lock();
    let url = format!("http://{}:{}", state.local_ip, state.port);
    drop(state);
    match crate::qrcode_gen::generate_qr_png_bytes(&url) {
        Ok(bytes) => (
            [("content-type", "image/png")],
            bytes,
        ).into_response(),
        Err(_) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to generate QR code",
        ).into_response(),
    }
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

pub fn broadcast_scenes() {
    let scenes = scene::get_scenes();
    let msg = serde_json::json!({
        "type": "scenes",
        "scenes": scenes
    })
    .to_string();
    let _ = SCENE_TX.send(msg);
}

async fn handle_socket(socket: WebSocket) {
    CLIENT_COUNT.fetch_add(1, Ordering::Relaxed);
    emit_client_count();
    log::info!(
        "Client connected. Total: {}",
        CLIENT_COUNT.load(Ordering::Relaxed)
    );

    let (mut sender, mut receiver) = socket.split();

    // Send welcome message
    let _ = sender
        .send(Message::Text(
            serde_json::json!({
                "type": "connected",
                "message": "已连接到电脑"
            })
            .to_string()
            .into(),
        ))
        .await;

    // Send available scenes
    let scenes = scene::get_scenes();
    let _ = sender
        .send(Message::Text(
            serde_json::json!({
                "type": "scenes",
                "scenes": scenes
            })
            .to_string()
            .into(),
        ))
        .await;

    let mut scene_rx = SCENE_TX.subscribe();
    let mut active_scene_id = String::from("default");

    loop {
        tokio::select! {
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&text);
                        match parsed {
                            Ok(msg_val) => {
                                let msg_type = msg_val["type"].as_str().unwrap_or("");
                                match msg_type {
                                    "switch_scene" => {
                                        if let Some(id) = msg_val["scene_id"].as_str() {
                                            active_scene_id = id.to_string();
                                            log::info!("Scene switched to: {}", id);
                                        }
                                    }
                                    "trigger_scene" => {
                                        let sync_text =
                                            msg_val["text"].as_str().unwrap_or("").to_string();
                                        let scene_id = active_scene_id.clone();
                                        std::thread::spawn(move || {
                                            scene::execute_scene(&scene_id, &sync_text);
                                        });
                                    }
                                    _ => {
                                        handle_text_message(&msg_val);
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("Failed to parse message: {}", e);
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Err(e)) => {
                        log::error!("WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
            Ok(scene_msg) = scene_rx.recv() => {
                if sender.send(Message::Text(scene_msg.into())).await.is_err() {
                    break;
                }
            }
        }
    }

    CLIENT_COUNT.fetch_sub(1, Ordering::Relaxed);
    emit_client_count();
    log::info!(
        "Client disconnected. Total: {}",
        CLIENT_COUNT.load(Ordering::Relaxed)
    );
}

fn handle_text_message(msg: &serde_json::Value) {
    let msg_type = msg["type"].as_str().unwrap_or("");
    match msg_type {
        "input" => {
            if let Some(content) = msg["text"].as_str() {
                if !content.is_empty() {
                    log::info!("Typing: {}", content);
                    keyboard::type_text(content);
                }
            }
        }
        "enter" => {
            log::info!("Pressing enter");
            keyboard::press_enter();
        }
        "sync" => {
            if let Some(content) = msg["text"].as_str() {
                log::info!("Sync: {} chars", content.len());
                keyboard::replace_all_text(content);
            }
        }
        "clear" => {
            log::info!("Clearing current input");
            keyboard::clear_current();
        }
        "ping" => {}
        _ => {
            log::warn!("Unknown message type: {}", msg_type);
        }
    }
}

async fn serve_manifest() -> impl IntoResponse {
    (
        [("content-type", "application/manifest+json")],
        r##"{"name":"InputSync","short_name":"InputSync","start_url":"/","display":"standalone","background_color":"#0f0f1a","theme_color":"#6366f1"}"##,
    )
}

async fn serve_mobile_page() -> Html<String> {
    Html(include_str!("mobile.html").to_string())
}
