use std::{collections::HashMap, fs};

use rdkafka::{
    producer::{BaseProducer, BaseRecord},
    ClientConfig,
};

use crate::{
    input::Buffer,
    settings::{self},
};

use super::Output;

pub struct KafkaOutput {
    producer: BaseProducer,
    topic_name: String,
    partition_no: i32,
}

impl KafkaOutput {
    pub fn new() -> KafkaOutput {
        let settings = settings::get();

        // Read kakfa config to hashmap
        let kafka_config = fs::read_to_string(&settings.kafka_config_path).unwrap();
        let kafka_config: HashMap<String, String> = serde_json::from_str(&kafka_config).unwrap();

        let mut config = &mut ClientConfig::new();

        // build kafka producer with provided kakfa config
        for (key, val) in kafka_config {
            config = config.set(key, val);
        }

        // Build producer from config
        let producer: BaseProducer = config
            .set("bootstrap.servers", &settings.kafka_brokers)
            .create()
            .expect("Producer creation failed");

        KafkaOutput {
            producer,
            partition_no: settings.kafka_partition_no as i32,
            topic_name: settings.kafka_topic_name.clone(),
        }
    }
}

impl Output for KafkaOutput {
    fn write(&mut self, data: Buffer) {
        let payload = BaseRecord::to(&self.topic_name)
            .partition(self.partition_no)
            .key(&())
            .payload(&data);

        self.producer.send(payload).unwrap();

        self.producer.poll(None);
    }
}
