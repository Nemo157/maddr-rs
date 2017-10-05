use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use mhash::MultiHash;

/// The possible multiaddr segments.
///
/// # Examples
///
/// This type can be converted from some of the standard library types, via
/// `From`, e.g. from `Ipv4Addr`:
///
/// ```rust
/// use std::net::Ipv4Addr;
/// use maddr::Segment;
///
/// let addr = Ipv4Addr::new(1, 2, 3, 4);
/// let segment = addr.into();
///
/// assert_eq!(Segment::IP4(addr), segment);
/// ```
///
/// Look at the [implementations](#implementations) section below for more.
pub trait Segment: Eq + PartialEq + Clone {
    fn code() -> u64;
    fn name() -> &'static str;
    fn data<'a>(&'a self) -> Box<Iterator<Item=&'a fmt::Display> + 'a>;
}

macro_rules! segment {
    ($code:expr, $name:expr, { $(#[$doc:meta])* $ty:ident }) => {
        $(#[$doc])*
        #[derive(Eq, PartialEq, Clone)]
        pub struct $ty;
        impl Segment for $ty {
            fn code() -> u64 { $code }
            fn name() -> &'static str { $name }
            fn data<'a>(&'a self) -> Box<Iterator<Item=&'a fmt::Display> + 'a> {
                Box::new(None.into_iter())
            }
        }
    };

    ($code:expr, $name:expr, { $(#[$doc:meta])* $ty:ident { $($arg_name:ident : $arg_ty:path),* } }) => {
        $(#[$doc])*
        #[derive(Eq, PartialEq, Clone)]
        pub struct $ty { $( $arg_name: $arg_ty),* }
        impl Segment for $ty {
            fn code() -> u64 { $code }
            fn name() -> &'static str { $name }
            fn data<'a>(&'a self) -> Box<Iterator<Item=&'a fmt::Display> + 'a> {
                let v: Vec<&fmt::Display> = vec![$(&self.$arg_name),*];
                Box::new(v.into_iter())
            }
        }
    };
}

segment!(33, "dccp", {
    /// Datagram Congestion Control Protocol, a transport layer protocol.
    /// The argument is the port number.
    Dccp { port: u16 }
});

segment!(480, "http", {
    /// Hypertext Transfer Protocol, an application layer protocol.
    Http
});

segment!(443, "https", {
    /// Hypertext Transfer Protocol layered on top of Transport Layer Security,
    /// an application layer protocol.
    Https
});

segment!(4, "ip4", {
    /// Internet Protocol version 4, an internet layer protocol.
    IP4 { ip: Ipv4Addr }
});

segment!(41, "ip6", {
    /// Internet Protocol version 6, an internet layer protocol.
    IP6 { ip: Ipv6Addr }
});

segment!(421, "ipfs", {
    /// The InterPlanetary File System, an application layer protocol.
    Ipfs { hash: MultiHash }
});

segment!(132, "sctp", {
    /// Stream Control Transmission Protocol, a transport layer protocol.
    Sctp { port: u16 }
});

segment!(6, "tcp", {
    /// Transmission Control Protocol, a transport layer protocol.
    Tcp { port: u16 }
});

segment!(17, "udp", {
    /// User Datagram Protocol, a transport layer protocol.
    Udp { port: u16 }
});

segment!(301, "udt", {
    /// UDP-based Data Transfer Protocol, an application layer protocol.
    Udt
});

segment!(302, "utp", {
    /// Micro Transport Protocol, an application? layer protocol.
    Utp
});

impl From<Ipv4Addr> for IP4 {
    fn from(ip: Ipv4Addr) -> IP4 {
        IP4 { ip }
    }
}

impl From<Ipv6Addr> for IP6 {
    fn from(ip: Ipv6Addr) -> IP6 {
        IP6 { ip }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    use Segment;

    #[test]
    fn from_ip4() {
        assert_eq!(
            Segment::IP4(Ipv4Addr::new(1, 2, 3, 4)),
            Ipv4Addr::new(1, 2, 3, 4).into());
    }

    #[test]
    fn from_ip6() {
        assert_eq!(
            Segment::IP6(Ipv6Addr::new(0x2a02, 0x6b8, 0, 0, 0, 0, 0x11, 0x11)),
            Ipv6Addr::new(0x2a02, 0x6b8, 0, 0, 0, 0, 0x11, 0x11).into());
    }

    #[test]
    fn from_ip_ip4() {
        assert_eq!(
            Segment::IP4(Ipv4Addr::new(1, 2, 3, 4)),
            IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)).into());
    }

    #[test]
    fn from_ip_ip6() {
        assert_eq!(
            Segment::IP6(Ipv6Addr::new(0x2a02, 0x6b8, 0, 0, 0, 0, 0x11, 0x11)),
            IpAddr::V6(Ipv6Addr::new(0x2a02, 0x6b8, 0, 0, 0, 0, 0x11, 0x11)).into());
    }
}
