use crate::sys::unix::net::{new_ip_socket, socket_addr};
use crate::sys::unix::SourceFd;
use crate::{event, Interest, Registry, Token};

use std::fmt;
use std::io;
use std::mem::size_of;
use std::net::{self, SocketAddr};
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

pub struct TcpListener {
    inner: net::TcpListener,
}

impl TcpListener {
    pub fn bind(addr: SocketAddr) -> io::Result<TcpListener> {
        new_ip_socket(addr, libc::SOCK_STREAM).and_then(|socket| {
            // Set SO_REUSEADDR (mirrors what libstd does).
            syscall!(setsockopt(
                socket,
                libc::SOL_SOCKET,
                libc::SO_REUSEADDR,
                &1 as *const libc::c_int as *const libc::c_void,
                size_of::<libc::c_int>() as libc::socklen_t,
            ))
            .and_then(|_| {
                let (raw_addr, raw_addr_length) = socket_addr(&addr);
                syscall!(bind(socket, raw_addr, raw_addr_length))
            })
            .and_then(|_| syscall!(listen(socket, 1024)))
            .map_err(|err| {
                // Close the socket if we hit an error, ignoring the error
                // from closing since we can't pass back two errors.
                let _ = unsafe { libc::close(socket) };
                err
            })
            .map(|_| TcpListener {
                inner: unsafe { net::TcpListener::from_raw_fd(socket) },
            })
        })
    }

    pub fn from_std(inner: net::TcpListener) -> TcpListener {
        TcpListener { inner }
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.inner.local_addr()
    }

    pub fn accept(&self) -> io::Result<(net::TcpStream, SocketAddr)> {
        self.inner.accept()
        // .and_then(|(stream, addr)| stream.set_nonblocking(true).map(|()| (stream, addr)))
        // .and_then(|(stream, addr)| )
    }

    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        self.inner.set_ttl(ttl)
    }

    pub fn ttl(&self) -> io::Result<u32> {
        self.inner.ttl()
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.inner.take_error()
    }
}

impl event::Source for TcpListener {
    fn register(
        &mut self,
        registry: &Registry,
        token: Token,
        interests: Interest,
    ) -> io::Result<()> {
        SourceFd(&self.as_raw_fd()).register(registry, token, interests)
    }

    fn reregister(
        &mut self,
        registry: &Registry,
        token: Token,
        interests: Interest,
    ) -> io::Result<()> {
        SourceFd(&self.as_raw_fd()).reregister(registry, token, interests)
    }

    fn deregister(&mut self, registry: &Registry) -> io::Result<()> {
        SourceFd(&self.as_raw_fd()).deregister(registry)
    }
}

impl fmt::Debug for TcpListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}

impl FromRawFd for TcpListener {
    unsafe fn from_raw_fd(fd: RawFd) -> TcpListener {
        TcpListener {
            inner: net::TcpListener::from_raw_fd(fd),
        }
    }
}

impl IntoRawFd for TcpListener {
    fn into_raw_fd(self) -> RawFd {
        self.inner.into_raw_fd()
    }
}

impl AsRawFd for TcpListener {
    fn as_raw_fd(&self) -> RawFd {
        self.inner.as_raw_fd()
    }
}
