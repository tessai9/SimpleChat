// connection module for P2P
use libp2p::{Multiaddr, Transport, tcp::TcpConfig, identity::Keypair};


#[derive(Debug, Clone)]
pub struct P2pNode {
    target_addr: Multiaddr,
}

impl P2pNode{
    fn new(target_addr: &str) -> Self {
        Self {
            target_addr: ,
        }
    }

    // fn make_connection(target_addr: &str) {
    //     let mut addr = format!("{}{}{}", "/ip4/", target_addr.to_string(), "/tcp/20500");
    //     let addr: Multiaddr = addr.parse().expect("invalid multiaddr");

    //     let tcp = TcpConfig::new();
    //     tcp.dial(addr);
    // }

    fn make_onnection(addr: Multiaddr) {
        let keypair = Keypair::generate_ed25519();
        let _transport = libp2p::build_development_transport();
        _transport.dial(addr)
    }
}
