use std::env;
use std::net;
use std::net::UdpSocket;
use std::thread;

use crate::models::error::EmpResult;
use crate::net::buffer::Buffer;
use crate::net::packet::get_packet;

fn listen(socket: &UdpSocket, mut buffer: &mut [u8]) -> EmpResult<(usize, net::SocketAddr)> {
    let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer)?;

    log::info!("{:?}", number_of_bytes);
    log::info!("{:?}", src_addr);

    Ok((number_of_bytes, src_addr))
}

fn send(socket: &UdpSocket, receiver: &str, msg: &Vec<u8>) -> EmpResult<usize> {
    log::info!("sending data");
    let result = socket.send_to(msg, receiver)?;

    Ok(result)
}

fn init_host(host: &str) -> UdpSocket {
    log::info!("initializing host");
    let socket = UdpSocket::bind(host).expect("failed to bind host socket");

    socket
}

pub fn start() -> EmpResult<()> {
    let socket = init_host("127.0.0.1:12345");
    let mut buf: Vec<u8> = Vec::with_capacity(4096);

    loop {
        let sock = socket.try_clone()?;
        match listen(&sock, &mut buf) {
            Ok((amt, src)) => {
                thread::spawn(move || {
                    log::info!("Handling connection from {}", &src);
                    let buf = &mut buf[..amt];
                    let packet =
                        get_packet(&mut Buffer::from(buf.to_vec())).expect("error getting packet");
                    log::info!("Received packet: {:?}", packet);
                    send(&sock, &src.to_string(), buf.into())
                });
            }
            Err(err) => {
                log::error!("Err: {}", err);
            }
        }
    }
}
