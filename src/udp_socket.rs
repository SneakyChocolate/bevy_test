use std::error::Error;

pub struct UdpSocket {
	socket: std::net::UdpSocket,
	buffer: [u8; 10000],
}

impl UdpSocket {
	pub fn receive_from(&mut self) -> Result<&[u8], Box<dyn Error>> {
		let (length, address) = self.socket.recv_from(&mut self.buffer)?;
	    Ok(&self.buffer[..length])
	}
}

