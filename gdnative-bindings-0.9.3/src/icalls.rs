#![allow(unused_variables)]

use libc;
use std::ptr;

use gdnative_core::core_types::*;
use gdnative_core::private::get_api;
use gdnative_core::{sys, *};

include!(concat!(env!("OUT_DIR"), "/icalls.rs"));
