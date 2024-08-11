use evdev::Key as KeyCode;
use evdev::LedType as LedCode;

pub fn get_caps_lock_state() -> bool {
    let devices: Vec<_> = evdev::enumerate()
        .filter(|x| {
            x.1.supported_keys()
                .map_or(false, |keys| keys.contains(KeyCode::KEY_CAPSLOCK))
        })
        .collect();

    devices.iter().any(|d| {
        d.1.get_led_state()
            .map_or(false, |s| s.contains(LedCode::LED_CAPSL))
    })
}
