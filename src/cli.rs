use clap::Parser;

#[derive(Parser)]
#[command(
    name = "monox",
    version = "0.4.0",
    about = "Launch a single GUI app on X11 without a desktop environment",
    after_help = "Examples:\n  monox firefox\n  monox gimp ~/photo.png\n  monox --check firefox\n  monox --dry-run chromium"
)]

pub struct Cli {
    /// Application to launch, followed by its arguments.
    #[arg(trailing_var_arg = true)]
    pub app: Vec<String>,

    /// Check dependencies without launching.
    #[arg(long)]
    pub check: bool,

    /// Show generated xinitrc without executing.
    #[arg(long)]
    pub dry_run: bool,
}
