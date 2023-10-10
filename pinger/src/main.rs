use bytes::Bytes;
use std::time::Duration;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use tokio::time;

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("pinger", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 20);
    client.subscribe("hello/rumqtt/pong", QoS::AtMostOnce).await.unwrap();

    client.publish_bytes("hello/rumqtt/ping", QoS::AtLeastOnce, false, Bytes::from("Ping")).await.unwrap();

    loop {
        let notification = eventloop.poll().await.unwrap();
        match notification {
            rumqttc::Event::Incoming(rumqttc::Incoming::Publish(p)) => {
                println!("Received = {:?}", p.payload);
                println!("Sending Ping");
                time::sleep(Duration::from_secs(1)).await;
                client.publish_bytes("hello/rumqtt/ping", QoS::AtLeastOnce, false, Bytes::from("Ping")).await.unwrap();
            }
            _ => {}
        }
    }
}