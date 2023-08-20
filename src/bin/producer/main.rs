// use chrono::{DateTime, Utc};
use kafka::producer::{Producer, Record, RequiredAcks};
use serde::Serialize;
use std::{
    thread,
    time::{self, Duration, /* SystemTime */},
};

#[derive(Serialize)]
struct Message {
    value: String,
}

static THOUSAND_MILLIS: Duration = time::Duration::from_millis(1000);

fn send_message(producer: &mut Producer, messages: &Vec<Message>) {
    let records: Vec<Record<&str, String>> = messages
        .iter()
        .map(|message| {
            Record::from_key_value(
                "my-topic",
                "same_key",
                serde_json::to_string(message).unwrap(),
            )
        })
        .collect();

    producer.send_all(&records).unwrap();
}

fn main() {
    println!("hello from producer");
    println!("creating kafka producer");

    let mut totally_sent = 0;

    let mut producer = Producer::from_hosts(vec!["localhost:9092".to_owned()])
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    loop {
        // let dt: DateTime<Utc> = SystemTime::now().into();
        // dt.to_rfc3339()
        let messages = (1..=1)
            .map(|x| Message {
                value: format!("hello there {}", x),
            })
            .collect();

        send_message(&mut producer, &messages);

        totally_sent += messages.len();
        println!("{}", totally_sent);

        thread::sleep(THOUSAND_MILLIS);
    }
}
