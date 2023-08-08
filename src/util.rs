/*
Copyright Juniper Gardiner - MIT
Jul 24 2023
*/

use std::iter::{Enumerate, Rev};

use std::str::Chars;

/// Returns a number as a prettified string (e.g. 10000 to 10,000)
pub fn prettify_number(x: usize) -> String {
    let mut s: String = String::new();
    let i_str: String = x.to_string();
    let a: Enumerate<Rev<Chars>> = i_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            s.insert(0, ',');
        }
        s.insert(0, val);
    }
    s
}
