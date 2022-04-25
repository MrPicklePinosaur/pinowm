
use x11rb::connection::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (conn, screen_num) = x11rb::connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    drop(conn);
    Ok(())
}
