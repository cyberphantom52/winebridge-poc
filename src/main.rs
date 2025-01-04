use windows::{core::*, Win32::UI::Shell::*, Win32::UI::WindowsAndMessaging::*};

fn main() {
    unsafe {
        MessageBoxW(None, h!("WinRT"), h!("World"), MB_OK);
    }
}
