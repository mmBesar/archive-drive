mod html;
mod scanner;

use clap::Parser;
use std::path::PathBuf;
use std::time::Instant;

/// Scan a folder/drive and generate a fast, searchable, offline HTML catalog.
#[derive(Parser)]
#[command(name = "archive-drive", version, about)]
struct Args {
    /// Directory or drive to scan
    path: PathBuf,

    /// Write the searchable HTML catalog to this file
    #[arg(long)]
    html: Option<PathBuf>,

    /// Also print the recursive folder tree in the terminal (verbose, can be long)
    #[arg(long)]
    show_tree: bool,

    /// Depth for --show-tree
    #[arg(long, default_value_t = 2)]
    tree_depth: usize,
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

    if let Some(out_path) = &args.html {
        let html_start = Instant::now();
        let final_html = html::render(&args.path, &root);
        std::fs::write(out_path, final_html).expect("write HTML output");
        println!(
            "\nHTML catalog written to {} in {:?}",
            out_path.display(),
            html_start.elapsed()
        );
    }
}
