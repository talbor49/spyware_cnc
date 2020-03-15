mod server;
use server::run_server;

pub const BIND_PORT: u16 = 9393;

fn main() {
    // println!("{} {}", serial.serial_lower(), serial.serial_middle());
    // Blocking call
    run_server(BIND_PORT);
}
