use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::{Html, IntoResponse, Json},
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use once_cell::sync::OnceCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use tauri::AppHandle;
use tauri::Emitter;
use tower_http::cors::CorsLayer;

use crate::keyboard;

static APP_HANDLE: OnceCell<AppHandle> = OnceCell::new();
static CLIENT_COUNT: AtomicUsize = AtomicUsize::new(0);

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
        "name": "TalkType",
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

async fn handle_socket(socket: WebSocket) {
    CLIENT_COUNT.fetch_add(1, Ordering::Relaxed);
    emit_client_count();
    log::info!(
        "Client connected. Total: {}",
        CLIENT_COUNT.load(Ordering::Relaxed)
    );

    let (mut sender, mut receiver) = socket.split();

    let _ = sender
        .send(Message::Text(
            serde_json::json!({
                "type": "connected",
                "message": "Connected to computer"
            })
            .to_string()
            .into(),
        ))
        .await;

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(msg_val) = serde_json::from_str::<serde_json::Value>(&text) {
                    handle_message(&msg_val);
                }
            }
            Ok(Message::Close(_)) => break,
            Err(e) => {
                log::error!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }

    CLIENT_COUNT.fetch_sub(1, Ordering::Relaxed);
    emit_client_count();
    log::info!(
        "Client disconnected. Total: {}",
        CLIENT_COUNT.load(Ordering::Relaxed)
    );
}

fn handle_message(msg: &serde_json::Value) {
    let msg_type = msg["type"].as_str().unwrap_or("");
    match msg_type {
        "send" => {
            if let Some(text) = msg["text"].as_str() {
                if !text.is_empty() {
                    log::info!("Send: {} chars", text.len());
                    keyboard::type_text(text);
                }
            }
            if msg["auto_enter"].as_bool().unwrap_or(false) {
                std::thread::sleep(std::time::Duration::from_millis(30));
                keyboard::press_enter();
            }
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
        r##"{"name":"TalkType","short_name":"TalkType","description":"Speak on your phone, text appears on your computer. Voice input for vibe coding.","start_url":"/","display":"standalone","background_color":"#0f0f1a","theme_color":"#6366f1"}"##,
    )
}

async fn serve_mobile_page() -> Html<String> {
    Html(include_str!("mobile.html").to_string())
}
