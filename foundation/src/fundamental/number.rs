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

/// When building 32-bit applications, NSInteger is a 32-bit integer. A 64-bit application treats NSInteger as a 64-bit integer.
#[cfg(target_pointer_width = "32")]
pub type NSInteger = libc::c_int;
#[cfg(target_pointer_width = "64")]
pub type NSInteger = libc::c_long;

/// When building 32-bit applications, NSUInteger is a 32-bit unsigned integer. A 64-bit application treats NSUInteger as a 64-bit unsigned integer
#[cfg(target_pointer_width = "32")]
pub type NSUInteger = libc::c_uint;
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = libc::c_ulong;

pub const NSIntegerMax: NSInteger = NSInteger::max_value();
pub const NSIntegerMin: i64 = NSInteger::min_value();

pub const NSNotFound: NSInteger = NSIntegerMax;

#[cfg(target_pointer_width = "32")]
pub type NSFloat = libc::c_float;
#[cfg(target_pointer_width = "64")]
pub type NSFloat = libc::c_double;

#[repr(C)]
pub struct NSPoint {
    pub x: NSFloat,
    pub y: NSFloat,
}

#[repr(C)]
pub struct NSSize {
    pub width: NSFloat,
    pub height: NSFloat,
}

#[repr(C)]
pub struct NSRect {
    pub origin: NSPoint,
    pub size: NSSize,
}
