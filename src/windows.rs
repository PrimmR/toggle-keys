use windows::Win32::UI::Input::KeyboardAndMouse::*;

pub fn get_caps_lock_state() -> bool {
    unsafe { GetKeyState(VK_CAPITAL.0 as i32) & 0x0001 != 0 }
}

pub fn get_num_lock_state() -> bool {
    unsafe { GetKeyState(VK_NUMLOCK.0 as i32) & 0x0001 != 0 }
}

pub fn get_scroll_lock_state() -> bool {
    unsafe { GetKeyState(VK_SCROLL.0 as i32) & 0x0001 != 0 }
}

pub fn get_kana_lock_state() -> bool {
    unsafe { GetKeyState(VK_KANA.0 as i32) & 0x0001 != 0 }
}
