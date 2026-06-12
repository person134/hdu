use std::path::Path;

use hdu::backend;
use hdu::config::Config;
use hdu::scanner;

fn main() {
    let platform = backend::detect_platform();
    println!("Platform: {}", platform);

    match scanner::scan(Path::new(".")) {
        Ok(root) => {
            println!("Directory: {}", root.name);
            println!("Total size: {} bytes ({})", root.size, format_bytes(root.size));
            println!("Items: {}", root.item_count);

            println!("\nTop 10 entries by size:");
            let mut entries = root.children.clone();
            scanner::sort_entries(&mut entries, scanner::SortField::Size, scanner::SortOrder::Desc);
            for e in entries.iter().take(10) {
                let pct = if root.size > 0 {
                    (e.size as f64 / root.size as f64) * 100.0
                } else {
                    0.0
                };
                println!(
                    "  {:>10} {:>5.1}%  {}{}",
                    format_bytes(e.size),
                    pct,
                    e.name,
                    if e.is_dir { "/" } else { "" }
                );
            }
        }
        Err(e) => println!("Error scanning: {}", e),
    }

    let config = Config::load();
    println!(
        "\nConfig: refresh={}ms sort={} theme={}",
        config.settings.refresh_rate, config.settings.sort_by, config.settings.theme
    );
}

fn format_bytes(bytes: u64) -> String {
    if bytes >= 1024 * 1024 * 1024 {
        format!("{:.2} GiB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    } else if bytes >= 1024 * 1024 {
        format!("{:.2} MiB", bytes as f64 / (1024.0 * 1024.0))
    } else if bytes >= 1024 {
        format!("{:.2} KiB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}
