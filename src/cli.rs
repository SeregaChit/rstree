use clap::Parser;

#[derive(Parser)]
#[command(name = "rstree", version, about = "Directory rstree visualizer", disable_version_flag = true, disable_help_flag = true)]
pub struct Cli {
    #[arg(short = 'f', long = "files", help = "Show files")]
    pub show_files: bool,

    #[arg(short = 'a', long = "all", help = "Show hidden files")]
    pub show_hidden: bool,

    #[arg(short = 'r', long = "rev", help = "Reverse sort order")]
    pub reverse: bool,

    #[arg(short = 'l', long = "long", help = "Long format (perms, size, date)")]
    pub long: bool,

    #[arg(short = 'v', long = "version", help = "Print version")]
    pub show_version: bool,

    #[arg(short = 'L', long = "max-depth", help = "Max display depth")]
    pub max_depth: Option<usize>,

    #[arg(short = 'h', long = "help", help = "Print help")]
    pub show_help: bool,

    #[arg(default_value = ".", help = "Path to display")]
    pub path: String,
}
