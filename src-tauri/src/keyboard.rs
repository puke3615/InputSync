use enigo::{Enigo, Keyboard as _, Settings};

pub fn type_text(text: &str) {
    let mut enigo = match Enigo::new(&Settings::default()) {
        Ok(e) => e,
        Err(e) => {
            log::error!("Failed to create Enigo: {}", e);
            return;
        }
    };
    if let Err(e) = enigo.text(text) {
        log::error!("Failed to type text: {}", e);
    }
}

pub fn press_enter() {
    #[cfg(target_os = "macos")]
    {
        macos::post_key(macos::VK_RETURN, 0);
    }
    #[cfg(target_os = "windows")]
    {
        win::press_enter();
    }
    #[cfg(target_os = "linux")]
    {
        use enigo::{Key, Direction};
        let mut enigo = match Enigo::new(&Settings::default()) {
            Ok(e) => e,
            Err(e) => { log::error!("Enigo: {}", e); return; }
        };
        let _ = enigo.key(Key::Return, Direction::Click);
    }
}

pub fn replace_all_text(text: &str) {
    select_all();
    std::thread::sleep(std::time::Duration::from_millis(30));

    if text.is_empty() {
        press_backspace();
    } else {
        type_text(text);
    }
}

pub fn clear_current() {
    replace_all_text("");
}

/// Press a key combination specified by logical key names.
/// Modifiers (Ctrl, Shift, Alt, Cmd, CmdOrCtrl) are held while the main key is pressed.
/// Example: `["CmdOrCtrl", "C"]` → Cmd+C on macOS, Ctrl+C on Windows/Linux.
pub fn press_keys(keys: &[String]) {
    if keys.is_empty() {
        return;
    }

    #[cfg(target_os = "macos")]
    {
        let mut flags: u64 = 0;
        let mut main_keys: Vec<u16> = Vec::new();

        for key in keys {
            if let Some(flag) = macos::modifier_flag(key) {
                flags |= flag;
            } else if let Some(vk) = macos::keycode(key) {
                main_keys.push(vk);
            } else {
                log::warn!("Unknown key: {}", key);
            }
        }

        if main_keys.is_empty() {
            if flags != 0 {
                log::warn!("No main key specified in combination, only modifiers");
            }
            return;
        }

        for (i, vk) in main_keys.iter().enumerate() {
            macos::post_key(*vk, flags);
            if i < main_keys.len() - 1 {
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        let mut modifiers: Vec<u16> = Vec::new();
        let mut main_keys: Vec<u16> = Vec::new();

        for key in keys {
            if win::is_modifier(key) {
                if let Some(vk) = win::vk_code(key) {
                    modifiers.push(vk);
                }
            } else if let Some(vk) = win::vk_code(key) {
                main_keys.push(vk);
            } else {
                log::warn!("Unknown key: {}", key);
            }
        }

        for &vk in &modifiers {
            win::send_key(vk, false);
        }
        for &vk in &main_keys {
            win::send_key(vk, false);
            win::send_key(vk, true);
        }
        for &vk in modifiers.iter().rev() {
            win::send_key(vk, true);
        }
    }

    #[cfg(target_os = "linux")]
    {
        use enigo::{Key, Direction};
        let mut enigo = match Enigo::new(&Settings::default()) {
            Ok(e) => e,
            Err(e) => { log::error!("Enigo: {}", e); return; }
        };

        // Press modifiers
        for key_name in keys {
            if linux_is_modifier(key_name) {
                if let Some(k) = linux_enigo_key(key_name) {
                    let _ = enigo.key(k, Direction::Press);
                }
            }
        }
        // Click main keys
        for key_name in keys {
            if !linux_is_modifier(key_name) {
                if let Some(k) = linux_enigo_key(key_name) {
                    let _ = enigo.key(k, Direction::Click);
                }
            }
        }
        // Release modifiers in reverse
        for key_name in keys.iter().rev() {
            if linux_is_modifier(key_name) {
                if let Some(k) = linux_enigo_key(key_name) {
                    let _ = enigo.key(k, Direction::Release);
                }
            }
        }
    }
}

fn select_all() {
    #[cfg(target_os = "macos")]
    {
        macos::post_key(macos::VK_A, macos::FLAG_CMD);
    }
    #[cfg(target_os = "windows")]
    {
        win::select_all();
    }
    #[cfg(target_os = "linux")]
    {
        use enigo::{Key, Direction};
        let mut enigo = match Enigo::new(&Settings::default()) {
            Ok(e) => e,
            Err(e) => { log::error!("Enigo: {}", e); return; }
        };
        let _ = enigo.key(Key::Control, Direction::Press);
        let _ = enigo.key(Key::Unicode('a'), Direction::Click);
        let _ = enigo.key(Key::Control, Direction::Release);
    }
}

fn press_backspace() {
    #[cfg(target_os = "macos")]
    {
        macos::post_key(macos::VK_DELETE, 0);
    }
    #[cfg(target_os = "windows")]
    {
        win::press_backspace();
    }
    #[cfg(target_os = "linux")]
    {
        use enigo::{Key, Direction};
        let mut enigo = match Enigo::new(&Settings::default()) {
            Ok(e) => e,
            Err(e) => { log::error!("Enigo: {}", e); return; }
        };
        let _ = enigo.key(Key::Backspace, Direction::Click);
    }
}

// ─── Linux key mapping helpers ───

#[cfg(target_os = "linux")]
fn linux_enigo_key(name: &str) -> Option<enigo::Key> {
    use enigo::Key;
    match name.to_lowercase().as_str() {
        "enter" | "return" => Some(Key::Return),
        "tab" => Some(Key::Tab),
        "space" => Some(Key::Space),
        "backspace" => Some(Key::Backspace),
        "delete" => Some(Key::Delete),
        "escape" | "esc" => Some(Key::Escape),
        "shift" => Some(Key::Shift),
        "ctrl" | "control" | "cmdorctrl" => Some(Key::Control),
        "alt" | "option" => Some(Key::Alt),
        "cmd" | "command" | "meta" | "super" => Some(Key::Meta),
        "capslock" => Some(Key::CapsLock),
        "f1" => Some(Key::F1),
        "f2" => Some(Key::F2),
        "f3" => Some(Key::F3),
        "f4" => Some(Key::F4),
        "f5" => Some(Key::F5),
        "f6" => Some(Key::F6),
        "f7" => Some(Key::F7),
        "f8" => Some(Key::F8),
        "f9" => Some(Key::F9),
        "f10" => Some(Key::F10),
        "f11" => Some(Key::F11),
        "f12" => Some(Key::F12),
        "arrowup" | "up" => Some(Key::UpArrow),
        "arrowdown" | "down" => Some(Key::DownArrow),
        "arrowleft" | "left" => Some(Key::LeftArrow),
        "arrowright" | "right" => Some(Key::RightArrow),
        "home" => Some(Key::Home),
        "end" => Some(Key::End),
        "pageup" => Some(Key::PageUp),
        "pagedown" => Some(Key::PageDown),
        s if s.len() == 1 => s.chars().next().map(Key::Unicode),
        _ => None,
    }
}

#[cfg(target_os = "linux")]
fn linux_is_modifier(name: &str) -> bool {
    matches!(
        name.to_lowercase().as_str(),
        "shift" | "ctrl" | "control" | "alt" | "option"
            | "cmd" | "command" | "meta" | "super" | "cmdorctrl"
    )
}

// ─── macOS: use CoreGraphics CGEvent API directly ───

#[cfg(target_os = "macos")]
mod macos {
    use std::ffi::c_void;

    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C" {
        fn CGEventCreateKeyboardEvent(
            source: *const c_void,
            virtual_key: u16,
            key_down: bool,
        ) -> *mut c_void;
        fn CGEventSetFlags(event: *mut c_void, flags: u64);
        fn CGEventPost(tap: u32, event: *mut c_void);
    }

    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        fn CFRelease(cf: *mut c_void);
    }

    pub const FLAG_CMD: u64 = 1 << 20;
    pub const FLAG_SHIFT: u64 = 1 << 17;
    pub const FLAG_CTRL: u64 = 1 << 18;
    pub const FLAG_ALT: u64 = 1 << 19;

    pub const VK_A: u16 = 0;
    pub const VK_RETURN: u16 = 36;
    pub const VK_DELETE: u16 = 51;

    pub fn post_key(vk: u16, flags: u64) {
        unsafe {
            let down = CGEventCreateKeyboardEvent(std::ptr::null(), vk, true);
            if !down.is_null() {
                if flags != 0 {
                    CGEventSetFlags(down, flags);
                }
                CGEventPost(0, down);
                CFRelease(down);
            }

            std::thread::sleep(std::time::Duration::from_millis(5));

            let up = CGEventCreateKeyboardEvent(std::ptr::null(), vk, false);
            if !up.is_null() {
                if flags != 0 {
                    CGEventSetFlags(up, flags);
                }
                CGEventPost(0, up);
                CFRelease(up);
            }
        }
    }

    pub fn keycode(name: &str) -> Option<u16> {
        match name.to_lowercase().as_str() {
            "a" => Some(0x00),
            "s" => Some(0x01),
            "d" => Some(0x02),
            "f" => Some(0x03),
            "h" => Some(0x04),
            "g" => Some(0x05),
            "z" => Some(0x06),
            "x" => Some(0x07),
            "c" => Some(0x08),
            "v" => Some(0x09),
            "b" => Some(0x0B),
            "q" => Some(0x0C),
            "w" => Some(0x0D),
            "e" => Some(0x0E),
            "r" => Some(0x0F),
            "y" => Some(0x10),
            "t" => Some(0x11),
            "1" => Some(0x12),
            "2" => Some(0x13),
            "3" => Some(0x14),
            "4" => Some(0x15),
            "5" => Some(0x17),
            "6" => Some(0x16),
            "7" => Some(0x1A),
            "8" => Some(0x1C),
            "9" => Some(0x19),
            "0" => Some(0x1D),
            "o" => Some(0x1F),
            "u" => Some(0x20),
            "i" => Some(0x22),
            "p" => Some(0x23),
            "l" => Some(0x25),
            "j" => Some(0x26),
            "k" => Some(0x28),
            "n" => Some(0x2D),
            "m" => Some(0x2E),
            "enter" | "return" => Some(0x24),
            "tab" => Some(0x30),
            "space" => Some(0x31),
            "backspace" | "delete" => Some(0x33),
            "forwarddelete" => Some(0x75),
            "escape" | "esc" => Some(0x35),
            "f1" => Some(0x7A),
            "f2" => Some(0x78),
            "f3" => Some(0x63),
            "f4" => Some(0x76),
            "f5" => Some(0x60),
            "f6" => Some(0x61),
            "f7" => Some(0x62),
            "f8" => Some(0x64),
            "f9" => Some(0x65),
            "f10" => Some(0x6D),
            "f11" => Some(0x67),
            "f12" => Some(0x6F),
            "arrowup" | "up" => Some(0x7E),
            "arrowdown" | "down" => Some(0x7D),
            "arrowleft" | "left" => Some(0x7B),
            "arrowright" | "right" => Some(0x7C),
            "home" => Some(0x73),
            "end" => Some(0x77),
            "pageup" => Some(0x74),
            "pagedown" => Some(0x79),
            "-" | "minus" => Some(0x1B),
            "=" | "equal" => Some(0x18),
            "[" | "leftbracket" => Some(0x21),
            "]" | "rightbracket" => Some(0x1E),
            ";" | "semicolon" => Some(0x29),
            "'" | "quote" => Some(0x27),
            "\\" | "backslash" => Some(0x2A),
            "," | "comma" => Some(0x2B),
            "." | "period" => Some(0x2F),
            "/" | "slash" => Some(0x2C),
            "`" | "grave" => Some(0x32),
            _ => None,
        }
    }

    pub fn modifier_flag(name: &str) -> Option<u64> {
        match name.to_lowercase().as_str() {
            "shift" => Some(FLAG_SHIFT),
            "ctrl" | "control" => Some(FLAG_CTRL),
            "alt" | "option" => Some(FLAG_ALT),
            "cmd" | "command" | "meta" | "super" | "cmdorctrl" => Some(FLAG_CMD),
            _ => None,
        }
    }
}

// ─── Windows: use winapi SendInput ───

#[cfg(target_os = "windows")]
mod win {
    use std::mem;

    #[repr(C)]
    struct KeybdInput {
        w_vk: u16,
        w_scan: u16,
        dw_flags: u32,
        time: u32,
        dw_extra_info: usize,
    }

    #[repr(C)]
    struct Input {
        input_type: u32,
        ki: KeybdInput,
        padding: [u8; 8],
    }

    extern "system" {
        fn SendInput(c_inputs: u32, p_inputs: *const Input, cb_size: i32) -> u32;
    }

    const INPUT_KEYBOARD: u32 = 1;
    const KEYEVENTF_KEYUP: u32 = 0x0002;
    const VK_CONTROL: u16 = 0x11;
    const VK_A: u16 = 0x41;
    const VK_RETURN: u16 = 0x0D;
    const VK_BACK: u16 = 0x08;

    pub fn send_key(vk: u16, up: bool) {
        let input = Input {
            input_type: INPUT_KEYBOARD,
            ki: KeybdInput {
                w_vk: vk,
                w_scan: 0,
                dw_flags: if up { KEYEVENTF_KEYUP } else { 0 },
                time: 0,
                dw_extra_info: 0,
            },
            padding: [0; 8],
        };
        unsafe {
            SendInput(1, &input, mem::size_of::<Input>() as i32);
        }
    }

    pub fn select_all() {
        send_key(VK_CONTROL, false);
        send_key(VK_A, false);
        send_key(VK_A, true);
        send_key(VK_CONTROL, true);
    }

    pub fn press_enter() {
        send_key(VK_RETURN, false);
        send_key(VK_RETURN, true);
    }

    pub fn press_backspace() {
        send_key(VK_BACK, false);
        send_key(VK_BACK, true);
    }

    pub fn vk_code(name: &str) -> Option<u16> {
        match name.to_lowercase().as_str() {
            "a" => Some(0x41), "b" => Some(0x42), "c" => Some(0x43),
            "d" => Some(0x44), "e" => Some(0x45), "f" => Some(0x46),
            "g" => Some(0x47), "h" => Some(0x48), "i" => Some(0x49),
            "j" => Some(0x4A), "k" => Some(0x4B), "l" => Some(0x4C),
            "m" => Some(0x4D), "n" => Some(0x4E), "o" => Some(0x4F),
            "p" => Some(0x50), "q" => Some(0x51), "r" => Some(0x52),
            "s" => Some(0x53), "t" => Some(0x54), "u" => Some(0x55),
            "v" => Some(0x56), "w" => Some(0x57), "x" => Some(0x58),
            "y" => Some(0x59), "z" => Some(0x5A),
            "0" => Some(0x30), "1" => Some(0x31), "2" => Some(0x32),
            "3" => Some(0x33), "4" => Some(0x34), "5" => Some(0x35),
            "6" => Some(0x36), "7" => Some(0x37), "8" => Some(0x38),
            "9" => Some(0x39),
            "enter" | "return" => Some(0x0D),
            "tab" => Some(0x09),
            "space" => Some(0x20),
            "backspace" => Some(0x08),
            "delete" | "forwarddelete" => Some(0x2E),
            "escape" | "esc" => Some(0x1B),
            "shift" => Some(0x10),
            "ctrl" | "control" | "cmdorctrl" => Some(0x11),
            "alt" | "option" => Some(0x12),
            "cmd" | "command" | "meta" | "super" => Some(0x5B),
            "f1" => Some(0x70), "f2" => Some(0x71), "f3" => Some(0x72),
            "f4" => Some(0x73), "f5" => Some(0x74), "f6" => Some(0x75),
            "f7" => Some(0x76), "f8" => Some(0x77), "f9" => Some(0x78),
            "f10" => Some(0x79), "f11" => Some(0x7A), "f12" => Some(0x7B),
            "arrowup" | "up" => Some(0x26),
            "arrowdown" | "down" => Some(0x28),
            "arrowleft" | "left" => Some(0x25),
            "arrowright" | "right" => Some(0x27),
            "home" => Some(0x24),
            "end" => Some(0x23),
            "pageup" => Some(0x21),
            "pagedown" => Some(0x22),
            "-" | "minus" => Some(0xBD),
            "=" | "equal" => Some(0xBB),
            "[" | "leftbracket" => Some(0xDB),
            "]" | "rightbracket" => Some(0xDD),
            ";" | "semicolon" => Some(0xBA),
            "'" | "quote" => Some(0xDE),
            "\\" | "backslash" => Some(0xDC),
            "," | "comma" => Some(0xBC),
            "." | "period" => Some(0xBE),
            "/" | "slash" => Some(0xBF),
            "`" | "grave" => Some(0xC0),
            _ => None,
        }
    }

    pub fn is_modifier(name: &str) -> bool {
        matches!(
            name.to_lowercase().as_str(),
            "shift" | "ctrl" | "control" | "alt" | "option"
                | "cmd" | "command" | "meta" | "super" | "cmdorctrl"
        )
    }
}
