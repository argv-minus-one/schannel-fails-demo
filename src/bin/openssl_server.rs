use std::{
	env,
	fs,
	process::Command,
};

fn main() {
	// Get the listening address from the command line, if any.
	let listen_addr = env::args().skip(1).next();

	// Generate some certificates and keys, and write them to disk.
	let keys = test_cert_gen::gen_keys();

	fs::write("ca.pem", keys.client.ca.to_pem()).unwrap();
	fs::write("server.pem", keys.server.cert_and_key.to_pem_incorrect()).unwrap();

	// Prepare a command to run an OpenSSL test server.
	let mut server_cmd = Command::new("openssl");
	server_cmd.args([
			"s_server",
			"-debug",
			"-cert",
			"server.pem",
			"-key",
			"server.pem",
			"-www",
		]);
	
	if let Some(listen_addr) = listen_addr {
		server_cmd.args([
			"-accept",
			listen_addr.as_str(),
		]);
	}

	// Run the OpenSSL test server, and wait for it to terminate.
	server_cmd.status().unwrap();
}
