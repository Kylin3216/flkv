use rusty_leveldb::{Options, DB, WriteBatch};
use std::os::raw::{c_char, c_uchar};
use std::ptr;
use std::ffi::{CStr};

/// Creates a [`&str`] from c_string.
/// - Creates a [`&str`] from c_string
/// ```
/// let c_string=CString::new("aaaa").unwrap();
/// let str = cstr!(c_string.as_ptr());
/// assert_eq!("aaaa",str);
/// ```
/// - Creates a [`&str`] from c_string with default
/// ```
/// let str = cstr!(std::ptr::null());
/// assert_eq!("default",str);
/// let str_with_default = cstr!(std::ptr::null(),"custom");
/// assert_eq!("custom",str_with_default);
/// ```
///
macro_rules! cstr {
    ($ptr:expr) => {
       cstr!($ptr,"default")
    };
    ($ptr:expr,$default:expr) => {
       if ($ptr as *const c_char).is_null(){
           $default
       } else{
           match unsafe { CStr::from_ptr($ptr as *const c_char).to_str() }{
            Ok(value) => value,
            Err(_) => $default
           }
       }
    };
}
/// get [`&[u8]`] from KvBuffer pointer.
macro_rules! buffer {
    ($ptr:expr) => {{
        unsafe {
            let key = *Box::from_raw($ptr as *mut KvBuffer);
            std::slice::from_raw_parts(key.data, key.length)
        }
    }};
}

/// get [`DB`] from FlKv pointer.
macro_rules! db {
    ($ptr:expr) => {{
       db!($ptr,false)
    }};
    ($ptr:expr,$rt:expr) => {{
      if ($ptr as *mut FlKv).is_null() {
        return $rt;
      } else {
       let kv= unsafe { &mut *$ptr };
       &mut kv.db
      }
    }};
}
/// get [`WriteBatch`] from FlKvBatch pointer.
macro_rules! wbr {
    ($ptr:expr) => {{
       wbr!($ptr,false)
    }};
    ($ptr:expr,$rt:expr) => {{
      if ($ptr as *mut FlKvBatch).is_null() {
        return $rt;
      } else {
       let kv = unsafe { Box::from_raw($ptr as *mut FlKvBatch) };
       kv.wb
      }
    }};
}
/// get  [`WriteBatch`]  refrence from FlKvBatch pointer.
macro_rules! wb {
    ($ptr:expr) => {{
       wb!($ptr,false)
    }};
    ($ptr:expr,$rt:expr) => {{
      if ($ptr as *mut FlKvBatch).is_null() {
        return $rt;
      } else {
       let kv = unsafe { &mut *$ptr };
       &mut kv.wb
      }
    }};
}


/// Array struct
#[repr(C)]
pub struct KvBuffer {
    data: *const c_uchar,
    length: usize,
}

impl KvBuffer {
    fn empty() -> *mut KvBuffer {
        Box::into_raw(Box::new(KvBuffer {
            data: Vec::new().as_ptr() as *const u8,
            length: 0,
        }))
    }
    fn from_vec(vec: Vec<u8>) -> *mut KvBuffer {
        let data = KvBuffer {
            data: vec.as_ptr() as *const u8,
            length: vec.len(),
        };
        std::mem::forget(vec);
        Box::into_raw(Box::new(data))
    }
}

/// keep db pointer
pub struct FlKv {
    db: DB,
}

/// keep writeBatch pointer
pub struct FlKvBatch {
    wb: WriteBatch,
}


#[no_mangle]
pub extern "C" fn db_open(name: *const c_char, memory: bool) -> *mut FlKv {
    let name = cstr!(name);
    match if memory {
        DB::open(name, rusty_leveldb::in_memory())
    } else {
        DB::open(name, Options::default())
    } {
        Ok(db) => Box::into_raw(Box::new(FlKv { db })),
        Err(e) => {
            println!("{:?}", e);
            ptr::null_mut()
        }
    }
}

// #[no_mangle]
// pub extern "C" fn db_check_kv(flkv: *mut FlKv) -> *const c_char {
//     let db = kv!(flkv);
//     match &db.error {
//         Some(e) => FlKvError::from_status(e),
//         None => FlKvError::empty()
//     }
// }

#[no_mangle]
pub extern "C" fn db_put(flkv: *mut FlKv, key: *mut KvBuffer, value: *mut KvBuffer) -> bool {
    let db = db!(flkv);
    match db.put(buffer!(key), buffer!(value)) {
        Ok(_) => true,
        Err(e) => {
            println!("{:?}", e);
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn db_create_batch() -> *mut FlKvBatch {
    Box::into_raw(Box::new(FlKvBatch {
        wb: WriteBatch::new()
    }))
}

#[no_mangle]
pub extern "C" fn batch_add_kv(batch: *mut FlKvBatch, key: *mut KvBuffer, value: *mut KvBuffer) -> bool {
    let wb = wb!(batch);
    wb.put(buffer!(key), buffer!(value));
    true
}

#[no_mangle]
pub extern "C" fn batch_clear(batch: *mut FlKvBatch) -> bool {
    let wb = wb!(batch);
    wb.clear();
    true
}

#[no_mangle]
pub extern "C" fn db_put_batch(flkv: *mut FlKv, batch: *mut FlKvBatch, sync: bool) -> bool {
    let wb = wbr!(batch);
    let db = db!(flkv);
    match db.write(wb, sync) {
        Ok(_) => true,
        Err(e) => {
            println!("{:?}", e);
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn db_get(flkv: *mut FlKv, key: *mut KvBuffer) -> *mut KvBuffer {
    let db = db!(flkv,ptr::null_mut());
    match db.get(buffer!(key)) {
        Some(data) => KvBuffer::from_vec(data),
        None => KvBuffer::empty()
    }
}

#[no_mangle]
pub extern "C" fn db_delete(flkv: *mut FlKv, key: *mut KvBuffer) -> bool {
    let db = db!(flkv);
    match db.delete(buffer!(key)) {
        Ok(_) => true,
        Err(e) => {
            println!("{:?}", e);
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn db_flush(flkv: *mut FlKv) -> bool {
    let db = db!(flkv);
    match db.flush() {
        Ok(_) => true,
        Err(e) => {
            println!("{:?}", e);
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn db_close(flkv: *mut FlKv) {
    if flkv.is_null() {
        return;
    }
    unsafe { Box::from_raw(flkv) };
}

#[cfg(test)]
mod tests {
    use std::ffi::{CString, CStr};
    use std::os::raw::c_char;
    use std::ptr;

    /// test macro cstr!
    #[test]
    fn test_cstr() {
        let c_string = CString::new("aaaa").unwrap();
        let str = cstr!(c_string.as_ptr());
        assert_eq!("aaaa", str);
        let str_with_default = cstr!(ptr::null());
        assert_eq!("default", str_with_default);
        let str_with_custom = cstr!(ptr::null(),"custom");
        assert_eq!("custom", str_with_custom);
    }
}
