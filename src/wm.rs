
use std::collections::HashMap;
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
    frame_gc: Gcontext,
    clients: HashMap<Window, Window>, // index frames by the window id
    running: bool,
}

pub enum WMError {
    
}

impl<'a, C: Connection> WM<'a, C> {

    pub fn new(conn: &'a C, screen: &'a Screen) -> Result<WM<'a, C>, ReplyOrIdError>{

        let frame_gc = conn.generate_id()?;
        let values_list = CreateGCAux::new()
            .foreground(screen.white_pixel)
            .background(screen.black_pixel);
        conn.create_gc(frame_gc, screen.root, &values_list)?;

        Ok(WM {
            conn: conn,
            screen: screen,
            frame_gc: frame_gc,
            clients: HashMap::new(),
            running: true
        })
    }

    pub fn handle_event(&mut self, event: &Event) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            Event::CreateNotify(event) => {
                // we don't need to do anything here
            }
            Event::DestroyNotify(event) => {
                // also don't need to do anything
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
            Event::UnmapNotify(event) => {
                self.handle_unmap_window(event)?;
            }
            Event::KeyPress(event) => {
                hotkey::handle_keypress(self, event)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_map_window(&mut self, event: &MapRequestEvent) -> Result<(), ReplyOrIdError> {

        // create frame
        let frame_win = self.create_frame(event)?;

        // reparent
        self.clients.insert(event.window, frame_win);
        change_save_set(self.conn, SetMode::INSERT, event.window)?;
        reparent_window(self.conn, event.window, frame_win, 0, 0)?;

        self.conn.map_window(event.window)?;
        self.conn.map_window(frame_win)?;

        Ok(())
    }

    fn handle_unmap_window(&mut self, event: &UnmapNotifyEvent) -> Result<(), ReplyOrIdError> {

        let frame_win = self.clients.get(&event.window).unwrap().clone(); // all windows (should?) have a frame

        self.conn.unmap_window(frame_win)?;

        reparent_window(self.conn, event.window, self.screen.root, 0, 0)?;
        change_save_set(self.conn, SetMode::DELETE, event.window)?;
        self.clients.remove(&event.window);

        self.conn.destroy_window(frame_win)?;

        Ok(())
    }

    fn create_frame(&self, event: &MapRequestEvent) -> Result<Window, ReplyOrIdError> {

        let win_geom = get_geometry(self.conn, event.window)?.reply()?;

        let frame_id = self.conn.generate_id()?;

        let values_list = CreateWindowAux::default()
            .border_pixel(self.screen.white_pixel)
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

    pub fn render(&self) -> Result<(), ConnectionError> {
        self.draw_bar()?;
        Ok(())
    }

    fn draw_bar(&self) -> Result<(), ConnectionError> {

        let rect = Rectangle {
            x: 0,
            y: 0,
            width: 600,
            height: 10,
        };
        self.conn.poly_fill_rectangle(self.screen.root, self.frame_gc, &[rect])?;

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

