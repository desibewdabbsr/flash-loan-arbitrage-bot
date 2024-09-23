use super::*;
use std::fs;
use std::io::Read;
use tempfile::NamedTempFile;

#[test]
fn test_logger_initialization() {
    let temp_file = NamedTempFile::new().unwrap();
    let log_path = temp_file.path().to_str().unwrap();

    assert!(init(log_path).is_ok());
}

#[test]
fn test_log_writing() {
    let temp_file = NamedTempFile::new().unwrap();
    let log_path = temp_file.path().to_str().unwrap();

    init(log_path).unwrap();

    log::info!("Test log message");
    log::warn!("Test warning message");

    // Read the contents of the log file
    let mut file_content = String::new();
    fs::File::open(log_path)
        .unwrap()
        .read_to_string(&mut file_content)
        .unwrap();

    assert!(file_content.contains("INFO - test_log_writing: Test log message"));
    assert!(file_content.contains("WARN - test_log_writing: Test warning message"));
}

#[test]
fn test_log_levels() {
    let temp_file = NamedTempFile::new().unwrap();
    let log_path = temp_file.path().to_str().unwrap();

    init(log_path).unwrap();

    log::trace!("Trace message");
    log::debug!("Debug message");
    log::info!("Info message");
    log::warn!("Warn message");
    log::error!("Error message");

    // Read the contents of the log file
    let mut file_content = String::new();
    fs::File::open(log_path)
        .unwrap()
        .read_to_string(&mut file_content)
        .unwrap();

    assert!(file_content.contains("TRACE"));
    assert!(file_content.contains("DEBUG"));
    assert!(file_content.contains("INFO"));
    assert!(file_content.contains("WARN"));
    assert!(file_content.contains("ERROR"));
}

#[test]
fn test_logger_flush() {
    let temp_file = NamedTempFile::new().unwrap();
    let log_path = temp_file.path().to_str().unwrap();

    init(log_path).unwrap();

    log::info!("Test flush message");

    // Force a flush
    if let Some(logger) = LOGGER.lock().unwrap().as_ref() {
        logger.flush();
    }

    // Read the contents of the log file
    let mut file_content = String::new();
    fs::File::open(log_path)
        .unwrap()
        .read_to_string(&mut file_content)
        .unwrap();

    assert!(file_content.contains("Test flush message"));
}
