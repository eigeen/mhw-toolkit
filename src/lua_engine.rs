use std::ffi::c_char;

use mlua::{lua_CFunction, lua_State};

pub type LuaCoreMapStateProcessor = unsafe extern "C-unwind" fn(L: *mut lua_State);

#[repr(usize)]
pub enum LogLevel {
    Debug = 0,
    Info,
    Warn,
    Error,
}

#[link(name = "LuaEngine", kind = "static")]
extern "C" {
    pub fn Log(level: LogLevel, content: *const c_char);
    pub fn LuaCoreAddFunction(name: *const c_char, func: lua_CFunction);
    pub fn LuaCoreAddStateProcessor(func: LuaCoreMapStateProcessor);
}
