use std::io::Write;
use std::net::*;
use std::io;
use std::process;

type Utype = u32;

/// Some code adapted from [the official Rust docs][https://doc.rust-lang.org/std/io/struct.Stdin.html]
fn read_input() -> Result<String, io::Error> {
	// Allocate a new String
    let mut buffer = String::new();
	// Obtain Stdin object
    let stdin = io::stdin();
	// Read a line from Stdin (and return an error of failed to do so)
    match stdin.read_line(&mut buffer) {
		Ok(_) => {
			buffer.truncate(Utype::BITS as usize);
			Ok(buffer)
		},
		Err(error) => Err(error)
	}
}

fn main() {
	let mut stream = TcpStream::connect("localhost:1010").unwrap();

	let data_to_send = match read_input() {
		Ok(string) => {
			let trimmed_string = string.trim();
			trimmed_string.to_string()
		},
		Err(error) => {
			eprintln!("{}", error);
			process::exit(1)
		}
	};

	stream.write(&(data_to_send.len() as Utype).to_be_bytes()).unwrap();

	stream.write(data_to_send.as_bytes()).unwrap();

	loop {
		match stream.shutdown(Shutdown::Both) {
			Ok(_) => break,
			Err(_) => ()
		}
	}
}