
use x11rb::connection::Connection;
use x11rb::protocol::{
    Event,
    xproto::*
};
use x11rb::errors::{ReplyOrIdError, ReplyError};

struct WM<'a, C: Connection> {
    conn: &'a C,
    screen: &'a Screen,
    gc_id: Gcontext,
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
            gc_id: gc_id 
        })
    }

    pub fn handle_event(&self, event: &Event) {
        match event {
            Event::Expose(event) => {}
            Event::CreateNotify(event) => {}
            Event::DestroyNotify(event) => {}
            _ => {}
        }
    }

    pub fn become_wm(&self) -> Result<(), ReplyError> {

        // set root window mask
        let values_list = ChangeWindowAttributesAux::default()
            .event_mask(EventMask::SUBSTRUCTURE_REDIRECT|EventMask::SUBSTRUCTURE_NOTIFY);
        change_window_attributes(self.conn, self.screen.root, &values_list)?.check()?;

        Ok(())
    }

}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (conn, screen_num) = x11rb::connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    let wm = WM::new(&conn, &screen)?;
    wm.become_wm()?;

    let mut running = true;
    while running {
        let event = conn.wait_for_event()?;
        wm.handle_event(&event);
    }


    drop(conn);
    Ok(())
}

