use crate::beslink::errors::BESLinkError;
use crate::beslink::message::{BesMessage, MessageTypes};
use crate::beslink::BES_SYNC;
use serialport::SerialPort;
use std::io::ErrorKind::TimedOut;
use std::io::{Read, Write};
use tracing::error;
use tracing::info;
use tracing::warn;

pub fn send_packet(serial_port: &mut Box<dyn SerialPort>, msg: BesMessage) -> std::io::Result<()> {
    let packet = msg.to_vec();
    return match serial_port.write_all(packet.as_slice()) {
        Ok(_) => {
            info!("Wrote {} bytes", packet.len());
            Ok(())
        }
        Err(e) => {
            error!("Writing to port raised {:?}", e);
            Err(e)
        }
    };
}

pub fn read_packet(serial_port: &mut Box<dyn SerialPort>) -> Result<BesMessage, BESLinkError> {
    //
    let mut packet: Vec<u8> = vec![];
    let mut packet_len: usize = 3; //Start expectations at the minimum
    let mut buffer: [u8; 1] = [0; 1];

    while packet.len() < packet_len {
        match serial_port.read(&mut buffer) {
            Ok(n) => {
                if n == 1 {
                    // Only grab if actual data
                    if !(packet.len() == 0 && buffer[0] != BES_SYNC) {
                        packet.push(buffer[0]);
                    }
                }
            }
            Err(e) => {
                if e.kind() != TimedOut {
                    println!("Error reading packet header {:?}", e);
                    return Err(BESLinkError::from(e));
                }
            }
        }
        if packet.len() == 3 && packet_len == 3 {
            //Check actual packet length
            packet_len = decode_packet_length(&packet) as usize;
        }
        //TODO timeout
    }
    return match validate_packet_checksum(&packet) {
        Ok(_) => Ok(BesMessage::from(packet)),
        Err(e) => Err(e),
    };
}
pub fn validate_packet_checksum(packet: &Vec<u8>) -> Result<(), BESLinkError> {
    let mut inner_packet = packet.clone();
    let _ = inner_packet.pop();
    let checksum = calculate_packet_checksum(&inner_packet);
    if checksum == packet[packet.len() - 1] {
        return Ok(());
    }
    let e = BESLinkError::BadChecksumError {
        failed_packet: packet.clone(),
        got: packet[packet.len() - 1],
        wanted: checksum,
    };
    warn!("Bad Checksum!! {:?}", e);
    // return Err(e);
    return Ok(());
}
pub fn calculate_packet_checksum(packet: &Vec<u8>) -> u8 {
    let target: u8 = 0xFF;
    let mut sum: u32 = 0;
    for b in packet {
        sum += *b as u32;
        sum = sum % 0xFF;
    }
    return target - (sum as u8) + 1;
}
fn decode_packet_length(packet: &Vec<u8>) -> u16 {
    if packet.len() < 3 {
        return 3; // fail safe
    }
    let packet_id1 = packet[1];
    let packet_id2 = packet[2];

    return match packet_id1.try_into() {
        Ok(type1) => match type1 {
            MessageTypes::Sync => 8,
            MessageTypes::StartProgrammer => 6,
            MessageTypes::ProgrammerRunning => 6,
            MessageTypes::ProgrammerInit => 11,
            MessageTypes::FlashCommand => {
                if packet_id2 == 2 {
                    return 9;
                } else if packet_id2 == 0x08 {
                    return 6;
                }
                return 22;
            }
            MessageTypes::EraseBurnStart => 6,
            MessageTypes::FlashBurnData => 8,
            MessageTypes::ProgrammerStart => 6,
        },
        Err(_) => {
            println!(
                "Unknown packet len 0x{:02X}/0x{:02X}",
                packet_id1, packet_id2
            );
            return 3;
        }
    };
}
