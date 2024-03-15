use std::fs;

use tauri::{AppHandle, Manager};
use tracing::metadata::LevelFilter;
use tracing_append::RollingFileAppender;

pub fn init(app: &AppHandle) {
    let logs_dir = app
        .path()
        .app_log_dir()
        .expect("failed to get logs dir");

    fs::create_dir_all(&logs_dir).expect("failed to create logs dir");

    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .max_log_files(14)
        .filename_prefix("CView.log")
        .build(&logs_dir)
        .expect("initialising rolling file appender failed");
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);
    app.manage(guard); // keep the guard alive for lifetime of tauri app
}