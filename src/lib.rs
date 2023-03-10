#![feature(vec_into_raw_parts)]
//#![feature(vec_from_raw_parts)]
use std::os::raw::c_char;
use std::ffi::CString;

#[repr(C)]
pub struct c_string_vec {
    ptr: *mut *mut c_char,
    len: u64,
    cap: u64,
}

// TODO:" bounds checking
unsafe fn write_string_to_buf(s: &str, buf: *mut c_char) {
    //println!("attempting to write {}", s);
    let bytes = s.as_bytes();
    std::ptr::copy(bytes.as_ptr().cast(), buf, bytes.len());
    std::ptr::write(buf.offset(bytes.len() as isize) as *mut u8, 0u8);
    //println!("Done writing {}", s);
}

// read the first name from file
#[no_mangle]
pub extern "C" fn read_name() -> *const c_char {
    let contents = std::fs::read_to_string("names.txt").unwrap();
    let first = contents.split("\n").next().unwrap().to_string();
    let c_string = CString::new(first).unwrap();
    c_string.into_raw()
}

// NOTE: you may want to pass size in so rust doesnt overflow the buffer
#[no_mangle]
pub extern "C" fn read_name_to_buf(buf: *mut c_char) { 
    let contents = std::fs::read_to_string("names.txt").unwrap();
    let first = contents.split("\n").next().unwrap().to_string();
    unsafe {
        write_string_to_buf(&first, buf);
    }
}

#[no_mangle]
pub extern "C" fn read_names_to_bufs(bufs: *mut *mut c_char) -> u64 { 
    let contents = std::fs::read_to_string("names.txt").unwrap();
    let strs: Vec<&str> = contents.split("\n").collect();
    let mut i: u64 = 0;
    for s in strs {
        unsafe {
            write_string_to_buf(s, *(bufs.offset(i as isize)));
        }
        i += 1;
    }
    i
}


#[no_mangle]
pub extern "C" fn allocate_names() -> c_string_vec {
    let mut c_string_ptrs: Vec<*mut c_char> = Vec::new();
    let contents = std::fs::read_to_string("names.txt").unwrap();
    let strs: Vec<&str> = contents.split("\n").collect();
    for s in strs {
        let c_string = CString::new(s).unwrap();
        let raw = c_string.into_raw();
        c_string_ptrs.push(raw);
    }
    let (ptr, len, cap) = c_string_ptrs.into_raw_parts();
    c_string_vec {
        ptr: ptr,
        len: len as u64,
        cap: cap as u64,
    }
}

#[no_mangle]
pub extern "C" fn free_names(to_free: c_string_vec) {
    unsafe {
        let vec = Vec::from_raw_parts(to_free.ptr, to_free.len as usize, to_free.cap as usize);
        for s in vec {
            let _ = CString::from_raw(s);
        }
    }
}
