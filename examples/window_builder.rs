use macos_app_builder::foundation::*;
use macos_app_builder::window::*;

fn main() {
    let color = color::Color::new(0.0, 0.0, 1.0, 0.7);
    let ns_rect = NSRect {
        origin: NSPoint { x: 0.0, y: 0.0 },
        size: NSSize {
            width: 800.0,
            height: 600.0,
        },
    };

    WindowBuilder::new()
        .rectangle(ns_rect)
        .background_color(color)
        .build()
        .run();
}
