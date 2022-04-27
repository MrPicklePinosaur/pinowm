
use x11rb::connection::Connection;
use pinowm::wm::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (conn, screen_num) = x11rb::connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    let mut wm = WM::new(&conn, &screen)?;
    wm.become_wm()?;

    println!("starting pinowm...");

    while wm.is_running() {
        let event = conn.wait_for_event()?;

        wm.handle_event(&event)?;
    }

    drop(conn);
    Ok(())
}

