use std::{
	env::args,
	io::{Read, stdout, Write},
	net::TcpStream,
};

fn main() {
	// Get the server address from the command line.
	let server_addr = args().skip(1).next().expect("please give a server address to connect to");

	// Configure the TLS connector. For simplicity, turn off certificate and host name validation.
	let tls_connector =
		native_tls::TlsConnector::builder()
		.danger_accept_invalid_certs(true)
		.danger_accept_invalid_hostnames(true)
		.build()
		.unwrap();

	eprintln!("Connecting");
	let stream = TcpStream::connect(server_addr.as_str()).unwrap();

	eprintln!("Negotiating TLS");
	let mut tls_stream = tls_connector.connect(server_addr.as_str(), stream).unwrap();

	eprintln!("Sending HTTP request");
	tls_stream.write_all(format!("GET / HTTP/1.1\r\nHost: {server_addr}\r\nConnection: close\r\n\r\n").as_bytes()).unwrap();
	tls_stream.flush().unwrap();

	eprintln!("Reading HTTP response");
	let mut output = Vec::new();
	tls_stream.read_to_end(&mut output).unwrap();
	tls_stream.shutdown().unwrap();

	stdout().lock().write_all(&output).unwrap();
}
