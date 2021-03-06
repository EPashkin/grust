// This file is part of Grust, GObject introspection bindings for Rust
//
// Copyright (C) 2014  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA

use types::{gboolean,FALSE};

use std::ascii;
use std::ascii::AsciiExt;
use std::borrow::Cow;
use std::str;

#[inline]
pub fn is_true(v: gboolean) -> bool { v != FALSE }

#[inline]
pub fn is_false(v: gboolean) -> bool { v == FALSE }

pub fn escape_bytestring<'a>(s: &'a [u8]) -> Cow<'a, str> {
    if s.is_ascii() {
        let s = unsafe { str::from_utf8_unchecked(s) };
        return s.into();
    }
    let mut acc = Vec::with_capacity(s.len());
    acc.extend(s.iter().cloned().flat_map(ascii::escape_default));
    let string = unsafe { String::from_utf8_unchecked(acc) };
    string.into()
}
