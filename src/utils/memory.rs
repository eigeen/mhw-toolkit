use std::{ffi::c_void, slice};

use thiserror::Error;
use windows::Win32::System::Memory::{
    VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS,
};

const PATTERN_WILDCARD: u8 = 0xFF;

#[derive(Debug, Error)]
pub enum PatternScanError {
    #[error("pattern not found")]
    NotFound,
    #[error("more than one pattern found, expected exactly one")]
    MultipleMatchesFound,
    #[error("invalid pattern format: {0}")]
    Format(String),
}

pub struct PatternScan;

impl PatternScan {
    pub fn search(text: &[u8], pattern: &[u8], wildcard: u8) -> Vec<usize> {
        boyer_moore_search_all(text, pattern, wildcard)
    }

    /// 扫描内存，查找匹配的第一个地址
    pub fn scan_first(pattern: &[u8]) -> Result<u64, PatternScanError> {
        for now_ptr in (0x140000000_u64..0x143000000_u64).step_by(0x1000000) {
            let part = unsafe { slice::from_raw_parts(now_ptr as *const u8, 0x1000100) };
            let matches: Option<usize> = boyer_moore_search_first(part, pattern, PATTERN_WILDCARD);
            if let Some(matches) = matches {
                let real_ptr = now_ptr + matches as u64;
                return Ok(real_ptr);
            }
        }

        Err(PatternScanError::NotFound)
    }

    /// 扫描内存，查找匹配的所有地址
    pub fn scan_all(pattern: &[u8]) -> Result<Vec<u64>, PatternScanError> {
        let mut result = Vec::new();
        for now_ptr in (0x140000000_u64..0x143000000_u64).step_by(0x1000000) {
            let part = unsafe { slice::from_raw_parts(now_ptr as *const u8, 0x1000100) };
            let matches = boyer_moore_search_all(part, pattern, PATTERN_WILDCARD);
            if !matches.is_empty() {
                matches
                    .into_iter()
                    .for_each(|x| result.push(x as u64 + now_ptr));
            }
        }
        if result.is_empty() {
            Err(PatternScanError::NotFound)
        } else {
            Ok(result)
        }
    }

    /// 扫描内存，查找匹配的地址，如果有且仅有一个，则返回地址，否则返回错误
    pub fn safe_scan(pattern: &[u8]) -> Result<u64, PatternScanError> {
        let mut result = Vec::new();
        for now_ptr in (0x140000000_u64..0x143000000_u64).step_by(0x1000000) {
            let part = unsafe { slice::from_raw_parts(now_ptr as *const u8, 0x1000100) };
            let matches = boyer_moore_search_all(part, pattern, PATTERN_WILDCARD);
            if !matches.is_empty() {
                matches
                    .into_iter()
                    .for_each(|x| result.push(x as u64 + now_ptr));
            }
        }
        match result.len() {
            0 => Err(PatternScanError::NotFound),
            1 => Ok(result[0]),
            _ => Err(PatternScanError::MultipleMatchesFound),
        }
    }
}

pub unsafe fn patch(position: *const c_void, bytes: &[u8]) -> Result<(), String> {
    let patch_len = bytes.len();
    let dwsize = (patch_len / 4096 + 1) * 4096;
    let mut old_protection = PAGE_PROTECTION_FLAGS::default();
    VirtualProtect(
        position,
        dwsize,
        PAGE_EXECUTE_READWRITE,
        &mut old_protection,
    )
    .map_err(|e| e.to_string())?;

    let patch_ptr = position as *mut u8;
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), patch_ptr, patch_len);

    let mut _temp = PAGE_PROTECTION_FLAGS::default();
    VirtualProtect(position, dwsize, old_protection, &mut _temp).map_err(|e| e.to_string())?;

    Ok(())
}

pub unsafe fn patch_nop(position: *const c_void, length: usize) -> Result<(), String> {
    let nop_bytes = vec![0x90; length];
    patch(position, &nop_bytes)?;

    Ok(())
}

fn build_bad_character_table(pattern: &[u8], wildcard: u8) -> Vec<isize> {
    let mut table = vec![-1; 256];
    for (i, &byte) in pattern.iter().enumerate() {
        if byte != wildcard {
            // 忽略通配符
            table[byte as usize] = i as isize;
        }
    }
    table
}

pub fn boyer_moore_search_first(text: &[u8], pattern: &[u8], wildcard: u8) -> Option<usize> {
    let bct = build_bad_character_table(pattern, wildcard);
    let m = pattern.len();
    let n = text.len();
    let mut i = 0;

    while i <= n - m {
        let mut j = (m - 1) as isize;
        while j >= 0
            && (pattern[j as usize] == wildcard || pattern[j as usize] == text[i + j as usize])
        {
            j -= 1;
        }
        if j < 0 {
            return Some(i);
        } else {
            let bad_char_shift = bct[text[i + j as usize] as usize];
            i += std::cmp::max(1, j - bad_char_shift) as usize;
        }
    }
    None
}

pub fn boyer_moore_search_all(text: &[u8], pattern: &[u8], wildcard: u8) -> Vec<usize> {
    let bct = build_bad_character_table(pattern, wildcard);
    let mut matches = Vec::new();
    let m = pattern.len();
    let n = text.len();
    let mut i = 0;

    while i <= n - m {
        let mut j = (m - 1) as isize;
        while j >= 0
            && (pattern[j as usize] == wildcard || pattern[j as usize] == text[i + j as usize])
        {
            j -= 1;
        }
        if j < 0 {
            matches.push(i);
            i += 1;
        } else {
            let bad_char_shift = bct[text[i + j as usize] as usize];
            i += std::cmp::max(1, j - bad_char_shift) as usize;
        }
    }

    matches
}

/// 将特征码模板字符串转换为字节数组
///
/// 提示：如果你使用字符串字面量（即编译期确定的字符串常量 `"..."`）而不是动态的字符串，
/// 则建议使用过程宏 `hex_str_to_bytes!()` 来代替。
///
/// Input:
///
/// ```rust
/// "F3 48 0F 2A F0 85 ** 7E ** 49 8B ? ?? ** 00 00 ** C0 48 85 ** 74"
/// //       wildcards:      ^^       ^ ^^
/// //                       OK       OK OK
/// ```
///
/// Output:
///
/// ```rust
/// vec![0xF3, 0x48, 0x0F, 0x2A, 0xF0, 0x85, 0xFF, 0x7E, 0xFF, 0x49, 0x8B, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xC0, 0x48, 0x85, 0xFF, 0x74]
/// ```
///
/// 通配符：支持 `**`, `??`, `?`，通配符转换为 `0xFF`
pub fn space_hex_to_bytes(text_hex: &str) -> Result<Vec<u8>, PatternScanError> {
    text_hex
        .split_whitespace()
        .map(|byte_str| {
            if byte_str == "**" || byte_str == "??" || byte_str == "?" {
                Ok(0xFF_u8)
            } else {
                u8::from_str_radix(byte_str, 16)
            }
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| PatternScanError::Format(err.to_string()))
}

pub fn bytes_to_space_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

/// 相对地址计算
///
/// src: 调用者的原地址\
/// dst: 跳转或内存目标地址\
/// cmd_length: 调用者指令长度
pub fn relative_address(src: *const c_void, dst: *const c_void, cmd_length: usize) -> isize {
    let src_addr = src as usize;
    let dst_addr = dst as usize;

    dst_addr as isize - (src_addr as isize + cmd_length as isize)
}

#[cfg(test)]
mod tests {
    use address_scanner::hex_str_to_bytes;

    use super::*;

    #[test]
    fn test_space_hex_to_bytes() {
        assert!(hex_str_to_bytes!("03 4C 8B F2 48") == [0x03, 0x4C, 0x8B, 0xF2, 0x48]);
        assert!(
            hex_str_to_bytes!("03 4C 8B F2 48 ** 48") == [0x03, 0x4C, 0x8B, 0xF2, 0x48, 0xFF, 0x48]
        );
    }

    #[test]
    fn test_bm() {
        let text = hex_str_to_bytes!("03 4C 8B F2 48 8D 4A 70 45 33 C0 48 8D 15 5F 90 D7 03 E8 CA EC 06 01 49 8B CE E8 62 09 7C 00 89 84 24 C0 00 00 00 48 8B 7B 08 45 0F 57 D2 41 B9 01 00 00 00 4C 8B 87 B0 76 00 00 4D 85 C0 0F 84 CE 00 00 00 41 F6 40 0C 0E 0F 84 C3 00 00 00 41 8B 80 14 1D 00 00 0F 57 F6 49 63 90 5C 22 00 00 F3 48 0F 2A F0 85 D2 7E 5A 49 8B 88 70 1D 00 00 32 C0 48 85 C9 74 09 80 79 1C 08 0F 93 C0 EB 17 49 8B 88 78 1D 00 00 48 85 C9 74 0B 80 79 20 08 0F B6 C0 41 0F 43 C1 84 C0 48 8B 05 29 80 E5 03 74 0E 0F B6 8C 02 77 30 00 00 66 0F 6E C1 EB 0C 0F B6 94 02 53 30 00 00 66 0F 6E C2 0F 5B C0 F3 0F 58 F0 49 8B C8 E8 66 E4 F5 00 8B C0 0F 57 C0 F3 48 0F 2A C0 48 8B 43 08 48 8B 90 B0 76 00 00 F3 0F 58 F0 48 85 D2 74 06 F6 42 0C 0E 75 03 49 8B D7 B9 01 00 00 00 E8 D5 BF FA 00 48 8B 7B 08");
        let pattern =
            hex_str_to_bytes!("F3 48 0F 2A F0 85 ** 7E ** 49 8B ** ** ** 00 00 ** C0 48 85 ** 74");
        let wildcard = 0xFF;

        let matches = boyer_moore_search_all(&text, &pattern, wildcard);
        assert!(matches.len() == 1);
        for &match_pos in &matches {
            println!("Pattern found at position: {}", match_pos);
        }
    }

    #[test]
    fn test_bm2() {
        let text = hex_str_to_bytes!("45 33 C0 48 8D 81 08 10 00 00 48 8D 15 B7 FF AA 00 66 44 89 01 48 3B D0 74 0A 44 89 81 04 10 00 00 44 88 00");
        let pattern = hex_str_to_bytes!("81 08 10 00 00 48");
        let wildcard = 0xFF;

        let matches = boyer_moore_search_all(&text, &pattern, wildcard);
        assert!(matches.len() == 1);
        for &match_pos in &matches {
            println!("Pattern found at position: {}", match_pos);
        }
    }

    #[test]
    fn test_relative_address() {
        let addr = relative_address(0x109DA7FF as *const c_void, 0x073C99D0 as *const c_void, 5);
        eprintln!("{:X?}", addr.to_le_bytes());
    }
}
