// P2P Node Behavior
use libp2p::{
    PeerId,
    NetworkBehaviour,
    Swarm,
    Transport,
    mdns::{Mdns, MdnsEvent},
    floodsub::{self, Floodsub, FloodsubEvent},
    swarm::NetworkBehaviourEventProcess,
    identity,
};

#[derive(NetworkBehaviour)]
pub struct NodeBehaviour {
    pub floodsub: Floodsub,
    mdns: Mdns,
    #[behaviour(ignore)]
    #[allow(dead_code)]
    pub keypair: identity::Keypair,
}

impl Default for NodeBehaviour {
    fn default() -> Self {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        NodeBehaviour {
            floodsub: Floodsub::new(local_peer_id.clone()),
            mdns: Mdns::new().expect("failed to build behaviour"),
            keypair: local_key,
        }
    }
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

pub fn create_connection(behaviour: NodeBehaviour) {
    let _transport = libp2p::build_development_transport(behaviour.keypair).expect("failed to build transport");
    // create floodsub topic
    let floodsub_topic = floodsub::Topic::new("chat-topic");
}

// create topic
pub fn create_client() {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    let _transport = libp2p::build_development_transport(local_key).expect("failed to build transport");

    // create floodsub topic
    let floodsub_topic = floodsub::Topic::new("chat-topic");


    // // create Swarm
    // let mut swarm = {
    //     let mdns = Mdns::new().expect("failed to build behaviour");
    //     let mut behaviour = NodeBehaviour {
    //         floodsub: Floodsub::new(local_peer_id.clone()),
    //         mdns,
    //     };

    //     behaviour.floodsub.subscribe(floodsub_topic.clone());
    //     Swarm::new(_transport, behaviour, local_peer_id)
    // };

    // let addr = format!("{}{}{}", "/ip4/", target_addr.to_string(), "/tcp/0");
    // Swarm::listen_on(&mut swarm, addr.parse().expect("invalid address")).expect("failed to listen");

    // swarm

    // // Kick it off
    // let mut listening = false;
    // task::block_on(future::poll_fn(move |cx: &mut Context<'_>| {
    //     loop {
    //         match stdin.try_poll_next_unpin(cx)? {
    //             Poll::Ready(Some(line)) => swarm.floodsub.publish(floodsub_topic.clone(), line.as_bytes()),
    //             Poll::Ready(None) => panic!("Stdin closed"),
    //             Poll::Pending => break
    //         }
    //     }
    //     loop {
    //         match swarm.poll_next_unpin(cx) {
    //             Poll::Ready(Some(event)) => println!("{:?}", event),
    //             Poll::Ready(None) => return Poll::Ready(Ok(())),
    //             Poll::Pending => {
    //                 if !listening {
    //                     for addr in Swarm::listeners(&swarm) {
    //                         println!("Listening on {:?}", addr);
    //                         listening = true;
    //                     }
    //                 }
    //                 break
    //             }
    //         }
    //     }
    //     Poll::Pending
    // }))
}
