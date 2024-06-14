#[cfg(target_os = "linux")]
mod linux;

pub fn get_locale() -> Vec<String> {
    #[cfg(target_os = "linux")]
    return linux::get_loclae();
}
