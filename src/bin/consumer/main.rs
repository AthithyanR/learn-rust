use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct Message {
    value: String,
}

fn main() {
    println!("hello from consumer");

    let mut consumer = Consumer::from_hosts(vec!["localhost:9092".to_owned()])
        .with_topic("my-topic".to_owned())
        .with_fallback_offset(FetchOffset::Earliest)
        .with_group("my-group".to_owned())
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .unwrap();

    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                let msg: Message = serde_json::from_slice(m.value).unwrap();
                println!("{:?}", msg)
            }
            consumer.consume_messageset(ms);
        }
        consumer.commit_consumed().unwrap();
    }
}
