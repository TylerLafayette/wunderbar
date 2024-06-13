use libc::c_void;

use crate::ui::CGError;

use super::{CGRect, CGResult};

/// A reference to a CGSRegion object. Must be freed using [`CFRelease`] after use.
pub type CGSRegionRef = *const c_void;

/// An untyped "generic" reference to any Core Foundation object.
///
/// The CFTypeRef type is the base type defined in Core Foundation. It is used as
/// the type and return value in several polymorphic functions. It is a generic
/// object reference that acts as a placeholder for other true Core Foundation
/// objects.
pub type CFTypeRef = *const c_void;

pub type CFDictionaryRef = *const c_void;

#[repr(C)]
#[allow(non_snake_case)]
pub struct CFDictionaryKeyCallBacks {
    pub version: isize,
    pub retain: extern "C" fn(_: *const c_void, _: *const c_void) -> *const c_void,
    pub release: extern "C" fn(_: *const c_void, _: *const c_void),
    pub copyDescription: extern "C" fn(_: *const c_void) -> *const c_void,
    pub equal: extern "C" fn(_: *const c_void, _: *const c_void) -> u8,
    pub hash: extern "C" fn(_: *const c_void) -> usize,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct CFDictionaryValueCallBacks {
    pub version: isize,
    pub retain: extern "C" fn(_: *const c_void, _: *const c_void) -> *const c_void,
    pub release: extern "C" fn(_: *const c_void, _: *const c_void),
    pub copyDescription: extern "C" fn(_: *const c_void) -> *const c_void,
    pub equal: extern "C" fn(_: *const c_void, _: *const c_void) -> u8,
}

#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    /// Allocates a new CGSRegion using the given [`CGRect`]'s bounds, setting the `out`
    /// parameter to point to the newly created CGSRegion on success.
    pub fn CGSNewRegionWithRect(rect: *const CGRect, out: *mut CGSRegionRef) -> i32;

    /// Runs the current thread’s CFRunLoop object in its default mode indefinitely.
    ///
    /// The current thread’s run loop runs in the default mode (see Default Run Loop Mode) until
    /// the run loop is stopped with CFRunLoopStop or all the sources and timers are removed from
    /// the default run loop mode.
    ///
    /// Run loops can be run recursively. You can call CFRunLoopRun from within any run loop
    /// callout and create nested run loop activations on the current thread’s call stack.
    pub fn CFRunLoopRun() -> c_void;

    /// Releases a Core Foundation object.
    ///
    /// If the retain count of cf becomes zero the memory allocated to the object
    /// is deallocated and the object is destroyed. If you create, copy, or
    /// explicitly retain (see the CFRetain function) a Core Foundation object,
    /// you are responsible for releasing it when you no longer need it (see
    /// Memory Management Programming Guide for Core Foundation).
    ///
    /// If cf is NULL, this will cause a runtime error and your application will crash.
    pub fn CFRelease(object: CFTypeRef);

    pub static kCFTypeDictionaryKeyCallBacks: CFDictionaryKeyCallBacks;
    pub static kCFTypeDictionaryValueCallBacks: CFDictionaryValueCallBacks;

    pub fn CFDictionaryCreate(
        allocator: *const c_void,
        keys: *const *const c_void,
        values: *const *const c_void,
        num_values: u64,
        key_call_backs: *const CFDictionaryKeyCallBacks,
        value_call_backs: *const CFDictionaryValueCallBacks,
    ) -> CFDictionaryRef;

    pub fn CFNumberCreate(
        allocator: *const c_void,
        the_type: usize,
        value_ptr: *const c_void,
    ) -> *const c_void;

    pub fn CFStringCreateWithBytes(
        alloc: *const c_void,
        bytes: *const u8,
        numBytes: u64,
        encoding: u32,
        isExternalRepresentation: bool,
    ) -> *const c_void;

    pub fn CFDictionaryContainsKey(theDict: *const c_void, key: *const c_void) -> bool;
}

#[allow(non_upper_case_globals)]
const kCFStringEncodingUTF8: u32 = 0x08000100;

#[derive(Debug)]
pub struct CFDictionary {
    inner: CFDictionaryRef,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CFValue {
    Index(usize),
    F32(f32),
    F64(f64),
    I32(i32),
    I64(i64),
    String(String),
}

struct CFValueRef {
    inner: *const c_void,
}

impl CFValueRef {
    fn from_cf_value(value: CFValue) -> CGResult<Self> {
        let ptr = unsafe {
            match value {
                CFValue::Index(val) => {
                    CFNumberCreate(std::ptr::null(), 14, std::mem::transmute(&val))
                }
                CFValue::I32(val) => CFNumberCreate(std::ptr::null(), 3, std::mem::transmute(&val)),
                CFValue::I64(val) => CFNumberCreate(std::ptr::null(), 4, std::mem::transmute(&val)),
                CFValue::F32(val) => CFNumberCreate(std::ptr::null(), 5, std::mem::transmute(&val)),
                CFValue::F64(val) => CFNumberCreate(std::ptr::null(), 6, std::mem::transmute(&val)),
                CFValue::String(str) => {
                    let bytes = str.into_bytes();
                    CFStringCreateWithBytes(
                        std::ptr::null(),
                        &bytes[0] as *const _,
                        bytes.len() as u64,
                        kCFStringEncodingUTF8,
                        false,
                    )
                }
            }
        };
        if ptr.is_null() {
            return Err(CGError::Failure);
        }

        Ok(Self { inner: ptr })
    }

    unsafe fn as_ptr(&self) -> *const c_void {
        self.inner
    }
}

impl Drop for CFValueRef {
    fn drop(&mut self) {
        println!("releasing cfvalue");
        unsafe { CFRelease(self.inner) }
    }
}

impl CFDictionary {
    pub fn new_from_entries<const N: usize>(entries: [(CFValue, CFValue); N]) -> CGResult<Self> {
        let len = entries.len();
        let mut keys = Vec::with_capacity(len);
        let mut values = Vec::with_capacity(len);
        for (key, value) in entries {
            keys.push(CFValueRef::from_cf_value(key)?);
            values.push(CFValueRef::from_cf_value(value)?);
        }
        let key_ptrs: Vec<_> = keys.iter().map(|k| unsafe { k.as_ptr() }).collect();
        let value_ptrs: Vec<_> = values.iter().map(|v| unsafe { v.as_ptr() }).collect();

        let ptr = unsafe {
            CFDictionaryCreate(
                std::ptr::null(),
                &key_ptrs[0],
                &value_ptrs[0],
                len as u64,
                &kCFTypeDictionaryKeyCallBacks,
                &kCFTypeDictionaryValueCallBacks,
            )
        };

        Ok(Self { inner: ptr })
    }

    pub fn contains_key(&mut self, key: CFValue) -> CGResult<bool> {
        let val = CFValueRef::from_cf_value(key)?;

        unsafe {
            let ptr = val.as_ptr();
            Ok(CFDictionaryContainsKey(self.inner, ptr))
        }
    }

    pub unsafe fn as_ptr(&self) -> *const c_void {
        self.inner
    }
}

impl Drop for CFDictionary {
    fn drop(&mut self) {
        unsafe { CFRelease(self.inner) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dictionary_basic() {
        let key = CFValue::String("test".to_string());
        let value = CFValue::Index(64);
        let mut dict = CFDictionary::new_from_entries([(key.clone(), value)])
            .expect("dictionary creation should not fail");

        assert!(dict
            .contains_key(key)
            .expect("`contains_key` operation should not fail"));

        assert!(!dict
            .contains_key(CFValue::String("test2".to_string()))
            .expect("`contains_key` operation should not fail"));
    }
}
