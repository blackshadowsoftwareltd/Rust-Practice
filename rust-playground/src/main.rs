#![deny(clippy::all)]

use log::{debug, error, info, warn, LevelFilter};
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Logger, Root},
    encode::pattern::PatternEncoder,
    Config,
};

fn main() {
    log_config();

    debug!("Mary has a little lamb");
    error!("{}", "Its fleece was white as snow");
    info!("{:?}", "And every where that Mary went");
    warn!("{:#?}", "The lamb was sure to go");
}

fn log_config() {
    let stdout = ConsoleAppender::builder().build();
    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("log/requests.log")
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(Logger::builder().build("app::backend::db", LevelFilter::Info))
        .logger(
            Logger::builder()
                .appender("requests")
                .additive(false)
                .build("app::requests", LevelFilter::Info),
        )
        .build(Root::builder().appender("stdout").build(LevelFilter::Warn))
        .unwrap();
    let _ = log4rs::init_config(config).unwrap();
}
