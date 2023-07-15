use std::{
    sync::{Arc, RwLock},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use digitalis_protobuf::{
    base::Timestamp, protobuf::MessageFull, Color, CubePrimitive, Pose, Quaternion, SceneEntity,
    SceneUpdate, Vector3,
};
use digitalis_server::{
    channel::{Channel, ChannelQueue},
    client::Client,
    core::{
        client::ClientMessage,
        common::{AdvertiseChannel, Id, MessageEncoding},
        server, Control, DigitalisResult,
    },
    server::{DigitalisServer, ServerOption},
};
use tokio::time;

#[derive(Debug, Default)]
struct Server {
    channels: RwLock<Vec<Option<(AdvertiseChannel, ChannelQueue)>>>,
}

impl Server {
    fn new() -> Self {
        Self::default()
    }

    fn add_protobuf_channel<T: MessageFull>(&self, topic: &str) -> ChannelQueue {
        let mut channels = self.channels.write().unwrap();
        let id = channels.len() as Id;
        let sender = Channel::start(id);

        channels.insert(
            0,
            Some((AdvertiseChannel::protobuf::<T>(id, topic), sender.clone())),
        );

        sender
    }

    async fn listen(self: &Arc<Self>, option: ServerOption) {
        let listener = option.tcp_connect().await.unwrap();

        let mut handles = vec![];

        let mut client_id = 0;
        while let Ok((stream, _)) = listener.accept().await {
            let server_clone = Arc::clone(self);
            let handle =
                tokio::spawn(
                    async move { server_clone.accept_connection(client_id, stream).await },
                );

            handles.push(handle);
            client_id += 1;
        }

        for handle in handles {
            handle.await.unwrap();
        }
    }
}

impl DigitalisServer for Server {
    fn on_open(&self, client: &Client) -> DigitalisResult<()> {
        client.nonblocking_send(
            server::ServerInfo {
                name: "digitalis-server-test".into(),
                supported_encodings: vec![MessageEncoding::Protobuf],
                ..Default::default()
            }
            .into_message()?,
        )?;

        client.nonblocking_send(
            server::Advertise {
                channels: self
                    .channels
                    .read()
                    .unwrap()
                    .iter()
                    .filter_map(|v| v.as_ref())
                    .map(|(channel, _)| channel.clone())
                    .collect(),
            }
            .into_message()?,
        )?;

        Ok(())
    }

    fn handle_message(&self, client: &Client, msg: ClientMessage) -> DigitalisResult<Control> {
        use ClientMessage::*;
        match msg {
            Subscribe(msg) => {
                let channels = self.channels.read().unwrap();
                for sub in msg.subscriptions {
                    if let Some(Some((_, channel_sender))) = channels.get(sub.channel_id as usize) {
                        channel_sender.subscribe(client.clone(), sub.id).unwrap()
                    }
                }
            }

            Unsubscribe(msg) => {
                let ids = Arc::new(msg.subscription_ids);

                for (_, sender) in self
                    .channels
                    .read()
                    .unwrap()
                    .iter()
                    .filter_map(|v| v.as_ref())
                {
                    sender.unsubscribe(client.id(), Arc::clone(&ids)).unwrap()
                }
            }

            Close => {
                for (_, sender) in self
                    .channels
                    .read()
                    .unwrap()
                    .iter()
                    .filter_map(|v| v.as_ref())
                {
                    sender.unsubscribe_all(client.id()).unwrap()
                }
                return Ok(Control::Exit);
            }

            _ => {
                log::warn!("unhandled message: {:?}", msg);
            }
        }

        Ok(Control::Continue)
    }
}

fn now() -> Timestamp {
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).unwrap();
    Timestamp {
        seconds: duration.as_secs() as i64,
        nanos: duration.subsec_nanos() as i32,
        ..Default::default()
    }
}

fn cube(t: &Timestamp) -> CubePrimitive {
    let angle = (t.seconds as f64 + t.nanos as f64 * 1e-9) * 0.5;

    CubePrimitive {
        size: Some(Vector3 {
            x: 1.,
            y: 1.,
            z: 1.,
            ..Default::default()
        })
        .into(),
        pose: Some(Pose {
            position: Some(Vector3 {
                x: 2.,
                y: 0.,
                z: 0.,
                ..Default::default()
            })
            .into(),
            orientation: Some(Quaternion {
                x: 0.,
                y: 0.,
                z: (angle * 0.5).sin(),
                w: (angle * 0.5).cos(),
                ..Default::default()
            })
            .into(),
            ..Default::default()
        })
        .into(),
        color: Some(Color {
            r: 0.6,
            g: 0.2,
            b: 1.,
            a: 1.,
            ..Default::default()
        })
        .into(),
        ..Default::default()
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let server = Arc::new(Server::new());

    let handle = {
        let server = Arc::clone(&server);
        let sender = server.add_protobuf_channel::<SceneUpdate>("example_msg");

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(50));

            loop {
                let now = now();
                let msg = SceneUpdate {
                    entities: vec![SceneEntity {
                        frame_id: "root".into(),
                        timestamp: Some(now.clone()).into(),
                        cubes: vec![cube(&now)],
                        ..Default::default()
                    }],
                    ..Default::default()
                };
                sender
                    .broadcast(msg, now.seconds as u64 * 1_000_000_000 + now.nanos as u64)
                    .unwrap();

                interval.tick().await;
            }
        })
    };

    server.listen(ServerOption::default()).await;
    handle.await.unwrap();
}
