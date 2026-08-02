mod html;
mod scanner;

use clap::Parser;
use std::path::PathBuf;
use std::time::Instant;

/// Fast, searchable, offline HTML catalogs for cold storage drives.
///
/// Point it at a folder or drive — by default it writes a self-contained
/// HTML catalog named after that folder, in the current directory.
#[derive(Parser)]
#[command(name = "archive-drive", version, about, arg_required_else_help = true)]
struct Args {
    /// Directory or drive to scan
    path: PathBuf,

    /// Directory to write the HTML catalog into
    #[arg(long, default_value = ".")]
    output_dir: PathBuf,

    /// Filename for the HTML catalog (default: <folder-name>.html)
    #[arg(long)]
    output_name: Option<String>,

    /// Also print the recursive folder tree in the terminal (verbose, can be long)
    #[arg(long)]
    show_tree: bool,

    /// Depth for --show-tree
    #[arg(long, default_value_t = 2)]
    tree_depth: usize,
}

/// Derives the default output filename from the scanned folder's own name,
/// e.g. `/mnt/data` -> `data.html`. Falls back to `catalog.html` for edge
/// cases like scanning `/` itself, where there's no folder name to use.
fn default_output_name(path: &std::path::Path) -> String {
    let base = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "catalog".to_string());
    format!("{base}.html")
}

fn main() {
    let args = Args::parse();
    let start = Instant::now();

    let result = scanner::scan(&args.path);
    let root = result.root;
    let scan_elapsed = start.elapsed();

    println!("Scanned: {}", args.path.display());
    println!("Files found: {}", result.file_count);
    println!("Total recursive size: {} bytes", root.recursive_size);
    println!("Top-level type breakdown:");
    let mut types: Vec<_> = root.type_breakdown.iter().collect();
    types.sort_by(|a, b| b.1 .1.cmp(&a.1 .1));
    for (ext, (count, size)) in types.iter().take(10) {
        println!("  .{:<10} count={:<8} size={} bytes", ext, count, size);
    }
    println!("Scan elapsed: {:?}", scan_elapsed);

    if args.show_tree {
        println!("\nFolder tree (depth {}):", args.tree_depth);
        scanner::print_tree(&root, ".", 0, args.tree_depth);
    }

    let output_name = args
        .output_name
        .unwrap_or_else(|| default_output_name(&args.path));
    let out_path = args.output_dir.join(output_name);

    let html_start = Instant::now();
    let final_html = html::render(&args.path, &root);
    std::fs::write(&out_path, final_html).expect("write HTML output");
    println!(
        "\nHTML catalog written to {} in {:?}",
        out_path.display(),
        html_start.elapsed()
    );
}
