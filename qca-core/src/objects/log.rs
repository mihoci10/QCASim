use log::{Metadata, Record};
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

pub struct FileLogger {
    file: Mutex<File>,
}

pub fn create_file_logger(filename: &str) -> Result<FileLogger, std::io::Error> {
    let file = File::create(filename)?;
    Ok(FileLogger {
        file: Mutex::new(file),
    })
}

impl log::Log for FileLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let mut file = self.file.lock().unwrap();
        file.write_all(format!("[{}] - {}\n", record.level(), record.args()).as_bytes())
            .ok();
    }

    fn flush(&self) {}
}
