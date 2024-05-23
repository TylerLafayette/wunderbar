use libc::c_void;

use super::{CGError, CGRect};

/// A reference to a CGSRegion object. Must be freed using [`CFRelease`] after use.
pub type CGSRegionRef = *const c_void;

/// An untyped "generic" reference to any Core Foundation object.
///
/// The CFTypeRef type is the base type defined in Core Foundation. It is used as
/// the type and return value in several polymorphic functions. It is a generic
/// object reference that acts as a placeholder for other true Core Foundation
/// objects.
pub type CFTypeRef = *const c_void;

#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    /// Allocates a new CGSRegion using the given [`CGRect`]'s bounds, setting the `out`
    /// parameter to point to the newly created CGSRegion on success.
    pub fn CGSNewRegionWithRect(rect: *const CGRect, out: *mut CGSRegionRef) -> CGError;

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
}
