use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use chrono::Local;
use std::sync::Mutex;
use std::fs::OpenOptions;
use std::io::Write;

static LOGGER: Mutex<Option<Logger>> = Mutex::new(None);

struct Logger {
    log_file: Mutex<std::fs::File>,
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let now = Local::now();
            let message = format!("[{}] {} - {}: {}\n",
                now.format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                record.args()
            );
            
            let mut file = self.log_file.lock().unwrap();
            if let Err(e) = file.write_all(message.as_bytes()) {
                eprintln!("Failed to write to log file: {}", e);
            }
        }
    }

    fn flush(&self) {
        if let Ok(mut file) = self.log_file.lock() {
            if let Err(e) = file.flush() {
                eprintln!("Failed to flush log file: {}", e);
            }
        }
    }
}

pub fn init(log_file_path: &str) -> Result<(), SetLoggerError> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)
        .expect("Failed to open log file");

    let logger = Logger {
        log_file: Mutex::new(file),
    };

    let mut global_logger = LOGGER.lock().unwrap();
    *global_logger = Some(logger);

    log::set_logger(global_logger.as_ref().unwrap())?;
    log::set_max_level(LevelFilter::Trace);

    Ok(())
}
