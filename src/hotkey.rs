
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{KeyPressEvent, KeyButMask};
use std::process::Command;
use std::fmt;
use xmodmap::{KeyTable, KeySym};

use super::config;
use super::wm::WM;
use super::error::BoxResult;

pub struct KeyHandler {
    keytable: KeyTable,
}

#[derive(Debug)]
pub enum Error {
}

impl KeyHandler {

    pub fn new() -> BoxResult<Self> {
        let keytable = KeyTable::new()?;
        Ok(KeyHandler{keytable: keytable})
    }

    pub fn handle_keypress(
        &self,
        event: &KeyPressEvent
    ) -> Result<(), Box<dyn std::error::Error>> {

        // TODO current issue with xmodmap library (event state uses the entire state instead of just shift)
        let shift_pressed = event.state & u16::from(KeyButMask::SHIFT);

        let keysym = self.keytable.get_keysym(shift_pressed, event.detail);
        if keysym.is_err() { return Ok(()); }
        let keysym = keysym.unwrap();
        
        // these keys all use the mod key
        if event.state & u16::from(config::MOD_KEY) != 0 {
            match keysym {
                KeySym::KEY_Q => {
                    // TODO this is not a great way of killing the window manager
                    Command::new("killall").args(["xinit"]).spawn()?;
                }
                KeySym::KEY_n => {
                    Command::new("st").spawn()?;
                }
                _ => {}
            }
        }
        Ok(())
    }

}

impl std::error::Error for Error { }
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => Ok(())
        }
    }
}

