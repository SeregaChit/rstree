use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use ignore::gitignore::{Gitignore, GitignoreBuilder};
use ignore::Match;

use crate::cli::Cli;
use crate::display::{format_permissions, format_size, format_time, print_entry};
use crate::style::get_style;

#[must_use]
pub fn load_gitignore(root: &Path) -> Gitignore {
    let mut builder = GitignoreBuilder::new(root);
    let gi_path = root.join(".gitignore");
    if gi_path.exists() {
        builder.add(gi_path);
    }
    builder.build().unwrap_or(Gitignore::empty())
}

#[must_use]
pub fn is_hidden(name: &str, meta: Option<&std::fs::Metadata>) -> bool {
    if name.starts_with('.') {
        return true;
    }
    #[cfg(windows)]
    if let Some(m) = meta {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;
        return m.file_attributes() & FILE_ATTRIBUTE_HIDDEN != 0;
    }
    #[cfg(not(windows))]
    let _ = meta;
    false
}

#[must_use]
pub fn is_ignored_by_gitignore(
    name: &str,
    is_dir: bool,
    parent: &Path,
    gitignore_stack: &[(PathBuf, Gitignore)],
) -> bool {
    let full_path = parent.join(name);
    for (gi_root, gi) in gitignore_stack.iter().rev() {
        if let Ok(relative) = full_path.strip_prefix(gi_root) {
            match gi.matched(relative, is_dir) {
                Match::Ignore(_) => return true,
                Match::Whitelist(_) => return false,
                Match::None => {}
            }
        }
    }
    false
}

const MAX_WALK_DEPTH: usize = 512;

pub fn walk(
    dir: &Path,
    prefix: &mut String,
    cli: &Cli,
    gitignore_stack: &mut Vec<(PathBuf, Gitignore)>,
    visited: &mut HashSet<PathBuf>,
    depth: usize,
) -> io::Result<(usize, usize)> {
    if depth > MAX_WALK_DEPTH {
        eprintln!("{}    [max depth exceeded, skipping]", prefix);
        return Ok((0, 0));
    }

    let entries: Vec<_> = match fs::read_dir(dir) {
        Ok(rd) => rd.filter_map(|e| e.ok()).collect(),
        Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
            eprintln!("{}    [permission denied]", prefix);
            return Ok((0, 0));
        }
        Err(e) => return Err(e),
    };

    let mut visible: Vec<_> = entries
        .iter()
        .filter(|e| {
            let fname = e.file_name();
            let name = fname.to_string_lossy();
            if is_ignored_by_gitignore(&name, e.file_type().map_or(false, |t| t.is_dir()), dir, gitignore_stack) {
                return false;
            }
            let meta = e.metadata().ok();
            if !cli.show_hidden && is_hidden(&name, meta.as_ref()) {
                return false;
            }
            if !cli.show_files && !e.file_type().map_or(false, |t| t.is_dir()) {
                return false;
            }
            true
        })
        .collect();

    visible.sort_by(|a, b| {
        let a_name = a.file_name().to_string_lossy().to_ascii_lowercase();
        let b_name = b.file_name().to_string_lossy().to_ascii_lowercase();
        a_name.cmp(&b_name)
    });
    if cli.reverse {
        visible.reverse();
    }

    let count = visible.len();
    let mut file_count = 0;
    let mut dir_count = 0;

    for (i, entry) in visible.iter().enumerate() {
        let is_last = i == count - 1;
        let connector = if is_last { "\u{2570}\u{2500}\u{2500} " } else { "\u{251C}\u{2500}\u{2500} " };
        let child_prefix = if is_last { "    " } else { "\u{2502}   " };

        let ft = entry.file_type()?;
        let is_symlink = ft.is_symlink();
        let (is_dir, meta) = match entry.metadata() {
            Ok(m) => (m.is_dir(), Some(m)),
            Err(_) => (ft.is_dir(), None),
        };

        let name = entry.file_name().to_string_lossy().to_string();
        let style = get_style(&name, is_dir, is_symlink);

        let long_part = if cli.long {
            if let Some(ref m) = meta {
                let perms = format_permissions(m);
                let size = format_size(m.len());
                let time = m.modified()
                    .map(|t| format_time(t))
                    .unwrap_or_else(|_| "         ???".to_string());
                format!(" {} {} {} ", perms, size, time)
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        print_entry(prefix, connector, &style, &long_part, &name);

        if !is_dir {
            file_count += 1;
            continue;
        }

        if cli.max_depth.is_some_and(|m| depth + 1 > m) {
            dir_count += 1;
            continue;
        }

        if is_symlink {
            if let Ok(canon) = fs::canonicalize(entry.path()) {
                if !visited.insert(canon) {
                    eprintln!("{}    (symlink loop detected, skipping)", prefix);
                    continue;
                }
            }
        }

        let child_gi_path = entry.path().join(".gitignore");
        let has_gi = child_gi_path.exists();
        if has_gi {
            let mut builder = GitignoreBuilder::new(entry.path());
            builder.add(child_gi_path);
            let child_gi = builder.build().unwrap_or(Gitignore::empty());
            gitignore_stack.push((entry.path().to_path_buf(), child_gi));
        }

        dir_count += 1;
        let len_before = prefix.len();
        prefix.push_str(child_prefix);
        let (fc, dc) = walk(&entry.path(), prefix, cli, gitignore_stack, visited, depth + 1)?;
        prefix.truncate(len_before);
        file_count += fc;
        dir_count += dc;

        if has_gi {
            gitignore_stack.pop();
        }
    }

    Ok((file_count, dir_count))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::Cli;
    use tempfile::TempDir;

    fn create_test_tree(base: &Path) -> io::Result<()> {
        fs::create_dir_all(base.join("sub"))?;
        fs::write(base.join("a.txt"), b"")?;
        fs::write(base.join("b.rs"), b"")?;
        fs::write(base.join("sub").join("c.py"), b"")?;
        Ok(())
    }

    fn make_stack(gi: Gitignore) -> Vec<(PathBuf, Gitignore)> {
        vec![(PathBuf::new(), gi)]
    }

    #[test]
    fn test_is_hidden_dot() {
        assert!(is_hidden(".git", None));
        assert!(is_hidden(".hidden", None));
        assert!(is_hidden(".", None));
    }

    #[test]
    fn test_is_hidden_normal() {
        assert!(!is_hidden("src", None));
        assert!(!is_hidden("main.rs", None));
        assert!(!is_hidden("Cargo.toml", None));
    }

    #[test]
    fn test_walk_empty_dir() -> io::Result<()> {
        let dir = TempDir::new()?;
        let cli = Cli { max_depth: None, show_files: false, show_hidden: false, reverse: false, long: false, path: ".".into(), show_version: false, show_help: false };
        let mut gi_stack = make_stack(Gitignore::empty());
        let mut visited = HashSet::new();
        let mut prefix = String::new();
        let (files, dirs) = walk(dir.path(), &mut prefix, &cli, &mut gi_stack, &mut visited, 0)?;
        assert_eq!(files, 0);
        assert_eq!(dirs, 0);
        Ok(())
    }

    #[test]
    fn test_walk_dirs_only() -> io::Result<()> {
        let dir = TempDir::new()?;
        create_test_tree(dir.path())?;

        let cli = Cli { max_depth: None, show_files: false, show_hidden: false, reverse: false, long: false, path: ".".into(), show_version: false, show_help: false };
        let mut gi_stack = make_stack(Gitignore::empty());
        let mut visited = HashSet::new();
        let mut prefix = String::new();
        let (files, dirs) = walk(dir.path(), &mut prefix, &cli, &mut gi_stack, &mut visited, 0)?;
        assert_eq!(files, 0);
        assert_eq!(dirs, 1);
        Ok(())
    }

    #[test]
    fn test_walk_show_files() -> io::Result<()> {
        let dir = TempDir::new()?;
        create_test_tree(dir.path())?;

        let cli = Cli { max_depth: None, show_files: true, show_hidden: false, reverse: false, long: false, path: ".".into(), show_version: false, show_help: false };
        let mut gi_stack = make_stack(Gitignore::empty());
        let mut visited = HashSet::new();
        let mut prefix = String::new();
        let (files, dirs) = walk(dir.path(), &mut prefix, &cli, &mut gi_stack, &mut visited, 0)?;
        assert_eq!(files, 3);
        assert_eq!(dirs, 1);
        Ok(())
    }

    #[test]
    fn test_walk_hidden_excluded() -> io::Result<()> {
        let dir = TempDir::new()?;
        create_test_tree(dir.path())?;
        fs::write(dir.path().join(".hidden_file"), b"")?;
        fs::create_dir_all(dir.path().join(".hidden_dir"))?;

        let cli = Cli { max_depth: None, show_files: true, show_hidden: false, reverse: false, long: false, path: ".".into(), show_version: false, show_help: false };
        let mut gi_stack = make_stack(Gitignore::empty());
        let mut visited = HashSet::new();
        let mut prefix = String::new();
        let (files, dirs) = walk(dir.path(), &mut prefix, &cli, &mut gi_stack, &mut visited, 0)?;
        assert_eq!(files, 3);
        assert_eq!(dirs, 1);
        Ok(())
    }

    #[test]
    fn test_walk_hidden_included() -> io::Result<()> {
        let dir = TempDir::new()?;
        create_test_tree(dir.path())?;
        fs::write(dir.path().join(".hidden_file"), b"")?;
        fs::create_dir_all(dir.path().join(".hidden_dir"))?;

        let cli = Cli { max_depth: None, show_files: true, show_hidden: true, reverse: false, long: false, path: ".".into(), show_version: false, show_help: false };
        let mut gi_stack = make_stack(Gitignore::empty());
        let mut visited = HashSet::new();
        let mut prefix = String::new();
        let (files, dirs) = walk(dir.path(), &mut prefix, &cli, &mut gi_stack, &mut visited, 0)?;
        assert_eq!(files, 4);
        assert_eq!(dirs, 2);
        Ok(())
    }

    #[test]
    fn test_walk_gitignore_excludes() -> io::Result<()> {
        let dir = TempDir::new()?;
        fs::write(dir.path().join("keep.txt"), b"")?;
        fs::write(dir.path().join("Cargo.lock"), b"")?;
        fs::create_dir_all(dir.path().join("target"))?;
        fs::write(dir.path().join(".gitignore"), b"Cargo.lock\ntarget/\n")?;

        let cli = Cli { max_depth: None, show_files: true, show_hidden: false, reverse: false, long: false, path: ".".into(), show_version: false, show_help: false };
        let mut gi_stack = make_stack(load_gitignore(dir.path()));
        let mut visited = HashSet::new();
        let mut prefix = String::new();
        let (files, dirs) = walk(dir.path(), &mut prefix, &cli, &mut gi_stack, &mut visited, 0)?;
        assert_eq!(files, 1);
        assert_eq!(dirs, 0);
        Ok(())
    }

    #[test]
    fn test_walk_gitignore_with_hidden() -> io::Result<()> {
        let dir = TempDir::new()?;
        fs::write(dir.path().join("keep.txt"), b"")?;
        fs::write(dir.path().join("Cargo.lock"), b"")?;
        fs::write(dir.path().join(".gitignore"), b"Cargo.lock\n")?;

        let cli = Cli { max_depth: None, show_files: true, show_hidden: true, reverse: false, long: false, path: ".".into(), show_version: false, show_help: false };
        let mut gi_stack = make_stack(load_gitignore(dir.path()));
        let mut visited = HashSet::new();
        let mut prefix = String::new();
        let (files, dirs) = walk(dir.path(), &mut prefix, &cli, &mut gi_stack, &mut visited, 0)?;
        assert_eq!(files, 2);
        assert_eq!(dirs, 0);
        Ok(())
    }

    #[test]
    fn test_walk_reverse() -> io::Result<()> {
        let dir = TempDir::new()?;
        create_test_tree(dir.path())?;

        let cli = Cli { max_depth: None, show_files: true, show_hidden: false, reverse: true, long: false, path: ".".into(), show_version: false, show_help: false };
        let mut gi_stack = make_stack(Gitignore::empty());
        let mut visited = HashSet::new();
        let mut prefix = String::new();
        let (files, dirs) = walk(dir.path(), &mut prefix, &cli, &mut gi_stack, &mut visited, 0)?;
        assert_eq!(files, 3);
        assert_eq!(dirs, 1);
        Ok(())
    }

    #[test]
    fn test_walk_multiple_dirs() -> io::Result<()> {
        let dir = TempDir::new()?;
        fs::create_dir_all(dir.path().join("a"))?;
        fs::create_dir_all(dir.path().join("b"))?;
        fs::create_dir_all(dir.path().join("c"))?;

        let cli = Cli { max_depth: None, show_files: false, show_hidden: false, reverse: false, long: false, path: ".".into(), show_version: false, show_help: false };
        let mut gi_stack = make_stack(Gitignore::empty());
        let mut visited = HashSet::new();
        let mut prefix = String::new();
        let (files, dirs) = walk(dir.path(), &mut prefix, &cli, &mut gi_stack, &mut visited, 0)?;
        assert_eq!(files, 0);
        assert_eq!(dirs, 3);
        Ok(())
    }

    #[test]
    fn test_walk_nested() -> io::Result<()> {
        let dir = TempDir::new()?;
        fs::create_dir_all(dir.path().join("a").join("b").join("c"))?;
        fs::write(dir.path().join("a").join("b").join("f.txt"), b"")?;

        let cli = Cli { max_depth: None, show_files: true, show_hidden: false, reverse: false, long: false, path: ".".into(), show_version: false, show_help: false };
        let mut gi_stack = make_stack(Gitignore::empty());
        let mut visited = HashSet::new();
        let mut prefix = String::new();
        let (files, dirs) = walk(dir.path(), &mut prefix, &cli, &mut gi_stack, &mut visited, 0)?;
        assert_eq!(files, 1);
        assert_eq!(dirs, 3);
        Ok(())
    }

    #[test]
    fn test_walk_long_format() -> io::Result<()> {
        let dir = TempDir::new()?;
        create_test_tree(dir.path())?;

        let cli = Cli { max_depth: None, show_files: true, show_hidden: false, reverse: false, long: true, path: ".".into(), show_version: false, show_help: false };
        let mut gi_stack = make_stack(Gitignore::empty());
        let mut visited = HashSet::new();
        let mut prefix = String::new();
        let (files, dirs) = walk(dir.path(), &mut prefix, &cli, &mut gi_stack, &mut visited, 0)?;
        assert_eq!(files, 3);
        assert_eq!(dirs, 1);
        Ok(())
    }

    #[test]
    fn test_walk_gitignore_negate() -> io::Result<()> {
        let dir = TempDir::new()?;
        fs::write(dir.path().join("keep.txt"), b"")?;
        fs::write(dir.path().join("ignore.me"), b"")?;
        fs::write(dir.path().join("but_not_me.keep"), b"")?;
        fs::write(dir.path().join(".gitignore"), b"*.me\n!but_*.keep\n")?;

        let cli = Cli { max_depth: None, show_files: true, show_hidden: false, reverse: false, long: false, path: ".".into(), show_version: false, show_help: false };
        let mut gi_stack = make_stack(load_gitignore(dir.path()));
        let mut visited = HashSet::new();
        let mut prefix = String::new();
        let (files, _dirs) = walk(dir.path(), &mut prefix, &cli, &mut gi_stack, &mut visited, 0)?;
        assert_eq!(files, 2);
        Ok(())
    }

    #[test]
    fn test_walk_single_file() -> io::Result<()> {
        let dir = TempDir::new()?;
        let f = dir.path().join("single.txt");
        fs::write(&f, b"hello")?;

        let cli = Cli { max_depth: None, show_files: true, show_hidden: false, reverse: false, long: false, path: ".".into(), show_version: false, show_help: false };
        let mut gi_stack = make_stack(load_gitignore(dir.path()));
        let mut visited = HashSet::new();
        let mut prefix = String::new();
        let (files, dirs) = walk(dir.path(), &mut prefix, &cli, &mut gi_stack, &mut visited, 0)?;
        assert_eq!(files, 1);
        assert_eq!(dirs, 0);
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn test_walk_symlink_loop_detected() -> io::Result<()> {
        use std::os::unix::fs::symlink;
        let dir = TempDir::new()?;
        fs::create_dir_all(dir.path().join("child"))?;
        symlink(dir.path(), dir.path().join("child").join("back"))?;

        let cli = Cli { max_depth: None, show_files: true, show_hidden: false, reverse: false, long: false, path: ".".into(), show_version: false, show_help: false };
        let mut gi_stack = make_stack(Gitignore::empty());
        let mut visited = HashSet::new();
        let mut prefix = String::new();
        let (files, dirs) = walk(dir.path(), &mut prefix, &cli, &mut gi_stack, &mut visited, 0)?;
        assert_eq!(files, 0);
        assert_eq!(dirs, 2);
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn test_walk_symlink_dir() -> io::Result<()> {
        use std::os::unix::fs::symlink;
        let dir = TempDir::new()?;
        fs::create_dir_all(dir.path().join("real"))?;
        fs::write(dir.path().join("real").join("f.txt"), b"")?;
        symlink(dir.path().join("real"), dir.path().join("link"))?;

        let cli = Cli { max_depth: None, show_files: true, show_hidden: false, reverse: false, long: false, path: ".".into(), show_version: false, show_help: false };
        let mut gi_stack = make_stack(Gitignore::empty());
        let mut visited = HashSet::new();
        let mut prefix = String::new();
        let (files, dirs) = walk(dir.path(), &mut prefix, &cli, &mut gi_stack, &mut visited, 0)?;
        assert_eq!(files, 1);
        assert_eq!(dirs, 2);
        Ok(())
    }

    #[test]
    fn test_load_gitignore_nonexistent() {
        let dir = TempDir::new().unwrap();
        let gi = load_gitignore(dir.path());
        assert!(matches!(gi.matched("foo", false), Match::None));
    }

    #[test]
    fn test_load_gitignore_with_file() -> io::Result<()> {
        let dir = TempDir::new()?;
        fs::write(dir.path().join(".gitignore"), b"*.log\n")?;
        let gi = load_gitignore(dir.path());
        assert!(matches!(gi.matched("test.log", false), Match::Ignore(_)));
        assert!(matches!(gi.matched("test.rs", false), Match::None));
        Ok(())
    }
}
