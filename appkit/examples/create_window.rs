use objc::*;

use macos_foundation::fundamental::*;

#[link(name = "Foundation", kind = "framework")]
#[link(name = "AppKit", kind = "framework")]
extern "C" {}

fn main() {
    let alloc_window: *mut runtime::Object = unsafe { msg_send![class!(NSWindow), alloc] };
    let window: *mut runtime::Object = unsafe { msg_send![alloc_window, init] };
    println!("hello");
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

    let window_release: *mut runtime::Object = unsafe { msg_send![ns_window, autorelease] };

    let blue_color: *mut runtime::Object = unsafe { msg_send![class!(NSColor), whiteColor] };

    let set_bgc: *mut runtime::Object =
        unsafe { msg_send![ns_window, setBackgroundColor: blue_color] };

    let _: *mut runtime::Object =
        unsafe { msg_send![window_release, makeKeyAndOrderFront: set_bgc] };

    let app: *mut runtime::Object = unsafe { msg_send![class!(NSApplication), sharedApplication] };
    let _: *mut runtime::Object = unsafe { msg_send![app, run] };
}
