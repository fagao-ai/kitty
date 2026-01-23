use log::{LevelFilter, Log, Metadata, Record};
use simplelog::{Config, SharedLogger};
use std::sync::{mpsc, Mutex};

pub struct KittyLogger {
    level: LevelFilter,
    config: Config,
    sender: Mutex<mpsc::Sender<String>>,
}

impl KittyLogger {
    pub fn new(
        log_level: LevelFilter,
        config: Config,
        sender: mpsc::Sender<String>,
    ) -> Box<KittyLogger> {
        Box::new(KittyLogger {
            level: log_level,
            config,
            sender: Mutex::new(sender),
        })
    }
}

impl Log for KittyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let sender = self.sender.lock().unwrap();
            sender.send(record.args().to_string()).unwrap();
        }
    }

    fn flush(&self) {}
}

impl SharedLogger for KittyLogger {
    fn level(&self) -> LevelFilter {
        self.level
    }

    fn config(&self) -> Option<&Config> {
        Some(&self.config)
    }

    fn as_log(self: Box<Self>) -> Box<dyn Log> {
        Box::new(*self)
    }
}
