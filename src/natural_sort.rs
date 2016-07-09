#![feature(libc, custom_attribute, plugin)]
#![plugin(postgres_extension_macros)]

extern crate postgres_extension;

#[macro_use]
extern crate postgres_extension_macros;

extern crate libc;

use libc::{c_int};

pg_module!(version: 90500);

#[pg_export]
pub fn natural_sort(a: c_int) -> c_int {
  if a == 0 {
    return 1
  } else {
    return 0
  }
}
