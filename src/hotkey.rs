
use x11rb::connection::Connection;
use x11rb::protocol::{
    xproto::{
        KeyPressEvent 
    }
};
use super::wm::WM;

pub fn handle_keypress<C: Connection>(wm: &mut WM<C>, event: &KeyPressEvent) {
    
    println!("{:?}, {:?}", event.state, event.detail);

    // TODO hardcoded keysyms - use xmodmap later
    match event.detail {
        24 => { // q
            wm.terminate();
        }
        57 => { // n

        }
        _ => {}
    }
}

