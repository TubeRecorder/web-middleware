use log::LevelFilter;
use std::path::PathBuf;

use crate::errors::Error;

pub fn setup_logger(
    is_debug: bool,
    is_stdout_log: bool,
    log_file_path: Option<PathBuf>,
) -> Result<(), Error> {
    let level = match is_debug {
        true => LevelFilter::Debug,
        false => LevelFilter::Info,
    };

    let mut logger = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(level);

    if is_stdout_log {
        logger = logger.chain(std::io::stdout());
    }

    if let Some(file_path) = log_file_path {
        let log_file = fern::log_file(file_path)?;

        logger = logger.chain(log_file);
    }

    logger.apply()?;

    Ok(())
}
