use colored::Color;
use phf::phf_map;
use std::path::Path;

pub struct FileStyle {
    pub color: Color,
    pub symbol: char,
}

static STYLES: phf::Map<&'static str, (Color, char)> = phf_map! {
    "rs" => (Color::TrueColor { r: 183, g: 65, b: 14 }, '\u{e7a8}'),
    "c" => (Color::TrueColor { r: 0, g: 89, b: 156 }, '\u{e61e}'),
    "h" => (Color::TrueColor { r: 0, g: 89, b: 156 }, '\u{e61e}'),
    "cpp" => (Color::TrueColor { r: 0, g: 89, b: 156 }, '\u{e61d}'),
    "cc" => (Color::TrueColor { r: 0, g: 89, b: 156 }, '\u{e61d}'),
    "cxx" => (Color::TrueColor { r: 0, g: 89, b: 156 }, '\u{e61d}'),
    "hpp" => (Color::TrueColor { r: 0, g: 89, b: 156 }, '\u{e61d}'),
    "hh" => (Color::TrueColor { r: 0, g: 89, b: 156 }, '\u{e61d}'),
    "hxx" => (Color::TrueColor { r: 0, g: 89, b: 156 }, '\u{e61d}'),
    "go" => (Color::TrueColor { r: 0, g: 173, b: 216 }, '\u{e626}'),
    "java" => (Color::TrueColor { r: 237, g: 139, b: 0 }, '\u{e738}'),
    "class" => (Color::TrueColor { r: 237, g: 139, b: 0 }, '\u{e738}'),
    "jar" => (Color::TrueColor { r: 237, g: 139, b: 0 }, '\u{e738}'),
    "gradle" => (Color::TrueColor { r: 237, g: 139, b: 0 }, '\u{e738}'),
    "rb" => (Color::TrueColor { r: 204, g: 0, b: 0 }, '\u{e634}'),
    "erb" => (Color::TrueColor { r: 204, g: 0, b: 0 }, '\u{e634}'),
    "rake" => (Color::TrueColor { r: 204, g: 0, b: 0 }, '\u{e634}'),
    "gemspec" => (Color::TrueColor { r: 204, g: 0, b: 0 }, '\u{e634}'),
    "swift" => (Color::TrueColor { r: 240, g: 81, b: 56 }, '\u{e71f}'),
    "kt" => (Color::TrueColor { r: 127, g: 79, b: 255 }, '\u{f126}'),
    "kts" => (Color::TrueColor { r: 127, g: 79, b: 255 }, '\u{f126}'),
    "dart" => (Color::TrueColor { r: 0, g: 180, b: 171 }, '\u{e60c}'),
    "lua" => (Color::TrueColor { r: 0, g: 0, b: 128 }, '\u{e620}'),
    "pl" => (Color::TrueColor { r: 0, g: 104, b: 139 }, '\u{e618}'),
    "pm" => (Color::TrueColor { r: 0, g: 104, b: 139 }, '\u{e618}'),
    "t" => (Color::TrueColor { r: 0, g: 104, b: 139 }, '\u{e618}'),
    "pod" => (Color::TrueColor { r: 0, g: 104, b: 139 }, '\u{e618}'),
    "hs" => (Color::TrueColor { r: 94, g: 80, b: 134 }, '\u{e61b}'),
    "lhs" => (Color::TrueColor { r: 94, g: 80, b: 134 }, '\u{e61b}'),
    "ex" => (Color::TrueColor { r: 78, g: 42, b: 142 }, '\u{e62d}'),
    "exs" => (Color::TrueColor { r: 78, g: 42, b: 142 }, '\u{e62d}'),
    "clj" => (Color::TrueColor { r: 88, g: 129, b: 217 }, '\u{f126}'),
    "cljs" => (Color::TrueColor { r: 88, g: 129, b: 217 }, '\u{f126}'),
    "cljc" => (Color::TrueColor { r: 88, g: 129, b: 217 }, '\u{f126}'),
    "edn" => (Color::TrueColor { r: 88, g: 129, b: 217 }, '\u{f126}'),
    "erl" => (Color::TrueColor { r: 163, g: 38, b: 56 }, '\u{e619}'),
    "hrl" => (Color::TrueColor { r: 163, g: 38, b: 56 }, '\u{e619}'),
    "scala" => (Color::TrueColor { r: 220, g: 50, b: 47 }, '\u{f126}'),
    "sc" => (Color::TrueColor { r: 220, g: 50, b: 47 }, '\u{f126}'),
    "zig" => (Color::TrueColor { r: 247, g: 164, b: 29 }, '\u{f126}'),
    "nim" => (Color::TrueColor { r: 255, g: 194, b: 0 }, '\u{f126}'),
    "cr" => (Color::White, '\u{f126}'),
    "ml" => (Color::TrueColor { r: 221, g: 118, b: 0 }, '\u{f126}'),
    "mli" => (Color::TrueColor { r: 221, g: 118, b: 0 }, '\u{f126}'),
    "fs" => (Color::TrueColor { r: 55, g: 139, b: 186 }, '\u{f126}'),
    "fsx" => (Color::TrueColor { r: 55, g: 139, b: 186 }, '\u{f126}'),
    "r" => (Color::TrueColor { r: 25, g: 140, b: 231 }, '\u{f126}'),
    "R" => (Color::TrueColor { r: 25, g: 140, b: 231 }, '\u{f126}'),
    "Rmd" => (Color::TrueColor { r: 25, g: 140, b: 231 }, '\u{f126}'),
    "sql" => (Color::TrueColor { r: 227, g: 155, b: 0 }, '\u{f1c0}'),
    "php" => (Color::TrueColor { r: 119, g: 123, b: 179 }, '\u{f126}'),
    "asm" => (Color::TrueColor { r: 110, g: 110, b: 110 }, '\u{f085}'),
    "s" => (Color::TrueColor { r: 110, g: 110, b: 110 }, '\u{f085}'),
    "S" => (Color::TrueColor { r: 110, g: 110, b: 110 }, '\u{f085}'),
    "nasm" => (Color::TrueColor { r: 110, g: 110, b: 110 }, '\u{f085}'),
    "masm" => (Color::TrueColor { r: 110, g: 110, b: 110 }, '\u{f085}'),
    "toml" => (Color::Yellow, '\u{f013}'),
    "json" => (Color::BrightBlue, '\u{e60b}'),
    "md" => (Color::Magenta, '\u{e609}'),
    "py" => (Color::BrightGreen, '\u{e606}'),
    "js" => (Color::Yellow, '\u{e74e}'),
    "mjs" => (Color::Yellow, '\u{e74e}'),
    "cjs" => (Color::Yellow, '\u{e74e}'),
    "ts" => (Color::BrightBlue, '\u{e628}'),
    "tsx" => (Color::BrightBlue, '\u{e628}'),
    "jsx" => (Color::BrightCyan, '\u{e7ba}'),
    "html" => (Color::BrightMagenta, '\u{e60e}'),
    "htm" => (Color::BrightMagenta, '\u{e60e}'),
    "css" => (Color::BrightMagenta, '\u{e749}'),
    "scss" => (Color::BrightMagenta, '\u{e749}'),
    "sass" => (Color::BrightMagenta, '\u{e749}'),
    "less" => (Color::BrightMagenta, '\u{e749}'),
    "png" => (Color::BrightMagenta, '\u{f1c5}'),
    "jpg" => (Color::BrightMagenta, '\u{f1c5}'),
    "jpeg" => (Color::BrightMagenta, '\u{f1c5}'),
    "gif" => (Color::BrightMagenta, '\u{f1c5}'),
    "svg" => (Color::BrightMagenta, '\u{f1c5}'),
    "ico" => (Color::BrightMagenta, '\u{f1c5}'),
    "bmp" => (Color::BrightMagenta, '\u{f1c5}'),
    "webp" => (Color::BrightMagenta, '\u{f1c5}'),
    "zip" => (Color::BrightRed, '\u{f1c6}'),
    "tar" => (Color::BrightRed, '\u{f1c6}'),
    "gz" => (Color::BrightRed, '\u{f1c6}'),
    "bz2" => (Color::BrightRed, '\u{f1c6}'),
    "xz" => (Color::BrightRed, '\u{f1c6}'),
    "7z" => (Color::BrightRed, '\u{f1c6}'),
    "rar" => (Color::BrightRed, '\u{f1c6}'),
    "exe" => (Color::Green, '\u{f085}'),
    "bat" => (Color::Green, '\u{f085}'),
    "cmd" => (Color::Green, '\u{f085}'),
    "com" => (Color::Green, '\u{f085}'),
    "ps1" => (Color::Green, '\u{f085}'),
    "sh" => (Color::Green, '\u{f085}'),
    "bash" => (Color::Green, '\u{f085}'),
    "zsh" => (Color::Green, '\u{f085}'),
    "fish" => (Color::Green, '\u{f085}'),
    "lock" => (Color::BrightBlack, '\u{f023}'),
    "yml" => (Color::BrightBlue, '\u{f15b}'),
    "yaml" => (Color::BrightBlue, '\u{f15b}'),
    "txt" => (Color::BrightBlack, '\u{f15c}'),
    "log" => (Color::BrightBlack, '\u{f15c}'),
    "pdf" => (Color::BrightRed, '\u{f1c1}'),
    "mp3" => (Color::BrightMagenta, '\u{f001}'),
    "wav" => (Color::BrightMagenta, '\u{f001}'),
    "flac" => (Color::BrightMagenta, '\u{f001}'),
    "aac" => (Color::BrightMagenta, '\u{f001}'),
    "ogg" => (Color::BrightMagenta, '\u{f001}'),
    "wma" => (Color::BrightMagenta, '\u{f001}'),
    "mp4" => (Color::BrightMagenta, '\u{f008}'),
    "avi" => (Color::BrightMagenta, '\u{f008}'),
    "mkv" => (Color::BrightMagenta, '\u{f008}'),
    "mov" => (Color::BrightMagenta, '\u{f008}'),
    "webm" => (Color::BrightMagenta, '\u{f008}'),
    "flv" => (Color::BrightMagenta, '\u{f008}'),
    "wmv" => (Color::BrightMagenta, '\u{f008}'),
    "xml" => (Color::BrightBlue, '\u{f126}'),
    "xsl" => (Color::BrightBlue, '\u{f126}'),
    "xsd" => (Color::BrightBlue, '\u{f126}'),
    "xslt" => (Color::BrightBlue, '\u{f126}'),
    "csv" => (Color::BrightGreen, '\u{f15c}'),
    "env" => (Color::Yellow, '\u{f013}'),
    "ini" => (Color::Yellow, '\u{f013}'),
    "cfg" => (Color::Yellow, '\u{f013}'),
    "conf" => (Color::Yellow, '\u{f013}'),
    "db" => (Color::BrightCyan, '\u{f1c0}'),
    "sqlite" => (Color::BrightCyan, '\u{f1c0}'),
    "sqlite3" => (Color::BrightCyan, '\u{f1c0}'),
    "diff" => (Color::BrightBlue, '\u{f126}'),
    "patch" => (Color::BrightBlue, '\u{f126}'),
    "ttf" => (Color::BrightBlue, '\u{f031}'),
    "otf" => (Color::BrightBlue, '\u{f031}'),
    "woff" => (Color::BrightBlue, '\u{f031}'),
    "woff2" => (Color::BrightBlue, '\u{f031}'),
    "eot" => (Color::BrightBlue, '\u{f031}'),
    "pem" => (Color::Yellow, '\u{f023}'),
    "crt" => (Color::Yellow, '\u{f023}'),
    "key" => (Color::Yellow, '\u{f023}'),
    "csr" => (Color::Yellow, '\u{f023}'),
    "o" => (Color::BrightBlack, '\u{f085}'),
    "obj" => (Color::BrightBlack, '\u{f085}'),
    "lib" => (Color::BrightBlack, '\u{f085}'),
    "a" => (Color::BrightBlack, '\u{f085}'),
    "so" => (Color::BrightBlack, '\u{f085}'),
    "dylib" => (Color::BrightBlack, '\u{f085}'),
    "dll" => (Color::BrightBlack, '\u{f085}'),
    "iso" => (Color::BrightRed, '\u{f0a0}'),
    "img" => (Color::BrightRed, '\u{f0a0}'),
    "vhd" => (Color::BrightRed, '\u{f0a0}'),
    "vmdk" => (Color::BrightRed, '\u{f0a0}'),
};

#[must_use]
pub fn lookup_style(ext: &str) -> Option<FileStyle> {
    STYLES.get(ext).map(|&(color, symbol)| FileStyle { color, symbol })
}

#[must_use]
pub fn get_style(name: &str, is_dir: bool, is_symlink: bool) -> FileStyle {
    if is_dir {
        return FileStyle { color: Color::BrightBlue, symbol: '\u{f07c}' };
    }
    if is_symlink {
        return FileStyle { color: Color::Cyan, symbol: '\u{f0c1}' };
    }

    match name {
        "Dockerfile" | ".dockerignore" => {
            return FileStyle { color: Color::TrueColor { r: 0, g: 119, b: 196 }, symbol: '\u{f308}' };
        }
        "Makefile" | "makefile" | "GNUmakefile" => {
            return FileStyle { color: Color::Yellow, symbol: '\u{f126}' };
        }
        "Gemfile" | "Rakefile" => {
            return FileStyle { color: Color::TrueColor { r: 204, g: 0, b: 0 }, symbol: '\u{e634}' };
        }
        "README" | "Readme" => {
            return FileStyle { color: Color::BrightBlue, symbol: '\u{f02d}' };
        }
        "LICENSE" | "COPYING" => {
            return FileStyle { color: Color::Green, symbol: '\u{f0e3}' };
        }
        ".env" | ".env.local" | ".env.production" | ".env.development" => {
            return FileStyle { color: Color::Yellow, symbol: '\u{f013}' };
        }
        ".gitignore" | ".gitattributes" | ".gitmodules" => {
            return FileStyle { color: Color::BrightBlack, symbol: '\u{e702}' };
        }
        _ => {}
    }

    let ext = Path::new(name)
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    lookup_style(&ext).unwrap_or(FileStyle { color: Color::White, symbol: '\0' })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("folder", true, false, Color::BrightBlue, '\u{f07c}')]
    #[case("link", false, true, Color::Cyan, '\u{f0c1}')]
    #[case("main.rs", false, false, Color::TrueColor { r: 183, g: 65, b: 14 }, '\u{e7a8}')]
    #[case("main.c", false, false, Color::TrueColor { r: 0, g: 89, b: 156 }, '\u{e61e}')]
    #[case("main.h", false, false, Color::TrueColor { r: 0, g: 89, b: 156 }, '\u{e61e}')]
    #[case("main.cpp", false, false, Color::TrueColor { r: 0, g: 89, b: 156 }, '\u{e61d}')]
    #[case("main.hpp", false, false, Color::TrueColor { r: 0, g: 89, b: 156 }, '\u{e61d}')]
    #[case("main.cc", false, false, Color::TrueColor { r: 0, g: 89, b: 156 }, '\u{e61d}')]
    #[case("main.hh", false, false, Color::TrueColor { r: 0, g: 89, b: 156 }, '\u{e61d}')]
    #[case("main.go", false, false, Color::TrueColor { r: 0, g: 173, b: 216 }, '\u{e626}')]
    #[case("Main.java", false, false, Color::TrueColor { r: 237, g: 139, b: 0 }, '\u{e738}')]
    #[case("app.jar", false, false, Color::TrueColor { r: 237, g: 139, b: 0 }, '\u{e738}')]
    #[case("App.class", false, false, Color::TrueColor { r: 237, g: 139, b: 0 }, '\u{e738}')]
    #[case("app.rb", false, false, Color::TrueColor { r: 204, g: 0, b: 0 }, '\u{e634}')]
    #[case("Gemfile", false, false, Color::TrueColor { r: 204, g: 0, b: 0 }, '\u{e634}')]
    #[case("Rakefile", false, false, Color::TrueColor { r: 204, g: 0, b: 0 }, '\u{e634}')]
    #[case("app.swift", false, false, Color::TrueColor { r: 240, g: 81, b: 56 }, '\u{e71f}')]
    #[case("app.kt", false, false, Color::TrueColor { r: 127, g: 79, b: 255 }, '\u{f126}')]
    #[case("app.dart", false, false, Color::TrueColor { r: 0, g: 180, b: 171 }, '\u{e60c}')]
    #[case("app.lua", false, false, Color::TrueColor { r: 0, g: 0, b: 128 }, '\u{e620}')]
    #[case("app.pl", false, false, Color::TrueColor { r: 0, g: 104, b: 139 }, '\u{e618}')]
    #[case("lib.pm", false, false, Color::TrueColor { r: 0, g: 104, b: 139 }, '\u{e618}')]
    #[case("lib.t", false, false, Color::TrueColor { r: 0, g: 104, b: 139 }, '\u{e618}')]
    #[case("app.hs", false, false, Color::TrueColor { r: 94, g: 80, b: 134 }, '\u{e61b}')]
    #[case("app.ex", false, false, Color::TrueColor { r: 78, g: 42, b: 142 }, '\u{e62d}')]
    #[case("app.clj", false, false, Color::TrueColor { r: 88, g: 129, b: 217 }, '\u{f126}')]
    #[case("app.erl", false, false, Color::TrueColor { r: 163, g: 38, b: 56 }, '\u{e619}')]
    #[case("app.scala", false, false, Color::TrueColor { r: 220, g: 50, b: 47 }, '\u{f126}')]
    #[case("main.zig", false, false, Color::TrueColor { r: 247, g: 164, b: 29 }, '\u{f126}')]
    #[case("main.nim", false, false, Color::TrueColor { r: 255, g: 194, b: 0 }, '\u{f126}')]
    #[case("main.cr", false, false, Color::White, '\u{f126}')]
    #[case("app.ml", false, false, Color::TrueColor { r: 221, g: 118, b: 0 }, '\u{f126}')]
    #[case("app.fs", false, false, Color::TrueColor { r: 55, g: 139, b: 186 }, '\u{f126}')]
    #[case("app.r", false, false, Color::TrueColor { r: 25, g: 140, b: 231 }, '\u{f126}')]
    #[case("query.sql", false, false, Color::TrueColor { r: 227, g: 155, b: 0 }, '\u{f1c0}')]
    #[case("index.php", false, false, Color::TrueColor { r: 119, g: 123, b: 179 }, '\u{f126}')]
    #[case("main.asm", false, false, Color::TrueColor { r: 110, g: 110, b: 110 }, '\u{f085}')]
    #[case("script.zsh", false, false, Color::Green, '\u{f085}')]
    #[case("script.fish", false, false, Color::Green, '\u{f085}')]
    #[case("Dockerfile", false, false, Color::TrueColor { r: 0, g: 119, b: 196 }, '\u{f308}')]
    #[case("Makefile", false, false, Color::Yellow, '\u{f126}')]
    #[case("README", false, false, Color::BrightBlue, '\u{f02d}')]
    #[case("LICENSE", false, false, Color::Green, '\u{f0e3}')]
    #[case("config.xml", false, false, Color::BrightBlue, '\u{f126}')]
    #[case("data.csv", false, false, Color::BrightGreen, '\u{f15c}')]
    #[case("photo.png", false, false, Color::BrightMagenta, '\u{f1c5}')]
    #[case("photo.svg", false, false, Color::BrightMagenta, '\u{f1c5}')]
    #[case("style.css", false, false, Color::BrightMagenta, '\u{e749}')]
    #[case("style.scss", false, false, Color::BrightMagenta, '\u{e749}')]
    #[case("page.htm", false, false, Color::BrightMagenta, '\u{e60e}')]
    #[case("app.mjs", false, false, Color::Yellow, '\u{e74e}')]
    #[case("app.tsx", false, false, Color::BrightBlue, '\u{e628}')]
    #[case("archive.tar", false, false, Color::BrightRed, '\u{f1c6}')]
    #[case("archive.gz", false, false, Color::BrightRed, '\u{f1c6}')]
    #[case("data.yml", false, false, Color::BrightBlue, '\u{f15b}')]
    #[case("readme.txt", false, false, Color::BrightBlack, '\u{f15c}')]
    #[case("doc.pdf", false, false, Color::BrightRed, '\u{f1c1}')]
    #[case("track.mp3", false, false, Color::BrightMagenta, '\u{f001}')]
    #[case("video.mkv", false, false, Color::BrightMagenta, '\u{f008}')]
    #[case("package.lock", false, false, Color::BrightBlack, '\u{f023}')]
    #[case(".gitattributes", false, false, Color::BrightBlack, '\u{e702}')]
    #[case(".dockerignore", false, false, Color::TrueColor { r: 0, g: 119, b: 196 }, '\u{f308}')]
    #[case(".env", false, false, Color::Yellow, '\u{f013}')]
    #[case(".env.local", false, false, Color::Yellow, '\u{f013}')]
    #[case("config.ini", false, false, Color::Yellow, '\u{f013}')]
    #[case("data.db", false, false, Color::BrightCyan, '\u{f1c0}')]
    #[case("change.diff", false, false, Color::BrightBlue, '\u{f126}')]
    #[case("font.ttf", false, false, Color::BrightBlue, '\u{f031}')]
    #[case("cert.pem", false, false, Color::Yellow, '\u{f023}')]
    #[case("main.o", false, false, Color::BrightBlack, '\u{f085}')]
    #[case("disk.iso", false, false, Color::BrightRed, '\u{f0a0}')]
    fn test_get_style(#[case] name: &str, #[case] is_dir: bool, #[case] is_sym: bool, #[case] color: Color, #[case] symbol: char) {
        let s = get_style(name, is_dir, is_sym);
        assert_eq!(s.color, color);
        assert_eq!(s.symbol, symbol);
    }
}
