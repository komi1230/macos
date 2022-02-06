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

use crate::foundation::{Id, NSFloat};

use objc::*;

pub struct Color {
    pub _object: Id,
    pub red: NSFloat,
    pub green: NSFloat,
    pub blue: NSFloat,
    pub alpha: NSFloat,
}

impl Color {
    pub fn new(red: NSFloat, green: NSFloat, blue: NSFloat, alpha: NSFloat) -> Self {
        let object = unsafe {
            msg_send![class!(NSColor), colorWithRed: red green: green blue: blue alpha: alpha]
        };

        Self {
            _object: object,
            red,
            green,
            blue,
            alpha,
        }
    }
}
