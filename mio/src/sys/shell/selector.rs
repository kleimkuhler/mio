use std::io;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, RawFd};
use std::time::Duration;

#[derive(Debug)]
pub struct Selector {}

pub type Event = usize;

pub type Events = Vec<Event>;

impl Selector {
    pub fn try_clone(&self) -> io::Result<Selector> {
        os_required!();
    }

    pub fn select(&self, _: &mut Events, _: Option<Duration>) -> io::Result<()> {
        os_required!();
    }
}

#[cfg(unix)]
cfg_any_os_util! {
    use crate::{Interest, Token};

    impl Selector {
        pub fn register(&self, _: RawFd, _: Token, _: Interest) -> io::Result<()> {
            os_required!();
        }

        pub fn reregister(&self, _: RawFd, _: Token, _: Interest) -> io::Result<()> {
            os_required!();
        }

        pub fn deregister(&self, _: RawFd) -> io::Result<()> {
            os_required!();
        }
    }
}

cfg_net! {
    impl Selector {
        #[cfg(debug_assertions)]
        pub fn id(&self) -> usize {
            os_required!();
        }
    }
}

#[cfg(unix)]
impl AsRawFd for Selector {
    fn as_raw_fd(&self) -> RawFd {
        os_required!()
    }
}

pub mod event {
    use crate::sys::Event;
    use crate::Token;
    use std::fmt;

    pub fn token(_: &Event) -> Token {
        os_required!();
    }

    pub fn is_readable(_: &Event) -> bool {
        os_required!();
    }

    pub fn is_writable(_: &Event) -> bool {
        os_required!();
    }

    pub fn is_error(_: &Event) -> bool {
        os_required!();
    }

    pub fn is_read_closed(_: &Event) -> bool {
        os_required!();
    }

    pub fn is_write_closed(_: &Event) -> bool {
        os_required!();
    }

    pub fn is_priority(_: &Event) -> bool {
        os_required!();
    }

    pub fn is_aio(_: &Event) -> bool {
        os_required!();
    }

    pub fn is_lio(_: &Event) -> bool {
        os_required!();
    }

    pub fn debug_details(_: &mut fmt::Formatter<'_>, _: &Event) -> fmt::Result {
        os_required!();
    }
}