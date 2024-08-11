use windows::Win32::UI::Input::KeyboardAndMouse::*;

pub fn get_caps_lock_state() -> bool {
    unsafe { GetKeyState(VK_CAPITAL.0 as i32) & 0x0001 != 0 }
}