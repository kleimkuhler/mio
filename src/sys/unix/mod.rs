use std::io;

/// Helper macro to execute a system call that returns an `io::Result`.
//
// Macro must be defined before any modules that uses them.
macro_rules! syscall {
    ($fn: ident ( $($arg: expr),* $(,)* ) ) => {{
        let res = unsafe { libc::$fn($($arg, )*) };
        if res == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(res)
        }
    }};
}

mod net;

mod selector;
pub use self::selector::{event, Event, Events, Selector};

mod sourcefd;
pub use self::sourcefd::SourceFd;

mod tcp;
pub use self::tcp::{TcpListener, TcpStream};

mod udp;
pub use self::udp::UdpSocket;

mod uds;
pub use self::uds::{SocketAddr, UnixDatagram, UnixListener, UnixStream};

mod waker;
pub use self::waker::Waker;

// Both `kqueue` and `epoll` don't need to hold any user space state.
pub struct IoSourceState;

impl IoSourceState {
    pub fn new() -> IoSourceState {
        IoSourceState
    }

    pub fn do_io<T, F, R>(&mut self, f: F, io: &mut T) -> io::Result<R>
    where
        F: FnOnce(&mut T) -> io::Result<R>,
    {
        // We don't hold state, so we can just all the function and return.
        f(io)
    }
}
