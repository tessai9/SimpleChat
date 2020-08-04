// P2P Node Behavior
use futures::{future, prelude::*};
use libp2p::{
    Multiaddr,
    PeerId,
    NetworkBehaviour,
    mdns::{Mdns, MdnsEvent},
    floodsub::{self, Floodsub, FloodsubEvent},
    swarm::NetworkBehaviourEventProcess,
    Transport,
    tcp::TcpConfig,
    identity::ed25519::Keypair
};

pub fn make_connection(target_addr: &str) {
    let mut addr = format!("{}{}{}", "/ip4/", target_addr.to_string(), "/tcp/20500");
    let addr: Multiaddr = addr.parse().expect("invalid multiaddr");

    let tcp = TcpConfig::new();
    tcp.dial(addr);
}

pub fn create_client(target_addr: &str) {
    let local_key = Keypair::generate();
    let public_key = local_key.public();
    let _transport = libp2p::build_development_transport(local_key)?;

    let mut addr = format!("{}{}{}", "/ip4/", target_addr.to_string(), "/tcp/20500");
    let addr: Multiaddr = addr.parse().expect("invalid multiaddr");

    _transport.dial(addr);
    _transport.listen_on(addr);

    // create floodsub topic
    let floodsub_topic = floodsub::Topic::new("sample-topic");

    // create custom network behaviour
    #[derive[NetworkBehaviour]]
    struct NodeBehaviour {
        floodsub: Floodsub,
        mdns: Mdns,
        #[behaviour(ignore)]
        #[allow(dead_code)]
        ignored_member: bool,
    }

    // network event for floodsub
    impl NetworkBehaviourEventProcess<FloodsubEvent> for NodeBehaviour {
        fn inject_event(&mut self, message: FloodsubEvent) {
            if let FloodsubEvent::Message(message) = message {
                &message.data
            }
        }
    }

    // network event for mDNS
    impl NetworkBehaviourEventProcess<MdnsEvent> for NodeBehaviour {
        fn inject_event(&mut self, event: MdnsEvent) {
            match event {
                MdnsEvent::Discovered(list) => {
                    for (peer, _) in list {
                        if !self.mdns.has_node(&peer) {
                            self.floodsub.add_node_to_partial_view(peer);
                        }
                    }
                }
                MdnsEvent::Expired(list) => {
                    for (peer, _) in list {
                        if !self.mdns.has_node(&peer) {
                            self.floodsub.remove_node_from_partial_view(&peer)
                        }
                    }
                }
            }
        }
    }

    // create DTH(Swarm)
}
