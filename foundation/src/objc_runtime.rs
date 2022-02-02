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

use crate::fundamental::number::NSInteger;

#[repr(C)]
pub struct OperatingSystemVersion {
    pub majorVersion: NSInteger,
    pub minor_version: NSInteger,
    pub patchVersion: NSInteger,
}

#[cfg(test)]
mod test {
    use super::*;
    use objc::runtime::Object;
    use objc::{class, msg_send, sel, sel_impl};

    #[test]
    fn os_version() {
        let cls: *mut Object = unsafe { msg_send![class!(NSProcessInfo), alloc] };
        let a: OperatingSystemVersion = unsafe { msg_send![cls, operatingSystemVersion] };
        println!("{}", a.minor_version);
    }
}
