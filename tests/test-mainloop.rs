// This file is part of Grust, GObject introspection bindings for Rust
//
// Copyright (C) 2015  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
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

extern crate grust;

use grust::mainloop::{LoopRunner, Continue, Remove};

use std::thread;

#[test]
fn test_invoke_once() {
    let runner = LoopRunner::new();
    runner.run_after(|mainloop| {
        const THREAD_NAME: &'static str = "invoker";
        thread::Builder::new().name(THREAD_NAME.to_string()).spawn(move || {
            let mlc = mainloop.clone();
            let ctx = mainloop.get_context();
            ctx.invoke_once(move || {
                assert!(thread::current().name() != Some(THREAD_NAME));
                mlc.quit();
            });
        }).unwrap();
    });
}

#[test]
fn test_invoke() {
    let runner = LoopRunner::new();
    runner.run_after(|mainloop| {
        const THREAD_NAME: &'static str = "invoker";
        thread::Builder::new().name(THREAD_NAME.to_string()).spawn(move || {
            let mlc = mainloop.clone();
            let mut count = 0;
            let ctx = mainloop.get_context();
            ctx.invoke(move || {
                assert!(thread::current().name() != Some(THREAD_NAME));
                count += 1;
                if count < 2 {
                    Continue
                } else {
                    mlc.quit();
                    Remove
                }
            });
        }).unwrap();
    });
}
