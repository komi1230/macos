use objc;

#[link(name = "AppKit", kind = "framework")]
extern "C" {}

#[cfg(test)]
mod tests {
    use super::*;
    use macos_foundation::fundamental::*;
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
                styleMask: 15
                    backing: 0
                    defer: 1
            ]
        };

        let app: *mut runtime::Object =
            unsafe { msg_send![class!(NSApplication), sharedApplication] };
        let _: *mut runtime::Object = unsafe { msg_send![app, run] };
    }
}
