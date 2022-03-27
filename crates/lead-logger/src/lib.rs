use lead_mem::{singleton, pointer::Pointer};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum LogLevel {
    Critical = 1,
    Error,
    Warning,
    Info,
    Debug
}

pub struct Log {
    pub level: LogLevel,
    pub target: String,
    pub message: String,
    pub file: String,
    pub line: u32,
}

singleton!(func: get_logger, LOGGER, Logger, Logger::new());

pub struct Logger {
    level: LogLevel
}

impl Logger {

    fn new() -> Logger {
        return Logger {
            level: LogLevel::Info
        }
    }

    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    pub fn get_level(&self) -> LogLevel {
        return self.level;
    }

    pub fn log(&mut self, log: Log) {
        if log.level <= self.level {
            println!("{:?} in ('{}':{}) [{}]: {}", log.level, log.file, log.line, log.target, log.message);
        }
    }

}

#[macro_export]
macro_rules! critical {
    ($target: expr, $($arg: tt)+) => {
        get_logger().as_mut().log(Log {
            level: LogLevel::Critical,
            target: $target.to_string(),
            message: format!($($arg)+),
            file: file!().to_string(),
            line: line!()
        });
        exit(-1)
    };
}

#[macro_export]
macro_rules! error {
    ($target: expr, $($arg: tt)+) => {
        get_logger().as_mut().log(Log {
            level: LogLevel::Error,
            target: $target.to_string(),
            message: format!($($arg)+),
            file: file!().to_string(),
            line: line!()
        });
    };
}

#[macro_export]
macro_rules! warn {
    ($target: expr, $($arg: tt)+) => {
        get_logger().as_mut().log(Log {
            level: LogLevel::Warning,
            target: $target.to_string(),
            message: format!($($arg)+),
            file: file!().to_string(),
            line: line!()
        });
    };
}

#[macro_export]
macro_rules! info {
    ($target: expr, $($arg: tt)+) => {
        get_logger().as_mut().log(Log {
            level: LogLevel::Info,
            target: $target.to_string(),
            message: format!($($arg)+),
            file: file!().to_string(),
            line: line!()
        });
    };
}

#[macro_export]
macro_rules! debug {
    ($target: expr, $($arg: tt)+) => {
        get_logger().as_mut().log(Log {
            level: LogLevel::Debug,
            target: $target.to_string(),
            message: format!($($arg)+),
            file: file!().to_string(),
            line: line!()
        });
    };
}