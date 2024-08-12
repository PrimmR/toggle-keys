#![doc = include_str!("../README.md")]

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows as os;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux as os;

/// Returns the current state of the Caps Lock toggle from the OS
///
/// **Linux:** This requires the user to have read permissions for `/dev/input` devices
pub fn get_caps_lock_state() -> bool {
    os::get_caps_lock_state()
}

pub fn get_num_lock_state() -> bool {
    os::get_num_lock_state()
}

pub fn get_scroll_lock_state() -> bool {
    os::get_scroll_lock_state()
}

pub fn get_kana_lock_state() -> bool {
    os::get_kana_lock_state()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caps() {
        println!("Caps: {}", get_caps_lock_state())
    }

    #[test]
    pub fn num() {
        println!("Num: {}", get_num_lock_state())
    }

    #[test]
    pub fn scroll() {
        println!("Scroll: {}", get_scroll_lock_state())
    }

    #[test]
    pub fn kana() {
        println!("Kana: {}", get_kana_lock_state())
    }
}
