use jwalk::WalkDir;
use serde::Serialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Default, Serialize)]
pub struct FileEntry {
    pub name: String,
    pub size: u64,
}

#[derive(Default, Serialize)]
pub struct FolderNode {
    pub own_size: u64,
    pub recursive_size: u64,
    pub own_file_count: u64,
    pub recursive_file_count: u64,
    /// extension -> (count, total size)
    pub type_breakdown: HashMap<String, (u64, u64)>,
    pub files: Vec<FileEntry>,
    pub children: HashMap<String, FolderNode>,
}

/// Result of a full drive scan.
pub struct ScanResult {
    pub root: FolderNode,
    pub file_count: u64,
}

fn insert(root: &mut FolderNode, rel_path: &Path, size: u64) {
    let file_name = rel_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let ext = rel_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("(none)")
        .to_lowercase();

    let mut components: Vec<_> = rel_path.components().collect();
    components.pop(); // drop the filename, keep only folder components

    let mut node = root;
    for c in components {
        let name = c.as_os_str().to_string_lossy().to_string();
        node = node.children.entry(name).or_insert_with(FolderNode::default);
    }

    node.own_size += size;
    node.own_file_count += 1;
    let entry = node.type_breakdown.entry(ext).or_insert((0, 0));
    entry.0 += 1;
    entry.1 += size;
    node.files.push(FileEntry { name: file_name, size });
}

fn rollup(node: &mut FolderNode) -> (u64, u64) {
    let mut total_size = node.own_size;
    let mut total_files = node.own_file_count;
    for child in node.children.values_mut() {
        let (s, f) = rollup(child);
        total_size += s;
        total_files += f;
    }
    node.recursive_size = total_size;
    node.recursive_file_count = total_files;
    (total_size, total_files)
}

/// Walks `scan_path` in parallel (via jwalk) and builds the recursive
/// folder tree with rolled-up sizes and per-folder type breakdowns.
pub fn scan(scan_path: &Path) -> ScanResult {
    let mut root = FolderNode::default();
    let mut file_count = 0u64;

    for entry in WalkDir::new(scan_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            let full_path = entry.path();
            let rel: PathBuf = full_path
                .strip_prefix(scan_path)
                .unwrap_or(&full_path)
                .to_path_buf();
            insert(&mut root, &rel, size);
            file_count += 1;
        }
    }

    rollup(&mut root);
    ScanResult { root, file_count }
}

pub fn print_tree(node: &FolderNode, name: &str, indent: usize, depth_left: usize) {
    let pad = "  ".repeat(indent);
    println!(
        "{}{}/  ({} files, {} bytes)",
        pad, name, node.recursive_file_count, node.recursive_size
    );
    if depth_left == 0 {
        return;
    }
    let mut children: Vec<_> = node.children.iter().collect();
    children.sort_by(|a, b| b.1.recursive_size.cmp(&a.1.recursive_size));
    for (child_name, child_node) in children {
        print_tree(child_node, child_name, indent + 1, depth_left - 1);
    }
}
