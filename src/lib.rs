use std::ffi::CString;

extern "C" {
    fn c_init(process_name: *const i8) -> bool;
    fn c_get_module_list(process_name: *const i8) -> *const i8;
    fn c_read(address: u64, buffer: *mut u8, size: usize) -> bool;
    fn c_write(address: u64, buffer: *const u8, size: usize) -> bool;
    fn c_get_base_address(module_name: *const i8) -> u64;
    fn c_get_base_size(module_name: *const i8) -> u64;
    fn c_init_keyboard() -> bool;
    fn c_is_key_down(key: u32) -> bool;
    fn c_get_heap_regions( out_heaps: *mut HeapRegion, max_heaps: usize, out_count: *mut usize,) -> bool;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HeapRegion {
    pub va_start: usize,
    pub va_end: usize,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
} 

pub fn init(process_name: &str) -> bool {
    let process_name_c = CString::new(process_name).unwrap();
    unsafe {
        c_init(process_name_c.as_ptr())
    }
}

pub fn baseaddy(module_name: &str) -> u64 {
    let module_name_c = CString::new(module_name).unwrap();
    unsafe {
        c_get_base_address(module_name_c.as_ptr())
    }
}

pub fn basesize(module_name: &str) -> u64 {
    let module_name_c = CString::new(module_name).unwrap();
    unsafe {
        c_get_base_size(module_name_c.as_ptr())
    }
} 

pub fn modlist(process_name: &str) -> Vec<String> {
    let process_name_c = CString::new(process_name).unwrap();
    unsafe {
        let ptr = c_get_module_list(process_name_c.as_ptr());
        let mut modules = Vec::new();
        let mut current_ptr = ptr;
        
        loop {
            if *current_ptr == 0 {
                break;
            }
            
            let module_name = std::ffi::CStr::from_ptr(current_ptr).to_string_lossy().into_owned();
            modules.push(module_name);
            
            while *current_ptr != 0 {
                current_ptr = current_ptr.add(1);
            }
            current_ptr = current_ptr.add(1);
        }
        
        modules
    }
}

pub fn read(address: u64, buffer: &mut [u8]) -> bool {
    unsafe {
        c_read(address, buffer.as_mut_ptr(), buffer.len())
    }
}

pub fn read_float(address: u64) -> f32 {
    let mut buffer = [0u8; 4];
    if read(address, &mut buffer) {
        f32::from_le_bytes(buffer)
    } else {
        0.0
    }
}   

pub fn read_u64(address: u64) -> u64 {
    let mut buffer = [0u8; 8];
    if read(address, &mut buffer) {
        u64::from_le_bytes(buffer)
    } else {
        0
    }
}

pub fn read_i32(address: u64) -> i32 {
    let mut buffer = [0u8; 4];
    if read(address, &mut buffer) {
        i32::from_le_bytes(buffer)
    } else {
        0
    }
}

pub fn read_bool(address: u64) -> bool {
    let mut buffer = [0u8; 1];
    if read(address, &mut buffer) {
        buffer[0] != 0
    } else {
        false
    }
}

pub fn read_vector2(address: u64) -> Vec2 {
    let mut buffer = [0u8; 8];
    if read(address, &mut buffer) {
        Vec2 {
            x: f32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]),
            y: f32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
        }
    } else {
        Vec2::default()
    }
}

pub fn read_vector3(address: u64) -> Vec3 {
    let mut buffer = [0u8; 12];
    if read(address, &mut buffer) {
        Vec3 {
            x: f32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]),
            y: f32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
            z: f32::from_le_bytes([buffer[8], buffer[9], buffer[10], buffer[11]]),
        }
    } else {
        Vec3::default()
    }
}

pub fn write(address: u64, buffer: &[u8]) -> bool {
    unsafe {
        c_write(address, buffer.as_ptr(), buffer.len())
    }
}

pub fn write_u64(address: u64, value: u64) -> bool {
    let bytes = value.to_le_bytes();
    write(address, &bytes)
}

pub fn write_i32(address: u64, value: i32) -> bool {
    let bytes = value.to_le_bytes();
    write(address, &bytes)
}

pub fn write_bool(address: u64, value: bool) -> bool {
    let byte = [if value { 1u8 } else { 0u8 }];
    write(address, &byte)
}

pub fn write_vector2(address: u64, value: Vec2) -> bool {
    let mut buffer = [0u8; 8];
    buffer[0..4].copy_from_slice(&value.x.to_le_bytes());
    buffer[4..8].copy_from_slice(&value.y.to_le_bytes());
    write(address, &buffer)
}

pub fn write_float(address: u64, value: f32) -> bool {
    let bytes = value.to_le_bytes();
    write(address, &bytes)
}

pub fn write_vector3(address: u64, value: Vec3) -> bool {
    let mut buffer = [0u8; 12];
    buffer[0..4].copy_from_slice(&value.x.to_le_bytes());
    buffer[4..8].copy_from_slice(&value.y.to_le_bytes());
    buffer[8..12].copy_from_slice(&value.z.to_le_bytes());
    write(address, &buffer)
}

pub fn init_keyboard() -> bool {
    unsafe {
        c_init_keyboard()
    }
}

pub fn is_key_down(key_code: i32) -> bool {
    unsafe {
        c_is_key_down(key_code as u32)
    }
}

pub fn heap_regions() -> Vec<HeapRegion> {
    unsafe {
        let mut count: usize = 0;
        if !c_get_heap_regions(std::ptr::null_mut(), 0, &mut count) || count == 0 {
            return Vec::new();
        }
        let mut v: Vec<HeapRegion> = Vec::with_capacity(count);
        if c_get_heap_regions(v.as_mut_ptr(), count, &mut count) {
            v.set_len(count);
            v
        } else {
            Vec::new()
        }
    }
}

pub fn parse(signature: &str) -> (Vec<u8>, Vec<bool>) {
    let mut bytes = Vec::new();
    let mut mask = Vec::new();
    
    for part in signature.split_whitespace() {
        if part == "?" || part == "??" {
            bytes.push(0);
            mask.push(false);
        } else {
            if let Ok(byte) = u8::from_str_radix(part, 16) {
                bytes.push(byte);
                mask.push(true);
            }
        }
    }
        
    (bytes, mask)
}

pub fn sigscan(signature: &str, start: u64, end: u64) -> u64 {
   let (bytes, mask) = parse(signature);
        
    if bytes.is_empty() {
        return 0;
    }
        
    let size = bytes.len();
    let chunk = 0x100000;
    let mut buffer = vec![0u8; chunk];
        
    for current in (start..end).step_by(chunk - size + 1) {
        let q = std::cmp::min(chunk, (end - current) as usize);
            
        if q < size {
            break;
        }

        if !read(current, &mut buffer[..q]) {
            continue;
        }
            
        for i in 0..=q - size {
            let mut found = true;
                
        for j in 0..size {
            if mask[j] && buffer[i + j] != bytes[j] {
                found = false;
                    break;
                }
            }
                
            if found {
                return current + i as u64;

            }
        }
    }
    0
}