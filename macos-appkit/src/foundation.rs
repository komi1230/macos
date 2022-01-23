#[cfg(target_pointer_width = "64")]
pub type NSInteger = i64;
#[cfg(target_pointer_width = "32")]
pub type NSInteger = i32;
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = u64;
#[cfg(target_pointer_width = "32")]
pub type NSUInteger = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NSRange {
    pub location: NSUInteger,
    pub length: NSUInteger,
}

impl NSRange {
    #[inline]
    pub fn new(location: NSUInteger, length: NSUInteger) -> NSRange {
        NSRange { location, length }
    }
}
