
use x11rb::connection::Connection;
use x11rb::COPY_DEPTH_FROM_PARENT;
use x11rb::protocol::{
    Event,
    xproto::*
};
use x11rb::errors::{ReplyOrIdError, ReplyError, ConnectionError};

use super::hotkey;
use super::config;

pub struct WM<'a, C: Connection> {
    conn: &'a C,
    screen: &'a Screen,
    gc_id: Gcontext,
    running: bool,
}

impl<'a, C: Connection> WM<'a, C> {

    pub fn new(conn: &'a C, screen: &'a Screen) -> Result<WM<'a, C>, ReplyOrIdError>{

        let gc_id = conn.generate_id()?;
        let values_list = CreateGCAux::new()
            .foreground(screen.white_pixel)
            .background(screen.black_pixel);
        conn.create_gc(gc_id, screen.root, &values_list)?;

        Ok(WM {
            conn: conn,
            screen: screen,
            gc_id: gc_id,
            running: true
        })
    }

    pub fn handle_event(&mut self, event: &Event) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            Event::CreateNotify(event) => {
                // we don't need to do anything here
            }
            Event::DestroyNotify(event) => {
                println!("destroy event");
            }
            Event::ConfigureRequest(event) => {
                println!("configure request");

                let values_list = ConfigureWindowAux::from_configure_request(&event)
                    .sibling(None)
                    .stack_mode(None);
                self.conn.configure_window(event.window, &values_list)?;

            }
            Event::MapRequest(event) => {
                self.handle_map_window(event)?;
            }
            Event::KeyPress(event) => {
                hotkey::handle_keypress(self, event)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_map_window(&self, event: &MapRequestEvent) -> Result<(), ReplyOrIdError> {

        // create frame
        let frame_win_id = self.create_frame(event)?;

        // reparent
        change_save_set(self.conn, SetMode::INSERT, event.window)?;
        reparent_window(self.conn, event.window, frame_win_id, 0, 0)?;
        self.conn.map_window(event.window)?;
        self.conn.map_window(frame_win_id)?;

        Ok(())
    }

    fn create_frame(&self, event: &MapRequestEvent) -> Result<Window, ReplyOrIdError> {

        let win_geom = get_geometry(self.conn, event.window)?.reply()?;

        let frame_id = self.conn.generate_id()?;

        let values_list = CreateWindowAux::default()
            .background_pixel(self.screen.white_pixel)
            .event_mask(EventMask::SUBSTRUCTURE_REDIRECT|EventMask::SUBSTRUCTURE_NOTIFY);
        self.conn.create_window(
            COPY_DEPTH_FROM_PARENT,
            frame_id,
            self.screen.root,
            win_geom.x,
            win_geom.y,
            win_geom.width,
            win_geom.height,
            config::BORDER_WIDTH,
            WindowClass::INPUT_OUTPUT,
            self.screen.root_visual,
            &values_list,
        )?;

        Ok(frame_id)
    }

    pub fn draw_bar(&self) -> Result<(), ConnectionError> {

        let rect = Rectangle {
            x: 0,
            y: 0,
            width: 600,
            height: 10,
        };
        self.conn.poly_fill_rectangle(self.screen.root, self.gc_id, &[rect])?;

        Ok(())
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

