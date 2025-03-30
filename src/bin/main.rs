 use messaging::mqtt_handler;

fn main() {
    let (to_main_tx, to_main_rx) = std::sync::mpsc::channel(); // Incoming messages from MQTT
    let (to_mqtt_tx, to_mqtt_rx) = std::sync::mpsc::channel(); // Outgoing messages to MQTT

    mqtt_handler(
        to_main_tx,
        to_mqtt_rx,
        "printers",
        "localhost",
        1883,
        "hello/rumqtt",
    );

    // Simulate sending a message to MQTT
    /* to_mqtt_tx
        .send(("hello/rumqtt".to_string(), b"Hello from main!".to_vec()))
        .unwrap(); */

    for msg in to_main_rx {
        println!("Got from MQTT: {}", msg);
    }}

