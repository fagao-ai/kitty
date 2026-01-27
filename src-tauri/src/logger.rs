use log::{LevelFilter, Log, Metadata, Record};
use simplelog::{Config, SharedLogger};
use std::sync::{mpsc, Mutex};
use tracing_subscriber::fmt::MakeWriter;

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

/// Writer that sends log messages to the frontend via KittyLogger
#[derive(Clone)]
pub struct FrontendWriter {
    sender: tokio::sync::mpsc::UnboundedSender<String>,
}

impl FrontendWriter {
    pub fn new(sender: tokio::sync::mpsc::UnboundedSender<String>) -> Self {
        Self { sender }
    }
}

impl std::io::Write for FrontendWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let msg = String::from_utf8_lossy(buf);
        // Use send - unbounded channel won't block, but may drop if receiver is gone
        let _ = self.sender.send(msg.to_string());
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// MakeWriter implementation for tracing subscriber
impl<'a> MakeWriter<'a> for FrontendWriter {
    type Writer = FrontendWriter;

    fn make_writer(&'a self) -> Self::Writer {
        FrontendWriter::new(self.sender.clone())
    }
}
