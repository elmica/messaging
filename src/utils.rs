use rumqttc::{MqttOptions, Client, QoS};
use std::time::Duration;
use std::thread;

pub fn printers(){
    let mut mqttoptions = MqttOptions::new("rumqtt-sync", "localhost", 1883);

    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (mut client, mut connection) = Client::new(mqttoptions, 10);

    client.subscribe("hello/rumqtt", QoS::AtMostOnce).unwrap();

    thread::spawn(move || for i in 0..10 {
        client.publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize]).unwrap();
        thread::sleep(Duration::from_millis(100));
    });

    // Iterate to poll the eventloop for connection progress
    use rumqttc::Event;

    for (i, notification) in connection.iter().enumerate() {
        match notification {
            Ok(Event::Incoming(incoming)) => match incoming {
                rumqttc::Packet::Publish(publish) => {
                    println!(
                        "Received message on topic {}: {:?}",
                        publish.topic,
                        String::from_utf8_lossy(&publish.payload)
                    );
                }
                _ => {
                    println!("Incoming = {:?}", incoming);
                }
            },
            Ok(Event::Outgoing(outgoing)) => {
                println!("Outgoing = {:?}", outgoing);
            }
            Err(e) => {
                eprintln!("Error = {:?}", e);
            }
        }
    }

}
