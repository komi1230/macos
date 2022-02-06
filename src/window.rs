// Copyright 2022 Yusuke Kominami
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod color;

use crate::foundation::{Id, NSRect};
use crate::window::color::Color;

use objc::*;

pub struct Window {
    _object: Id,
    //    pub title: String,
    pub rectangle: NSRect,
    pub background_color: Color,
}

impl Window {
    pub fn run(self) {
        let app: *mut runtime::Object =
            unsafe { msg_send![class!(NSApplication), sharedApplication] };
        let _: *mut runtime::Object = unsafe { msg_send![app, run] };
    }
}

pub struct WindowBuilder<Rectangle, BackgroundColor> {
    _object: Id,
    // pub title: Title,
    pub rectangle: Rectangle,
    pub background_color: BackgroundColor,
}

impl WindowBuilder<(), ()> {
    pub fn new() -> Self {
        let object: Id = unsafe { msg_send![class!(NSWindow), alloc] };
        let object: Id = unsafe { msg_send![object, init] };
        let object: Id = unsafe { msg_send![object, autorelease] };
        WindowBuilder {
            _object: object,
            //  title: (),
            rectangle: (),
            background_color: (),
        }
    }
}

impl WindowBuilder<NSRect, Color> {
    pub fn build(self) -> Window {
        let _: Id = unsafe { msg_send![self._object, makeKeyAndOrderFront: self._object] };
        Window {
            _object: self._object,
            rectangle: self.rectangle,
            background_color: self.background_color,
        }
    }
}

impl<Rectangle, BackgroundColor> WindowBuilder<Rectangle, BackgroundColor> {
    // pub fn title(self, title: String) -> WindowBuilder<String, Rectangle, BackgroundColor> {
    //     WindowBuilder {
    //         _object: self._object,
    //         // title,
    //         rectangle: self.rectangle,
    //         background_color: self.background_color,
    //     }
    // }

    pub fn background_color(self, background_color: Color) -> WindowBuilder<Rectangle, Color> {
        let _: Id =
            unsafe { msg_send![self._object, setBackgroundColor: background_color._object] };
        WindowBuilder {
            _object: self._object,
            // title: self.title,
            rectangle: self.rectangle,
            background_color,
        }
    }

    pub fn rectangle(self, rectangle: NSRect) -> WindowBuilder<NSRect, BackgroundColor> {
        let object: Id = unsafe {
            msg_send![self._object,
                      initWithContentRect: rectangle.clone()
                      styleMask: 15
                      backing: 1
                      defer: 1
            ]
        };
        WindowBuilder {
            _object: object,
            rectangle,
            background_color: self.background_color,
        }
    }
}
