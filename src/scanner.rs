use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Entry {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
    pub children: Vec<Entry>,
    pub item_count: u64,
    pub modified: Option<SystemTime>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortField {
    Size,
    Name,
    Count,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Asc,
    Desc,
}

pub fn scan(path: &Path) -> std::io::Result<Entry> {
    let metadata = fs::symlink_metadata(path)?;
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string_lossy().to_string());

    let mut entry = Entry {
        path: path.to_path_buf(),
        name,
        size: 0,
        is_dir: metadata.is_dir(),
        children: Vec::new(),
        item_count: 0,
        modified: metadata.modified().ok(),
    };

    if metadata.is_dir() {
        let mut total_size = 0u64;
        let mut total_items = 0u64;

        if let Ok(rd) = fs::read_dir(path) {
            for child in rd {
                if let Ok(child) = child {
                    let child_path = child.path();
                    if child_path.is_symlink() {
                        if let Ok(m) = fs::symlink_metadata(&child_path) {
                            let size = m.len();
                            let name = child.file_name().to_string_lossy().to_string();
                            total_size += size;
                            total_items += 1;
                            entry.children.push(Entry {
                                path: child_path,
                                name,
                                size,
                                is_dir: false,
                                children: Vec::new(),
                                item_count: 1,
                                modified: m.modified().ok(),
                            });
                        }
                        continue;
                    }
                    if let Ok(child_entry) = scan(&child_path) {
                        total_size += child_entry.size;
                        total_items += child_entry.item_count;
                        entry.children.push(child_entry);
                    }
                }
            }
        }

        entry.size = total_size;
        entry.item_count = total_items;
        entry.children.sort_by(|a, b| b.size.cmp(&a.size));
    } else {
        entry.size = metadata.len();
        entry.item_count = 1;
    }

    Ok(entry)
}

pub fn sort_entries(entries: &mut [Entry], field: SortField, order: SortOrder) {
    entries.sort_by(|a, b| {
        let cmp = match field {
            SortField::Size => a.size.cmp(&b.size),
            SortField::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            SortField::Count => a.item_count.cmp(&b.item_count),
        };
        match order {
            SortOrder::Desc => cmp.reverse(),
            SortOrder::Asc => cmp,
        }
    });
}

pub fn filter_entries<'a>(entries: &'a [Entry], query: &str) -> Vec<&'a Entry> {
    if query.is_empty() {
        return entries.iter().collect();
    }
    let lower = query.to_lowercase();
    entries
        .iter()
        .filter(|e| e.name.to_lowercase().contains(&lower))
        .collect()
}

pub fn find_child<'a>(entry: &'a Entry, name: &str) -> Option<&'a Entry> {
    entry.children.iter().find(|c| c.name == name)
}

pub fn find_child_mut<'a>(entry: &'a mut Entry, name: &str) -> Option<&'a mut Entry> {
    entry.children.iter_mut().find(|c| c.name == name)
}

pub fn find_entry_by_path<'a>(root: &'a Entry, path: &[String]) -> Option<&'a Entry> {
    let mut current = root;
    for component in path {
        current = current.children.iter().find(|c| c.name == *component)?;
    }
    Some(current)
}
