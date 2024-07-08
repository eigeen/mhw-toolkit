use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use windows::Win32::System::Threading::GetCurrentProcessId;
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};

/// 设置指针所指向的值
#[inline]
pub unsafe fn set_value<T>(ptr: *mut T, value: T) {
    *ptr = value;
}

/// 获取某个地址包含的值
pub unsafe fn get_value<T>(base_addr: *const T) -> Option<T>
where
    T: Copy,
{
    if base_addr.is_null() {
        return None;
    }
    Some(*base_addr)
}

/// 获取某个地址经过多级偏移后指向的值（的副本） \
/// 该函数与CE中多级偏移取值算法一致
///
/// addr: 裸指针 \
/// offsets: 多级偏移量（单位：byte） \
/// return: 若多级偏移时出现空指针，则返回None，否则返回Some(T)
///
/// 注意：只能检查多次取值时出现的空指针问题。 \
/// 若应用程序出现野指针可能触发异常
pub fn get_value_with_offset<T>(base_addr: *const T, offsets: &[isize]) -> Option<T>
where
    T: Copy,
{
    if base_addr.is_null() {
        return None;
    }
    let mut addr = base_addr;
    unsafe {
        // 取值+偏移
        // 取值后需要判断是否为空指针
        for &offset in offsets.iter() {
            let valptr = *(addr as *const *const T);
            if valptr.is_null() {
                return None;
            }
            addr = valptr.byte_offset(offset);
        }
        // 最后一级取值作为真实值返回
        Some(*addr)
    }
}

/// 获取某个地址经过多级偏移后指向的值的引用 \
/// 该函数与CE中多级偏移取值算法一致
///
/// addr: 裸指针 \
/// offsets: 多级偏移量（单位：byte） \
/// return: 若多级偏移时出现空指针，则返回None，否则返回Some(T)
pub fn get_ref_with_offset<T>(base_addr: *const T, offsets: &[isize]) -> Option<&'static T> {
    if base_addr.is_null() {
        return None;
    }
    let mut addr = base_addr;
    unsafe {
        // 取值+偏移
        // 取值后需要判断是否为空指针
        for &offset in offsets.iter() {
            let valptr = *(addr as *const *const T);
            if valptr.is_null() {
                return None;
            }
            addr = valptr.byte_offset(offset);
        }
        Some(addr.as_ref().unwrap())
    }
}

pub fn get_mut_with_offset<T>(base_addr: *mut T, offsets: &[isize]) -> Option<&'static mut T> {
    if base_addr.is_null() {
        return None;
    }
    let mut addr = base_addr;
    unsafe {
        // 取值+偏移
        // 取值后需要判断是否为空指针
        for &offset in offsets.iter() {
            let valptr = *(addr as *mut *mut T);
            if valptr.is_null() {
                return None;
            }
            addr = valptr.byte_offset(offset);
        }
        Some(addr.as_mut().unwrap())
    }
}

/// 获取某个地址经过多级偏移后的地址 \
/// 该函数与CE中多级偏移取值算法一致
///
/// addr: 裸指针 \
/// offsets: 多级偏移量（单位：byte） \
/// return: 若多级偏移时出现空指针，则返回None，否则返回Some(*const T)
///
/// 注意：只能检查多次取值时出现的空指针问题。 \
/// 若应用程序出现野指针可能触发异常
pub fn get_ptr_with_offset<T>(base_addr: *const T, offsets: &[isize]) -> Option<*const T> {
    if base_addr.is_null() {
        return None;
    }
    let mut addr = base_addr;
    unsafe {
        // 取值+偏移
        // 取值后需要判断是否为空指针
        for &offset in offsets.iter() {
            let valptr = *(addr as *const *const T);
            if valptr.is_null() {
                return None;
            }
            addr = valptr.byte_offset(offset);
        }
        // 返回最后一级指针
        Some(addr)
    }
}

/// 检查当前活动窗口是否为游戏窗口
pub fn is_mhw_foreground() -> bool {
    // 获取当前前台窗口句柄
    let foreground_hwnd = unsafe { GetForegroundWindow() };
    if foreground_hwnd.0 == 0 {
        return false;
    }

    // 获取窗口所属进程ID
    let mut window_pid = 0;
    unsafe {
        GetWindowThreadProcessId(foreground_hwnd, Some(&mut window_pid));
    };
    if window_pid == 0 {
        return false;
    }

    // 获取当前进程ID
    // 伪句柄无需关闭
    let current_pid = unsafe { GetCurrentProcessId() };

    window_pid == current_pid
}

pub struct TimeLockManager {
    lockers: Arc<Mutex<HashMap<String, TimeLock>>>,
}

impl Default for TimeLockManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeLockManager {
    pub fn new() -> Self {
        TimeLockManager {
            lockers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn set(&mut self, key: &str, dur: Duration) {
        self.lockers
            .lock()
            .unwrap()
            .insert(key.to_string(), TimeLock::new(dur));
    }

    pub fn check(&self, key: &str) -> bool {
        if let Some(locker) = self.lockers.lock().unwrap().get(key) {
            if locker.check() {
                return true;
            }
        }
        false
    }

    pub fn remove(&mut self, key: &str) {
        self.lockers.lock().unwrap().remove(key);
    }

    // fn new_auto_cleaner(&mut self) {
    //     let lockers = self.lockers.clone();
    //     thread::spawn(move || loop {
    //         thread::sleep(Duration::from_secs(10));
    //         // 用迭代器遍历self.lockers，如果有过期的timer则删除
    //         lockers.lock().unwrap().retain(|_, v| !v.check());
    //     });
    // }
}

pub struct TimeLock {
    dur: Duration,
    setup_time: Instant,
}

impl TimeLock {
    #[inline]
    pub fn new(dur: Duration) -> Self {
        TimeLock {
            dur,
            setup_time: Instant::now(),
        }
    }

    /// check if the time lock is working
    #[inline]
    pub fn check(&self) -> bool {
        self.setup_time.elapsed() > self.dur
    }
}
