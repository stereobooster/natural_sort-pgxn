#![feature(libc, custom_attribute, plugin)]
#![plugin(postgres_extension_macros)]

extern crate postgres_extension;

#[macro_use]
extern crate postgres_extension_macros;

use postgres_extension::{VarChar};

extern crate libc;
use libc::{c_int};

use std::cmp::Ordering;
extern crate natord;
use std::mem;

pg_module!(version: 90400);

#[pg_export]
pub fn natural_compare(a: VarChar, b: VarChar) -> c_int {
  unsafe {
    let a_str = mem::transmute::<VarChar, &str>(a);
    let b_str = mem::transmute::<VarChar, &str>(b);
    let res = natord::compare(&*a_str, &*b_str);

    if res == Ordering::Greater {
      return 1;
    } else if res == Ordering::Less {
      return -1;
    } else {
      return 0;
    }
  }
}
