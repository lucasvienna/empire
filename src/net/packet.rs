use crate::models::error::{EmpError, EmpResult, ErrorKind};
use crate::net::buffer;
use crate::net::buffer::Buffer;

#[derive(Debug, PartialEq)]
pub enum Packet {
    Login { username: String, password: String },
    Logout(String),
    Chat(String),
    Build(i32),
    Upgrade(i32),
    Destroy(i32),
    Cancel(i32),
    Error { message: String },
}

fn get_packet_type(packet: &Packet) -> u8 {
    match packet {
        Packet::Login { .. } => 0,
        Packet::Logout { .. } => 1,
        Packet::Chat { .. } => 2,
        Packet::Build { .. } => 3,
        Packet::Upgrade { .. } => 4,
        Packet::Destroy { .. } => 5,
        Packet::Cancel { .. } => 6,
        Packet::Error { .. } => 7,
    }
}

fn get_packet_by_bit(bit: u8) -> EmpResult<Packet> {
    match bit {
        0 => Ok(Packet::Login {
            username: String::new(),
            password: String::new(),
        }),
        1 => Ok(Packet::Logout(String::new())),
        2 => Ok(Packet::Chat(String::new())),
        3 => Ok(Packet::Build(0)),
        4 => Ok(Packet::Upgrade(0)),
        5 => Ok(Packet::Destroy(0)),
        6 => Ok(Packet::Cancel(0)),
        7 => Ok(Packet::Error {
            message: String::new(),
        }),
        _ => Err(EmpError::from((
            ErrorKind::InvalidPacket,
            "Invalid packet type",
        ))),
    }
}

pub fn get_packet(buffer: &mut Buffer) -> EmpResult<Packet> {
    let byte = buffer::read_byte(buffer)?;
    let packet = get_packet_by_bit(byte)?;
    match packet {
        Packet::Login { .. } => {
            let username = buffer::read_string(buffer).unwrap();
            let password = buffer::read_string(buffer).unwrap();
            Ok(Packet::Login { username, password })
        }
        Packet::Logout { .. } => {
            let token = buffer::read_string(buffer).unwrap();
            Ok(Packet::Logout(token))
        }
        Packet::Chat { .. } => {
            let message = buffer::read_string(buffer).unwrap();
            Ok(Packet::Chat(message))
        }
        Packet::Build { .. } => {
            let building = buffer::read_integer(buffer).unwrap() as i32;
            Ok(Packet::Build(building))
        }
        Packet::Upgrade { .. } => {
            let building = buffer::read_integer(buffer).unwrap() as i32;
            Ok(Packet::Upgrade(building))
        }
        Packet::Destroy { .. } => {
            let building = buffer::read_integer(buffer).unwrap() as i32;
            Ok(Packet::Destroy(building))
        }
        Packet::Cancel { .. } => {
            let building = buffer::read_integer(buffer).unwrap() as i32;
            Ok(Packet::Cancel(building))
        }
        Packet::Error { .. } => {
            let message = buffer::read_string(buffer).unwrap();
            Ok(Packet::Error { message })
        }
    }
}

pub fn set_packet(buffer: &mut Buffer, p: &Packet) {
    let ptype = get_packet_type(p);
    buffer::write_byte(buffer, &ptype).unwrap();
    match p {
        Packet::Login { username, password } => {
            buffer::write_string(buffer, username).unwrap();
            buffer::write_string(buffer, password).unwrap();
        }
        Packet::Logout(token) => {
            buffer::write_string(buffer, token).unwrap();
        }
        Packet::Chat(msg) => {
            buffer::write_string(buffer, msg).unwrap();
        }
        Packet::Build(id) => {
            buffer::write_integer(buffer, &(*id as u32)).unwrap();
        }
        Packet::Upgrade(id) => {
            buffer::write_integer(buffer, &(*id as u32)).unwrap();
        }
        Packet::Destroy(id) => {
            buffer::write_integer(buffer, &(*id as u32)).unwrap();
        }
        Packet::Cancel(id) => {
            buffer::write_integer(buffer, &(*id as u32)).unwrap();
        }
        Packet::Error { message } => {
            buffer::write_string(buffer, message).unwrap();
        }
    }
}

#[test]
fn test_write_packet() {
    let mut buffer = Buffer::new();
    let packet = Packet::Login {
        username: "lorem".to_string(),
        password: "ipsum".to_string(),
    };
    set_packet(&mut buffer, &packet);

    let buffer_len = 1 + 2 /* length flag */ + "lorem".len() + 2 /* length flag */ + "ipsum".len();
    assert_eq!(buffer.get_size(), buffer_len);
    assert_eq!(buffer.get_write_data().len(), buffer_len);
}

#[test]
fn test_read_packet() {
    let mut buffer = Buffer::new();
    let packet = Packet::Login {
        username: "lorem".to_string(),
        password: "ipsum".to_string(),
    };
    set_packet(&mut buffer, &packet);

    let read_packet = get_packet(&mut buffer).unwrap();
    assert_eq!(packet, read_packet);
}
