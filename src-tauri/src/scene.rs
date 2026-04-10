use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::keyboard;

static SCENES: Lazy<Arc<Mutex<Vec<Scene>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new())));
static EXECUTING: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_true")]
    pub realtime_sync: bool,
    pub trigger: Trigger,
    pub actions: Vec<Action>,
    #[serde(default)]
    pub builtin: bool,
}

fn default_true() -> bool {
    true
}
fn default_one() -> u32 {
    1
}
fn default_repeat_delay() -> u64 {
    50
}
fn default_wait_timeout() -> u64 {
    5000
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Trigger {
    Manual,
    Silence { timeout_ms: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Action {
    KeyPress {
        keys: Vec<String>,
        #[serde(default = "default_one")]
        repeat: u32,
        #[serde(default = "default_repeat_delay")]
        repeat_delay_ms: u64,
    },
    TypeText {
        text: String,
    },
    SyncText,
    Delay {
        ms: u64,
    },
    ClearInput,
    SetClipboard,
    TextTransform {
        transform: TransformType,
    },
    FocusApp {
        app_name: String,
    },
    OpenUrl {
        url_template: String,
    },
    Notification {
        #[serde(default)]
        message: String,
    },
    WaitForWindow {
        app_name: String,
        #[serde(default = "default_wait_timeout")]
        timeout_ms: u64,
    },
    Assert {
        condition: ConditionType,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TransformType {
    Trim,
    ToUpperCase,
    ToLowerCase,
    UrlEncode,
    StripPunctuation,
    StripLeadingPunctuation,
    AddPrefix { prefix: String },
    AddSuffix { suffix: String },
    Replace { from: String, to: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ConditionType {
    ForegroundApp { app_name: String },
    TextContains { keyword: String },
    TextNotContains { keyword: String },
    TextStartsWith { prefix: String },
    TextEndsWith { suffix: String },
    TextMinLength { min: usize },
}

// ─── Punctuation helpers ───

fn is_cjk_punctuation(c: char) -> bool {
    matches!(
        c,
        '，' | '、'
            | '。'
            | '！'
            | '？'
            | '；'
            | '：'
            | '\u{201c}'
            | '\u{201d}'
            | '\u{2018}'
            | '\u{2019}'
            | '（'
            | '）'
            | '【'
            | '】'
            | '《'
            | '》'
            | '「'
            | '」'
            | '『'
            | '』'
            | '〔'
            | '〕'
            | '…'
            | '—'
            | '～'
            | '·'
    )
}

fn strip_punctuation_prefix(text: &str) -> &str {
    text.trim_start_matches(|c: char| c.is_ascii_punctuation() || c.is_whitespace() || is_cjk_punctuation(c))
}

fn strip_all_punctuation(text: &str) -> String {
    text.chars()
        .filter(|c| !c.is_ascii_punctuation() && !is_cjk_punctuation(*c))
        .collect()
}

// ─── Transform helpers ───

fn apply_transform(text: &str, transform: &TransformType) -> String {
    match transform {
        TransformType::Trim => text.trim().to_string(),
        TransformType::ToUpperCase => text.to_uppercase(),
        TransformType::ToLowerCase => text.to_lowercase(),
        TransformType::UrlEncode => url_encode(text),
        TransformType::StripPunctuation => strip_all_punctuation(text),
        TransformType::StripLeadingPunctuation => strip_punctuation_prefix(text).to_string(),
        TransformType::AddPrefix { prefix } => format!("{}{}", prefix, text),
        TransformType::AddSuffix { suffix } => format!("{}{}", text, suffix),
        TransformType::Replace { from, to } => text.replace(from.as_str(), to.as_str()),
    }
}

fn url_encode(text: &str) -> String {
    let mut result = String::with_capacity(text.len() * 3);
    for b in text.as_bytes() {
        match *b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(*b as char);
            }
            _ => {
                result.push_str(&format!("%{:02X}", b));
            }
        }
    }
    result
}

// ─── Condition evaluation ───

fn evaluate_condition(condition: &ConditionType, text: &str) -> bool {
    match condition {
        ConditionType::ForegroundApp { app_name } => {
            if let Some(fg) = get_foreground_app() {
                let matches = fg.to_lowercase().contains(&app_name.to_lowercase());
                log::info!("ForegroundApp check: '{}' contains '{}' = {}", fg, app_name, matches);
                matches
            } else {
                log::warn!("Could not determine foreground app");
                false
            }
        }
        ConditionType::TextContains { keyword } => text.contains(keyword.as_str()),
        ConditionType::TextNotContains { keyword } => !text.contains(keyword.as_str()),
        ConditionType::TextStartsWith { prefix } => text.starts_with(prefix.as_str()),
        ConditionType::TextEndsWith { suffix } => text.ends_with(suffix.as_str()),
        ConditionType::TextMinLength { min } => text.chars().count() >= *min,
    }
}

// ─── Platform: get foreground app ───

fn get_foreground_app() -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("osascript")
            .args([
                "-e",
                "tell application \"System Events\" to get name of first application process whose frontmost is true",
            ])
            .output()
            .ok()?;
        if output.status.success() {
            Some(
                String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .to_string(),
            )
        } else {
            None
        }
    }
    #[cfg(target_os = "windows")]
    {
        let output = std::process::Command::new("powershell")
            .args([
                "-Command",
                "(Get-Process | Where-Object {$_.MainWindowHandle -eq (Add-Type -MemberDefinition '[DllImport(\"user32.dll\")] public static extern IntPtr GetForegroundWindow();' -Name Win32 -Namespace Temp -PassThru)::GetForegroundWindow()}).ProcessName",
            ])
            .output()
            .ok()?;
        if output.status.success() {
            Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            None
        }
    }
    #[cfg(target_os = "linux")]
    {
        let output = std::process::Command::new("xdotool")
            .args(["getactivewindow", "getwindowpid"])
            .output()
            .ok()?;
        if output.status.success() {
            let pid = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let comm = std::fs::read_to_string(format!("/proc/{}/comm", pid)).ok()?;
            Some(comm.trim().to_string())
        } else {
            None
        }
    }
}

// ─── Platform: focus application ───

fn focus_application(app_name: &str) {
    #[cfg(target_os = "macos")]
    {
        let script = format!(
            "tell application \"{}\" to activate",
            app_name.replace('"', "\\\"")
        );
        let _ = std::process::Command::new("osascript")
            .args(["-e", &script])
            .output();
    }
    #[cfg(target_os = "windows")]
    {
        let script = format!(
            "(New-Object -ComObject WScript.Shell).AppActivate('{}')",
            app_name.replace('\'', "''")
        );
        let _ = std::process::Command::new("powershell")
            .args(["-Command", &script])
            .output();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("wmctrl")
            .args(["-a", app_name])
            .output();
    }
}

// ─── Platform: open URL ───

fn open_url(url: &str) {
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(url).output();
    }
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start", "", url])
            .output();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open").arg(url).output();
    }
}

// ─── Platform: notification ───

fn show_notification(msg: &str) {
    #[cfg(target_os = "macos")]
    {
        let script = format!(
            "display notification \"{}\" with title \"TalkType\"",
            msg.replace('"', "\\\"").replace('\n', " ")
        );
        let _ = std::process::Command::new("osascript")
            .args(["-e", &script])
            .output();
    }
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("powershell")
            .args([
                "-Command",
                &format!(
                    "Add-Type -AssemblyName PresentationFramework; [System.Windows.MessageBox]::Show('{}', 'TalkType')",
                    msg.replace('\'', "''")
                ),
            ])
            .output();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("notify-send")
            .args(["TalkType", msg])
            .output();
    }
}

// ─── Clipboard ───

fn set_clipboard(text: &str) {
    match arboard::Clipboard::new() {
        Ok(mut cb) => {
            if let Err(e) = cb.set_text(text) {
                log::error!("Failed to set clipboard: {}", e);
            } else {
                log::info!("Clipboard set: {} chars", text.len());
            }
        }
        Err(e) => {
            log::error!("Failed to access clipboard: {}", e);
        }
    }
}

// ─── Default scenes ───

pub fn default_scenes() -> Vec<Scene> {
    let s1: Scene = serde_json::from_str(include_str!("../scenes/real-time-sync.json"))
        .expect("Failed to parse real-time-sync.json");
    let s2: Scene = serde_json::from_str(include_str!("../scenes/send-message.json"))
        .expect("Failed to parse send-message.json");
    vec![s1, s2]
}

// ─── Scene store ───

pub fn init(scenes: Vec<Scene>) {
    let mut store = SCENES.lock();
    *store = scenes;
}

pub fn get_scenes() -> Vec<Scene> {
    SCENES.lock().clone()
}

pub fn save_scene(scene: Scene) {
    let mut store = SCENES.lock();
    if let Some(existing) = store.iter_mut().find(|s| s.id == scene.id) {
        *existing = scene;
    } else {
        store.push(scene);
    }
}

pub fn delete_scene(id: &str) -> bool {
    let mut store = SCENES.lock();
    let before = store.len();
    store.retain(|s| s.id != id || s.builtin);
    store.len() < before
}

pub fn find_scene(id: &str) -> Option<Scene> {
    SCENES.lock().iter().find(|s| s.id == id).cloned()
}

// ─── Scene execution ───

pub fn execute_scene(scene_id: &str, text: &str) {
    if EXECUTING
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        log::warn!("Scene already executing, skipping trigger");
        return;
    }

    let scene = match find_scene(scene_id) {
        Some(s) => s,
        None => {
            log::error!("Scene not found: {}", scene_id);
            EXECUTING.store(false, Ordering::SeqCst);
            return;
        }
    };

    if scene.actions.is_empty() {
        log::info!("Scene '{}' has no actions", scene.name);
        EXECUTING.store(false, Ordering::SeqCst);
        return;
    }

    log::info!(
        "Executing scene '{}' with {} actions",
        scene.name,
        scene.actions.len()
    );

    let mut current_text = text.to_string();

    for (i, action) in scene.actions.iter().enumerate() {
        log::info!("  Action {}: {:?}", i + 1, action);
        match action {
            Action::KeyPress {
                keys,
                repeat,
                repeat_delay_ms,
            } => {
                let count = (*repeat).max(1);
                for r in 0..count {
                    keyboard::press_keys(keys);
                    if r < count - 1 && *repeat_delay_ms > 0 {
                        std::thread::sleep(Duration::from_millis(*repeat_delay_ms));
                    }
                }
            }
            Action::TypeText { text: t } => {
                let t = t.replace("{text}", &current_text);
                keyboard::type_text(&t);
            }
            Action::SyncText => {
                if !current_text.is_empty() {
                    keyboard::type_text(&current_text);
                }
            }
            Action::Delay { ms } => {
                std::thread::sleep(Duration::from_millis(*ms));
            }
            Action::ClearInput => {
                keyboard::clear_current();
            }
            Action::SetClipboard => {
                set_clipboard(&current_text);
            }
            Action::TextTransform { transform } => {
                current_text = apply_transform(&current_text, transform);
                log::info!("    Text after transform: '{}'", current_text);
            }
            Action::FocusApp { app_name } => {
                focus_application(app_name);
                std::thread::sleep(Duration::from_millis(300));
            }
            Action::OpenUrl { url_template } => {
                let url = url_template.replace("{text}", &url_encode(&current_text));
                log::info!("    Opening URL: {}", url);
                open_url(&url);
            }
            Action::Notification { message } => {
                let msg = if message.is_empty() {
                    format!("Scene '{}' executed", scene.name)
                } else {
                    message.replace("{text}", &current_text)
                };
                show_notification(&msg);
            }
            Action::WaitForWindow {
                app_name,
                timeout_ms,
            } => {
                let start = std::time::Instant::now();
                let timeout = Duration::from_millis(*timeout_ms);
                let target = app_name.to_lowercase();
                loop {
                    if let Some(fg) = get_foreground_app() {
                        if fg.to_lowercase().contains(&target) {
                            log::info!("    Window '{}' found", fg);
                            break;
                        }
                    }
                    if start.elapsed() > timeout {
                        log::warn!("    WaitForWindow timeout for '{}'", app_name);
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(200));
                }
            }
            Action::Assert { condition } => {
                if !evaluate_condition(condition, &current_text) {
                    log::info!("    Assertion failed, stopping scene");
                    break;
                }
            }
        }
        if !matches!(action, Action::Delay { .. }) {
            std::thread::sleep(Duration::from_millis(30));
        }
    }

    log::info!("Scene '{}' execution complete", scene.name);
    EXECUTING.store(false, Ordering::SeqCst);
}
