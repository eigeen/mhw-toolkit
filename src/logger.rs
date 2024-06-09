use log::{Metadata, Record};
use std::ffi::{CString, OsString};
use std::os::windows::ffi::OsStrExt;
use winapi::um::stringapiset::WideCharToMultiByte;
use winapi::um::winnls::CP_ACP;

// from mhwloader
extern "C" {
    fn Log(level: i32, message: *const i8);
}

#[repr(i32)]
#[derive(Clone)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
}

impl From<log::Level> for LogLevel {
    fn from(level: log::Level) -> Self {
        match level {
            log::Level::Error => Self::Error,
            log::Level::Warn => Self::Warn,
            log::Level::Info => Self::Info,
            log::Level::Debug => Self::Debug,
            log::Level::Trace => Self::Debug,
        }
    }
}

impl From<LogLevel> for log::Level {
    fn from(val: LogLevel) -> Self {
        match val {
            LogLevel::Error => log::Level::Error,
            LogLevel::Warn => log::Level::Warn,
            LogLevel::Info => log::Level::Info,
            LogLevel::Debug => log::Level::Debug,
        }
    }
}

pub fn log_to_loader(level: LogLevel, message: &str) {
    // 将Rust字符串转换为宽字符串
    let wide: Vec<u16> = OsString::from(message)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    // 计算所需的ANSI字符串长度
    let len = unsafe {
        WideCharToMultiByte(
            CP_ACP,
            0,
            wide.as_ptr(),
            -1,
            std::ptr::null_mut(),
            0,
            std::ptr::null(),
            std::ptr::null_mut(),
        )
    };
    if len == 0 {
        return;
    }

    // 转换宽字符串为ANSI编码的字符串
    let mut buffer: Vec<u8> = vec![0; len as usize];
    let _ = unsafe {
        WideCharToMultiByte(
            CP_ACP,
            0,
            wide.as_ptr(),
            -1,
            buffer.as_mut_ptr() as *mut i8,
            buffer.len() as i32,
            std::ptr::null(),
            std::ptr::null_mut(),
        )
    };

    // 将ANSI编码的字符串转换为CString
    let c_str = unsafe { CString::from_vec_unchecked(buffer) };

    unsafe { Log(level as i32, c_str.as_ptr()) }
}

/// logger 的 `log` crate 基本实现
pub struct MHWLogger {
    prefix: String,
}

impl MHWLogger {
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
        }
    }
}

impl log::Log for MHWLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            log_to_loader(
                record.level().into(),
                &format!("[{}] {}", self.prefix, record.args()),
            );
        }
    }

    fn flush(&self) {}
}
