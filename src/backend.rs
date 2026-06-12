use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Platform {
    Linux,
    Windows,
    MacOS,
    Unknown,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Platform::Linux => write!(f, "Linux"),
            Platform::Windows => write!(f, "Windows"),
            Platform::MacOS => write!(f, "macOS"),
            Platform::Unknown => write!(f, "Unknown"),
        }
    }
}

pub fn detect_platform() -> Platform {
    #[cfg(target_os = "linux")]
    { Platform::Linux }

    #[cfg(target_os = "windows")]
    { Platform::Windows }

    #[cfg(target_os = "macos")]
    { Platform::MacOS }

    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    { Platform::Unknown }
}
