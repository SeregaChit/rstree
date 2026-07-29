use chrono::{DateTime, Local, Utc};
use colored::Colorize;
use unicode_width::UnicodeWidthStr;

use crate::style::FileStyle;

#[must_use]
pub fn strip_ansi(s: &str) -> String {
    String::from_utf8_lossy(&strip_ansi_escapes::strip(s.as_bytes())).to_string()
}

#[must_use]
pub fn frame(lines: &[&str]) -> String {
    let plain: Vec<String> = lines.iter().map(|l| strip_ansi(l)).collect();
    let width = plain.iter().map(|l| UnicodeWidthStr::width(l.as_str())).max().unwrap_or(0);
    let top = format!("\u{256D}{}\u{256E}", "\u{2500}".repeat(width + 2));
    let mids: Vec<_> = lines
        .iter()
        .zip(plain.iter())
        .map(|(colored, plain)| {
            let pad = width - UnicodeWidthStr::width(plain.as_str());
            format!("\u{2502} {}{} \u{2502}", colored, " ".repeat(pad))
        })
        .collect();
    let bot = format!("\u{2570}{}\u{256F}", "\u{2500}".repeat(width + 2));
    format!("{}\n{}\n{}", top, mids.join("\n"), bot)
}

pub fn print_entry(prefix: &str, connector: &str, style: &FileStyle, long_part: &str, name: &str) {
    let symbol = if style.symbol == '\0' {
        String::new()
    } else {
        format!("{} ", style.symbol.to_string().color(style.color))
    };
    println!("{}{}{}{}{}", prefix, connector, symbol, long_part, name.color(style.color));
}

#[must_use]
pub fn format_permissions(metadata: &std::fs::Metadata) -> String {
    let file_type = if metadata.is_dir() {
        'd'
    } else if metadata.file_type().is_symlink() {
        'l'
    } else {
        '-'
    };

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = metadata.permissions().mode();
        let triplet = |shift: u8| -> String {
            let bits = (mode >> (shift * 3)) & 0o7;
            let r = if bits & 0o4 != 0 { 'r' } else { '-' };
            let w = if bits & 0o2 != 0 { 'w' } else { '-' };
            let x = if bits & 0o1 != 0 { 'x' } else { '-' };
            format!("{}{}{}", r, w, x)
        };
        format!("{}{}{}{}", file_type, triplet(2), triplet(1), triplet(0))
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        let attrs = metadata.file_attributes();
        let r = if attrs & 0x1 != 0 { 'r' } else { '-' };
        let h = if attrs & 0x2 != 0 { 'h' } else { '-' };
        let s = if attrs & 0x4 != 0 { 's' } else { '-' };
        let a = if attrs & 0x20 != 0 { 'a' } else { '-' };
        let c = if attrs & 0x800 != 0 { 'c' } else { '-' };
        let e = if attrs & 0x4000 != 0 { 'e' } else { '-' };
        format!("{}{}{}{}{}{}{}", file_type, r, h, s, a, c, e)
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = metadata;
        format!("{}-------", file_type)
    }
}

#[must_use]
pub fn format_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "K", "M", "G", "T", "P", "E"];
    let mut size = size as f64;
    for unit in UNITS {
        if size < 1024.0 {
            if unit == &"B" {
                return format!("{:>4} {}", size as u64, unit);
            }
            return format!("{:>5.1} {}", size, unit);
        }
        size /= 1024.0;
    }
    format!("{:>5.1} {}", size, "E")
}

#[must_use]
pub fn format_time(st: std::time::SystemTime) -> String {
    let utc: DateTime<Utc> = st.into();
    let local = utc.with_timezone(&Local);
    local.format("%Y-%m-%d %H:%M").to_string()
}

#[must_use]
pub fn format_duration(duration: std::time::Duration) -> String {
    let micros = duration.as_micros();
    if micros < 1_000 {
        format!("{}µs", micros)
    } else if micros < 1_000_000 {
        format!("{:.1}ms", micros as f64 / 1000.0)
    } else if micros < 60_000_000 {
        format!("{:.2}s", micros as f64 / 1_000_000.0)
    } else {
        let total_secs = micros / 1_000_000;
        format!("{}m {}s", total_secs / 60, total_secs % 60)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_frame_simple() {
        let result = frame(&["test"]);
        assert_eq!(result, "\u{256D}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{256E}\n\u{2502} test \u{2502}\n\u{2570}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{256F}");
    }

    #[test]
    fn test_frame_counts() {
        let result = frame(&["10 directories, 42 files"]);
        assert_eq!(result, "\u{256D}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{256E}\n\u{2502} 10 directories, 42 files \u{2502}\n\u{2570}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{256F}");
    }

    #[test]
    fn test_frame_multi_line() {
        let result = frame(&["a", "bb", "ccc"]);
        assert_eq!(result, "\u{256D}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{256E}\n\u{2502} a   \u{2502}\n\u{2502} bb  \u{2502}\n\u{2502} ccc \u{2502}\n\u{2570}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{256F}");
    }

    #[test]
    fn test_frame_with_ansi() {
        let red = "\x1b[31mhello\x1b[0m".to_string();
        let result = frame(&[&red]);
        assert_eq!(result, "\u{256D}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{256E}\n\u{2502} \x1b[31mhello\x1b[0m \u{2502}\n\u{2570}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{256F}");
    }

    #[test]
    fn test_strip_ansi_plain() {
        assert_eq!(strip_ansi("hello"), "hello");
    }

    #[test]
    fn test_strip_ansi_colored() {
        assert_eq!(strip_ansi("\x1b[31mred\x1b[0m"), "red");
    }

    #[test]
    fn test_strip_ansi_multi() {
        assert_eq!(strip_ansi("\x1b[1m\x1b[32mbold green\x1b[0m"), "bold green");
    }

    #[test]
    fn test_strip_ansi_empty() {
        assert_eq!(strip_ansi(""), "");
    }

    #[rstest]
    #[case(0, "0µs")]
    #[case(50, "50µs")]
    #[case(999, "999µs")]
    #[case(1_000, "1.0ms")]
    #[case(1_500, "1.5ms")]
    #[case(999_900, "999.9ms")]
    #[case(1_000_000, "1.00s")]
    #[case(1_500_000, "1.50s")]
    #[case(12_345_678, "12.35s")]
    #[case(120_000_000, "2m 0s")]
    #[case(125_000_000, "2m 5s")]
    fn test_format_duration(#[case] micros: u64, #[case] expected: &str) {
        assert_eq!(format_duration(std::time::Duration::from_micros(micros)), expected);
    }

    #[rstest]
    #[case(0, "   0 B")]
    #[case(1, "   1 B")]
    #[case(999, " 999 B")]
    #[case(1023, "1023 B")]
    #[case(1024, "  1.0 K")]
    #[case(1536, "  1.5 K")]
    #[case(1048576, "  1.0 M")]
    #[case(1073741824, "  1.0 G")]
    #[case(1099511627776, "  1.0 T")]
    fn test_format_size(#[case] bytes: u64, #[case] expected: &str) {
        assert_eq!(format_size(bytes), expected);
    }

    #[test]
    fn test_format_permissions_file() {
        let dir = tempfile::TempDir::new().unwrap();
        let p = dir.path().join("f.txt");
        std::fs::write(&p, b"").unwrap();
        let meta = std::fs::metadata(&p).unwrap();
        let result = format_permissions(&meta);
        assert_eq!(result.chars().next(), Some('-'));
        assert_eq!(result.len(), 7);
    }

    #[test]
    fn test_format_permissions_dir() {
        let dir = tempfile::TempDir::new().unwrap();
        let meta = std::fs::metadata(dir.path()).unwrap();
        let result = format_permissions(&meta);
        assert_eq!(result.chars().next(), Some('d'));
        assert_eq!(result.len(), 7);
    }

    #[test]
    fn test_format_time() {
        let now = std::time::SystemTime::now();
        let s = format_time(now);
        assert_eq!(s.len(), 16);
        assert_eq!(s.chars().nth(4), Some('-'));
        assert_eq!(s.chars().nth(7), Some('-'));
        assert_eq!(s.chars().nth(10), Some(' '));
        assert_eq!(s.chars().nth(13), Some(':'));
    }
}
