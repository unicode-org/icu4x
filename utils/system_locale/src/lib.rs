#[cfg(target_os = "linux")]
mod linux;

pub fn get_locale() -> Vec<String> {
    #[cfg(target_os = "linux")]
    return linux::get_loclae();
}

pub fn get_system_calendar() -> Vec<(String, String)> {
    #[cfg(target_os = "linux")]
    return linux::get_system_calendar();
}
