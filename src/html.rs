use crate::scanner::FolderNode;
use serde::Serialize;
use std::path::Path;

const TEMPLATE: &str = include_str!("../template.html");

#[derive(Serialize)]
struct CatalogPayload<'a> {
    scan_path: String,
    tree: &'a FolderNode,
}

/// Renders the final self-contained HTML catalog: the tree data embedded
/// as JSON inside the template, ready to write straight to disk.
pub fn render(scan_path: &Path, tree: &FolderNode) -> String {
    let payload = CatalogPayload {
        scan_path: scan_path.display().to_string(),
        tree,
    };
    let json = serde_json::to_string(&payload).expect("serialize catalog payload to JSON");
    TEMPLATE.replace("__ARCHIVE_DRIVE_DATA__", &json)
}
