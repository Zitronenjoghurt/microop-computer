use env_logger::Builder;
use log::LevelFilter;

pub const LOG_LEVEL_CPU: LevelFilter = LevelFilter::Debug;
pub const LOG_LEVEL_RAM: LevelFilter = LevelFilter::Debug;
pub const LOG_LEVEL_ROM: LevelFilter = LevelFilter::Debug;

pub fn initialize_logging() {
    let mut builder = Builder::new();
    builder.format_timestamp(None);
    builder.filter_module("cpu", LOG_LEVEL_CPU);
    builder.filter_module("ram", LOG_LEVEL_RAM);
    builder.filter_module("rom", LOG_LEVEL_ROM);
    builder.init();
}

#[macro_export]
macro_rules! log_microop_debug {
    ($op_name:expr, $fmt:expr $(, $arg:expr)*) => {
        debug!(
            target: &format!("cpu::{:17}", $op_name),
            $fmt $(, $arg)*
        )
    }
}
