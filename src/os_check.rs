use std::env;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref OS_INFO: Mutex<String> = Mutex::new(check_operating_system());
}

pub fn get_os_info() -> String {
    // Lock the mutex to access the OS_INFO safely
    let os_info = OS_INFO.lock().unwrap();
    os_info.clone() // Return a clone of the OS info
}

pub fn check_operating_system()  -> String {
    let os = env::consts::OS;
    match os {
        "macos" => "macos".to_string(),
        "linux" => "linux".to_string(),
        "windows" => {
            if is_wsl() {
                "windows-wsl2".to_string()
            } else {
                "windows".to_string()
            }
        },
        _ => format!("Unknown operating system: {}", os),
    }
}

// Function to check if the OS is WSL
pub fn is_wsl() -> bool {
    // Check for the presence of the WSL environment variable
    std::path::Path::new("/proc/version").exists() && 
    std::fs::read_to_string("/proc/version").unwrap_or_default().contains("Microsoft")
}