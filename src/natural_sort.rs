#![feature(libc, custom_attribute, plugin)]
#![plugin(postgres_extension_macros)]

extern crate postgres_extension;

#[macro_use]
extern crate postgres_extension_macros;

use postgres_extension::{VarChar};

extern crate libc;
use libc::{c_int};

use std::str::FromStr;
use std::cmp::Ordering;
extern crate natord;

pg_module!(version: 90500);

#[pg_export]
pub fn naturel_compare(a: VarChar, b: VarChar) -> c_int {
  let a_str = a.p;
  let b_str = b.p;
  let res = natord::compare(a_str, b_str);

  if res == Ordering::Greater {
    return 1;
  } else if res == Ordering::Less {
    return -1;
  } else {
    return 0;
  }
}
