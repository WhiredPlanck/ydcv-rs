use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct YdcvOptions {
    /// Show explaination of current selection
    #[cfg(feature = "clipboard")]
    #[arg(short = 'x', long)]
    pub selection: bool,

    /// Time interval between selection in msec (default: 1000 on windows and 0 on others)
    #[cfg(windows)]
    #[cfg(feature = "clipboard")]
    #[arg(short, long, default_value_t = 1000)]
    pub interval: u64,

    /// Time interval between selection in msec (default: 1000 on windows and 0 on others)
    #[cfg(unix)]
    #[cfg(feature = "clipboard")]
    #[arg(short, long, default_value_t = 0)]
    pub interval: u64,

    /// HTML-style output
    #[structopt(short = 'H', long, group = "reply")]
    pub html: bool,

    /// Send desktop notifications (implies -H on X11)
    #[cfg(feature = "notify")]
    #[arg(short, long, group = "reply")]
    pub notify: bool,

    /// Dump raw JSON reply from server
    #[arg(short, long, group = "reply")]
    pub raw: bool,

    /// [auto, always, never] use color
    #[arg(short, long, default_value = "auto")]
    pub color: String,

    /// Timeout of notification (second)
    #[cfg(unix)]
    #[cfg(feature = "notify")]
    #[arg(short, long, default_value_t = 30)]
    pub timeout: i32,

    #[arg(value_name = "WORDS")]
    pub free: Vec<String>,
}
