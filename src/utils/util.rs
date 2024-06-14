use std::{
    collections::HashMap,
    ptr,
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

/// 裸指针包装对象 \
/// 对多级偏移缓存以优化使用和效率
///
/// ## 注意
///
/// 该对象设计为只读，`offset`和`offsets`都会更改原对象数据，
/// 建议仅在初始化时使用方法
///
/// ## 安全性
///
/// 由于裸指针天然的不安全性，除非确保只有一个主线程，
/// 或者你知道你在做什么，否则强烈建议使用Arc+Mutex包装
///
/// ## Future
///
/// 未来会包装为 RawPtr(readonly) + RawPtrBuilder 的构造器模式
///
/// ## Example
///
/// Mostly used in static block.
///
/// ```rust
/// // single thread or controlled multi-thread
/// const BASE: *const i32 = 0x145011760 as *const i32;
/// static RAWPTR: Lazy<RawPtr<i32>> = Lazy::new(|| {
///     RawPtr::new(BASE).offsets(&[0x60, 0x8, 0x170, 0x58, 0x8])
/// });
///
/// // recommended for multi-thread
/// const BASE: *const i32 = 0x145011760 as *const i32;
/// static RAWPTR: Lazy<Arc<Mutex<RawPtr<i32>>>> = Lazy::new(|| {
///     Arc::new(Mutex::new(RawPtr::new(BASE).offsets(&[0x60, 0x8, 0x170, 0x58, 0x8])))
/// });
/// ```
pub struct RawPtr<T>
where
    T: Copy + Send + Sync,
{
    base_addr: *const T,
    offsets: Vec<isize>,
    offset_ptr: *mut T,
}

impl<T> RawPtr<T>
where
    T: Copy + Send + Sync,
{
    pub fn new(ptr: *const T) -> Self {
        RawPtr {
            base_addr: ptr,
            offsets: Vec::new(),
            offset_ptr: ptr::null_mut(),
        }
    }

    pub fn offset(mut self, offset: isize) -> Self {
        if !self.offset_ptr.is_null() {
            self.offset_ptr = ptr::null_mut();
        }
        self.offsets.push(offset);
        self
    }

    pub fn offsets(mut self, offsets: &[isize]) -> Self {
        if !self.offset_ptr.is_null() {
            self.offset_ptr = ptr::null_mut();
        }
        self.offsets.extend_from_slice(offsets);
        self
    }

    pub fn get_ptr(&mut self) -> Option<*const T> {
        if !self.offset_ptr.is_null() {
            return Some(self.offset_ptr);
        }
        match get_ptr_with_offset(self.base_addr, &self.offsets) {
            Some(addr) => {
                self.offset_ptr = addr as *mut T;
                Some(addr)
            }
            None => None,
        }
    }

    pub fn get_ptr_mut(&mut self) -> Option<*mut T> {
        if !self.offset_ptr.is_null() {
            return Some(self.offset_ptr);
        }
        match get_ptr_with_offset(self.base_addr, &self.offsets) {
            Some(addr) => {
                self.offset_ptr = addr as *mut T;
                Some(self.offset_ptr)
            }
            None => None,
        }
    }

    pub fn get_value(&mut self) -> Option<T> {
        if self.offset_ptr.is_null() {
            self.get_ptr().map(|ptr| unsafe { *ptr })
        } else {
            unsafe { Some(*self.offset_ptr) }
        }
    }
}

impl<T> Clone for RawPtr<T>
where
    T: Copy + Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            base_addr: self.base_addr,
            offsets: self.offsets.clone(),
            offset_ptr: self.offset_ptr,
        }
    }
}
unsafe impl<T> Send for RawPtr<T> where T: Copy + Send + Sync {}
unsafe impl<T> Sync for RawPtr<T> where T: Copy + Send + Sync {}

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
