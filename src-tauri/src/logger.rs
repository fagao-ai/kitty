use log::{Level, Log, Metadata, Record};

struct CustomLogger;

impl Log for CustomLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("Custom Handler: {} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}