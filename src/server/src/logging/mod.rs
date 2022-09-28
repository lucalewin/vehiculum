use log::LevelFilter;
use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use log::Level;
use colored::*;

pub fn init(colored: bool) {
    Builder::new()
        .format(move |buf, record| {
            writeln!(buf,
                "[{}] [{}]: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                if colored { level_color(record.level()) } else { record.level().to_string() },
                record.args()
            )
        })
        .filter(None, LevelFilter::Trace)
        .init();
}

fn level_color(level: log::Level) -> String {
    match level {
        Level::Error => level.as_str().red(),
        Level::Warn  => level.as_str().yellow(),
        Level::Info  => level.as_str().green(),
        Level::Debug => level.as_str().bright_blue(),
        Level::Trace => level.as_str().magenta(),
    }.to_string()
}
