// P2P Node Behavior
use libp2p::{
    Multiaddr,
    PeerId,
    NetworkBehaviour,
    Swarm,
    mdns::{Mdns, MdnsEvent},
    floodsub::{self, Floodsub, FloodsubEvent},
    swarm::NetworkBehaviourEventProcess,
    identity,
};

#[derive(NetworkBehaviour)]
pub struct NodeBehaviour {
    pub floodsub: Floodsub,
    mdns: Mdns,
}

// network event for floodsub
impl NetworkBehaviourEventProcess<FloodsubEvent> for NodeBehaviour {
    fn inject_event(&mut self, message: FloodsubEvent) {
        if let FloodsubEvent::Message(message) = message {
            // received a message from p2p network
            // and display the message to the application
            // let received_text = String::from_utf8_lossy(&message.data);
            println!("Receiced: {:?} from {:?}", String::from_utf8_lossy(&message.data), message.source);
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

pub fn subscribe_swarm(to_dial: String) {
    let local_key = identity::Keypair::generate_ed25519();
    let _local_peer_id = PeerId::from(local_key.public());
    let _transport = libp2p::build_development_transport(local_key).expect("failed to build transport");
    let _floodsub_topic = floodsub::Topic::new("chat-topic");

    let mut swarm = {
        let mut behaviour = NodeBehaviour {
            floodsub: Floodsub::new(_local_peer_id.clone()),
            mdns: Mdns::new().expect("failed to build behaviour"),
        };
        behaviour.floodsub.subscribe(_floodsub_topic.clone());
        Swarm::new(_transport, behaviour, PeerId::from(_local_peer_id))
    };
    Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/0".parse().expect("Failed to parse")).expect("failed to listen");

    let addr: Multiaddr =  format!("/ip4/{}/tcp/24915", to_dial).parse().expect("Invalid IP");
    Swarm::dial_addr(&mut swarm, addr).expect("Failed to dial");
}
