use std::fmt;

use {MultiAddr, Segment};
use multiaddr::{S, M};

#[cfg(test)]
mod tests {
    use std::net::{ Ipv4Addr, Ipv6Addr };

    use mhash::{ MultiHash, MultiHashVariant };

    use segment::{IP4, IP6, Ipfs};
    use { MultiAddr, Segment };

    #[test]
    fn ip4() {
        let addr = Ipv4Addr::new(1, 2, 3, 4);
        assert_eq!(IP4(addr).to_string(), "/ip4/1.2.3.4");
    }

    #[test]
    fn ip6() {
        let addr = Ipv6Addr::new(0x2a02, 0x6b8, 0, 0, 0, 0, 0x11, 0x11);
        assert_eq!(IP6(addr).to_string(), "/ip6/2a02:6b8::11:11");
    }

    #[test]
    fn ipfs() {
        let multihash = MultiHash::new(MultiHashVariant::Sha2_256, &[
            213, 46, 187, 137, 216, 91, 2, 162,
            132, 148, 130, 3, 166, 47, 242, 131,
            137, 197, 124, 159, 66, 190, 236, 78,
            194, 13, 183, 106, 104, 145, 28, 11,
        ]).unwrap();
        assert_eq!(
            Ipfs(multihash).to_string(),
            "/ipfs/QmcgpsyWgH8Y8ajJz1Cu72KnS5uo2Aa2LpzU7kinSupNKC");
    }

    #[test]
    fn ip4_and_ipfs() {
        let addr = Ipv4Addr::new(1, 2, 3, 4);
        let multihash = MultiHash::new(MultiHashVariant::Sha2_256, &[
            213, 46, 187, 137, 216, 91, 2, 162,
            132, 148, 130, 3, 166, 47, 242, 131,
            137, 197, 124, 159, 66, 190, 236, 78,
            194, 13, 183, 106, 104, 145, 28, 11,
        ]).unwrap();
        assert_eq!(
            M(Ip4(addr), Ipfs(multihash)).to_string(),
            "/ipfs/QmcgpsyWgH8Y8ajJz1Cu72KnS5uo2Aa2LpzU7kinSupNKC");
    }
}
