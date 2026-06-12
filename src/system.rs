#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DiskUsage {
    pub name: String,
    pub mount_point: String,
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub used_bytes: u64,
}

pub fn get_disk_usage() -> Vec<DiskUsage> {
    let mut disks = Vec::new();
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/proc/mounts") {
            let mut seen = std::collections::HashSet::new();
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 2 {
                    continue;
                }
                let dev = parts[0];
                let mount = parts[1];
                if dev.starts_with("/dev/") && seen.insert(mount.to_string()) {
                    if let Ok(usage) = statvfs(mount) {
                        disks.push(usage);
                    }
                }
            }
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        if let Ok(disks_info) = std::fs::read_dir("/") {
            // fallback: just show root
            if let Ok(usage) = statvfs("/") {
                disks.push(usage);
            }
        }
    }
    disks
}

#[cfg(target_os = "linux")]
fn statvfs(path: &str) -> Result<DiskUsage, std::io::Error> {
    use std::mem;
    use std::ffi::CString;

    let cpath = CString::new(path).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid path"))?;
    let mut stat: libc::statvfs = unsafe { mem::zeroed() };
    let ret = unsafe { libc::statvfs(cpath.as_ptr(), &mut stat) };
    if ret != 0 {
        return Err(std::io::Error::last_os_error());
    }

    let total = stat.f_blocks * stat.f_frsize;
    let available = stat.f_bavail * stat.f_frsize;
    let used = total - available;

    Ok(DiskUsage {
        name: path.to_string(),
        mount_point: path.to_string(),
        total_bytes: total,
        available_bytes: available,
        used_bytes: used,
    })
}

#[cfg(not(target_os = "linux"))]
fn statvfs(path: &str) -> Result<DiskUsage, std::io::Error> {
    let usage = std::fs::metadata(path)?;
    Ok(DiskUsage {
        name: path.to_string(),
        mount_point: path.to_string(),
        total_bytes: 0,
        available_bytes: 0,
        used_bytes: 0,
    })
}
