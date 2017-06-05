#![feature(use_extern_macros)]

extern crate serde;

#[macro_export]
extern crate snapshot_mbe_macro;
extern crate snapshot_proc_macro;

pub use snapshot_mbe_macro::{snap, test_metadata, __test_function_name};
pub use snapshot_proc_macro::*;

use std::convert::AsRef;
use std::fmt::Debug;
use std::path::{Path, PathBuf};

/*
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
pub trait Snapable {
    fn check_snapshot(&self, md: Metadata) -> Result<(), ()>;
    fn write_snapshot(&self, md: Metadata) -> Result<(), ()>;
    fn update_snapshot(&self, md: Metadata) -> Result<(), ()>;
}

impl<'de, T> Snapable for T
    where T: Debug + Deserialize<'de> + Serialize
{
    fn check_snapshot(&self, md: Metadata) -> Result<(), ()> {
        unimplemented!();
    }

    fn write_snapshot(&self, md: Metadata) -> Result<(), ()> {
        unimplemented!();
    }

    fn update_snapshot(&self, md: Metadata) -> Result<(), ()> {
        unimplemented!();
    }
}
*/




#[derive(Debug)]
pub struct Metadata<'a, P>
    where P: AsRef<Path>
{
    pub name: &'a str,
    pub test_function: String,
    pub file: P,
    pub line: u32,
    pub column: u32,
    pub module_path: &'a str,
}

impl<'a, P> Metadata<'a, P> where P: AsRef<Path> {}
