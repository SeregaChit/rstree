pub mod cli;
pub mod display;
pub mod style;
pub mod walk;

use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::Instant;

use clap::Parser;
use colored::Colorize;

use cli::Cli;
use display::{format_duration, frame};
use style::get_style;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};
use walk::{load_gitignore, walk};

pub fn run() -> io::Result<()> {
    let cli = Cli::parse();

    if cli.show_version {
        println!("{}", frame(&[&format!("\u{e77e} rstree {}", env!("CARGO_PKG_VERSION"))]));
        return Ok(());
    }

    if cli.show_help {
        let col = |s: &str| s.cyan().to_string();
        println!("{}", frame(&[
            &format!("{} rstree {} Directory tree visualizer", col("\u{f07c}"), "\u{f129}".white()),
            "",
            &format!("{} Usage: rstree [OPTIONS] [PATH]", col("\u{f0f6}")),
            "",
            &format!("{} Arguments:", col("\u{f013}")),
            &format!("  {} {}  Path to display [default: .]", col("\u{f07c}"), col("[PATH]")),
            "",
            &format!("{} Options:", col("\u{f013}")),
            &format!("  {} {}    Show files", col("\u{f016}"), col("-f, --files")),
            &format!("  {} {}      Show hidden files", col("\u{f06e}"), col("-a, --all")),
            &format!("  {} {}      Reverse sort order", col("\u{f0ec}"), col("-r, --rev")),
            &format!("  {} {}     Long format (perms, size, date)", col("\u{f022}"), col("-l, --long")),
            &format!("  {} {}   Max display depth", col("\u{f07c}"), col("-L, --max-depth <N>")),
            &format!("  {} {}  Print version", col("\u{e77e}"), col("-v, --version")),
            &format!("  {} {}     Print help", col("\u{f059}"), col("-h, --help")),
        ]));
        return Ok(());
    }

    let start = Instant::now();
    let root = PathBuf::from(&cli.path);

    if !root.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("path '{}' does not exist", cli.path)));
    }

    let root_gi = load_gitignore(&root);
    let mut gi_stack = vec![(root.clone(), root_gi)];
    let mut visited: HashSet<PathBuf> = HashSet::new();

    const MAX_PATH_DISPLAY: usize = 60;

    let root_str = if cli.path == "." {
        "./".to_string()
    } else {
        root.display().to_string()
    };
    let root_str = if UnicodeWidthStr::width(root_str.as_str()) > MAX_PATH_DISPLAY {
        let dots = "...";
        let keep = MAX_PATH_DISPLAY - dots.len();
        let mut suffix = String::new();
        for c in root_str.chars().rev() {
            let w = UnicodeWidthChar::width(c).unwrap_or(0);
            if UnicodeWidthStr::width(suffix.as_str()) + w > keep {
                break;
            }
            suffix.insert(0, c);
        }
        format!("{}{}", dots, suffix)
    } else {
        root_str
    };
    println!("{}", frame(&[&format!("\u{f07c} {}", root_str)]));

    let root_meta = fs::metadata(&root)?;
    let root_is_symlink = root.is_symlink();
    let (file_count, dir_count) = if root_is_symlink || root_meta.is_file() {
        let name = root.file_name().unwrap_or_default().to_string_lossy().to_string();
        let style = get_style(&name, false, root_is_symlink);

        let long_part = if cli.long {
            let perms = display::format_permissions(&root_meta);
            let size = display::format_size(root_meta.len());
            let time = root_meta.modified()
                .map(|t| display::format_time(t))
                .unwrap_or_else(|_| "         ???".to_string());
            format!(" {} {} {} ", perms, size, time)
        } else {
            String::new()
        };

        display::print_entry("", "", &style, &long_part, &name);
        (1, 0)
    } else {
        let mut prefix = String::new();
        walk(&root, &mut prefix, &cli, &mut gi_stack, &mut visited, 0)?
    };

    let elapsed = format_duration(start.elapsed());
    println!("\n{}", frame(&[
        &format!("\u{f07c} {} directories, \u{f016} {} files", dir_count, file_count),
        &format!("\u{f017} time {}", elapsed),
    ]));
    Ok(())
}
