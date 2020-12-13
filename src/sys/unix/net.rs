use std::{io, net::SocketAddr};

/// 封装平台相关的 socket API.
/// https://man7.org/linux/man-pages/man2/socket.2.html
pub(crate) fn new_ip_socket(addr: SocketAddr, socket_type: libc::c_int) -> io::Result<libc::c_int> {
    let address_family = match addr {
        SocketAddr::V4(..) => libc::AF_INET,
        SocketAddr::V6(..) => libc::AF_INET6
    };
    new_socket(address_family, socket_type)
}

/// 创建不阻塞的socket
/// The protocol specifies a particular protocol to be used with the
/// socket.  Normally only a single protocol exists to support a
/// particular socket type within a given protocol family, in which case
/// protocol can be specified as 0.
fn new_socket(af: libc::c_int, socket_type: libc::c_int) -> io::Result<libc::c_int>{
    let socket_type = socket_type | libc::SOCK_NONBLOCK | libc::SOCK_CLOEXEC;
    let socket = syscall!(socket(af, socket_type, 0));
    socket
}