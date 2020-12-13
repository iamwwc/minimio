use std::{io, net::SocketAddr};

pub type TcpSocket = libc::c_int;

pub (crate) fn bind(socket: TcpSocket, addr: SocketAddr) -> io::Result<()> {
    
}