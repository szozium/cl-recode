// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use cl_recode_lib::core::utils::args::Args;
#[cfg(target_os = "windows")]
use windows::Win32::UI::HiDpi::{SetProcessDpiAwareness, PROCESS_SYSTEM_DPI_AWARE};

fn main() {
    let args = Args::parse();
    args.process();

    let _ = dotenvy::dotenv();

    #[cfg(target_os = "windows")]
    unsafe {
        if let Err(e) = SetProcessDpiAwareness(PROCESS_SYSTEM_DPI_AWARE) {
            use cl_recode_lib::core::platform::error::StartupError;

            cl_recode_lib::log_error!("Failed to set DPI awareness: {}", e);
            cl_recode_lib::handle_startup_error(&StartupError::DpiAwarenessFailed(
                e.to_string(),
            ));
        }
    }

    if let Err(e) = cl_recode_lib::check_dependencies() {
        cl_recode_lib::log_error!("Dependency check failed: {}", e);
        cl_recode_lib::handle_startup_error(&e);
    }

    #[cfg(target_os = "linux")]
    if let Err(e) = cl_recode_lib::check_webkit_warning() {
        cl_recode_lib::handle_startup_error(&e);
    }

    cl_recode_lib::run()
}
