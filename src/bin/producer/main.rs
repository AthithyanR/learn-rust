use std::{thread, time::{self, Duration, SystemTime}};
use chrono::{DateTime, Utc};
use kafka::producer::{ Producer, Record, RequiredAcks };
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    value: String,
}

static TEN_MILLIS: Duration = time::Duration::from_millis(10);

fn send_message(producer: Producer, message: Message) {
    producer.send(&Record::from_key_value(
        "my-topic",
        "same_key",
        serde_json::to_string(&message).unwrap())
    ).unwrap();
}

fn main() {
    println!("hello from producer");
    println!("creating kafka producer");

    let producer = Producer::from_hosts(vec!("localhost:9092".to_owned()))
    .with_ack_timeout(Duration::from_secs(1))
    .with_required_acks(RequiredAcks::One)
    .create()
    .unwrap();

    loop {
        let dt: DateTime<Utc> = SystemTime::now().into();
        let message = Message { value : dt.to_rfc3339() };

        send_message(producer, message);

        thread::sleep(TEN_MILLIS);
    }
}