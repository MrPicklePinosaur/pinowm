
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{KeyPressEvent, KeyButMask};
use std::process::Command;
use std::fmt;
use xmodmap::{KeyTable, KeySym, Modifier};

use super::config;
use super::wm::WM;
use super::error::BoxResult;

pub struct KeyHandler {
    pub keytable: KeyTable,
}

#[derive(Debug)]
pub enum Error {
}

impl std::error::Error for Error { }
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => Ok(())
        }
    }
}

