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

use objc::*;

pub type Id = *mut runtime::Object;

/// When building 32-bit applications, NSInteger is a 32-bit integer. A 64-bit application treats NSInteger as a 64-bit integer.
#[cfg(target_pointer_width = "32")]
pub type NSInteger = i32;
#[cfg(target_pointer_width = "64")]
pub type NSInteger = i64;

/// When building 32-bit applications, NSUInteger is a 32-bit unsigned integer. A 64-bit application treats NSUInteger as a 64-bit unsigned integer
#[cfg(target_pointer_width = "32")]
pub type NSUInteger = u32;
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = u64;

pub const NS_INTEGER_MAX: NSInteger = NSInteger::max_value();
pub const NS_INTEGER_MIN: i64 = NSInteger::min_value();

pub const NS_NOT_FOUND: NSInteger = NS_INTEGER_MAX;

#[cfg(target_pointer_width = "32")]
pub type NSFloat = f32;
#[cfg(target_pointer_width = "64")]
pub type NSFloat = f64;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct NSPoint {
    pub x: NSFloat,
    pub y: NSFloat,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct NSSize {
    pub width: NSFloat,
    pub height: NSFloat,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct NSRect {
    pub origin: NSPoint,
    pub size: NSSize,
}
