//! main module of ydcv-rs
#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

use clap::Parser;
#[cfg(feature = "clipboard")]
use copypasta::ClipboardContext;

use copypasta::ClipboardProvider;
use reqwest::blocking::{Client, ClientBuilder};
use rustyline::Editor;

mod formatters;
mod ydclient;
mod ydresponse;
mod cli;

#[cfg(windows)]
#[cfg(feature = "notify")]
use crate::formatters::WinFormatter;
use crate::formatters::{AnsiFormatter, Formatter, HtmlFormatter, PlainFormatter};
use crate::ydclient::YdClient;
use crate::cli::YdcvOptions;

fn lookup_explain(client: &mut Client, word: &str, fmt: &mut dyn Formatter, raw: bool) {
    if raw {
        println!(
            "{}",
            serde_json::to_string(&client.lookup_word(word, true).unwrap()).unwrap()
        );
    } else {
        match client.lookup_word(word, false) {
            Ok(ref result) => {
                let exp = result.explain(fmt);
                fmt.print(word, &exp);
            }
            Err(err) => fmt.print(word, &format!("Error looking-up word {}: {:?}", word, err)),
        }
    }
}

fn main() {
    env_logger::init();

    let ydcv_options = YdcvOptions::parse();

    #[cfg(feature = "notify")]
    let notify_enabled = ydcv_options.notify;
    #[cfg(not(feature = "notify"))]
    let notify_enabled = false;

    #[cfg(feature = "clipboard")]
    let selection_enabled = ydcv_options.selection;

    #[cfg(feature = "clipboard")]
    let interval = ydcv_options.interval;

    #[cfg(not(feature = "clipboard"))]
    let selection_enabled = false;

    let mut client = ClientBuilder::new().build().unwrap();

    let mut html = HtmlFormatter::new(notify_enabled);
    let mut ansi = AnsiFormatter::new(notify_enabled);
    let mut plain = PlainFormatter::new(notify_enabled);
    #[cfg(windows)]
    #[cfg(feature = "notify")]
    let mut win = WinFormatter::new(notify_enabled);

    #[cfg(unix)]
    #[cfg(feature = "notify")]
    html.set_timeout(ydcv_options.timeout * 1000);

    let fmt: &mut dyn Formatter =
        if ydcv_options.html || (notify_enabled && cfg!(unix) && cfg!(feature = "notify")) {
            &mut html
        } else if notify_enabled {
            #[cfg(all(windows, feature = "notify"))]
            {
                &mut win
            }
            #[cfg(not(all(windows, feature = "notify")))]
            {
                &mut plain
            }
        } else if ydcv_options.color == "always"
            || atty::is(atty::Stream::Stdout) && ydcv_options.color != "never"
        {
            &mut ansi
        } else {
            &mut plain
        };

    if ydcv_options.free.is_empty() {
        if selection_enabled {
            #[cfg(feature = "clipboard")]
            {
                let mut clipboard = ClipboardContext::new().unwrap();
                let mut last = String::new();

                println!("Waiting for selection> ");

                loop {
                    std::thread::sleep(std::time::Duration::from_millis(interval));
                    if let Ok(curr) = clipboard.get_contents() {
                        let curr = curr.trim_matches('\u{0}').trim();
                        if !curr.is_empty() && last != curr {
                            last = curr.to_owned();
                            lookup_explain(&mut client, curr, fmt, ydcv_options.raw);
                            println!("Waiting for selection> ");
                        }
                    }
                }
            }
        } else {
            if let Ok(mut reader) = Editor::<()>::new() {
                while let Ok(w) = reader.readline("> ") {
                    let word = w.trim();
                    reader.add_history_entry(word);
                    if !word.is_empty() {
                        lookup_explain(&mut client, &word, fmt, ydcv_options.raw);
                    }
                }
            }
        }
    } else {
        for word in ydcv_options.free {
            lookup_explain(&mut client, &word.trim(), fmt, ydcv_options.raw);
        }
    }
}
