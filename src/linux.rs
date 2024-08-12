use evdev::Key as KeyCode;
use evdev::LedType as LedCode;

fn get_toggle_state(key: KeyCode, led: LedCode) -> bool {
    let devices: Vec<_> = evdev::enumerate()
        .filter(|x| {
            x.1.supported_keys()
                .map_or(false, |keys| keys.contains(key))
        })
        .collect();

    devices
        .iter()
        .any(|d| d.1.get_led_state().map_or(false, |s| s.contains(led)))
}

pub fn get_caps_lock_state() -> bool {
    get_toggle_state(KeyCode::KEY_CAPSLOCK, LedCode::LED_CAPSL)
}

pub fn get_num_lock_state() -> bool {
    get_toggle_state(KeyCode::KEY_NUMLOCK, LedCode::LED_NUML)
}

pub fn get_scroll_lock_state() -> bool {
    get_toggle_state(KeyCode::KEY_SCROLLLOCK, LedCode::LED_SCROLLL)
}

pub fn get_kana_lock_state() -> bool {
    get_toggle_state(KeyCode::KEY_KATAKANA, LedCode::LED_KANA)
}
