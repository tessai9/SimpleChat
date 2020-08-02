// P2P Node Behavior
use libp2p::{
    Multiaddr,
    Transport,
    tcp::TcpConfig,
    identity::ed25519::Keypair
};


#[derive(Debug, Clone)]
pub struct P2pNode {
    target_addr: Multiaddr,
}

pub fn make_connection(target_addr: &str) {
    let mut addr = format!("{}{}{}", "/ip4/", target_addr.to_string(), "/tcp/20500");
    let addr: Multiaddr = addr.parse().expect("invalid multiaddr");

    let tcp = TcpConfig::new();
    tcp.dial(addr);
}

pub fn create_client(target_addr: &str) {
    let local_key = Keypair::generate();
    let public_key = local_key.public();
    let _transport = libp2p::build_development_transport(local_key).expect("failed to setup transport");

    let mut addr = format!("{}{}{}", "/ip4/", target_addr.to_string(), "/tcp/20500");
    let addr: Multiaddr = addr.parse().expect("invalid multiaddr");

    _transport.dial(addr);
    _transport.listen_on(addr);

    // create floodsub or gossipsub
}
