
use x11rb::connection::Connection;
use x11rb::protocol::{
    Event,
    xproto::*
};
use x11rb::errors::{ReplyOrIdError, ReplyError};

use super::hotkey;

pub struct WM<'a, C: Connection> {
    conn: &'a C,
    screen: &'a Screen,
    gc_id: Gcontext,
    running: bool,
}

impl<'a, C: Connection> WM<'a, C> {

    pub fn new(conn: &'a C, screen: &'a Screen) -> Result<WM<'a, C>, ReplyOrIdError>{

        let gc_id = conn.generate_id()?;
        
        let gc_aux = CreateGCAux::new()
            .foreground(screen.black_pixel)
            .background(screen.white_pixel);
        conn.create_gc(gc_id, screen.root, &gc_aux)?;

        Ok(WM {
            conn: conn,
            screen: screen,
            gc_id: gc_id,
            running: true
        })
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Expose(event) => {
                println!("expose event");
            }
            Event::CreateNotify(event) => {
                println!("create event");
            }
            Event::DestroyNotify(event) => {
                println!("destroy event");
            }
            Event::KeyPress(event) => {
                hotkey::handle_keypress(self, event);
            }
            _ => {}
        }
    }

    pub fn become_wm(&self) -> Result<(), ReplyError> {

        // set root window mask
        let values_list = ChangeWindowAttributesAux::default()
            .event_mask(EventMask::SUBSTRUCTURE_REDIRECT|EventMask::SUBSTRUCTURE_NOTIFY|EventMask::KEY_PRESS);
        change_window_attributes(self.conn, self.screen.root, &values_list)?.check()?;

        Ok(())
    }

    pub fn is_running(&self) -> bool {
        return self.running;
    }

    pub fn terminate(&mut self) {
        self.running = false;
    }

}

