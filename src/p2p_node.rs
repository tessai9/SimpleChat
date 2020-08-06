// P2P Node Behavior
use async_std::{task};
use futures::{future, prelude::*};
use libp2p::{
    Multiaddr,
    PeerId,
    NetworkBehaviour,
    Swarm,
    mdns::{Mdns, MdnsEvent},
    floodsub::{self, Floodsub, FloodsubEvent},
    swarm::NetworkBehaviourEventProcess,
    Transport,
    tcp::TcpConfig,
    identity::ed25519::Keypair
};
use std::{task::{Context, Poll}};

pub fn make_connection(target_addr: &str) {
    let mut addr = format!("{}{}{}", "/ip4/", target_addr.to_string(), "/tcp/20500");
    let addr: Multiaddr = addr.parse().expect("invalid multiaddr");

    let tcp = TcpConfig::new();
    tcp.dial(addr);
}

// create topic
pub fn create_client(target_addr: &str) {
    let local_key = Keypair::generate();
    let local_peer_id = PeerId::from(local_key.public());
    let _transport = libp2p::build_development_transport(local_key)?;

    // create floodsub topic
    let floodsub_topic = floodsub::Topic::new("sample-topic");

    // create custom network behaviour
    #[derive(NetworkBehaviour)]
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

    // create Swarm
    let mut swarm = {
        let mdns = Mdns::new()?;
        let mut behaviour = NodeBehaviour {
            floodsub: Floodsub::new(local_peer_id.clone()),
            mdns,
            ignored_member: false,
        };

        behaviour.floodsub.subscribe(floodsub_topic.clone());
        Swarm::new(_transport, behaviour, local_peer_id)
    };

    let addr = format!("{}{}{}", "/ip4/", target_addr.to_string(), "/tcp/20500");
    Swarm::linsten_on(&mut swarm, addr.parse()?)?;

    swarm
}

// publish message to topic
pub fn publish_message(swarm: Swarm, message: &str){
    // Kick it off
    let mut listening = false;
    task::block_on(future::poll_fn(move |cx: &mut Context<'_>| {
        // loop {
        //     match stdin.try_poll_next_unpin(cx)? {
        //         Poll::Ready(Some(message)) => swarm.floodsub.publish(floodsub_topic.clone(), message.as_bytes()),
        //         Poll::Ready(None) => panic!("Stdin closed"),
        //         Poll::Pending => break
        //     }
        // }
        loop {
            match swarm.poll_next_unpin(cx) {
                Poll::Ready(Some(event)) => println!("{:?}", event),
                Poll::Ready(None) => return Poll::Ready(Ok(())),
                Poll::Pending => {
                    if !listening {
                        for addr in Swarm::listeners(&swarm) {
                            println!("Listening on {:?}", addr);
                            listening = true;
                        }
                    }
                    break
                }
            }
        }
        Poll::Pending
    }))
}
