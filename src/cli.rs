use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "ydcv", about = "A Rust version of ydcv")]
pub struct YdcvOptions {
    #[cfg(feature = "clipboard")]
    #[structopt(
        short = "x",
        long = "selection",
        help = "show explaination of current selection"
    )]
    pub selection: bool,

    #[cfg(windows)]
    #[cfg(feature = "clipboard")]
    #[structopt(
        short = "i",
        long = "interval",
        help = "time interval between selection in msec (default: 1000 on windows and 0 on others)",
        default_value = "1000"
    )]
    pub interval: u64,

    #[cfg(unix)]
    #[cfg(feature = "clipboard")]
    #[structopt(
        short = "i",
        long = "interval",
        help = "time interval between selection in msec (default: 1000 on windows and 0 on others)",
        default_value = "0"
    )]
    pub interval: u64,

    #[structopt(short = "H", long = "html", help = "HTML-style output")]
    pub html: bool,

    #[cfg(feature = "notify")]
    #[structopt(
        short = "n",
        long = "notify",
        help = "send desktop notifications (implies -H on X11)"
    )]
    pub notify: bool,

    #[structopt(
        short = "r",
        long = "raw",
        help = "dump raw json reply from server",
        conflicts_with = "html",
        conflicts_with = "notify"
    )]
    pub raw: bool,

    #[structopt(
        short = "c",
        long = "color",
        help = "[auto, always, never] use color",
        default_value = "auto"
    )]
    pub color: String,

    #[cfg(unix)]
    #[cfg(feature = "notify")]
    #[structopt(
        short = "t",
        long = "timeout",
        help = "timeout of notification (second)",
        default_value = "30"
    )]
    pub timeout: i32,

    #[structopt(value_name = "WORDS")]
    pub free: Vec<String>,
}
