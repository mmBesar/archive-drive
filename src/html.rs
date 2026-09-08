use crate::scanner::FolderNode;
use serde::Serialize;
use std::path::Path;

#[derive(Serialize)]
struct CatalogPayload<'a> {
    scan_path: String,
    tree: &'a FolderNode,
}

pub fn render(scan_path: &Path, tree: &FolderNode) -> String {
    let payload = CatalogPayload {
        scan_path: scan_path.display().to_string(),
        tree,
    };
    let json = serde_json::to_string(&payload).expect("serialize payload to JSON");
    let template = include_str!("../template.html");
    template.replace("__ARCHIVE_DRIVE_DATA__", &json)
}
