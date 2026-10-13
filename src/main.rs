#![windows_subsystem = "windows"]

use std::ptr::null_mut;
use std::thread::sleep;
use std::time::Duration;
use win_nightlight_lib::Nightlight;
use windows_sys::Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MB_OK, MessageBoxW};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let nightlight = Nightlight::new();
    nightlight.set_active(false)?;
    sleep(Duration::from_secs(1));
    nightlight.set_active(true)?;
    if !nightlight.read_state()?.is_active() {
        return Err("Windows did not save the new Night light state.".into());
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        let text: Vec<u16> = format!("Failed to refresh Night light.\n\n{error}")
            .encode_utf16()
            .chain(Some(0))
            .collect();
        let title: Vec<u16> = "Night Light Toggle".encode_utf16().chain(Some(0)).collect();
        // Both UTF-16 buffers remain alive throughout the modal call.
        unsafe {
            MessageBoxW(
                null_mut(),
                text.as_ptr(),
                title.as_ptr(),
                MB_OK | MB_ICONERROR,
            );
        }
        std::process::exit(1);
    }
}
