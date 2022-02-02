use objc;

#[link(name = "AppKit", kind = "framework")]
extern "C" {}

#[repr(C)]
pub struct NSPoint {
    pub x: f64,
    pub y: f64,
}

#[repr(C)]
pub struct NSSize {
    pub width: f64,
    pub height: f64,
}

#[repr(C)]
pub struct NSRect {
    pub origin: NSPoint,
    pub size: NSSize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use objc::*;

    #[test]
    fn it_works() {
        let cls = class!(NSWindow);
        let window: *mut runtime::Object = unsafe { msg_send![cls, alloc] };
        let ns_window: *mut runtime::Object = unsafe {
            msg_send![
                window,
                initWithContentRect: NSRect {
                    origin: NSPoint { x: 0.0, y: 0.0 },
                    size: NSSize {width: 800.0, height: 600.0}
                }
                styleMask: 0
                    backing: 0
                    defer: 1
            ]
        };
    }
}
