#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod apple;

#[cfg(target_os = "windows")]
mod windows;

pub fn get_locale() -> Vec<String> {
    #[cfg(target_os = "linux")]
    return linux::get_locale();

    #[cfg(target_os = "macos")]
    return apple::get_locale();

    #[cfg(target_os = "windows")]
    return windows::get_locale();
}

pub fn get_system_calendar() -> Vec<(String, String)> {
    #[cfg(target_os = "linux")]
    return linux::get_system_calendar();

    #[cfg(target_os = "macos")]
    return apple::get_system_calendar();

    #[cfg(target_os = "windows")]
    return windows::get_system_calendar();
}
