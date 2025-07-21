use std::error::Error;

pub struct UdpSocket {
	socket: std::net::UdpSocket,
	buffer: [u8; 10000],
}

impl UdpSocket {
	pub fn new(address: &str) -> Result<Self, Box<dyn std::error::Error>> {
		let socket = std::net::UdpSocket::bind(address)?;
		socket.set_nonblocking(true)?;
		let s = Self {
		    socket,
		    buffer: [0; 10000],
		};
		Ok(s)
	}
	pub fn receive_from(&mut self) -> Result<(&[u8], std::net::SocketAddr), Box<dyn Error>> {
		let (length, address) = self.socket.recv_from(&mut self.buffer)?;
	    Ok((&self.buffer[..length], address))
	}
	pub fn send_to(&self, buffer: &[u8], address: &std::net::SocketAddr) -> Result<(), Box<dyn Error>> {
		self.socket.send_to(buffer, address)?;
		Ok(())
	}
	pub fn connect(&self, address: &str) -> Result<(), std::io::Error> {
		return self.socket.connect(address);
	}
	pub fn recv(&mut self) -> Result<&[u8], Box<dyn std::error::Error>> {
		let length = self.socket.recv(&mut self.buffer)?;
	    Ok(&self.buffer[..length])
	}
	pub fn send(&self, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
		self.socket.send(bytes)?;
	    Ok(())
	}
}

