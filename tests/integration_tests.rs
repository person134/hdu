use std::path::Path;

use hdu::backend;
use hdu::config::Config;
use hdu::scanner;

#[test]
fn test_detect_platform() {
    let platform = backend::detect_platform();
    #[cfg(target_os = "linux")]
    assert_eq!(platform, backend::Platform::Linux);
    #[cfg(target_os = "windows")]
    assert_eq!(platform, backend::Platform::Windows);
    #[cfg(target_os = "macos")]
    assert_eq!(platform, backend::Platform::MacOS);
}

#[test]
fn test_scan_current_dir() {
    let root = scanner::scan(Path::new(".")).expect("Should scan current dir");
    assert!(root.is_dir, "Current dir should be a directory");
    assert!(root.size > 0, "Should have some content");
}

#[test]
fn test_sort_entries() {
    use scanner::{Entry, SortField, SortOrder};

    let e1 = Entry {
        path: "/a".into(), name: "a".into(), size: 100,
        is_dir: false, children: vec![], item_count: 1, modified: None,
    };
    let e2 = Entry {
        path: "/b".into(), name: "b".into(), size: 200,
        is_dir: false, children: vec![], item_count: 1, modified: None,
    };
    let mut entries = vec![e1, e2];

    scanner::sort_entries(&mut entries, SortField::Size, SortOrder::Desc);
    assert_eq!(entries[0].size, 200);
    assert_eq!(entries[1].size, 100);

    scanner::sort_entries(&mut entries, SortField::Size, SortOrder::Asc);
    assert_eq!(entries[0].size, 100);
    assert_eq!(entries[1].size, 200);
}

#[test]
fn test_filter_entries() {
    use scanner::Entry;

    let e1 = Entry {
        path: "/documents".into(), name: "Documents".into(), size: 100,
        is_dir: true, children: vec![], item_count: 5, modified: None,
    };
    let e2 = Entry {
        path: "/pictures".into(), name: "Pictures".into(), size: 200,
        is_dir: true, children: vec![], item_count: 10, modified: None,
    };
    let entries = vec![e1, e2];

    let filtered = scanner::filter_entries(&entries, "doc");
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].name, "Documents");

    let filtered = scanner::filter_entries(&entries, "");
    assert_eq!(filtered.len(), 2);
}

#[test]
fn test_config_default() {
    let config = Config::default();
    assert_eq!(config.settings.refresh_rate, 1000);
    assert_eq!(config.settings.sort_by, "size");
    assert_eq!(config.settings.theme, "dark");
}
