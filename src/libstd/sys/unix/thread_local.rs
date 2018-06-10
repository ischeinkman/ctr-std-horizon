// Copyright 2014-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)] // not used on all platforms

use mem;
use libc;

pub type Key = libc::pthread_key_t;

#[inline]
pub unsafe fn create(dtor: Option<unsafe extern fn(*mut u8)>) -> Key {
    0
}

#[inline]
pub unsafe fn set(key: Key, value: *mut u8) {
    ()
}

#[inline]
pub unsafe fn get(key: Key) -> *mut u8 {
    0 as *mut u8
}

#[inline]
pub unsafe fn destroy(key: Key) {
    ()
}

#[inline]
pub fn requires_synchronized_create() -> bool {
    false
}
