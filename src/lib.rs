use rumqttc::{MqttOptions, Client, Connection, QoS, Event, Packet};
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::Duration;

pub fn mqtt_handler(
    outbound_tx: Sender<String>,          // To send incoming messages to app
    inbound_rx: Receiver<(String, Vec<u8>)>, // From app to publish messages: (topic, payload)
    client_id: &str,
    host: &str,
    port: u16,
    subscribe_topic: &str,
) {
    let mut mqttoptions = MqttOptions::new(client_id, host, port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (mut client, mut connection) = Client::new(mqttoptions, 10);
    client.subscribe(subscribe_topic, QoS::AtMostOnce).unwrap();

    thread::spawn(move || {
        loop {
            // Check for either MQTT event or a message to publish
            if let Some(Ok(notification)) = connection.iter().next() {

        match notification {
            Event::Incoming(Packet::Publish(publish)) => {
                let msg = String::from_utf8_lossy(&publish.payload).to_string();
                let _ = outbound_tx.send(msg);
            }
            _ => {}
        }
}

            // Check if there's a message to publish
            while let Ok((topic, payload)) = inbound_rx.try_recv() {
                if let Err(e) = client.publish(topic, QoS::AtLeastOnce, false, payload) {
                    eprintln!("Failed to publish: {}", e);
                }
            }

            thread::sleep(Duration::from_millis(50)); // avoid tight loop
        }
    });
}
