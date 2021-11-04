#![allow(unused_variables)]

use libc;
use libc::c_char;
use std::sync::Once;
use std::{mem, ptr};

use gdnative_core::core_types::*;
use gdnative_core::object::*;
use gdnative_core::private::get_api;
use gdnative_core::sys::GodotApi;
use gdnative_core::{ref_kind, sys, thread_access, GodotResult};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));
