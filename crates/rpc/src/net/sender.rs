use std::io;
use std::net::UdpSocket;

use tracing::info;

// https://gist.github.com/lanedraex/bc01eb399614359470cfacc9d95993fb

const BUFFER_SIZE: usize = 2048;
const STRING_CAPACITY: usize = 128;

fn listen(socket: &UdpSocket) -> Vec<u8> {
	let mut buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
	let mut result: Vec<u8> = Vec::new();
	match socket.recv_from(&mut buf) {
		Ok((number_of_bytes, _)) => {
			info!("received message: {:?}", buf);
			result = Vec::from(&buf[0..number_of_bytes]);
		}
		Err(fail) => info!("failed listening {:?}", fail),
	}

	let display_result = result.clone();
	let result_str = String::from_utf8(display_result).unwrap();
	info!("received message: {:?}", result_str);
	result
}

fn send(socket: &UdpSocket, receiver: &str, msg: &Vec<u8>) -> usize {
	info!("sending message: {:?}", msg);
	let result: usize = 0;
	match socket.send_to(msg, receiver) {
		Ok(number_of_bytes) => info!("{:?}", number_of_bytes),
		Err(fail) => info!("failed sending {:?}", fail),
	}

	result
}

fn init_host(host: &str) -> UdpSocket {
	info!("initializing host: {:?}", host);
	UdpSocket::bind(host).expect("failed to bind host socket")
}

fn show_menu(config: &HostConfig, message: &str) {
	info!(
		"Menu:\n\
    Local ip: -local {local_ip} \n\
    Local port: -lport {local_port} \n\
    Remote ip: -remote {remote_ip} \n\
    Remote port: -rport {remote_port} \n\
    Start host: -lstart \n\
    Connect to remote: -rconnect \n\
    Send message: -msg {message}",
		local_ip = config.local_ip,
		local_port = config.local_port,
		remote_ip = config.remote_ip,
		remote_port = config.remote_port,
		message = message
	);
}

#[derive(Debug, Default)]
struct HostConfig {
	local_ip: String,
	local_port: String,
	local_host: String,
	remote_ip: String,
	remote_port: String,
	remote_host: String,
}

#[derive(Debug)]
enum CommandInput {
	LocalIp(String),
	LocalPort(String),
	RemoteIp(String),
	RemotePort(String),
	StartHost,
	ConnectRemote,
	Message(String),
	Unknown(String),
	Error(String),
}

fn identify_command(command: &str, data: &str) -> CommandInput {
	match command {
		"-local" => CommandInput::LocalIp(data.to_owned()),
		"-lport" => CommandInput::LocalPort(data.to_owned()),
		"-remote" => CommandInput::RemoteIp(data.to_owned()),
		"-rport" => CommandInput::RemotePort(data.to_owned()),
		"-lstart" => CommandInput::StartHost,
		"-rconnect" => CommandInput::ConnectRemote,
		"-msg" => CommandInput::Message(data.to_owned()),
		_ => CommandInput::Unknown(data.to_owned()),
	}
}

fn read_console() -> CommandInput {
	let mut input = String::with_capacity(STRING_CAPACITY);
	match io::stdin().read_line(&mut input) {
		Ok(_) => {
			info!("read: {}", input);
			let mut split_input = input.split_whitespace();
			let cmd = split_input.next().unwrap();
			let data = split_input.collect::<String>();
			info!("cmd: {} ------ data: {}", cmd, data);
			identify_command(cmd, &data)
		}
		Err(fail) => {
			info!("Failed to read console: {}", fail);
			let invalid_data = "failed to read console".to_owned();
			CommandInput::Error(invalid_data)
		}
	}
}

fn set_host_parameters(ip: &str, port: &str) -> String {
	let mut host = String::with_capacity(STRING_CAPACITY);
	host.push_str(ip);
	host.push(':');
	host.push_str(port);

	host
}

fn build_config(cmd_input: CommandInput, host_config: &mut HostConfig) {
	info!("build: {:?}", cmd_input);
	match cmd_input {
		CommandInput::LocalIp(ip) => {
			host_config.local_ip = ip;
			host_config.local_host =
				set_host_parameters(&host_config.local_ip, &host_config.local_port);
		}
		CommandInput::LocalPort(port) => {
			host_config.local_port = port;
			host_config.local_host =
				set_host_parameters(&host_config.local_ip, &host_config.local_port);
		}
		CommandInput::RemoteIp(ip) => {
			host_config.remote_ip = ip;
			host_config.remote_host =
				set_host_parameters(&host_config.remote_ip, &host_config.remote_port);
		}
		CommandInput::RemotePort(port) => {
			host_config.remote_port = port;
			host_config.remote_host =
				set_host_parameters(&host_config.remote_ip, &host_config.remote_port);
		}
		_ => info!("Not a configuration."),
	}
}

fn main() {
	let mut host_config = HostConfig {
		local_ip: "127.0.0.1".to_owned(),
		local_port: "40404".to_owned(),
		local_host: String::with_capacity(STRING_CAPACITY),
		remote_ip: "127.0.0.1".to_owned(),
		remote_port: "12345".to_owned(),
		remote_host: String::with_capacity(STRING_CAPACITY),
	};

	host_config.local_host = set_host_parameters(&host_config.local_ip, &host_config.local_port);
	host_config.remote_host = set_host_parameters(&host_config.remote_ip, &host_config.remote_port);

	/*let mut message = String::with_capacity(STRING_CAPACITY);
	let default_msg = "hello world";

	loop {
		show_menu(&host_config, &default_msg);
		match read_console() {
			CommandInput::start_host => {
				info!("starting host");
				break;
			}
			CommandInput::connect_remote => info!("connecting to remote host"),
			CommandInput::message(msg) => {
				message = msg;
			}
			CommandInput::unknown(unknown_data) => info!("unknown_data: {:?}", unknown_data),
			CommandInput::error(fail) => info!("error: {:?}", fail),
			input_cmd @ _ => build_config(input_cmd, &mut host_config),
		}
	}*/

	let socket: UdpSocket = init_host(&host_config.local_host);
	let msg_bytes = vec![
		0, 5, 0, 108, 111, 114, 101, 109, 5, 0, 105, 112, 115, 117, 109,
	];

	send(&socket, &host_config.remote_host, &msg_bytes);
	loop {
		let received_msg = listen(&socket);
		send(&socket, &host_config.remote_host, &msg_bytes);
	}
}
