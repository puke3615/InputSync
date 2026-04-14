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

    pub const VK_RETURN: u16 = 36;

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
    const VK_RETURN: u16 = 0x0D;

    fn send_key(vk: u16, up: bool) {
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

    pub fn press_enter() {
        send_key(VK_RETURN, false);
        send_key(VK_RETURN, true);
    }
}
