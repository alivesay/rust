// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Testing that method lookup automatically both borrows vectors to slices
// and also references them to create the &self pointer

trait MyIter {
    pure fn test_imm(&self);
    pure fn test_const(&const self);
}

impl &[int]: MyIter {
    pure fn test_imm(&self) { assert self[0] == 1 }
    pure fn test_const(&const self) { assert self[0] == 1 }
}

impl &str: MyIter {
    pure fn test_imm(&self) { assert *self == "test" }
    pure fn test_const(&const self) { assert *self == "test" }
}

fn main() {
    // NB: Associativity of ~, etc. in this context is surprising. These must be parenthesized

    ([1]).test_imm();
    (~[1]).test_imm();
    (@[1]).test_imm();
    (&[1]).test_imm();
    ("test").test_imm();
    (~"test").test_imm();
    (@"test").test_imm();
    (&"test").test_imm();

    // XXX: Other types of mutable vecs don't currently exist
    (@mut [1]).test_imm();

    ([1]).test_const();
    (~[1]).test_const();
    (@[1]).test_const();
    (&[1]).test_const();
    ("test").test_const();
    (~"test").test_const();
    (@"test").test_const();
    (&"test").test_const();

    (@mut [1]).test_const();

    // NB: We don't do this double autoreffing for &mut self because that would
    // allow creating a mutable pointer to a temporary, which would be a source
    // of confusion
}
