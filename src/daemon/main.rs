use std::net::*;
use std::io::Read;

type Utype = u32;

fn stream_handler(mut stream: TcpStream) {
	// We divide the Utype bits with 8 so that we turn them to bytes
	let mut size_buffer: [u8; Utype::BITS as usize/8] = [0; Utype::BITS as usize/8];
	stream.read_exact(&mut size_buffer).unwrap();

	let string_length = Utype::from_be_bytes(size_buffer);
	let mut byte_array: [u8; 1] = [0];
	let mut bytes_read: Utype = 0;
	let mut final_string = String::with_capacity(string_length as usize);

	while bytes_read < string_length {
		stream.read(&mut byte_array).unwrap();
		final_string.push(char::from_u32(byte_array[0] as u32).unwrap());
		bytes_read += 1;
	}

	println!("Received string with a length of {} bytes from IP address {} : {}", string_length, stream.peer_addr().unwrap().ip(), final_string);
}

fn main() {
    let tcp_listener = TcpListener::bind("localhost:1010").unwrap();

	println!("Listening at address {}", tcp_listener.local_addr().unwrap());

	for tcp_stream_attempt in tcp_listener.incoming() {
		match tcp_stream_attempt {
			Ok(tcp_stream) => {
				let peer_address = match tcp_stream.peer_addr() {
					Ok(peer_address) => peer_address.ip().to_string(),
					Err(_) => "UNKNOWN_IP".to_string()
				};
				println!("New connection from {}", peer_address);

				stream_handler(tcp_stream);

				println!("Connection from {} terminated", peer_address);
			},
			Err(error) => eprintln!("{}", error)
		}
	}
}