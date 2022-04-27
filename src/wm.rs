
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
use super::error::BoxResult;

pub struct WM<'a, C: Connection> {
    conn: &'a C,
    screen: &'a Screen,
    frame_gc: Gcontext,
    win_stack: Vec<Win>,
    running: bool,
}

struct Win {
    client: Window,
    frame: Window
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
            win_stack: Vec::new(),
            running: true
        })
    }

    pub fn become_wm(&self) -> Result<(), ReplyError> {

        // set root window mask
        let values_list = ChangeWindowAttributesAux::default()
            .event_mask(EventMask::SUBSTRUCTURE_REDIRECT|EventMask::SUBSTRUCTURE_NOTIFY|EventMask::KEY_PRESS);
        change_window_attributes(self.conn, self.screen.root, &values_list)?.check()?;

        Ok(())
    }

    pub fn render(&self) -> Result<(), ConnectionError> {
        self.draw_bar()?;
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        return self.running;
    }

    pub fn terminate(&mut self) {
        self.running = false;
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
                // block configure requests
                // let values_list = ConfigureWindowAux::from_configure_request(&event)
                //     .sibling(None)
                //     .stack_mode(None);
                // self.conn.configure_window(event.window, &values_list)?;
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

    fn handle_map_window(&mut self, event: &MapRequestEvent) -> BoxResult<()> {

        // create frame
        let frame_win = self.create_frame(event)?;

        // reparent
        self.win_stack.push(Win{
            client: event.window,
            frame: frame_win,
        });
        change_save_set(self.conn, SetMode::INSERT, event.window)?;
        reparent_window(self.conn, event.window, frame_win, 0, 0)?;

        self.conn.map_window(event.window)?;
        self.conn.map_window(frame_win)?;

        self.arrange_win()?;

        Ok(())
    }

    fn handle_unmap_window(&mut self, event: &UnmapNotifyEvent) -> BoxResult<()> {

        // TODO not sure why sometimes win not found
        let win = self.find_win_by_id(event.window);
        if win.is_none() { return Ok(()); }
        let win = win.unwrap();

        self.conn.unmap_window(win.frame)?;

        reparent_window(self.conn, event.window, self.screen.root, 0, 0)?;
        change_save_set(self.conn, SetMode::DELETE, event.window)?;
        self.conn.destroy_window(win.frame)?;

        self.remove_win_by_id(event.window);

        self.arrange_win()?;

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

    fn draw_bar(&self) -> Result<(), ConnectionError> {

        let rect = Rectangle {
            x: 0,
            y: 0,
            width: self.screen.width_in_pixels,
            height: config::BAR_HEIGHT,
        };
        self.conn.poly_fill_rectangle(self.screen.root, self.frame_gc, &[rect])?;

        Ok(())
    }

    // resizes windows to layout
    fn arrange_win(&self) -> Result<(), Box<dyn std::error::Error>> {

        // layout 1 (columns)
        let width = self.screen.width_in_pixels as u32 / self.win_stack.len() as u32;
        let height = (self.screen.height_in_pixels - config::BAR_HEIGHT) as u32;

        for i in 0..self.win_stack.len() {

            let win = &self.win_stack[i];

            let values_list = ConfigureWindowAux::new()
                .width(width)
                .height(height);
            self.conn.configure_window(win.client, &values_list)?;

            let values_list = ConfigureWindowAux::new()
                .x((i as u32 * width) as i32)
                .y(config::BAR_HEIGHT as i32)
                .width(width)
                .height(height);
            self.conn.configure_window(win.frame, &values_list)?;
        }
        Ok(())
    }

    fn find_win_by_id(&self, win: Window) -> Option<&Win> {
        self.win_stack.iter().find(|w| w.client == win)
    }

    fn remove_win_by_id(&mut self, win: Window) {
        let pos = self.win_stack.iter().position(|w| w.client == win).unwrap(); // assume window exists
        self.win_stack.remove(pos);
    }

}

