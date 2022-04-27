
use x11rb::connection::Connection;
use x11rb::protocol::{
    xproto::{
        KeyPressEvent 
    }
};
use std::process::Command;

use super::wm::WM;

pub fn handle_keypress<C: Connection>(wm: &mut WM<C>, event: &KeyPressEvent) -> Result<(), Box<dyn std::error::Error>> {
    
    println!("{:?}, {:?}", event.state, event.detail);

    // TODO hardcoded keysyms - use xmodmap later
    match event.detail {
        24 => { // q
            wm.terminate();
        }
        57 => { // n
            Command::new("st").spawn()?;
        }
        _ => {}
    }
    Ok(())
}

