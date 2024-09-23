mod logger;

fn main() {
    logger::init("flash_loan_bot.log").expect("Failed to initialize logger");

    log::error!("This is an error message");
    log::warn!("This is a warning message");
    log::info!("This is an info message");
    log::debug!("This is a debug message");
    log::trace!("This is a trace message");
}
