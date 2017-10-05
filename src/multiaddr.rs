use std::fmt;
use std::ops::Add;

use Segment;

/// A decoded multiaddr.
///
/// # Examples
///
/// This type can be converted from some standard library types via
/// `From<T> where Segment: From<T>`, e.g. from `Ipv4Addr`:
///
/// ```rust
/// use std::net::Ipv4Addr;
/// use maddr::{Segment, MultiAddr};
///
/// let addr = Ipv4Addr::new(1, 2, 3, 4);
/// let multiaddr = addr.into();
///
/// assert_eq!(MultiAddr::new(vec![Segment::IP4(addr)]), multiaddr);
/// ```
///
/// check the [segment trait implementations to see what types those
/// are](enum.Segment.html#implementations).
///
/// ---
///
/// You can construct more complicated `MultiAddr`'s via concatenation of
/// segments, for example creating a `MultiAddr` referring to tcp port 22 on
/// host 1.2.3.4
///
/// ```rust
/// use std::net::Ipv4Addr;
/// use maddr::{Segment, MultiAddr};
///
/// let addr = Ipv4Addr::new(1, 2, 3, 4);
/// let multiaddr = Segment::from(addr) + Segment::Tcp(22);
///
/// assert_eq!("/ip4/1.2.3.4/tcp/22", multiaddr.to_string());
/// ```
pub trait MultiAddr: fmt::Display + Eq + PartialEq + Clone {
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct S<T: Segment>(T);

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct M<T: MultiAddr, U: Segment>(T, U);

impl<T> MultiAddr for S<T> where T: Segment {
}

impl<T, U> MultiAddr for M<T, U> where T: MultiAddr, U: Segment {
}

impl<T> fmt::Display for S<T> where T: Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "/{}", T::name()));
        for datum in self.0.data() {
            try!(write!(f, "/{}", datum));
        }
        Ok(())
    }
}

impl<T, U> fmt::Display for M<T, U> where T: MultiAddr, U: Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.0));
        try!(write!(f, "{}", S(self.1.clone())));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use {MultiAddr, Segment};

    #[test]
    fn from_ip4() {
        assert_eq!(
            MultiAddr::new(vec![Segment::IP4(Ipv4Addr::new(1, 2, 3, 4))]),
            Ipv4Addr::new(1, 2, 3, 4).into());
    }

    #[test]
    fn add() {
        assert_eq!(
            MultiAddr::new(vec![
                Segment::IP4(Ipv4Addr::new(1, 2, 3, 4)),
                Segment::Tcp(22),
            ]),
            MultiAddr::from(Ipv4Addr::new(1, 2, 3, 4)) + Segment::Tcp(22));
    }
}
