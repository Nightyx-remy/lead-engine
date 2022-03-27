use lead_mem::singleton_mut;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                           Log Level                                            //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum LogLevel {
    Critical = 1,
    Error,
    Warning,
    Info,
    Debug
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                              Log                                               //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Log {
    pub level: LogLevel,
    pub target: String,
    pub message: String,
    pub file: String,
    pub line: u32,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             Logger                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

singleton_mut!(func: get_logger, LOGGER, Logger, Logger::new());

pub struct Logger {
    level: LogLevel
}

impl Logger {

    fn new() -> Logger {
        return Logger {
            level: LogLevel::Info
        }
    }

    pub fn log(&mut self, log: Log) {
        if log.level <= self.level {
            println!("{:?} in ('{}':{}) [{}]: {}", log.level, log.file, log.line, log.target, log.message);
        }
    }

    pub fn get_level(&self) -> LogLevel {
        return self.level;
    }

    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             Macros                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! critical {
    ($target: expr, $($arg: tt)+) => {
        $crate::get_logger().as_mut().log($crate::Log {
            level: $crate::LogLevel::Critical,
            target: $target.to_string(),
            message: format!($($arg)+),
            file: file!().to_string(),
            line: line!()
        });
        std::process::exit(-1);
    };
}

#[macro_export]
macro_rules! error {
    ($target: expr, $($arg: tt)+) => {
        $crate::get_logger().as_mut().log($crate::Log {
            level: $crate::LogLevel::Error,
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
        $crate::get_logger().as_mut().log($crate::Log {
            level: $crate::LogLevel::Warning,
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
        $crate::get_logger().as_mut().log($crate::Log {
            level: $crate::LogLevel::Info,
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
        $crate::get_logger().as_mut().log($crate::Log {
            level: $crate::LogLevel::Debug,
            target: $target.to_string(),
            message: format!($($arg)+),
            file: file!().to_string(),
            line: line!()
        });
    };
}