// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test for issue #50381: non-lifetime passed to :lifetime.

#![feature(macro_lifetime_matcher)]

macro_rules! m { ($x:lifetime) => { } }

fn main() {
    m!(a);
    //~^ ERROR no rules expected the token `a`
}